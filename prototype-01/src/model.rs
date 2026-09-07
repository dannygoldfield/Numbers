//! Pure rules: P02, RT01–RT05, SE01–SE06, UI02. No persistence or network I/O.
use crate::{values::*, Fault, Result};
use num_bigint::BigInt;
use serde_json::{json, Value};
use std::collections::BTreeSet;
pub const IDS: [&str; 2] = ["demo-1", "demo-2"];
pub const PARAMS: [&str; 8] = [
    "duration_seconds",
    "inter_auction_gap_seconds",
    "extension_window_seconds",
    "extension_increment_seconds",
    "max_extensions",
    "minimum_bid_rana",
    "minimum_increment_rana",
    "maximum_bid_rana",
];
fn zero() -> BigInt {
    BigInt::from(0)
}
fn assert_rule(ok: bool, message: &str) -> Result<()> {
    if ok {
        Ok(())
    } else {
        Err(Fault::history(message))
    }
}
fn exact_keys(value: &Value, keys: &[&str]) -> bool {
    value
        .as_object()
        .map(|object| {
            object.keys().map(String::as_str).collect::<BTreeSet<_>>()
                == keys.iter().copied().collect()
        })
        .unwrap_or(false)
}
pub fn parameters(input: &Value) -> Value {
    let mut payload = json!({});
    for key in PARAMS {
        payload[key] = input[key].clone();
    }
    payload
}
pub fn validate_params(payload: &Value) -> Result<()> {
    assert_rule(
        exact_keys(payload, &PARAMS),
        "unknown or missing auction parameter",
    )?;
    for key in PARAMS {
        if key == "maximum_bid_rana" {
            continue;
        }
        assert_rule(
            number(&payload[key])? >= BigInt::from(if key == "max_extensions" { 0 } else { 1 }),
            "invalid auction parameter",
        )?;
    }
    assert_rule(
        number(&payload["duration_seconds"])? == BigInt::from(225)
            && number(&payload["inter_auction_gap_seconds"])? == BigInt::from(12),
        "Prototype 1 requires duration 225 and gap 12",
    )?;
    if !payload["maximum_bid_rana"].is_null() {
        assert_rule(
            number(&payload["maximum_bid_rana"])? >= number(&payload["minimum_bid_rana"])?,
            "maximum below minimum",
        )?;
    }
    Ok(())
}
pub fn validate_config(mut input: Value) -> Result<Value> {
    let obj = input
        .as_object_mut()
        .ok_or_else(|| Fault::history("configuration must be an object"))?;
    obj.entry("starting_number").or_insert(json!(1));
    obj.entry("maximum_bid_rana").or_insert(Value::Null);
    let mut keys = PARAMS.to_vec();
    keys.extend([
        "starting_number",
        "demo_1_initial_rana",
        "demo_2_initial_rana",
    ]);
    assert_rule(
        exact_keys(&input, &keys),
        "unknown or missing behavioral configuration",
    )?;
    validate_params(&parameters(&input))?;
    assert_rule(
        number(&input["starting_number"])? > zero(),
        "invalid starting number",
    )?;
    for key in ["demo_1_initial_rana", "demo_2_initial_rana"] {
        assert_rule(number(&input[key])? >= zero(), "negative allocation")?;
    }
    Ok(input)
}

#[derive(Clone, Debug)]
pub struct State {
    pub records: Vec<Value>,
    pub number: Option<BigInt>,
    pub auction: Value,
    pub opening: Value,
    pub close: Value,
    pub resolution: Value,
    pub settlement: Value,
    pub ownership: Value,
    pub leader: Value,
    pub hold: Value,
    pub bids: Vec<Value>,
    pub extensions: usize,
    /// IDS order; each pair is [available, reserved], both exact Rana integers.
    pub balances: Vec<[BigInt; 2]>,
    pub issued: BigInt,
    pub protocol: BigInt,
}
impl Default for State {
    fn default() -> Self {
        Self {
            records: vec![],
            number: None,
            auction: Value::Null,
            opening: Value::Null,
            close: Value::Null,
            resolution: Value::Null,
            settlement: Value::Null,
            ownership: Value::Null,
            leader: Value::Null,
            hold: Value::Null,
            bids: vec![],
            extensions: 0,
            balances: vec![[zero(), zero()], [zero(), zero()]],
            issued: zero(),
            protocol: zero(),
        }
    }
}
impl State {
    pub fn phase(&self) -> &str {
        if !self.ownership.is_null() {
            "Finalized"
        } else if !self.resolution.is_null() {
            "AwaitingSettlement"
        } else if !self.close.is_null() {
            "Closed"
        } else if !self.opening.is_null() {
            "Open"
        } else if self.number.is_some() {
            "Scheduled"
        } else {
            "Empty"
        }
    }
    pub fn end(&self) -> Result<i64> {
        let base = millis(text(&self.opening["payload_json"]["base_end_time"])?)?;
        add_seconds(
            base,
            &jnum(
                &(number(&self.auction["extension_increment_seconds"])?
                    * BigInt::from(self.extensions)),
            ),
        )
    }
    pub fn next_time(&self) -> Result<i64> {
        add_seconds(
            millis(text(&self.ownership["server_time"])?)?,
            &self.auction["inter_auction_gap_seconds"],
        )
    }
    pub fn clock_check(&self, time_ms: i64) -> Result<()> {
        stamp(time_ms)?;
        if let Some(last) = self.records.last() {
            if time_ms < millis(text(&last["server_time"])?)? {
                return Err(Fault::new("clock_invalid"));
            }
        }
        Ok(())
    }
    pub fn fold(&mut self, group: &[Value]) -> Result<()> {
        for record in group {
            let payload = &record["payload_json"];
            match text(&record["record_type"])? {
                "RanaIssueRecord" => {
                    let i = identity(text(&payload["bidder_id"])?)?;
                    let a = number(&payload["amount_rana"])?;
                    self.balances[i][0] += &a;
                    self.issued += a;
                }
                "AuctionRecord" => {
                    self.number = Some(number(&record["number"])?);
                    self.auction = payload.clone();
                    self.opening = Value::Null;
                    self.close = Value::Null;
                    self.resolution = Value::Null;
                    self.settlement = Value::Null;
                    self.ownership = Value::Null;
                    self.leader = Value::Null;
                    self.hold = Value::Null;
                    self.bids.clear();
                    self.extensions = 0;
                }
                "BidRecord" => {
                    if payload["validity"] == "valid" {
                        self.leader = record.clone();
                        self.bids.push(record.clone());
                    }
                }
                "RanaReserveRecord" => {
                    let i = identity(text(&payload["bidder_id"])?)?;
                    let a = number(&payload["amount_rana"])?;
                    self.balances[i][0] -= &a;
                    self.balances[i][1] += a;
                    self.hold = record.clone();
                }
                "RanaReleaseRecord" | "RanaCaptureRecord" => {
                    let i = identity(text(&payload["bidder_id"])?)?;
                    let a = number(&payload["amount_rana"])?;
                    self.balances[i][1] -= &a;
                    if record["record_type"] == "RanaReleaseRecord" {
                        self.balances[i][0] += a;
                    } else {
                        self.protocol += a;
                    }
                    self.hold = Value::Null;
                }
                "AuctionOpenRecord" => self.opening = record.clone(),
                "ExtensionEventRecord" => self.extensions += 1,
                "AuctionCloseRecord" => self.close = record.clone(),
                "ResolutionRecord" => self.resolution = record.clone(),
                "SettlementRecord" => self.settlement = record.clone(),
                "FinalizationRecord" => self.ownership = record.clone(),
                _ => return Err(Fault::history("unknown record")),
            }
            self.records.push(record.clone());
        }
        let total = self
            .balances
            .iter()
            .flat_map(|entry| entry.iter())
            .fold(self.protocol.clone(), |a, entry| a + entry);
        assert_rule(total == self.issued, "Rana conservation failed")?;
        assert_rule(
            self.protocol >= zero() && self.balances.iter().flatten().all(|value| *value >= zero()),
            "negative balance",
        )?;
        let reserved = &self.balances[0][1] + &self.balances[1][1];
        if ["Open", "Closed", "AwaitingSettlement"].contains(&self.phase()) {
            assert_rule(
                !self.hold.is_null()
                    && self.hold["payload_json"]["bid_id"] == self.leader["payload_json"]["bid_id"],
                "missing leading hold",
            )?;
            assert_rule(
                reserved == number(&self.leader["payload_json"]["amount_rana"])?,
                "hold amount mismatch",
            )?;
        } else {
            assert_rule(self.hold.is_null() && reserved == zero(), "unexpected hold")?;
        }
        Ok(())
    }
}
fn identity(id: &str) -> Result<usize> {
    IDS.iter()
        .position(|x| *x == id)
        .ok_or_else(|| Fault::new("unknown_identity"))
}
/// PR01/PR02: assign global identifiers and seal each payload in one group.
fn records(
    state: &State,
    group_type: &str,
    time_ms: i64,
    auction_number: &BigInt,
    entries: Vec<(&str, Value)>,
) -> Result<Vec<Value>> {
    let first_sequence = state
        .records
        .len()
        .checked_add(1)
        .ok_or_else(Fault::overflow)?;
    let last_sequence = first_sequence
        .checked_add(entries.len() - 1)
        .ok_or_else(Fault::overflow)?;
    i64::try_from(last_sequence).map_err(|_| Fault::overflow())?;
    let group_size = entries.len();
    let server_time = stamp(time_ms)?;
    let mut result = Vec::with_capacity(group_size);
    for (index, (record_type, payload)) in entries.into_iter().enumerate() {
        let payload = normalized(&payload);
        let global_issue = record_type == "RanaIssueRecord";
        let number = if global_issue {
            Value::Null
        } else {
            jnum(auction_number)
        };
        let auction_id = if global_issue {
            Value::Null
        } else {
            json!(id("auc", auction_number))
        };
        result.push(json!({
            "sequence_index": first_sequence + index,
            "record_id": record_id(first_sequence + index),
            "record_type": record_type,
            "number": number,
            "auction_id": auction_id,
            "server_time": server_time,
            "group_id": record_id(first_sequence),
            "group_type": group_type,
            "group_index": index + 1,
            "group_size": group_size,
            "payload_hash": hash(&payload),
            "payload_json": payload
        }));
    }
    Ok(result)
}

/// SE03: the order below is the specified rejection priority, not an optimization.
fn rejection(state: &State, input: &Command) -> Result<Option<&'static str>> {
    let auction_number = cmd_int(input, "auction_number");
    let bidder = cmd_str(input, "bidder_id");
    let a = cmd_int(input, "amount_rana");
    if auction_number.is_some() && auction_number != state.number {
        return Ok(Some("wrong_auction_number"));
    }
    if ["auction_number", "bidder_id", "amount_rana"]
        .iter()
        .any(|key| !input.contains_key(*key))
    {
        return Ok(Some("missing_required_field"));
    }
    if input.len() != 3 || auction_number.is_none() || bidder.is_none() || a.is_none() {
        return Ok(Some("malformed_field"));
    }
    let bidder = bidder.unwrap();
    let a = a.unwrap();
    let i = match identity(&bidder) {
        Ok(i) => i,
        Err(_) => return Ok(Some("unknown_identity")),
    };
    if a < number(&state.auction["minimum_bid_rana"])? {
        return Ok(Some("amount_below_minimum"));
    }
    if !state.leader.is_null()
        && a < number(&state.leader["payload_json"]["amount_rana"])?
            + number(&state.auction["minimum_increment_rana"])?
    {
        return Ok(Some("amount_below_required_increment"));
    }
    if !state.auction["maximum_bid_rana"].is_null()
        && a > number(&state.auction["maximum_bid_rana"])?
    {
        return Ok(Some("amount_above_maximum"));
    }
    let own = if !state.hold.is_null() && state.hold["payload_json"]["bidder_id"] == bidder {
        number(&state.hold["payload_json"]["amount_rana"])?
    } else {
        zero()
    };
    if a > &state.balances[i][0] + own {
        return Ok(Some("insufficient_available_rana"));
    }
    Ok(None)
}

/// SE05/SE06: plan exactly one complete group without changing state.
///
/// Reconstruction uses the same pure rules as a consistency check. It never
/// commits a planned group or replays an external effect.
pub fn plan(
    state: &State,
    action: &str,
    time_ms: i64,
    raw: Option<&[u8]>,
    config: &Value,
) -> Result<Vec<Value>> {
    state.clock_check(time_ms)?;
    if action == "bid" {
        return plan_bid(state, time_ms, raw);
    }
    if action == "settlement" {
        return plan_settlement(state, time_ms, raw);
    }

    let server_time = stamp(time_ms)?;
    let mut auction_number = state.number.clone().unwrap_or_else(|| BigInt::from(1));
    let mut entries = vec![];
    match action {
        // G01/G10: initialization issues once; advancement only creates the next number.
        "initialize" | "advance" => {
            let config = validate_config(config.clone())?;
            if action == "initialize" {
                assert_rule(state.records.is_empty(), "second initialization")?;
                auction_number = number(&config["starting_number"])?;
                for (bidder_id, allocation_key) in IDS
                    .iter()
                    .zip(["demo_1_initial_rana", "demo_2_initial_rana"])
                {
                    entries.push((
                        "RanaIssueRecord",
                        json!({
                            "bidder_id": bidder_id,
                            "amount_rana": config[allocation_key]
                        }),
                    ));
                }
            } else {
                assert_rule(
                    state.phase() == "Finalized" && time_ms >= state.next_time()?,
                    "advance before finalization and gap",
                )?;
                auction_number += 1;
            }
            let mut payload = parameters(&config);
            payload["created_at"] = json!(server_time);
            entries.push(("AuctionRecord", payload));
        }
        // G06 and G07 are separate commits, permitting a durable Closed boundary.
        "close" => {
            assert_rule(
                state.phase() == "Open" && time_ms >= state.end()?,
                "close not due",
            )?;
            entries.push((
                "AuctionCloseRecord",
                json!({
                    "closed_at": server_time,
                    "reason": "duration_expired"
                }),
            ));
        }
        "resolve" => {
            assert_rule(
                state.phase() == "Closed" && !state.bids.is_empty(),
                "resolution not eligible",
            )?;
            let mut winner = &state.bids[0];
            let mut inputs = vec![];
            for bid in &state.bids {
                let payload = &bid["payload_json"];
                // Records are ordered: leaving an equal amount unchanged preserves the earliest tie.
                if number(&payload["amount_rana"])?
                    > number(&winner["payload_json"]["amount_rana"])?
                {
                    winner = bid;
                }
                inputs.push(json!({
                    "bid_id": payload["bid_id"],
                    "sequence_index": bid["sequence_index"],
                    "amount_rana": payload["amount_rana"],
                    "bidder_id": payload["bidder_id"],
                    "server_time": bid["server_time"]
                }));
            }
            entries.push((
                "ResolutionRecord",
                json!({
                    "winning_bid_id": winner["payload_json"]["bid_id"],
                    "winning_amount_rana": winner["payload_json"]["amount_rana"],
                    "resolution_time": server_time,
                    "resolution_inputs_hash": hash(&json!(inputs))
                }),
            ));
        }
        _ => return Err(Fault::history("unknown transition")),
    }
    records(state, action, time_ms, &auction_number, entries)
}

/// RT02 and SE03: admission is evaluated against the old hold before replacement.
fn plan_bid(state: &State, time_ms: i64, raw: Option<&[u8]>) -> Result<Vec<Value>> {
    if !["Scheduled", "Open"].contains(&state.phase())
        || (state.phase() == "Open" && time_ms >= state.end()?)
    {
        return Err(Fault::new("auction_not_accepting_bids"));
    }
    let raw = raw.ok_or_else(|| Fault::new("malformed_request"))?;
    let input = command(raw)?;
    let reason = rejection(state, &input)?;
    let bidder = cmd_str(&input, "bidder_id");
    let amount = cmd_int(&input, "amount_rana");
    let submitted_number = cmd_int(&input, "auction_number").as_ref().map(jnum);
    let bid_id = bid_id(state.records.len() + 1);
    let validity = if reason.is_some() { "invalid" } else { "valid" };
    let mut entries = vec![(
        "BidRecord",
        json!({
            "bid_id": bid_id,
            "command_utf8_hex": hex::encode(raw),
            "submitted_auction_number": submitted_number,
            "bidder_id": bidder,
            "amount_rana": amount.as_ref().map(jnum),
            "validity": validity,
            "rejection_reason": reason
        }),
    )];
    let group_type;
    if reason.is_some() {
        group_type = "bid_rejected"; // G05 records the rejection without moving Rana.
    } else {
        // These fields are present because all admission predicates passed above.
        let bidder = bidder.expect("admitted identity");
        let amount = jnum(&amount.expect("admitted integer"));
        if !state.hold.is_null() {
            let hold = &state.hold["payload_json"];
            let reason = if hold["bidder_id"] == bidder {
                "replaced"
            } else {
                "outbid"
            };
            entries.push((
                "RanaReleaseRecord",
                json!({
                    "reservation_record_id": state.hold["record_id"],
                    "bid_id": hold["bid_id"],
                    "bidder_id": hold["bidder_id"],
                    "amount_rana": hold["amount_rana"],
                    "reason": reason
                }),
            ));
        }
        entries.push((
            "RanaReserveRecord",
            json!({
                "bid_id": bid_id,
                "bidder_id": bidder,
                "amount_rana": amount
            }),
        ));
        if state.phase() == "Scheduled" {
            group_type = "bid_open"; // G02: the opening bid never triggers an extension.
            let base_end_time = stamp(add_seconds(time_ms, &state.auction["duration_seconds"])?)?;
            entries.push((
                "AuctionOpenRecord",
                json!({
                    "opening_bid_id": bid_id,
                    "opened_at": stamp(time_ms)?,
                    "base_end_time": base_end_time
                }),
            ));
        } else {
            let window_start = BigInt::from(state.end()?)
                - number(&state.auction["extension_window_seconds"])? * 1000;
            let in_window = BigInt::from(time_ms) >= window_start;
            let below_cap =
                BigInt::from(state.extensions) < number(&state.auction["max_extensions"])?;
            if in_window && below_cap {
                group_type = "bid_extend"; // G04: one accepted bid adds one extension.
                add_seconds(state.end()?, &state.auction["extension_increment_seconds"])?;
                entries.push((
                    "ExtensionEventRecord",
                    json!({
                        "trigger_bid_id": bid_id,
                        "extension_increment_seconds": state.auction["extension_increment_seconds"],
                        "extension_index": state.extensions + 1
                    }),
                ));
            } else {
                group_type = "bid_replace";
            } // G03
        }
    }
    records(
        state,
        group_type,
        time_ms,
        state.number.as_ref().expect("existing auction"),
        entries,
    )
}

/// RT03–RT05: fixed winner → one terminal group → no live hold.
fn plan_settlement(state: &State, time_ms: i64, raw: Option<&[u8]>) -> Result<Vec<Value>> {
    if state.phase() != "AwaitingSettlement" {
        return Err(Fault::new("settlement_not_available"));
    }
    let raw = raw.ok_or_else(|| Fault::new("malformed_request"))?;
    let input = command(raw)?;
    if !input.contains_key("auction_id") || !input.contains_key("outcome") {
        return Err(Fault::new("missing_required_field"));
    }
    let auction_id = cmd_str(&input, "auction_id");
    let outcome = cmd_str(&input, "outcome");
    let valid_id = auction_id
        .as_deref()
        .and_then(|value| value.strip_prefix("auc_"))
        .is_some_and(|digits| {
            digits.len() >= 12 && digits.bytes().all(|byte| byte.is_ascii_digit())
        });
    if input.len() != 2 || !valid_id || outcome.is_none() {
        return Err(Fault::new("malformed_field"));
    }
    let auction_number = state.number.as_ref().expect("resolved auction");
    if auction_id.expect("validated ID") != id("auc", auction_number) {
        return Err(Fault::new("wrong_auction"));
    }
    let outcome = outcome.expect("validated outcome string");
    if !["settled", "expired"].contains(&outcome.as_str()) {
        return Err(Fault::new("invalid_settlement_outcome"));
    }
    let hold = &state.hold["payload_json"];
    assert_rule(
        !state.hold.is_null()
            && hold["bid_id"] == state.resolution["payload_json"]["winning_bid_id"]
            && hold["amount_rana"] == state.resolution["payload_json"]["winning_amount_rana"],
        "winning hold mismatch",
    )?;

    let success = outcome == "settled";
    let group_type = if success { "settle" } else { "expire" };
    let ledger_type = if success {
        "RanaCaptureRecord"
    } else {
        "RanaReleaseRecord"
    };
    let ownership_kind = if success { "winner" } else { "PublicLand" };
    let holder_id = if success {
        hold["bidder_id"].clone()
    } else {
        Value::Null
    };
    let finalization_reason = if success {
        "settled_to_winner"
    } else {
        "expired_to_publicland"
    };
    let server_time = stamp(time_ms)?;
    let settlement_id = record_id(state.records.len() + 1);
    let mut ledger = json!({
        "reservation_record_id": state.hold["record_id"],
        "bid_id": hold["bid_id"],
        "bidder_id": hold["bidder_id"],
        "amount_rana": hold["amount_rana"]
    });
    if success {
        ledger["recipient"] = json!("protocol");
    } else {
        ledger["reason"] = json!("settlement_expired");
    }
    let entries = vec![
        (
            "SettlementRecord",
            json!({
                "command_utf8_hex": hex::encode(raw),
                "resolution_record_id": state.resolution["record_id"],
                "status": outcome,
                "settlement_time": server_time
            }),
        ),
        (ledger_type, ledger),
        (
            "FinalizationRecord",
            json!({
                "settlement_record_id": settlement_id,
                "title_kind": ownership_kind,
                "holder_id": holder_id,
                "finalization_time": server_time,
                "finalization_reason": finalization_reason
            }),
        ),
    ];
    add_seconds(time_ms, &state.auction["inter_auction_gap_seconds"])?;
    records(state, group_type, time_ms, auction_number, entries)
}

pub fn shape(kind: &str) -> Option<usize> {
    match kind {
        "initialize" | "bid_open" | "bid_replace" | "settle" | "expire" => Some(3),
        "bid_extend" => Some(4),
        "bid_rejected" | "close" | "resolve" | "advance" => Some(1),
        _ => None,
    }
}
pub fn reconstruct(rows: &[(i64, String)], live: &Value) -> Result<State> {
    let mut state = State::default();
    let mut offset = 0;
    while offset < rows.len() {
        let validated = (|| -> Result<usize> {
            let first: Value = serde_json::from_str(&rows[offset].1)
                .map_err(|_| Fault::history("invalid stored JSON"))?;
            let kind = text(&first["group_type"])?;
            let size = shape(kind).ok_or_else(|| Fault::history("unknown group"))?;
            assert_rule(offset + size <= rows.len(), "incomplete group")?;
            let mut group = vec![];
            for (position, raw) in &rows[offset..offset + size] {
                let record: Value =
                    serde_json::from_str(raw).map_err(|_| Fault::history("invalid record JSON"))?;
                assert_rule(
                    canonical(&record) == *raw && record["sequence_index"] == json!(position),
                    "noncanonical record or sequence",
                )?;
                group.push(record);
            }
            let time_ms = millis(text(&first["server_time"])?)?;
            let mut config = live.clone();
            let body;
            let (action, raw) = if kind == "initialize" || kind == "advance" {
                let a = &group[size - 1];
                for key in PARAMS {
                    config[key] = a["payload_json"][key].clone();
                }
                if kind == "initialize" {
                    config["starting_number"] = a["number"].clone();
                    config["demo_1_initial_rana"] = group[0]["payload_json"]["amount_rana"].clone();
                    config["demo_2_initial_rana"] = group[1]["payload_json"]["amount_rana"].clone();
                }
                (kind, None)
            } else if kind.starts_with("bid_") || kind == "settle" || kind == "expire" {
                body = hex::decode(text(&first["payload_json"]["command_utf8_hex"])?)
                    .map_err(|_| Fault::history("invalid command encoding"))?;
                (
                    if kind.starts_with("bid_") {
                        "bid"
                    } else {
                        "settlement"
                    },
                    Some(body.as_slice()),
                )
            } else {
                (kind, None)
            };
            let expected = plan(&state, action, time_ms, raw, &config)?;
            assert_rule(
                canonical(&json!(group)) == canonical(&json!(expected)),
                "group violates schema, hashes, references, or transition",
            )?;
            state.fold(&group)?;
            Ok(size)
        })();
        match validated {
            Ok(size) => offset += size,
            Err(error) => {
                return Err(Fault::history(format!(
                    "Invalid history at group starting sequence {}: {}. No repair was performed.",
                    offset + 1,
                    error
                )))
            }
        }
    }
    Ok(state)
}
fn select(record: &Value, keys: &[&str]) -> Value {
    if record.is_null() {
        return Value::Null;
    }
    let mut payload = json!({
        "record_id": record["record_id"]
    });
    for key in keys {
        payload[*key] = record["payload_json"][*key].clone();
    }
    payload
}
/// UI02: project only recorded facts and specified derivations; no outcomes are chosen here.
pub fn projection(state: &State, time_ms: i64, validated: usize) -> Result<Value> {
    let auction_number = state
        .number
        .as_ref()
        .ok_or_else(|| Fault::history("uninitialized projection"))?;
    let mut resolution = select(
        &state.resolution,
        &["winning_bid_id", "winning_amount_rana", "resolution_time"],
    );
    if !resolution.is_null() {
        resolution["winning_bidder_id"] = state.leader["payload_json"]["bidder_id"].clone();
    }
    let leading_bid = if state.leader.is_null() {
        Value::Null
    } else {
        let payload = &state.leader["payload_json"];
        json!({
            "bid_id": payload["bid_id"],
            "bidder_id": payload["bidder_id"],
            "amount_rana": payload["amount_rana"]
        })
    };
    let finalized = !state.ownership.is_null();
    let sequence_phase = if finalized {
        "rhythm_gap"
    } else if state.phase() == "Scheduled" {
        "auction_available"
    } else {
        "auction_active"
    };
    let next_number = if finalized {
        jnum(&(auction_number + 1))
    } else {
        Value::Null
    };
    let next_available_at = if finalized {
        json!(stamp(state.next_time()?)?)
    } else {
        Value::Null
    };
    let current_end_time = if state.opening.is_null() {
        Value::Null
    } else {
        json!(stamp(state.end()?)?)
    };
    let balances: Vec<Value> = IDS
        .iter()
        .enumerate()
        .map(|(index, bidder_id)| {
            json!({
                "bidder_id": bidder_id,
                "available_rana": jnum(&state.balances[index][0]),
                "reserved_rana": jnum(&state.balances[index][1])
            })
        })
        .collect();
    let settlement = select(&state.settlement, &["status", "settlement_time"]);
    let ownership = select(
        &state.ownership,
        &["title_kind", "holder_id", "finalization_time"],
    );
    let last = state
        .records
        .last()
        .ok_or_else(|| Fault::history("empty projection"))?;
    Ok(json!({
        "server_time": stamp(time_ms)?,
        "unit": "RANA",
        "current_number": jnum(auction_number),
        "auction_id": id("auc", auction_number),
        "auction_state": state.phase(),
        "parameters": parameters(&state.auction),
        "opened_at": state.opening["payload_json"]["opened_at"],
        "base_end_time": state.opening["payload_json"]["base_end_time"],
        "extension_count": state.extensions,
        "current_end_time": current_end_time,
        "closed_at": state.close["payload_json"]["closed_at"],
        "leading_bid": leading_bid,
        "resolution": resolution,
        "settlement": settlement,
        "title": ownership, // RT06: retain the revision 0.1 wire key.
        "balances": balances,
        "protocol_held_rana": jnum(&state.protocol),
        "sequence": {
            "phase": sequence_phase,
            "next_number": next_number,
            "next_available_at": next_available_at
        },
        "last_sequence_index": state.records.len(),
        "last_group_id": last["group_id"],
        "reconstruction": { "validated_through_sequence_index": validated }
    }))
}
