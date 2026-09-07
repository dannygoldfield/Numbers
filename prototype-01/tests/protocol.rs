//! Acceptance evidence for UI04, PR05, and the failure boundaries in PR06.
use numbers_prototype_01::{http::route, model::*, store::Engine, values::*};
use serde_json::{json, Value};
use std::path::{Path, PathBuf};
use tempfile::TempDir;

const BASE: i64 = 1_788_609_600_000;
fn config() -> Value {
    json!({"starting_number":1,"demo_1_initial_rana":100,"demo_2_initial_rana":100,"duration_seconds":225,"inter_auction_gap_seconds":12,"extension_window_seconds":10,"extension_increment_seconds":15,"max_extensions":2,"minimum_bid_rana":10,"minimum_increment_rana":10,"maximum_bid_rana":null})
}
struct Demo {
    _directory: TempDir,
    path: PathBuf,
    engine: Engine,
}
impl Demo {
    fn new() -> Self {
        Self::configured(config())
    }
    fn configured(config: Value) -> Self {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("history.sqlite3");
        let mut engine = Engine::open(&path, config).unwrap();
        assert_eq!(
            engine.request("state", None, BASE - 1000, 50, 0)["status"],
            "success"
        );
        Self {
            _directory: directory,
            path,
            engine,
        }
    }
    fn bid(&mut self, second: i64, identity: &str, amount: Value) -> Value {
        let body = json!({"auction_number":self.engine.state.number.as_ref().map(jnum),"bidder_id":identity,"amount_rana":amount}).to_string();
        self.engine
            .request("bid", Some(body.as_bytes()), BASE + second * 1000, 50, 0)
    }
    fn read(&mut self, second: i64) -> Value {
        self.engine
            .request("state", None, BASE + second * 1000, 50, 0)
    }
    fn settle(&mut self, second: i64, outcome: &str) -> Value {
        let body = json!({"auction_id":id("auc",self.engine.state.number.as_ref().unwrap()),"outcome":outcome}).to_string();
        self.engine.request(
            "settlement",
            Some(body.as_bytes()),
            BASE + second * 1000,
            50,
            0,
        )
    }
}
fn balances(state: &State) -> Value {
    json!([
        state
            .balances
            .iter()
            .map(|b| json!([jnum(&b[0]), jnum(&b[1])]))
            .collect::<Vec<_>>(),
        jnum(&state.protocol)
    ])
}
fn accepted(value: &Value) {
    assert_eq!(value["status"], "success", "{value}");
    assert_eq!(value["data"]["accepted"], true, "{value}");
}

#[test]
fn exact_worked_auction_has_31_records_and_both_ownership_outcomes() {
    let mut demo = Demo::new();
    assert_eq!(
        balances(&demo.engine.state),
        json!([[[100, 0], [100, 0]], 0])
    );
    accepted(&demo.bid(0, "demo-1", json!(20)));
    assert_eq!(
        balances(&demo.engine.state),
        json!([[[80, 20], [100, 0]], 0])
    );
    accepted(&demo.bid(30, "demo-2", json!(30)));
    assert_eq!(
        balances(&demo.engine.state),
        json!([[[100, 0], [70, 30]], 0])
    );
    let rejected = demo.bid(45, "demo-1", json!(30));
    assert_eq!(
        rejected["data"]["bid_record"]["payload_json"]["rejection_reason"],
        "amount_below_required_increment"
    );
    assert_eq!(demo.engine.state.records.len(), 10);
    accepted(&demo.bid(60, "demo-2", json!(40)));
    assert_eq!(
        balances(&demo.engine.state),
        json!([[[100, 0], [60, 40]], 0])
    );
    accepted(&demo.bid(90, "demo-1", json!(50)));
    assert_eq!(demo.engine.state.records.len(), 16);
    let resolved = demo.read(225);
    assert_eq!(resolved["data"]["auction_state"], "AwaitingSettlement");
    assert_eq!(
        resolved["data"]["resolution"]["winning_bidder_id"],
        "demo-1"
    );
    assert!(resolved["data"]["resolution"]
        .get("settlement_deadline")
        .is_none());
    assert_eq!(demo.engine.state.records.len(), 18);
    accepted(&demo.settle(226, "settled"));
    assert_eq!(
        balances(&demo.engine.state),
        json!([[[50, 0], [100, 0]], 50])
    );
    assert_eq!(
        demo.engine.state.ownership["payload_json"]["holder_id"],
        "demo-1"
    );
    assert_eq!(demo.read(237)["data"]["current_number"], 1);
    assert_eq!(demo.read(238)["data"]["current_number"], 2);
    assert_eq!(demo.engine.state.records.len(), 22);
    accepted(&demo.bid(240, "demo-2", json!(10)));
    demo.read(465);
    accepted(&demo.settle(466, "expired"));
    assert_eq!(
        balances(&demo.engine.state),
        json!([[[50, 0], [100, 0]], 50])
    );
    assert_eq!(
        demo.engine.state.ownership["payload_json"]["title_kind"],
        "PublicLand"
    );
    assert_eq!(
        demo.engine.state.resolution["payload_json"]["winning_bid_id"],
        "bid_000000000023"
    );
    assert_eq!(demo.read(478)["data"]["current_number"], 3);
    assert_eq!(demo.engine.state.records.len(), 31);
    // Every complete group is reconstructible; every partial group is rejected.
    let rows = demo.engine.rows().unwrap();
    for length in 0..=rows.len() {
        let complete = length == 0
            || demo.engine.state.records[length - 1]["group_index"]
                == demo.engine.state.records[length - 1]["group_size"];
        let result = reconstruct(&rows[..length], &config());
        assert_eq!(result.is_ok(), complete, "prefix length {length}");
    }
    let restored = reconstruct(&rows, &config()).unwrap();
    assert_eq!(
        projection(&restored, BASE + 478000, 0).unwrap(),
        projection(&demo.engine.state, BASE + 478000, 0).unwrap()
    );
}

#[test]
fn extension_window_equality_cap_and_close_precedence() {
    let mut d = Demo::new();
    accepted(&d.bid(0, "demo-1", json!(10)));
    accepted(&d.bid(214, "demo-2", json!(20)));
    assert_eq!(d.engine.state.end().unwrap(), BASE + 225000);
    accepted(&d.bid(215, "demo-1", json!(30)));
    assert_eq!(d.engine.state.end().unwrap(), BASE + 240000);
    accepted(&d.bid(230, "demo-2", json!(40)));
    assert_eq!(d.engine.state.end().unwrap(), BASE + 255000);
    accepted(&d.bid(245, "demo-1", json!(50)));
    assert_eq!(d.engine.state.end().unwrap(), BASE + 255000);
    let late = d.bid(255, "demo-2", json!(60));
    assert_eq!(late["error"]["code"], "auction_not_accepting_bids");
    assert_eq!(late["evaluation_records"].as_array().unwrap().len(), 2);
    assert_eq!(
        d.engine.state.resolution["payload_json"]["winning_amount_rana"],
        50
    );
}
#[test]
fn zero_extensions_and_invalid_opening_do_not_start_or_extend_time() {
    let mut c = config();
    c["max_extensions"] = json!(0);
    let mut d = Demo::configured(c);
    let rejection = d.bid(0, "demo-1", json!(9));
    assert_eq!(rejection["data"]["accepted"], false);
    assert_eq!(d.read(10000)["data"]["auction_state"], "Scheduled");
    accepted(&d.bid(10001, "demo-1", json!(10)));
    accepted(&d.bid(10220, "demo-2", json!(20)));
    assert_eq!(d.engine.state.extensions, 0);
    assert_eq!(d.engine.state.end().unwrap(), BASE + 10226000);
}
#[test]
fn same_leader_can_use_its_hold_but_cannot_overdraw() {
    let mut d = Demo::new();
    accepted(&d.bid(0, "demo-1", json!(60)));
    accepted(&d.bid(1, "demo-1", json!(100)));
    assert_eq!(balances(&d.engine.state), json!([[[0, 100], [100, 0]], 0]));
    let r = d.bid(2, "demo-1", json!(110));
    assert_eq!(
        r["data"]["bid_record"]["payload_json"]["rejection_reason"],
        "insufficient_available_rana"
    );
    assert_eq!(balances(&d.engine.state), json!([[[0, 100], [100, 0]], 0]));
}
#[test]
fn arbitrarily_large_amounts_remain_exact_through_capture_and_restart() {
    let huge =
        serde_json::from_str::<Value>("100000000000000000000000000000000000000000000000000003")
            .unwrap();
    let mut c = config();
    c["demo_1_initial_rana"] = huge.clone();
    let mut d = Demo::configured(c.clone());
    accepted(&d.bid(0, "demo-1", huge.clone()));
    d.read(225);
    accepted(&d.settle(226, "settled"));
    assert_eq!(jnum(&d.engine.state.protocol), huge);
    let rebuilt = reconstruct(&d.engine.rows().unwrap(), &c).unwrap();
    assert_eq!(jnum(&rebuilt.protocol), huge);
}
#[test]
fn transport_rejections_do_not_evaluate_an_overdue_auction() {
    let mut d = Demo::new();
    d.bid(0, "demo-1", json!(10));
    let count = d.engine.state.records.len();
    for raw in [
        b"[]".as_slice(),
        br#"{"x":1,"x":2}"#,
        br#"{"x":{"a":0,"a":1}}"#,
        br#"{"a":NaN}"#,
        b"\xff",
        br#"{"x":"\ud800"}"#,
    ] {
        let response = d.engine.request("bid", Some(raw), BASE + 300000, 50, 0);
        assert_eq!(response["error"]["code"], "malformed_request", "{response}");
        assert_eq!(d.engine.state.records.len(), count);
        assert_eq!(d.engine.state.phase(), "Open");
    }
    assert_eq!(d.read(301)["data"]["auction_state"], "AwaitingSettlement");
}
#[test]
fn integer_syntax_and_rejection_priority_preserve_the_submitted_bytes() {
    let mut d = Demo::new();
    for amount in ["10.0", "1e1", "true", "\"10\""] {
        let raw =
            format!("{{\"auction_number\":1,\"bidder_id\":\"demo-1\",\"amount_rana\":{amount}}}");
        let response = d.engine.request("bid", Some(raw.as_bytes()), BASE, 50, 0);
        let bid = &response["data"]["bid_record"]["payload_json"];
        assert_eq!(bid["rejection_reason"], "malformed_field");
        assert_eq!(bid["command_utf8_hex"], hex::encode(raw));
        assert!(bid["amount_rana"].is_null());
    }
    let r = d
        .engine
        .request("bid", Some(br#"{"auction_number":2}"#), BASE, 50, 0);
    assert_eq!(
        r["data"]["bid_record"]["payload_json"]["rejection_reason"],
        "wrong_auction_number"
    );
    let raw = "{\"auction_number\":1,\"bidder_id\":\"e\u{301}\",\"amount_rana\":10}";
    let r = d.engine.request("bid", Some(raw.as_bytes()), BASE, 50, 0);
    assert_eq!(r["data"]["bid_record"]["payload_json"]["bidder_id"], "é");
    assert_eq!(
        r["data"]["bid_record"]["payload_json"]["command_utf8_hex"],
        hex::encode(raw)
    );
    reconstruct(&d.engine.rows().unwrap(), &config()).unwrap();
}
#[test]
fn all_business_rejection_reasons_are_ordered() {
    let mut c = config();
    c["maximum_bid_rana"] = json!(50);
    let mut d = Demo::configured(c);
    let cases = [
        (r#"{}"#, "missing_required_field"),
        (
            r#"{"auction_number":1,"bidder_id":"demo-1","amount_rana":10,"extra":0}"#,
            "malformed_field",
        ),
        (
            r#"{"auction_number":1,"bidder_id":"stranger","amount_rana":0}"#,
            "unknown_identity",
        ),
        (
            r#"{"auction_number":1,"bidder_id":"demo-1","amount_rana":0}"#,
            "amount_below_minimum",
        ),
        (
            r#"{"auction_number":1,"bidder_id":"demo-1","amount_rana":60}"#,
            "amount_above_maximum",
        ),
    ];
    for (raw, reason) in cases {
        let r = d.engine.request("bid", Some(raw.as_bytes()), BASE, 50, 0);
        assert_eq!(
            r["data"]["bid_record"]["payload_json"]["rejection_reason"],
            reason
        );
    }
    assert_eq!(d.engine.state.phase(), "Scheduled");
    assert_eq!(balances(&d.engine.state), json!([[[100, 0], [100, 0]], 0]));
}
#[test]
fn settlement_waits_without_deadline_and_each_outcome_is_one_shot() {
    for outcome in ["settled", "expired"] {
        let mut d = Demo::new();
        d.bid(0, "demo-1", json!(60));
        d.read(225);
        assert_eq!(
            d.read(100000)["data"]["auction_state"],
            "AwaitingSettlement"
        );
        let before = d.engine.state.records.len();
        for raw in [
            r#"{}"#,
            r#"{"auction_id":12,"outcome":"settled"}"#,
            r#"{"auction_id":"auc_000000000002","outcome":"settled"}"#,
            r#"{"auction_id":"auc_000000000001","outcome":"other"}"#,
        ] {
            let r = d
                .engine
                .request("settlement", Some(raw.as_bytes()), BASE + 100000000, 50, 0);
            assert_eq!(r["status"], "error");
            assert_eq!(d.engine.state.records.len(), before);
        }
        accepted(&d.settle(100001, outcome));
        let committed = d.engine.rows().unwrap();
        let duplicate = d.settle(100002, outcome);
        assert_eq!(duplicate["error"]["code"], "settlement_not_available");
        assert_eq!(d.engine.rows().unwrap(), committed);
        let expected = if outcome == "settled" {
            json!([[[40, 0], [100, 0]], 60])
        } else {
            json!([[[100, 0], [100, 0]], 0])
        };
        assert_eq!(balances(&d.engine.state), expected);
    }
}
#[test]
fn changed_current_config_does_not_rewrite_or_reissue_captured_history() {
    let mut d = Demo::new();
    d.bid(0, "demo-1", json!(20));
    let rows = d.engine.rows().unwrap();
    let mut changed = config();
    changed["starting_number"] = json!(99);
    changed["demo_1_initial_rana"] = json!(999);
    changed["duration_seconds"] = json!(225);
    changed["minimum_bid_rana"] = json!(80);
    let rebuilt = reconstruct(&rows, &changed).unwrap();
    assert_eq!(balances(&rebuilt), json!([[[80, 20], [100, 0]], 0]));
    assert_eq!(rebuilt.auction["minimum_bid_rana"], 10);
    assert_eq!(rebuilt.number, Some(1.into()));
}
#[test]
fn closed_prefix_reconstructs_then_resolution_is_a_new_evaluation() {
    let mut d = Demo::new();
    d.bid(0, "demo-1", json!(10));
    d.read(225);
    let mut rows = d.engine.rows().unwrap();
    rows.pop();
    let mut closed = reconstruct(&rows, &config()).unwrap();
    assert_eq!(closed.phase(), "Closed");
    let before = closed.records.clone();
    let resolution = plan(&closed, "resolve", BASE + 300000, None, &config()).unwrap();
    closed.fold(&resolution).unwrap();
    assert_eq!(&closed.records[..before.len()], before);
    assert_eq!(
        closed.resolution["server_time"],
        stamp(BASE + 300000).unwrap()
    );
}
#[test]
fn replay_receipt_describes_startup_prefix_and_never_claims_new_appends() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("history.sqlite3");
    let mut e = Engine::open(&path, config()).unwrap();
    e.request("state", None, BASE - 1000, 50, 0);
    assert_eq!(e.validated, 0);
    e.request(
        "bid",
        Some(br#"{"auction_number":1,"bidder_id":"demo-1","amount_rana":10}"#),
        BASE,
        50,
        0,
    );
    drop(e);
    let mut e = Engine::open(&path, config()).unwrap();
    assert_eq!(e.validated, 6);
    assert_eq!(e.state.phase(), "Open");
    let r = e.request("state", None, BASE + 225000, 50, 0);
    assert_eq!(
        r["data"]["reconstruction"]["validated_through_sequence_index"],
        6
    );
    assert_eq!(e.state.records.len(), 8);
}
#[test]
fn malformed_history_including_valid_rehashed_lies_is_rejected() {
    let mut d = Demo::new();
    d.bid(0, "demo-1", json!(20));
    d.read(225);
    d.settle(226, "settled");
    let original = d.engine.rows().unwrap();
    let alterations = [
        (4, "/payload_json/amount_rana", json!(21)),
        (4, "/payload_json/bid_id", json!("bid_000000000999")),
        (
            5,
            "/payload_json/base_end_time",
            json!(stamp(BASE + 224000).unwrap()),
        ),
        (7, "/payload_json/winning_amount_rana", json!(30)),
        (10, "/payload_json/title_kind", json!("PublicLand")),
        (3, "/group_size", json!(1)),
        (3, "/sequence_index", json!(true)),
        (3, "/server_time", json!(stamp(BASE - 2000).unwrap())),
    ];
    for (index, pointer, value) in alterations {
        let mut rows = original.clone();
        let mut record: Value = serde_json::from_str(&rows[index].1).unwrap();
        *record.pointer_mut(pointer).unwrap() = value;
        record["payload_hash"] = json!(hash(&record["payload_json"]));
        rows[index].1 = canonical(&record);
        assert!(
            reconstruct(&rows, &config()).is_err(),
            "accepted corruption {pointer}"
        );
    }
    let mut bad = original.clone();
    bad[3].1 = bad[3].1.replace("payload_hash\":\"", "payload_hash\":\"0");
    assert!(reconstruct(&bad, &config()).is_err());
}
#[test]
fn backwards_clock_halts_future_processing_without_new_facts() {
    let mut d = Demo::new();
    d.bid(0, "demo-1", json!(10));
    let rows = d.engine.rows().unwrap();
    assert_eq!(d.read(-1)["error"]["code"], "clock_invalid");
    assert_eq!(d.read(1)["error"]["code"], "clock_invalid");
    assert_eq!(d.engine.rows().unwrap(), rows);
}
#[test]
fn single_writer_and_append_only_sql_constraints() {
    let d = Demo::new();
    assert!(Engine::open(&d.path, config()).is_err());
    let db = rusqlite::Connection::open(&d.path).unwrap();
    assert!(db.execute("DELETE FROM events", []).is_err());
    assert!(db
        .execute("UPDATE events SET record_json='{}'", [])
        .is_err());
}
#[test]
fn storage_failure_never_selects_unowned_or_acknowledges_a_bid() {
    let mut d = Demo::new();
    let db = rusqlite::Connection::open(&d.path).unwrap();
    db.execute_batch("BEGIN EXCLUSIVE;").unwrap();
    let r = d.bid(0, "demo-1", json!(10));
    assert_eq!(r["error"]["code"], "storage_unavailable");
    assert_eq!(d.engine.state.phase(), "Scheduled");
    assert!(d.engine.state.ownership.is_null());
    db.execute_batch("ROLLBACK;").unwrap();
    assert_eq!(d.read(1)["error"]["code"], "storage_unavailable");
    assert_eq!(d.engine.rows().unwrap().len(), 3);
}
#[test]
fn canonical_serialization_has_nfc_sorted_keys_and_json_control_escapes() {
    let value = json!({"z":"e\u{301}/\n\t\u{0001}","a":[true,null,123]});
    assert_eq!(
        canonical(&value),
        "{\"a\":[true,null,123],\"z\":\"é/\\n\\t\\u0001\"}"
    );
    assert_eq!(
        hash(&json!({})),
        "44136fa355b3678a1146ad16f7e8649e94fb4fc21fe77e8310c060f61caaff8a"
    );
}
#[test]
fn strict_http_surface_and_global_pagination() {
    for url in [
        "/auction/history?limit=0",
        "/auction/history?limit=101",
        "/auction/history?offset=-1",
        "/auction/history?limit=2&limit=3",
        "/auction/history?extra=1",
        "/state?x=1",
    ] {
        assert_eq!(
            route("GET", url, b"").err().unwrap().code,
            "invalid_read_parameters"
        );
    }
    assert_eq!(
        route("GET", "/bid", b"").err().unwrap().code,
        "method_not_allowed"
    );
    assert_eq!(
        route("GET", "/reset", b"").err().unwrap().code,
        "unknown_endpoint"
    );
    let mut d = Demo::new();
    let r = d.engine.request("history", None, BASE, 2, 0);
    assert_eq!(r["data"]["records"][0]["record_type"], "RanaIssueRecord");
    assert_eq!(r["data"]["pagination"]["next_offset"], 2);
    let r = d.engine.request("history", None, BASE, 2, 2);
    assert_eq!(r["data"]["records"].as_array().unwrap().len(), 1);
    assert!(r["data"]["pagination"]["next_offset"].is_null());
}
#[test]
fn timestamp_overflow_fails_before_a_bid_can_commit() {
    let mut d = Demo::new();
    let t = millis("9999-12-31T23:59:00.000Z").unwrap();
    let before = d.engine.rows().unwrap();
    let r = d.engine.request(
        "bid",
        Some(br#"{"auction_number":1,"bidder_id":"demo-1","amount_rana":10}"#),
        t,
        50,
        0,
    );
    assert_eq!(r["error"]["code"], "arithmetic_unrepresentable");
    assert_eq!(d.engine.rows().unwrap(), before);
}

// This helper is invoked only by the crash test's child process. It exposes no app route.
#[test]
fn crash_writer_child() {
    let Ok(directory) = std::env::var("NUMBERS_TEST_CRASH_DIRECTORY") else {
        return;
    };
    let directory = Path::new(&directory);
    let stage = std::env::var("NUMBERS_TEST_CRASH_STAGE").unwrap();
    let group: Vec<Value> =
        serde_json::from_slice(&std::fs::read(directory.join("group.json")).unwrap()).unwrap();
    let mut db = rusqlite::Connection::open(directory.join("history.sqlite3")).unwrap();
    db.execute_batch("PRAGMA synchronous=FULL;PRAGMA fullfsync=ON;")
        .unwrap();
    let tx = db.transaction().unwrap();
    for (index, r) in group.iter().enumerate() {
        tx.execute(
            "INSERT INTO events VALUES (?1,?2)",
            rusqlite::params![r["sequence_index"].as_i64().unwrap(), canonical(r)],
        )
        .unwrap();
        if stage == "partial" && index == 0 {
            break;
        }
    }
    if stage == "committed" {
        tx.commit().unwrap();
    } else {
        std::mem::forget(tx);
    }
    std::fs::write(directory.join("ready"), b"ready").unwrap();
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
#[test]
fn killed_process_preserves_atomic_terminal_groups_and_never_double_captures() {
    for outcome in ["settled", "expired"] {
        for stage in ["partial", "uncommitted", "committed"] {
            let mut d = Demo::new();
            d.bid(0, "demo-1", json!(60));
            d.read(225);
            let body = json!({"auction_id":"auc_000000000001","outcome":outcome}).to_string();
            let group = plan(
                &d.engine.state,
                "settlement",
                BASE + 226000,
                Some(body.as_bytes()),
                &config(),
            )
            .unwrap();
            let prefix = d.engine.rows().unwrap();
            let directory = d.path.parent().unwrap().to_path_buf();
            std::fs::write(
                directory.join("group.json"),
                serde_json::to_vec(&group).unwrap(),
            )
            .unwrap();
            drop(d.engine);
            let mut child = std::process::Command::new(std::env::current_exe().unwrap())
                .args(["--exact", "crash_writer_child", "--nocapture"])
                .env("NUMBERS_TEST_CRASH_DIRECTORY", &directory)
                .env("NUMBERS_TEST_CRASH_STAGE", stage)
                .stdout(std::process::Stdio::null())
                .spawn()
                .unwrap();
            let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
            while !directory.join("ready").exists() && std::time::Instant::now() < deadline {
                std::thread::sleep(std::time::Duration::from_millis(10));
            }
            let ready = directory.join("ready").exists();
            child.kill().unwrap();
            child.wait().unwrap();
            assert!(ready, "child did not reach crash boundary");
            let mut restarted = Engine::open(&directory.join("history.sqlite3"), config()).unwrap();
            if stage == "committed" {
                assert_eq!(restarted.state.phase(), "Finalized");
                assert_eq!(restarted.rows().unwrap().len(), prefix.len() + 3);
                let duplicate =
                    restarted.request("settlement", Some(body.as_bytes()), BASE + 227000, 50, 0);
                assert_eq!(duplicate["error"]["code"], "settlement_not_available");
                assert_eq!(restarted.rows().unwrap().len(), prefix.len() + 3);
            } else {
                assert_eq!(restarted.state.phase(), "AwaitingSettlement");
                assert_eq!(restarted.rows().unwrap(), prefix);
                assert_eq!(balances(&restarted.state), json!([[[40, 60], [100, 0]], 0]));
            }
        }
    }
}

#[test]
fn huge_history_offset_is_exact_and_returns_an_empty_page() {
    let offset = "100000000000000000000000000000000000000000003";
    let input = route("GET", &format!("/auction/history?offset={offset}"), b"").unwrap();
    let mut demo = Demo::new();
    let response = demo
        .engine
        .request("history", None, BASE, input.limit, input.offset);
    assert_eq!(response["data"]["pagination"]["offset"].to_string(), offset);
    assert_eq!(response["data"]["records"], json!([]));
    assert!(response["data"]["pagination"]["next_offset"].is_null());
}

#[test]
fn configuration_has_only_explicit_defaults_and_allows_zero_allocation() {
    let mut minimal = config();
    minimal.as_object_mut().unwrap().remove("starting_number");
    minimal.as_object_mut().unwrap().remove("maximum_bid_rana");
    let validated = validate_config(minimal.clone()).unwrap();
    assert_eq!(validated["starting_number"], 1);
    assert!(validated["maximum_bid_rana"].is_null());
    minimal
        .as_object_mut()
        .unwrap()
        .remove("extension_window_seconds");
    assert!(validate_config(minimal).is_err());
    let mut unknown = config();
    unknown["settlement_deadline_seconds"] = json!(60);
    assert!(validate_config(unknown).is_err());
    let mut empty = config();
    empty["demo_1_initial_rana"] = json!(0);
    empty["demo_2_initial_rana"] = json!(0);
    let mut demo = Demo::configured(empty);
    let response = demo.bid(0, "demo-1", json!(10));
    assert_eq!(
        response["data"]["bid_record"]["payload_json"]["rejection_reason"],
        "insufficient_available_rana"
    );
    assert_eq!(demo.engine.state.phase(), "Scheduled");
}
