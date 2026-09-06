pub mod http;
pub mod model;
pub mod store;
pub mod values;
use serde_json::{json, Value};

#[derive(Debug, Clone)]
pub struct Fault {
    pub code: String,
    pub message: String,
}
pub type Result<T> = std::result::Result<T, Fault>;
impl Fault {
    pub fn new(code: &str) -> Self {
        let message=match code {
            "wrong_auction_number"=>"This bid names a different number.",
            "missing_required_field"=>"A required field is missing.",
            "malformed_field"=>"A field has the wrong name, type, or integer format.",
            "unknown_identity"=>"Choose one of the two simulated identities.",
            "amount_below_minimum"=>"The bid is below the minimum opening amount.",
            "amount_below_required_increment"=>"The bid does not meet the required increase.",
            "amount_above_maximum"=>"The bid exceeds this auction’s maximum.",
            "insufficient_available_rana"=>"There is insufficient available rana for this bid.",
            "auction_not_accepting_bids"=>"This auction is no longer accepting bids.",
            "settlement_not_available"=>"Settlement is available only after resolution and before finalization.",
            "wrong_auction"=>"The settlement command names a different auction.",
            "invalid_settlement_outcome"=>"Choose settled or expired for this demonstration.",
            "malformed_request"=>"Submit one valid UTF-8 JSON object with unique keys.",
            "invalid_read_parameters"=>"Use integer limit (1–100) and offset (0 or greater).",
            "unknown_endpoint"=>"This endpoint does not exist.",
            "method_not_allowed"=>"This method is not available for this endpoint.",
            "clock_invalid"=>"Server time moved behind the latest committed record. Processing stopped; no repair was performed.",
            "arithmetic_unrepresentable"=>"Arithmetic exceeds the supported timestamp or sequence range. Processing stopped before further commit.",
            _=>code
        };
        Self {
            code: code.into(),
            message: message.into(),
        }
    }
    pub fn history(message: impl Into<String>) -> Self {
        Self {
            code: "history_invalid".into(),
            message: message.into(),
        }
    }
    pub fn storage(message: impl Into<String>) -> Self {
        Self {
            code: "storage_unavailable".into(),
            message: message.into(),
        }
    }
    pub fn overflow() -> Self {
        Self::new("arithmetic_unrepresentable")
    }
    pub fn response(&self, evaluation: &[Value]) -> Value {
        json!({"status":"error","error":{"code":self.code,"message":self.message},"evaluation_records":evaluation})
    }
}
impl std::fmt::Display for Fault {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}
impl std::error::Error for Fault {}
