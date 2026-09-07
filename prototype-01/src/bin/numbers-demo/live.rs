//! UI05: the terminal submits the same public commands as the browser, once each.
use super::{
    presentation::{show_records, show_result, show_state},
    Result,
};
use serde_json::{json, Value};
use std::{
    io::{self, Write},
    process::{Command, Stdio},
};

// The live client uses curl's HTTP transport. It never retries a command or supplies a clock.
fn http(base: &str, path: &str, body: Option<&str>) -> Result<Value> {
    let port = base
        .strip_prefix("http://127.0.0.1:")
        .ok_or("Use a loopback URL: http://127.0.0.1:PORT")?;
    let parsed: u16 = port.parse()?;
    if parsed == 0 {
        return Err("Port must be positive".into());
    }
    let mut command = Command::new("curl");
    command.args([
        "--disable",
        "--silent",
        "--show-error",
        "--noproxy",
        "*",
        "--connect-timeout",
        "3",
        "--max-time",
        "15",
    ]);
    if body.is_some() {
        command.args([
            "--request",
            "POST",
            "--header",
            "Content-Type: application/json",
            "--data-binary",
            "@-",
        ]);
    }
    let mut child = command
        .arg(format!("{base}{path}"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    if let Some(body) = body {
        child
            .stdin
            .take()
            .ok_or("Missing transport input")?
            .write_all(body.as_bytes())?;
    }
    let output = child.wait_with_output()?;
    if !output.status.success() {
        return Err(format!(
            "Response unavailable. {}{}",
            String::from_utf8_lossy(&output.stderr).trim(),
            if body.is_some() {
                " The command may have committed. Use state/history to inspect; it was not retried."
            } else {
                ""
            }
        )
        .into());
    }
    serde_json::from_slice(&output.stdout).map_err(|e| format!("Unreadable response: {e}. Inspect state/history before submitting again; no command was retried.").into())
}
pub(super) fn live(base: &str) -> Result<()> {
    println!("Numbers · Live terminal\nReal server time · {base}\nCommands: state | bid A 20 | bid B 30 | settle success | settle failure\n          history | record 1 | raw | help | quit\nEach command is sent once. Enter refreshes the state; this terminal shows snapshots.\n");
    let mut last = http(base, "/state", None)?;
    if last["status"] != "success" {
        show_result(&last);
        return Err("Live state unavailable".into());
    }
    let mut state = last["data"].clone();
    show_state(&state)?;
    loop {
        print!("\nnumbers> ");
        io::stdout().flush()?;
        let mut line = String::new();
        if io::stdin().read_line(&mut line)? == 0 {
            break;
        }
        let words: Vec<&str> = line.split_whitespace().collect();
        if words == ["quit"] || words == ["q"] {
            break;
        }
        if words == ["raw"] {
            println!("{}", serde_json::to_string_pretty(&last)?);
            continue;
        }
        if words == ["help"] {
            println!("state (or Enter): observe current state\nbid A AMOUNT / bid B AMOUNT: bid on the number currently displayed here\nsettle success / settle failure: explicit simulated outcome for the displayed auction\nhistory: show ordered records\nrecord SEQUENCE: inspect exact canonical JSON\nraw: inspect the last exact response\nquit: leave the client; the server remains in its own terminal");
            continue;
        }
        let result: Result<Value> = match words.as_slice() {
            [] | ["state"] => http(base, "/state", None),
            ["bid", identity, amount] if ["A", "B", "a", "b"].contains(identity) => {
                let bidder_id = if identity.eq_ignore_ascii_case("a") {
                    "demo-1"
                } else {
                    "demo-2"
                };
                let amount = if serde_json::from_str::<Value>(amount).is_ok() {
                    amount.to_string()
                } else {
                    json!(amount).to_string()
                };
                // Preserve the amount's spelling and the last displayed number, even if stale.
                let body = format!("{{\"auction_number\":{},\"bidder_id\":\"{bidder_id}\",\"amount_rana\":{amount}}}", state["current_number"]);
                println!("Submit once: {body}");
                http(base, "/bid", Some(&body))
            }
            ["settle", outcome] if ["success", "failure"].contains(outcome) => {
                let body=json!({"auction_id":state["auction_id"],"outcome":if *outcome=="success" {"settled"} else {"expired"}}).to_string();
                println!("Submit once: {body}");
                http(base, "/demo/settlement", Some(&body))
            }
            ["history"] => {
                let mut offset = Value::from(0);
                loop {
                    let response = http(
                        base,
                        &format!("/auction/history?limit=100&offset={offset}"),
                        None,
                    )?;
                    if response["status"] != "success" {
                        show_result(&response);
                        break;
                    }
                    show_records(
                        response["data"]["records"]
                            .as_array()
                            .ok_or("Missing history")?,
                    );
                    offset = response["data"]["pagination"]["next_offset"].clone();
                    last = response;
                    if offset.is_null() {
                        break;
                    }
                }
                continue;
            }
            ["record", sequence] => {
                if let Ok(sequence) = sequence.parse::<u64>() {
                    if sequence > 0 {
                        http(
                            base,
                            &format!("/auction/history?limit=1&offset={}", sequence - 1),
                            None,
                        )
                    } else {
                        Err("Record sequence starts at 1".into())
                    }
                } else {
                    Err("Use a positive integer sequence".into())
                }
            }
            _ => {
                println!("Unknown local command. Type help. Nothing submitted.");
                continue;
            }
        };
        match result {
            Err(error) => println!("{error}"),
            Ok(response) => {
                show_result(&response);
                let data = &response["data"];
                if data["auction_state"].is_string() {
                    state = data.clone();
                    show_state(&state)?;
                }
                if data["state"].is_object() {
                    state = data["state"].clone();
                    show_state(&state)?;
                }
                for key in ["evaluation_records", "command_records"] {
                    if let Some(records) = data[key].as_array().or_else(|| response[key].as_array())
                    {
                        show_records(records);
                    }
                }
                if words.first() == Some(&"record") {
                    println!("{}", serde_json::to_string_pretty(&data["records"])?);
                }
                last = response;
            }
        }
    }
    Ok(())
}
