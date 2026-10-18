//! SE07 acceptance through actual HTTP, including time-driven evaluation on reads.
use numbers_prototype_01::{access::Access, http, store::Engine, values::now};
use serde_json::{json, Value};
use std::{
    io::{Read, Write},
    net::TcpStream,
    sync::{Arc, Barrier},
    thread,
    time::Duration,
};
const ORIGIN: &str = "https://numbers.example.test";
fn config() -> Value {
    let mut c: Value = serde_json::from_str(include_str!("../demo-config.json")).unwrap();
    for k in [
        "database_path",
        "host",
        "port",
        "log_verbosity",
        "allowed_origin",
        "session_label",
    ] {
        c.as_object_mut().unwrap().remove(k);
    }
    c
}
fn raw(
    address: &str,
    method: &str,
    path: &str,
    headers: &[(&str, &str)],
    body: &[u8],
) -> (u16, String) {
    let mut stream = TcpStream::connect(address).unwrap();
    stream
        .set_read_timeout(Some(Duration::from_secs(8)))
        .unwrap();
    write!(
        stream,
        "{method} {path} HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n"
    )
    .unwrap();
    if !headers.iter().any(|(k, _)| {
        k.eq_ignore_ascii_case("transfer-encoding") || k.eq_ignore_ascii_case("content-length")
    }) {
        write!(stream, "Content-Length: {}\r\n", body.len()).unwrap();
    }
    for (k, v) in headers {
        write!(stream, "{k}: {v}\r\n").unwrap();
    }
    stream.write_all(b"\r\n").unwrap();
    stream.write_all(body).unwrap();
    let mut response = String::new();
    stream.read_to_string(&mut response).unwrap();
    let (head, body) = response.split_once("\r\n\r\n").unwrap();
    (
        head.split_whitespace().nth(1).unwrap().parse().unwrap(),
        body.to_owned(),
    )
}
fn headers(principal: &str) -> Vec<(&str, &str)> {
    vec![
        ("X-Numbers-Principal", principal),
        ("Origin", ORIGIN),
        ("Content-Type", "application/json"),
    ]
}
fn request(address: &str, method: &str, path: &str, principal: &str, body: &[u8]) -> Value {
    serde_json::from_str(&raw(address, method, path, &headers(principal), body).1).unwrap()
}
fn serve(mut engine: Engine, count: usize) -> (String, thread::JoinHandle<Engine>) {
    let server = tiny_http::Server::http("127.0.0.1:0").unwrap();
    let address = server.server_addr().to_string();
    let worker = thread::spawn(move || {
        let access = Access::new(ORIGIN.into(), "http-tests".into()).unwrap();
        for _ in 0..count {
            let req = server
                .recv_timeout(Duration::from_secs(10))
                .unwrap()
                .expect("request");
            http::serve_request(req, &mut engine, &access).unwrap();
        }
        engine
    });
    (address, worker)
}
fn bid(id: &str, amount: u64) -> Vec<u8> {
    json!({"auction_number":1,"bidder_id":id,"amount_rana":amount})
        .to_string()
        .into_bytes()
}
#[test]
fn denied_requests_cannot_evaluate_an_overdue_auction() {
    let dir = tempfile::tempdir().unwrap();
    let mut engine = Engine::open(&dir.path().join("history.sqlite3"), config()).unwrap();
    let past = now() - 1_000_000;
    engine.request("state", None, past, 100, 0);
    engine.request("bid", Some(&bid("demo-1", 5)), past, 100, 0);
    let before = engine.rows().unwrap();
    let (address, worker) = serve(engine, 17);
    let settle = br#"{"auction_id":"auc_000000000001","outcome":"expired"}"#;
    type RejectionCase = (
        &'static str,
        &'static str,
        Vec<(&'static str, &'static str)>,
        Vec<u8>,
        u16,
        &'static str,
    );
    let cases: Vec<RejectionCase> = vec![
        (
            "GET",
            "/state",
            vec![],
            vec![],
            401,
            "authentication_required",
        ),
        (
            "GET",
            "/auction/history",
            headers("unknown"),
            vec![],
            401,
            "authentication_required",
        ),
        (
            "GET",
            "/state",
            vec![
                ("X-Numbers-Principal", "bidder-a"),
                ("X-Numbers-Principal", "facilitator"),
            ],
            vec![],
            401,
            "authentication_required",
        ),
        ("GET", "/", vec![], vec![], 401, "authentication_required"),
        (
            "GET",
            "/access",
            vec![],
            vec![],
            401,
            "authentication_required",
        ),
        (
            "POST",
            "/bid",
            headers("bidder-a"),
            bid("demo-2", 6),
            403,
            "forbidden",
        ),
        (
            "POST",
            "/bid",
            headers("facilitator"),
            bid("demo-1", 6),
            403,
            "forbidden",
        ),
        (
            "POST",
            "/demo/settlement",
            headers("bidder-a"),
            settle.to_vec(),
            403,
            "forbidden",
        ),
        (
            "POST",
            "/bid",
            vec![
                ("X-Numbers-Principal", "bidder-a"),
                ("Content-Type", "application/json"),
            ],
            bid("demo-1", 6),
            403,
            "forbidden",
        ),
        (
            "GET",
            "/state",
            vec![
                ("X-Numbers-Principal", "bidder-a"),
                ("Origin", "https://elsewhere.test"),
            ],
            vec![],
            403,
            "forbidden",
        ),
        (
            "GET",
            "/state",
            vec![
                ("X-Numbers-Principal", "bidder-a"),
                ("Sec-Fetch-Site", "same-site"),
            ],
            vec![],
            403,
            "forbidden",
        ),
        (
            "POST",
            "/bid",
            vec![
                ("X-Numbers-Principal", "bidder-a"),
                ("Origin", ORIGIN),
                ("Content-Type", "text/plain"),
            ],
            bid("demo-1", 6),
            415,
            "unsupported_media_type",
        ),
        (
            "POST",
            "/bid",
            vec![
                ("X-Numbers-Principal", "bidder-a"),
                ("Origin", ORIGIN),
                ("Content-Type", "application/json"),
                ("Content-Encoding", "gzip"),
            ],
            bid("demo-1", 6),
            415,
            "unsupported_media_type",
        ),
        (
            "POST",
            "/bid",
            headers("bidder-a"),
            vec![b' '; 4097],
            413,
            "request_too_large",
        ),
        (
            "POST",
            "/bid",
            headers("bidder-a"),
            br#"{"bidder_id":"demo-1","bidder_id":"demo-2"}"#.to_vec(),
            400,
            "malformed_request",
        ),
    ];
    for (method, path, h, body, status, code) in cases {
        let (got, text) = raw(&address, method, path, &h, &body);
        assert_eq!(got, status, "{text}");
        let value: Value = serde_json::from_str(&text).unwrap();
        assert_eq!(value["error"]["code"], code);
        assert_eq!(value["evaluation_records"], json!([]));
    }
    let metadata = request(&address, "GET", "/access", "facilitator", b"");
    assert_eq!(metadata["data"]["role"], "facilitator");
    assert!(metadata["data"]["bidder_id"].is_null());
    // Access metadata must not have closed the overdue auction.
    let state = request(&address, "GET", "/state", "bidder-b", b"");
    assert_eq!(state["data"]["auction_state"], "AwaitingSettlement");
    let engine = worker.join().unwrap();
    let after = engine.rows().unwrap();
    assert_eq!(after.len(), before.len() + 2);
    assert_eq!(&after[..before.len()], before.as_slice());
}
#[test]
fn exact_body_limit_chunking_and_binding_preserve_business_validation() {
    let dir = tempfile::tempdir().unwrap();
    let mut engine = Engine::open(&dir.path().join("history.sqlite3"), config()).unwrap();
    engine.request("state", None, now(), 100, 0);
    let (address, worker) = serve(engine, 6);
    let mut body = bid("demo-1", 5);
    body.resize(4096, b' ');
    let result = request(&address, "POST", "/bid", "bidder-a", &body);
    assert_eq!(result["data"]["accepted"], true);
    let mut oversize = body.clone();
    oversize.push(b' ');
    let chunks = format!(
        "{:x}\r\n{}\r\n0\r\n\r\n",
        oversize.len(),
        String::from_utf8(oversize).unwrap()
    );
    let mut h = headers("bidder-a");
    h.push(("Transfer-Encoding", "chunked"));
    assert_eq!(raw(&address, "POST", "/bid", &h, chunks.as_bytes()).0, 413);
    let missing = request(
        &address,
        "POST",
        "/bid",
        "bidder-a",
        br#"{"auction_number":1,"amount_rana":6}"#,
    );
    assert_eq!(
        missing["data"]["bid_record"]["payload_json"]["rejection_reason"],
        "missing_required_field"
    );
    let wrong = request(
        &address,
        "POST",
        "/bid",
        "bidder-a",
        br#"{"auction_number":1,"bidder_id":1,"amount_rana":6}"#,
    );
    assert_eq!(
        wrong["data"]["bid_record"]["payload_json"]["rejection_reason"],
        "malformed_field"
    );
    assert_eq!(
        raw(&address, "GET", "/bid", &headers("bidder-a"), b"").0,
        405
    );
    let result = request(&address, "GET", "/access", "bidder-b", b"");
    assert_eq!(result["data"]["bidder_id"], "demo-2");
    assert_eq!(worker.join().unwrap().rows().unwrap().len(), 8);
}
#[test]
fn competing_bidders_have_one_committed_order_and_shared_facts() {
    let dir = tempfile::tempdir().unwrap();
    let mut engine = Engine::open(&dir.path().join("history.sqlite3"), config()).unwrap();
    engine.request("state", None, now(), 100, 0);
    let (address, worker) = serve(engine, 4);
    let barrier = Arc::new(Barrier::new(2));
    let mut clients = vec![];
    for (principal, id) in [("bidder-a", "demo-1"), ("bidder-b", "demo-2")] {
        let address = address.clone();
        let barrier = barrier.clone();
        clients.push(thread::spawn(move || {
            barrier.wait();
            request(&address, "POST", "/bid", principal, &bid(id, 5))
        }));
    }
    let results: Vec<_> = clients.into_iter().map(|c| c.join().unwrap()).collect();
    assert_eq!(
        results
            .iter()
            .filter(|v| v["data"]["accepted"] == true)
            .count(),
        1
    );
    let mut a = request(&address, "GET", "/state", "bidder-a", b"");
    let mut b = request(&address, "GET", "/state", "bidder-b", b"");
    a["data"].as_object_mut().unwrap().remove("server_time");
    b["data"].as_object_mut().unwrap().remove("server_time");
    assert_eq!(a, b);
    let engine = worker.join().unwrap();
    assert_eq!(engine.rows().unwrap().len(), 7);
    assert_eq!(engine.state().phase(), "Open");
}
#[test]
fn facilitator_controls_both_outcomes_without_duplicate_terminal_effects() {
    for outcome in ["settled", "expired"] {
        let dir = tempfile::tempdir().unwrap();
        let mut engine = Engine::open(&dir.path().join("history.sqlite3"), config()).unwrap();
        let past = now() - 1_000_000;
        engine.request("state", None, past, 100, 0);
        engine.request("bid", Some(&bid("demo-1", 5)), past, 100, 0);
        let (address, worker) = serve(engine, 2);
        let body = json!({"auction_id":"auc_000000000001","outcome":outcome}).to_string();
        let result = request(
            &address,
            "POST",
            "/demo/settlement",
            "facilitator",
            body.as_bytes(),
        );
        assert_eq!(result["data"]["accepted"], true);
        assert_eq!(
            result["data"]["state"]["ownership"]["ownership_kind"],
            if outcome == "settled" {
                "winner"
            } else {
                "unowned"
            }
        );
        let duplicate = request(
            &address,
            "POST",
            "/demo/settlement",
            "facilitator",
            body.as_bytes(),
        );
        assert_eq!(duplicate["error"]["code"], "settlement_not_available");
        assert_eq!(worker.join().unwrap().rows().unwrap().len(), 11);
    }
}
