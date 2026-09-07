//! UI05: each demonstration has its own directory and journal.
use super::{
    presentation::{show_records, show_state},
    Result, URL,
};
use numbers_prototype_01::{model::*, values::*};
use serde_json::{json, Value};
use std::{
    fs::{self, OpenOptions},
    io::{self, Write},
    path::{Path, PathBuf},
    process::Command,
};
const ROOT: &str = env!("CARGO_MANIFEST_DIR");

pub(super) fn fixture() -> Value {
    json!({"starting_number":1,"demo_1_initial_rana":600,"demo_2_initial_rana":600,
        "duration_seconds":225,"inter_auction_gap_seconds":12,"extension_window_seconds":10,
        "extension_increment_seconds":15,"max_extensions":2,"minimum_bid_rana":5,
        "minimum_increment_rana":1,"maximum_bid_rana":null})
}
pub(super) fn write_new(path: &Path, value: &Value) -> Result<()> {
    let mut file = OpenOptions::new().write(true).create_new(true).open(path)?;
    writeln!(file, "{}", serde_json::to_string_pretty(value)?)?;
    file.sync_all()?;
    Ok(())
}
pub(super) fn new_session(mode: &str, supplied: Option<&str>) -> Result<PathBuf> {
    let path = supplied.map(PathBuf::from).unwrap_or_else(|| {
        Path::new(ROOT).join("data/demonstrations").join(format!(
            "{mode}-{}-{}",
            now(),
            std::process::id()
        ))
    });
    if let Some(parent) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
        fs::create_dir_all(parent)?;
    }
    fs::create_dir(&path)?; // Existing sessions are never replaced or cleared.
    let path = path.canonicalize()?;
    let mut config = fixture();
    config["database_path"] = json!("history.sqlite3");
    config["host"] = json!("127.0.0.1");
    config["port"] = json!(8766);
    config["log_verbosity"] = json!("quiet");
    write_new(&path.join("config.json"), &config)?;
    write_new(
        &path.join("session.json"),
        &json!({"mode":mode,"created_at":stamp(now())?}),
    )?;
    Ok(path)
}
pub(super) fn session_config(path: &Path) -> Result<Value> {
    let mut config: Value = serde_json::from_slice(&fs::read(path.join("config.json"))?)?;
    for key in ["database_path", "host", "port", "log_verbosity"] {
        config
            .as_object_mut()
            .ok_or("Invalid session configuration")?
            .remove(key);
    }
    Ok(validate_config(config)?)
}
pub(super) fn inspect(session: &Path) -> Result<()> {
    let db = rusqlite::Connection::open_with_flags(
        session.join("history.sqlite3"),
        rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY,
    )?;
    let mut query =
        db.prepare("SELECT sequence_index,record_json FROM events ORDER BY sequence_index")?;
    let rows: Vec<(i64, String)> = query
        .query_map([], |r| Ok((r.get(0)?, r.get(1)?)))?
        .collect::<std::result::Result<_, _>>()?;
    let state = reconstruct(&rows, &session_config(session)?)?;
    let time = millis(text(
        &state.records.last().ok_or("Empty history")?["server_time"],
    )?)?;
    println!(
        "Read-only reconstruction · {}\nNo lifecycle evaluation or command is run.",
        session.display()
    );
    show_state(&projection(&state, time, state.records.len())?)?;
    show_records(&state.records);
    Ok(())
}
pub(super) fn serve(existing: Option<&str>) -> Result<()> {
    let session = if let Some(path) = existing {
        let path = PathBuf::from(path).canonicalize()?;
        let metadata: Value = serde_json::from_slice(&fs::read(path.join("session.json"))?)?;
        if metadata["mode"] != "live" {
            return Err("Only a live session can use the real-clock server. Use inspect for a guided history.".into());
        }
        path
    } else {
        new_session("live", None)?
    };
    println!("Numbers · Separate live demonstration\nBrowser: {URL}/\nTerminal client: ./demo live\nSession: {}\nRestart this history: ./demo serve --session '{}'\n",session.display(),session.display());
    io::stdout().flush()?;
    let server = std::env::current_exe()?.with_file_name("numbers-prototype-01");
    use std::os::unix::process::CommandExt;
    Err(Command::new(server)
        .arg("--config")
        .arg(session.join("config.json"))
        .exec()
        .into())
}
