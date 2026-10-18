//! Local process entry: explicit config → reconstruct → startup evaluation → requests.
use numbers_prototype_01::{
    access::Access,
    http,
    model::validate_config,
    store::{diagnostic, Engine},
    values::{command, now},
    Fault,
};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::net::IpAddr;
use std::path::PathBuf;
use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::Path,
};

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if !(args.len() == 3 || args.len() == 4 && args[3] == "--initialize") || args[1] != "--config" {
        return Err(
            "Usage: numbers-prototype-01 --config /path/to/config.json [--initialize]".into(),
        );
    }
    let config_path = PathBuf::from(&args[2]).canonicalize()?;
    let bytes = std::fs::read(&config_path)?;
    command(&bytes)?;
    let mut config: Value = serde_json::from_slice(&bytes)?;
    let object = config
        .as_object_mut()
        .ok_or("Configuration must be an object")?;
    let database = object
        .remove("database_path")
        .and_then(|v| v.as_str().map(String::from))
        .ok_or("database_path is required")?;
    if database.is_empty() {
        return Err("database_path must not be empty".into());
    }
    let host = object
        .remove("host")
        .and_then(|v| v.as_str().map(String::from))
        .ok_or("host is required")?;
    let address: IpAddr = host.parse()?;
    if !address.is_loopback() {
        return Err("Prototype 2 must bind to a loopback address".into());
    }
    let port = object
        .remove("port")
        .and_then(|v| v.as_u64())
        .ok_or("port is required as an integer")?;
    if !(1..=65535).contains(&port) {
        return Err("port must be between 1 and 65535".into());
    }
    let verbosity = object
        .remove("log_verbosity")
        .and_then(|v| v.as_str().map(String::from))
        .ok_or("log_verbosity is required")?;
    if !["quiet", "requests"].contains(&verbosity.as_str()) {
        return Err("log_verbosity must be quiet or requests".into());
    }
    let origin = object
        .remove("allowed_origin")
        .and_then(|v| v.as_str().map(String::from))
        .ok_or("allowed_origin is required")?;
    let session_label = object
        .remove("session_label")
        .and_then(|v| v.as_str().map(String::from))
        .ok_or("session_label is required")?;
    let access = Access::new(origin, session_label)?;
    let config = validate_config(config)?;
    let database_path = config_path
        .parent()
        .ok_or("config needs a directory")?
        .join(database);
    // Bind before opening the journal: an occupied port does not start another history.
    let server = tiny_http::Server::http((address, port as u16)).map_err(|e| e.to_string())?;
    bind_session(
        &config_path,
        &database_path,
        &bytes,
        &access,
        args.len() == 4,
    )?;
    let mut engine = Engine::open(&database_path, config)?;
    let initialized = engine.request("state", None, now(), 50, 0);
    if initialized["status"] != "success" {
        return Err(initialized.to_string().into());
    }
    println!("Numbers Prototype 2 · specification 0.3\nSession: {}\nHTTPS entry: {}\nPrivate backend: http://{host}:{port}\nJournal: {}\nReconstructed through record {}. Press Ctrl-C to stop.", access.session_label, access.origin, database_path.display(), engine.validated_through());
    for request in server.incoming_requests() {
        if verbosity == "requests" {
            eprintln!(
                "{} {}",
                request.method(),
                match request.url().split('?').next().unwrap_or("") {
                    "/" => "/",
                    "/record" => "/record",
                    "/app.js" => "/app.js",
                    "/style.css" => "/style.css",
                    "/access" => "/access",
                    "/state" => "/state",
                    "/auction/history" => "/auction/history",
                    "/bid" => "/bid",
                    "/demo/settlement" => "/demo/settlement",
                    _ => "unknown",
                }
            );
        }
        if let Err(error) = http::serve_request(request, &mut engine, &access) {
            let mut fault = Fault::storage(format!("HTTP response unavailable: {error}. Committed history remains authoritative; the command was not retried."));
            diagnostic(&database_path, &mut fault);
            eprintln!("{fault}");
        }
    }
    Ok(())
}
// P02: this noncanonical receipt binds a launch to its exact build/config/history.
// It never repairs or initializes a missing intended journal during restart.
fn bind_session(
    config_path: &Path,
    database: &Path,
    config_bytes: &[u8],
    access: &Access,
    initialize: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let receipt = config_path.with_extension("session.json");
    let executable = fs::read(std::env::current_exe()?)?;
    let expected = json!({
        "spec_revision":"0.3", "session_label":access.session_label,
        "config_sha256":hex::encode(Sha256::digest(config_bytes)),
        "executable_sha256":hex::encode(Sha256::digest(executable)),
        "database_path":database,
        "principals":{"bidder-a":"demo-1","bidder-b":"demo-2","facilitator":null}
    });
    if initialize {
        if fs::symlink_metadata(database).is_ok() || fs::symlink_metadata(&receipt).is_ok() {
            return Err("Initialization requires a new journal path and new session receipt; existing files were not replaced".into());
        }
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(receipt)?;
        writeln!(file, "{}", serde_json::to_string_pretty(&expected)?)?;
        file.sync_all()?;
    } else {
        let actual: Value = serde_json::from_slice(&fs::read(&receipt).map_err(|_| "Existing session receipt required. For an explicitly new history, use --initialize with a distinct session label and path")?)?;
        if actual != expected {
            return Err("Session association changed; use the matching build, configuration and journal. No substitute history was created".into());
        }
        if !database.is_file() || database.metadata()?.len() == 0 {
            return Err("Intended session journal is missing or empty. Processing stopped without initializing a replacement".into());
        }
    }
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("Numbers stopped: {error}");
        std::process::exit(1);
    }
}
