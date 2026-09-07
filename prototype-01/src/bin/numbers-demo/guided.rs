//! UI05: a prescribed fixture, not a second auction implementation.
//! Every step calls Engine::request; only this isolated fixture supplies simulated time.
use super::{
    presentation::{show_records, show_result, show_state},
    session::{fixture, new_session, session_config, write_new},
    Result,
};
use numbers_prototype_01::{model::*, store::Engine, values::*};
use serde_json::{json, Value};
use std::{
    io::{self, Write},
    path::Path,
    process::Command,
};
const BASE: i64 = 1_788_609_600_000;

enum Action {
    Read,
    Bid(&'static str, i64),
    Settle(&'static str),
    Restart,
}
struct Step {
    at: i64,
    heading: &'static str,
    why: &'static str,
    rule: &'static str,
    action: Action,
}
fn steps() -> [Step; 18] {
    use Action::*;
    [
        Step {
            at: -1,
            heading: "A number awaits its first bid",
            why: "Initialization issues each allocation exactly once. The clock is dormant.",
            rule: "G01 · src/model.rs: plan",
            action: Read,
        },
        Step {
            at: 0,
            heading: "A opens with 20 rana",
            why: "The accepted first bid reserves funds and starts the 225-second clock in one \
                transaction.",
            rule: "G02 / RT02 · src/model.rs: plan_bid",
            action: Bid("demo-1", 20),
        },
        Step {
            at: 30,
            heading: "B takes the lead with 30",
            why: "A's reservation is released; B's full bid is reserved.",
            rule: "G03 / RT02 · src/model.rs: plan_bid",
            action: Bid("demo-2", 30),
        },
        Step {
            at: 45,
            heading: "A's equal bid is rejected",
            why: "A bid of 30 is below the required 31. One invalid bid record is kept; funds \
                do not move.",
            rule: "SE03 / G05 · src/model.rs: rejection",
            action: Bid("demo-1", 30),
        },
        Step {
            at: 60,
            heading: "B raises its own bid to 40",
            why: "Replacement uses B's existing hold; only 10 additional rana becomes unavailable.",
            rule: "RT02 · src/model.rs: plan_bid",
            action: Bid("demo-2", 40),
        },
        Step {
            at: 90,
            heading: "A responds with 50",
            why: "The live hold follows the new leading bid.",
            rule: "RT02 · src/model.rs: plan_bid",
            action: Bid("demo-1", 50),
        },
        Step {
            at: 215,
            heading: "A late bid extends the clock",
            why: "B60 arrives at the 10-second boundary. The end moves from 225 to 240.",
            rule: "SE03 / G04 · src/model.rs: plan_bid",
            action: Bid("demo-2", 60),
        },
        Step {
            at: 230,
            heading: "A second extension",
            why: "A61 moves the end from 240 to 255. Both permitted extensions are now used.",
            rule: "SE03 / G04 · src/model.rs: plan_bid",
            action: Bid("demo-1", 61),
        },
        Step {
            at: 245,
            heading: "The extension limit is reached",
            why: "B62 is accepted, but the end stays 255. Prototype 2 soft-close ideas are not \
                active rules.",
            rule: "SE03 / G03 · src/model.rs: plan_bid",
            action: Bid("demo-2", 62),
        },
        Step {
            at: 255,
            heading: "The winner is determined",
            why: "Close and resolution are separate commits. B's 62 stays reserved while \
                ownership is pending.",
            rule: "SE04 / G06 / G07 · src/store.rs: evaluate",
            action: Read,
        },
        Step {
            at: 256,
            heading: "Successful settlement establishes ownership",
            why: "The explicit outcome captures 62 into the protocol balance. B becomes the owner.",
            rule: "RT03 / G08 · src/model.rs: plan_settlement",
            action: Settle("settled"),
        },
        Step {
            at: 256,
            heading: "Prove reconstruction in a new process",
            why: "Release the writer, start a separate engine process, and compare every \
                stored row and derived fact. No evaluation is run during this comparison.",
            rule: "PR04 · src/model.rs: reconstruct",
            action: Restart,
        },
        Step {
            at: 268,
            heading: "The next number arrives",
            why: "The recorded 12-second gap ends. Number 2 has no bids and no running countdown.",
            rule: "G10 · src/store.rs: evaluate",
            action: Read,
        },
        Step {
            at: 270,
            heading: "A opens Number 2 with 5",
            why: "A new number has its own auction and reservation.",
            rule: "G02 · src/model.rs: plan_bid",
            action: Bid("demo-1", 5),
        },
        Step {
            at: 495,
            heading: "Number 2 closes",
            why: "A is the fixed winner, with 5 still reserved. The outcome command remains \
                separate.",
            rule: "G06 / G07 · src/store.rs: evaluate",
            action: Read,
        },
        Step {
            at: 496,
            heading: "Failed settlement leaves the number Unowned",
            why: "Release A's 5. Nobody becomes the owner; this final outcome is distinct from \
                pending ownership.",
            rule: "RT03 / G09 · src/model.rs: plan_settlement",
            action: Settle("expired"),
        },
        Step {
            at: 496,
            heading: "Verify the Unowned outcome survives restart",
            why: "A second separate-process reconstruction must preserve the outcome, balances \
                and all records.",
            rule: "PR04 · src/model.rs: reconstruct",
            action: Restart,
        },
        Step {
            at: 508,
            heading: "A complete demonstration",
            why: "Number 3 is ready. A has 600, B has 538, protocol has 62, and no reservation \
                remains. All 42 records are kept.",
            rule: "G10 / RT01 · src/store.rs: evaluate",
            action: Read,
        },
    ]
}
fn snapshot(engine: &Engine, time: i64) -> Result<Value> {
    Ok(json!({"rows":engine.rows()?,"state":projection(&engine.state,time,engine.validated)?}))
}
pub(super) fn verify(session: &Path, time: i64) -> Result<()> {
    if !session.join("history.sqlite3").is_file() {
        return Err("No existing history to reconstruct".into());
    }
    let engine = Engine::open(&session.join("history.sqlite3"), session_config(session)?)?;
    println!("{}", snapshot(&engine, time)?);
    Ok(())
}
fn restarted(session: &Path, time: i64, engine: Engine) -> Result<(Engine, Value)> {
    let mut before = snapshot(&engine, time)?;
    drop(engine); // Release the writer before a genuinely separate process opens it.
    let output = Command::new(std::env::current_exe()?)
        .arg("verify-session")
        .arg(session)
        .arg(time.to_string())
        .output()?;
    if !output.status.success() {
        return Err(format!(
            "Reconstruction failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }
    let mut after: Value = serde_json::from_slice(&output.stdout)?;
    let validated = after["state"]["reconstruction"]["validated_through_sequence_index"].clone();
    before["state"]
        .as_object_mut()
        .ok_or("Missing prior projection")?
        .remove("reconstruction");
    after["state"]
        .as_object_mut()
        .ok_or("Missing restored projection")?
        .remove("reconstruction");
    if before != after {
        return Err("Restart changed ordered rows or derived state".into());
    }
    let receipt = json!({"separate_process":true,"ordered_rows_identical":true,"derived_state_identical":true,"validated_through":validated,"simulated_time":stamp(time)?});
    write_new(&session.join(format!("restart-{time}.json")), &receipt)?;
    println!("Restart verified in a new process: {validated} identical records and identical derived facts.");
    Ok((
        Engine::open(&session.join("history.sqlite3"), session_config(session)?)?,
        receipt,
    ))
}
pub(super) fn guided(automatic: bool, directory: Option<&str>) -> Result<()> {
    let session = new_session("guided", directory)?;
    let mut engine = Engine::open(&session.join("history.sqlite3"), fixture())?;
    let mut last_records = Vec::new();
    let mut last = Value::Null;
    println!("Numbers · Guided mechanics\nSimulated time · Real Rust rules and SQLite commits\nSession: {}\n\nEnter: next step · records: exact records from the last step · raw: last result · quit: stop\n",session.display());
    let steps = steps();
    for (index, step) in steps.iter().enumerate() {
        if !automatic {
            loop {
                print!(
                    "Next {}/{}: {} [Enter] > ",
                    index + 1,
                    steps.len(),
                    step.heading
                );
                io::stdout().flush()?;
                let mut input = String::new();
                if io::stdin().read_line(&mut input)? == 0 {
                    println!("Walkthrough stopped. Session retained.");
                    return Ok(());
                }
                match input.trim() {
                    "" | "next" => break,
                    "records" | "r" => println!("{}", serde_json::to_string_pretty(&last_records)?),
                    "raw" => println!("{}", serde_json::to_string_pretty(&last)?),
                    "quit" | "q" => {
                        println!("Session retained at {}", session.display());
                        return Ok(());
                    }
                    _ => println!("Use Enter, records, raw or quit. No event added."),
                }
            }
        }
        println!(
            "\n── Step {}/{} · t={}s (simulated) ──\n{}\n{}\nRule/code: {}",
            index + 1,
            steps.len(),
            step.at,
            step.heading,
            step.why,
            step.rule
        );
        let time = BASE + step.at * 1000;
        if matches!(step.action, Action::Restart) {
            (engine, last) = restarted(&session, time, engine)?;
            last_records.clear();
        } else {
            let count = engine.state.records.len();
            let (action, body) = match step.action {
                Action::Read => ("state",None),
                Action::Bid(identity,amount) => ("bid", Some(json!({
                    "auction_number": engine.state.number.as_ref().map(jnum),
                    "bidder_id": identity,
                    "amount_rana": amount
                }).to_string())),
                Action::Settle(outcome) => ("settlement", Some(json!({
                    "auction_id": id("auc", engine.state.number.as_ref().ok_or("No auction")?),
                    "outcome": outcome
                }).to_string())),
                Action::Restart => unreachable!(),
            };
            println!(
                "Command: {action} {}",
                body.as_deref().unwrap_or("(observe state)")
            );
            last = engine.request(action, body.as_deref().map(str::as_bytes), time, 100, 0);
            if last["status"] != "success" {
                return Err(format!("Unexpected fixture failure: {last}").into());
            }
            if action == "bid" && last["data"]["accepted"] != json!(step.at != 45) {
                return Err("Unexpected fixture admission result".into());
            }
            last_records = engine.state.records[count..].to_vec();
            show_result(&last);
            show_records(&last_records);
        }
        show_state(&projection(&engine.state, time, engine.validated)?)?;
        println!();
    }
    let state = projection(&engine.state, BASE + 508000, engine.validated)?;
    if state["last_sequence_index"] != 42
        || state["current_number"] != 3
        || state["protocol_held_rana"] != 62
        || state["balances"][0]["available_rana"] != 600
        || state["balances"][1]["available_rana"] != 538
        || state["balances"][0]["reserved_rana"] != 0
        || state["balances"][1]["reserved_rana"] != 0
    {
        return Err("Unexpected final fixture state".into());
    }
    write_new(
        &session.join("result.json"),
        &json!({"completed":true,"simulated_time":true,"state":state}),
    )?;
    println!("Walkthrough complete. Both settlement outcomes and separate-process reconstructions passed.\nSaved journal, results and restart receipts: {}\nInspect later: ./demo inspect '{}'",session.display(),session.display());
    Ok(())
}
