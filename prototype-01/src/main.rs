//! Local process entry: explicit config → reconstruct → startup evaluation → requests.
use numbers_prototype_01::{
    http,
    model::validate_config,
    store::{diagnostic, Engine},
    values::{command, now},
    Fault,
};
use serde_json::Value;
use std::net::IpAddr;
use std::path::PathBuf;

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 || args[1] != "--config" {
        return Err("Usage: numbers-prototype-01 --config /path/to/demo-config.json".into());
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
        return Err("Prototype 1 must bind to a loopback address".into());
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
    let config = validate_config(config)?;
    let database_path = config_path
        .parent()
        .ok_or("config needs a directory")?
        .join(database);
    // Bind before opening the journal: an occupied port does not start another history.
    let server = tiny_http::Server::http((address, port as u16)).map_err(|e| e.to_string())?;
    let mut engine = Engine::open(&database_path, config)?;
    let initialized = engine.request("state", None, now(), 50, 0);
    if initialized["status"] != "success" {
        return Err(initialized.to_string().into());
    }
    println!("Numbers Prototype 1 · Rust\nAuction: http://{host}:{port}/\nProtocol: http://{host}:{port}/protocol\nJournal: {}\nReconstructed through record {}. Press Ctrl-C to stop.", database_path.display(), engine.validated);
    for request in server.incoming_requests() {
        if verbosity == "requests" {
            eprintln!("{} {}", request.method(), request.url());
        }
        if let Err(error) = http::serve_request(request, &mut engine) {
            let mut fault = Fault::storage(format!("HTTP response unavailable: {error}. Committed history remains authoritative; the command was not retried."));
            diagnostic(&database_path, &mut fault);
            eprintln!("{fault}");
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
