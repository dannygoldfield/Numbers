//! Human-readable observations. Exact canonical values remain available through raw/records.
use super::Result;
use numbers_prototype_01::{values::*, Fault};
use serde_json::Value;

fn bidder(value: &Value) -> &str {
    match value.as_str() {
        Some("demo-1") => "Bidder A",
        Some("demo-2") => "Bidder B",
        _ => "No bidder",
    }
}
fn ownership(state: &Value) -> String {
    // RT06: revision 0.1 wire keys stay unchanged; presentation uses C61 vocabulary.
    let result = &state["title"];
    if result.is_null() {
        "Pending".into()
    } else if result["title_kind"] == "winner" {
        format!("Owned by {}", bidder(&result["holder_id"]))
    } else if result["title_kind"] == "PublicLand" {
        "Unowned".into()
    } else {
        "Unknown outcome; inspect raw response".into()
    }
}
pub(super) fn show_state(state: &Value) -> Result<()> {
    println!(
        "Number {} · {}",
        state["current_number"],
        state["auction_state"]
            .as_str()
            .ok_or("Missing auction state")?
    );
    for balance in state["balances"].as_array().ok_or("Missing balances")? {
        println!(
            "  {}: {} available · {} reserved",
            bidder(&balance["bidder_id"]),
            balance["available_rana"],
            balance["reserved_rana"]
        );
    }
    println!(
        "  Protocol: {} rana · Ownership: {}",
        state["protocol_held_rana"],
        ownership(state)
    );
    if !state["leading_bid"].is_null() {
        println!(
            "  Leading bid: {} · {} rana",
            bidder(&state["leading_bid"]["bidder_id"]),
            state["leading_bid"]["amount_rana"]
        );
    }
    let minimum = if state["leading_bid"].is_null() {
        number(&state["parameters"]["minimum_bid_rana"])?
    } else {
        number(&state["leading_bid"]["amount_rana"])?
            + number(&state["parameters"]["minimum_increment_rana"])?
    };
    if ["Scheduled", "Open"].contains(&state["auction_state"].as_str().unwrap_or("")) {
        println!("  Next minimum: {minimum} rana");
    }
    if !state["current_end_time"].is_null() {
        let remaining = (millis(text(&state["current_end_time"])?)?
            - millis(text(&state["server_time"])?)?)
        .max(0);
        println!(
            "  Time remaining at this observation: {} seconds · Extensions: {} of {}",
            (remaining + 999) / 1000,
            state["extension_count"],
            state["parameters"]["max_extensions"]
        );
    }
    println!(
        "  Committed records: {} · Reconstructed through: {}",
        state["last_sequence_index"], state["reconstruction"]["validated_through_sequence_index"]
    );
    Ok(())
}
fn label(record: &Value) -> &str {
    match record["record_type"].as_str().unwrap_or("") {
        "FinalizationRecord" => "Ownership finalized",
        "RanaIssueRecord" => "Rana issued",
        "RanaReserveRecord" => "Rana reserved",
        "RanaReleaseRecord" => "Rana released",
        "RanaCaptureRecord" => "Rana captured",
        "BidRecord" => "Bid recorded",
        "AuctionRecord" => "Number introduced",
        "AuctionOpenRecord" => "Auction opened",
        "ExtensionEventRecord" => "Closing time extended",
        "AuctionCloseRecord" => "Auction closed",
        "ResolutionRecord" => "Winner determined",
        "SettlementRecord" => "Settlement recorded",
        _ => "Record",
    }
}
pub(super) fn show_records(records: &[Value]) {
    if records.is_empty() {
        println!("  No new records.");
    }
    for record in records {
        println!(
            "  #{} {} · {}",
            record["sequence_index"],
            label(record),
            record["group_type"].as_str().unwrap_or("")
        );
    }
}
pub(super) fn show_result(response: &Value) {
    if response["status"] == "error" {
        println!(
            "Command unavailable: {}",
            response["error"]["message"]
                .as_str()
                .unwrap_or("Inspect raw response")
        );
    } else if response["data"]["accepted"] == false {
        let reason = response["data"]["bid_record"]["payload_json"]["rejection_reason"]
            .as_str()
            .unwrap_or("");
        println!("Bid rejected: {}", Fault::new(reason).message);
        println!("The invalid bid is recorded; it moves no rana.");
    } else if response["data"]["accepted"] == true {
        println!("Command accepted and committed.");
    }
}
