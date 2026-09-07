//! UI05: exercise the shipped executables, persisted history, and actual HTTP transport.
use numbers_prototype_01::{store::Engine, values::*};
use rusqlite::{Connection, OpenFlags};
use serde_json::{json, Value};
use std::{
    fs,
    io::Write,
    net::TcpListener,
    path::Path,
    process::{Child, Command, Output, Stdio},
    thread,
    time::{Duration, Instant},
};

const DEMO: &str = env!("CARGO_BIN_EXE_numbers-demo");
const SERVER: &str = env!("CARGO_BIN_EXE_numbers-prototype-01");
fn success(output: Output) -> String {
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}
fn rows(path: &Path) -> Vec<Value> {
    let db = Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_ONLY).unwrap();
    let mut query = db
        .prepare("SELECT record_json FROM events ORDER BY sequence_index")
        .unwrap();
    query
        .query_map([], |row| row.get::<_, String>(0))
        .unwrap()
        .map(|row| serde_json::from_str(&row.unwrap()).unwrap())
        .collect()
}
fn config() -> Value {
    let mut value: Value = serde_json::from_str(include_str!("../demo-config.json")).unwrap();
    for key in ["database_path", "host", "port", "log_verbosity"] {
        value.as_object_mut().unwrap().remove(key);
    }
    value
}
fn client(url: &str, commands: &str) -> String {
    let mut process = Command::new(DEMO)
        .args(["live", "--url", url])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    process
        .stdin
        .take()
        .unwrap()
        .write_all(commands.as_bytes())
        .unwrap();
    success(process.wait_with_output().unwrap())
}

#[test]
fn guided_proves_both_outcomes_restarts_and_preserves_its_journal() {
    let directory = tempfile::tempdir().unwrap();
    let session = directory.path().join("session");
    let output = success(
        Command::new(DEMO)
            .args(["guided", "--auto", "--directory"])
            .arg(&session)
            .output()
            .unwrap(),
    );
    assert!(output.contains("Ownership: Owned by Bidder B"));
    assert!(output.contains("Ownership: Unowned"));
    assert_eq!(
        output.matches("Restart verified in a new process:").count(),
        2
    );
    let result: Value =
        serde_json::from_slice(&fs::read(session.join("result.json")).unwrap()).unwrap();
    assert_eq!(result["completed"], true);
    let state = &result["state"];
    assert_eq!(state["current_number"], 3);
    assert_eq!(state["last_sequence_index"], 42);
    assert_eq!(
        state["balances"],
        json!([
            {"bidder_id":"demo-1","available_rana":600,"reserved_rana":0},
            {"bidder_id":"demo-2","available_rana":538,"reserved_rana":0}
        ])
    );
    assert_eq!(state["protocol_held_rana"], 62);
    let records = rows(&session.join("history.sqlite3"));
    assert_eq!(records.len(), 42);
    let finalizations: Vec<_> = records
        .iter()
        .filter(|r| r["record_type"] == "FinalizationRecord")
        .collect();
    assert_eq!(finalizations[0]["payload_json"]["title_kind"], "winner");
    assert_eq!(finalizations[0]["payload_json"]["holder_id"], "demo-2");
    assert_eq!(finalizations[1]["payload_json"]["title_kind"], "PublicLand");
    assert!(finalizations[1]["payload_json"]["holder_id"].is_null());
    assert_eq!(
        records
            .iter()
            .filter(|r| r["record_type"] == "ExtensionEventRecord")
            .count(),
        2
    );
    let receipts: Vec<Value> = fs::read_dir(&session)
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| {
            p.file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .starts_with("restart-")
        })
        .map(|p| serde_json::from_slice(&fs::read(p).unwrap()).unwrap())
        .collect();
    let mut validated = vec![];
    for receipt in receipts {
        assert_eq!(receipt["separate_process"], true);
        assert_eq!(receipt["ordered_rows_identical"], true);
        assert_eq!(receipt["derived_state_identical"], true);
        validated.push(receipt["validated_through"].as_u64().unwrap());
    }
    validated.sort();
    assert_eq!(validated, [32, 41]);
    let before = fs::read(session.join("history.sqlite3")).unwrap();
    let inspection = success(
        Command::new(DEMO)
            .arg("inspect")
            .arg(&session)
            .output()
            .unwrap(),
    );
    assert!(inspection.contains("Reconstructed through: 42"));
    let refused = Command::new(DEMO)
        .args(["guided", "--auto", "--directory"])
        .arg(&session)
        .output()
        .unwrap();
    assert!(!refused.status.success());
    let wrong_clock = Command::new(DEMO)
        .args(["serve", "--session"])
        .arg(&session)
        .output()
        .unwrap();
    assert!(!wrong_clock.status.success());
    assert!(String::from_utf8_lossy(&wrong_clock.stderr).contains("Only a live session"));
    assert_eq!(fs::read(session.join("history.sqlite3")).unwrap(), before);
}

// RAII also stops the test server if an assertion fails.
struct Server(Child);
impl Drop for Server {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}
#[test]
fn live_terminal_uses_the_real_server_and_preserves_submitted_number_syntax() {
    let directory = tempfile::tempdir().unwrap();
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    drop(listener);
    let mut config = config();
    config["database_path"] = json!("history.sqlite3");
    config["host"] = json!("127.0.0.1");
    config["port"] = json!(address.port());
    config["log_verbosity"] = json!("quiet");
    let path = directory.path().join("config.json");
    fs::write(&path, config.to_string()).unwrap();
    let mut server = Server(
        Command::new(SERVER)
            .arg("--config")
            .arg(path)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .unwrap(),
    );
    let deadline = Instant::now() + Duration::from_secs(10);
    while std::net::TcpStream::connect(address).is_err() {
        assert!(
            server.0.try_wait().unwrap().is_none(),
            "Test server stopped before listening"
        );
        assert!(Instant::now() < deadline, "Test server startup timed out");
        thread::sleep(Duration::from_millis(20));
    }
    let amount = "10000000000000000000000000000000000000003";
    let output = client(
        &format!("http://{address}"),
        &format!(
            "bid A 5\nbid B 6\nbid A 6\nbid B 6.0\nbid A {amount}\nhistory\nrecord 10\nquit\n"
        ),
    );
    assert!(output.contains("Bidder A: 600 available · 0 reserved"));
    assert!(output.contains("Bidder B: 594 available · 6 reserved"));
    assert!(output.contains("Bid rejected:"));
    let records = rows(&directory.path().join("history.sqlite3"));
    let bids: Vec<_> = records
        .iter()
        .filter(|r| r["record_type"] == "BidRecord")
        .collect();
    assert_eq!(bids.len(), 5);
    assert_eq!(bids[0]["payload_json"]["validity"], "valid");
    assert_eq!(bids[1]["payload_json"]["validity"], "valid");
    assert_eq!(
        bids[2]["payload_json"]["rejection_reason"],
        "amount_below_required_increment"
    );
    for (bid, spelling) in [(bids[3], "6.0"), (bids[4], amount)] {
        assert_eq!(bid["payload_json"]["validity"], "invalid");
        let bytes = hex::decode(bid["payload_json"]["command_utf8_hex"].as_str().unwrap()).unwrap();
        let raw = String::from_utf8(bytes).unwrap();
        assert!(raw.contains(&format!("\"amount_rana\":{spelling}")));
    }
    assert_eq!(bids[4]["payload_json"]["amount_rana"].to_string(), amount);
}

#[test]
fn live_terminal_does_not_retry_when_the_response_is_unreadable() {
    let directory = tempfile::tempdir().unwrap();
    let mut engine = Engine::open(&directory.path().join("history.sqlite3"), config()).unwrap();
    let observed = engine.request("state", None, now(), 100, 0);
    let server = tiny_http::Server::http("127.0.0.1:0").unwrap();
    let url = format!("http://{}", server.server_addr());
    let transport = thread::spawn(move || {
        let request = server
            .recv_timeout(Duration::from_secs(10))
            .unwrap()
            .unwrap();
        assert_eq!(request.url(), "/state");
        request
            .respond(tiny_http::Response::from_string(observed.to_string()))
            .unwrap();
        let mut request = server
            .recv_timeout(Duration::from_secs(10))
            .unwrap()
            .unwrap();
        assert_eq!(request.url(), "/bid");
        let mut raw = String::new();
        request.as_reader().read_to_string(&mut raw).unwrap();
        assert_eq!(
            serde_json::from_str::<Value>(&raw).unwrap()["auction_number"],
            1
        );
        request
            .respond(tiny_http::Response::from_string("unreadable response"))
            .unwrap();
        assert!(server
            .recv_timeout(Duration::from_millis(500))
            .unwrap()
            .is_none());
    });
    let output = client(&url, "bid A 5\nquit\n");
    assert!(output.contains("no command was retried"));
    transport.join().unwrap();
}
