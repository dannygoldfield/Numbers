//! UI05: exercise the shipped executables, persisted history, and actual HTTP transport.
use numbers_prototype_01::{store::Engine, values::*};
use rusqlite::{Connection, OpenFlags};
use serde_json::{json, Value};
use std::{
    fs,
    path::Path,
    process::{Command, Output},
    thread,
    time::Duration,
};

const DEMO: &str = env!("CARGO_BIN_EXE_numbers-demo");
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
    for key in [
        "database_path",
        "host",
        "port",
        "log_verbosity",
        "allowed_origin",
        "session_label",
    ] {
        value.as_object_mut().unwrap().remove(key);
    }
    value
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
    assert!(records
        .iter()
        .all(|record| record["spec_revision"] == "0.3"));
    let finalizations: Vec<_> = records
        .iter()
        .filter(|r| r["record_type"] == "FinalizationRecord")
        .collect();
    assert_eq!(finalizations[0]["payload_json"]["ownership_kind"], "winner");
    assert_eq!(finalizations[0]["payload_json"]["owner_id"], "demo-2");
    assert_eq!(
        finalizations[1]["payload_json"]["ownership_kind"],
        "unowned"
    );
    assert!(finalizations[1]["payload_json"]["owner_id"].is_null());
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
    assert!(String::from_utf8_lossy(&wrong_clock.stderr).contains("authenticated browser"));
    assert_eq!(fs::read(session.join("history.sqlite3")).unwrap(), before);
}

#[test]
fn retired_local_live_tools_refuse_without_network_access() {
    for args in [
        vec!["live"],
        vec!["serve"],
        vec!["serve", "--session", "missing"],
    ] {
        let output = Command::new(DEMO).args(args).output().unwrap();
        assert!(!output.status.success());
        assert!(String::from_utf8_lossy(&output.stderr).contains("authenticated browser"));
    }
}

#[test]
fn record_route_and_retired_route_do_not_evaluate_an_overdue_auction() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("history.sqlite3");
    let mut engine = Engine::open(&path, config()).unwrap();
    let past = now() - 1_000_000;
    engine.request("state", None, past, 100, 0);
    let bid = engine.request(
        "bid",
        Some(br#"{"auction_number":1,"bidder_id":"demo-1","amount_rana":5}"#),
        past,
        100,
        0,
    );
    assert_eq!(bid["data"]["accepted"], true);
    let before = engine.rows().unwrap();
    let server = tiny_http::Server::http("127.0.0.1:0").unwrap();
    let url = format!("http://{}", server.server_addr());
    let worker = thread::spawn(move || {
        for _ in 0..2 {
            let request = server
                .recv_timeout(Duration::from_secs(10))
                .unwrap()
                .unwrap();
            numbers_prototype_01::http::serve_request(
                request,
                &mut engine,
                &numbers_prototype_01::access::Access::new(
                    "https://numbers.example.test".into(),
                    "test".into(),
                )
                .unwrap(),
            )
            .unwrap();
        }
        assert_eq!(engine.state().phase(), "Open");
        assert_eq!(engine.rows().unwrap(), before);
    });
    let get = |route: &str| {
        success(
            Command::new("curl")
                .args([
                    "--disable",
                    "--silent",
                    "--show-error",
                    "--max-time",
                    "5",
                    "--header",
                    "X-Numbers-Principal: bidder-a",
                    "--write-out",
                    "\n%{http_code}",
                    &format!("{url}{route}"),
                ])
                .output()
                .unwrap(),
        )
    };
    let record = get("/record");
    assert!(record.ends_with("\n200"));
    assert!(record.contains("href=\"/record\""));
    assert!(record.contains("Specification 0.3"));
    let retired = get("/protocol");
    let (body, status) = retired.rsplit_once('\n').unwrap();
    assert_eq!(status, "404");
    let response: Value = serde_json::from_str(body).unwrap();
    assert_eq!(response["error"]["code"], "unknown_endpoint");
    assert_eq!(response["evaluation_records"], json!([]));
    worker.join().unwrap();
}

#[test]
fn read_only_inspector_rejects_preserved_legacy_format() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("history.sqlite3");
    let db = Connection::open(&path).unwrap();
    db.execute_batch(
        "CREATE TABLE events(sequence_index INTEGER PRIMARY KEY, record_json TEXT NOT NULL)",
    )
    .unwrap();
    let records: Vec<Value> =
        serde_json::from_str(include_str!("fixtures/revision-0.1-initialization.json")).unwrap();
    for record in records {
        db.execute(
            "INSERT INTO events VALUES (?1,?2)",
            rusqlite::params![
                record["sequence_index"].as_i64().unwrap(),
                canonical(&record)
            ],
        )
        .unwrap();
    }
    drop(db);
    fs::write(
        directory.path().join("config.json"),
        include_str!("../demo-config.json"),
    )
    .unwrap();
    let before = fs::read(&path).unwrap();
    let output = Command::new(DEMO)
        .arg("inspect")
        .arg(directory.path())
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("history_invalid"));
    assert_eq!(fs::read(&path).unwrap(), before);
}

#[test]
fn launcher_binds_build_config_and_history_and_refuses_silent_replacement() {
    use std::{net::TcpListener, time::Instant};
    const SERVER: &str = env!("CARGO_BIN_EXE_numbers-prototype-01");
    struct Running(std::process::Child);
    impl Drop for Running {
        fn drop(&mut self) {
            let _ = self.0.kill();
            let _ = self.0.wait();
        }
    }
    let dir = tempfile::tempdir().unwrap();
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    drop(listener);
    let mut c: Value = serde_json::from_str(include_str!("../demo-config.json")).unwrap();
    c["database_path"] = json!("history.sqlite3");
    c["port"] = json!(port);
    let path = dir.path().join("config.json");
    fs::write(&path, c.to_string()).unwrap();
    let refused = Command::new(SERVER)
        .arg("--config")
        .arg(&path)
        .output()
        .unwrap();
    assert!(!refused.status.success());
    assert!(!dir.path().join("history.sqlite3").exists());
    let start = |initialize: bool| {
        let mut cmd = Command::new(SERVER);
        cmd.arg("--config")
            .arg(&path)
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null());
        if initialize {
            cmd.arg("--initialize");
        }
        let mut process = Running(cmd.spawn().unwrap());
        let until = Instant::now() + Duration::from_secs(10);
        loop {
            assert!(process.0.try_wait().unwrap().is_none(), "server stopped");
            assert!(Instant::now() < until, "startup timeout");
            let output = Command::new("curl")
                .args([
                    "--disable",
                    "-s",
                    "--max-time",
                    "1",
                    "-H",
                    "X-Numbers-Principal: bidder-a",
                    &format!("http://127.0.0.1:{port}/access"),
                ])
                .output()
                .unwrap();
            if output.status.success()
                && String::from_utf8_lossy(&output.stdout).contains("spec_revision")
            {
                break;
            }
            thread::sleep(Duration::from_millis(20));
        }
        process
    };
    drop(start(true));
    let journal = dir.path().join("history.sqlite3");
    let before = fs::read(&journal).unwrap();
    assert_eq!(rows(&journal).len(), 3);
    drop(start(false));
    assert_eq!(fs::read(&journal).unwrap(), before);
    c["database_path"] = json!("substitute.sqlite3");
    fs::write(&path, c.to_string()).unwrap();
    let refused = Command::new(SERVER)
        .arg("--config")
        .arg(&path)
        .output()
        .unwrap();
    assert!(!refused.status.success());
    assert!(!dir.path().join("substitute.sqlite3").exists());
    assert_eq!(fs::read(&journal).unwrap(), before);
    c["database_path"] = json!("history.sqlite3");
    fs::write(&path, c.to_string()).unwrap();
    fs::remove_file(&journal).unwrap();
    let refused = Command::new(SERVER)
        .arg("--config")
        .arg(&path)
        .output()
        .unwrap();
    assert!(!refused.status.success());
    assert!(!journal.exists());
    assert!(String::from_utf8_lossy(&refused.stderr).contains("missing or empty"));
}
