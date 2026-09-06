//! PR02 exact JSON, raw commands, NFC payloads and UTC millisecond arithmetic.
use crate::{Fault, Result};
use chrono::{DateTime, SecondsFormat, Utc};
use num_bigint::BigInt;
use serde::de::{self, MapAccess, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer};
use serde_json::{value::RawValue, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use unicode_normalization::UnicodeNormalization;

struct Unique;
impl<'de> Deserialize<'de> for Unique {
    fn deserialize<D: Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        struct Check;
        impl<'de> Visitor<'de> for Check {
            type Value = Unique;
            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("JSON with unique keys")
            }
            fn visit_bool<E: de::Error>(self, _: bool) -> std::result::Result<Unique, E> {
                Ok(Unique)
            }
            fn visit_i64<E: de::Error>(self, _: i64) -> std::result::Result<Unique, E> {
                Ok(Unique)
            }
            fn visit_u64<E: de::Error>(self, _: u64) -> std::result::Result<Unique, E> {
                Ok(Unique)
            }
            fn visit_f64<E: de::Error>(self, _: f64) -> std::result::Result<Unique, E> {
                Ok(Unique)
            }
            fn visit_str<E: de::Error>(self, _: &str) -> std::result::Result<Unique, E> {
                Ok(Unique)
            }
            fn visit_unit<E: de::Error>(self) -> std::result::Result<Unique, E> {
                Ok(Unique)
            }
            fn visit_seq<A: SeqAccess<'de>>(
                self,
                mut a: A,
            ) -> std::result::Result<Unique, A::Error> {
                while a.next_element::<Unique>()?.is_some() {}
                Ok(Unique)
            }
            fn visit_map<A: MapAccess<'de>>(
                self,
                mut a: A,
            ) -> std::result::Result<Unique, A::Error> {
                let mut keys = BTreeSet::new();
                while let Some(key) = a.next_key::<String>()? {
                    if !keys.insert(key) {
                        return Err(de::Error::custom("duplicate key"));
                    }
                    a.next_value::<Unique>()?;
                }
                Ok(Unique)
            }
        }
        d.deserialize_any(Check)
    }
}

pub type Command = BTreeMap<String, Box<RawValue>>;
pub fn command(raw: &[u8]) -> Result<Command> {
    serde_json::from_slice::<Unique>(raw).map_err(|_| Fault::new("malformed_request"))?;
    serde_json::from_slice::<Command>(raw).map_err(|_| Fault::new("malformed_request"))
}
pub fn exact_token(s: &str) -> Option<BigInt> {
    let digits = s.strip_prefix('-').unwrap_or(s);
    if digits.is_empty()
        || !digits.bytes().all(|b| b.is_ascii_digit())
        || (digits.len() > 1 && digits.starts_with('0'))
    {
        return None;
    }
    s.parse().ok()
}
pub fn cmd_int(c: &Command, key: &str) -> Option<BigInt> {
    exact_token(c.get(key)?.get())
}
pub fn cmd_str(c: &Command, key: &str) -> Option<String> {
    serde_json::from_str(c.get(key)?.get()).ok()
}
pub fn number(v: &Value) -> Result<BigInt> {
    if !v.is_number() {
        return Err(Fault::history("expected exact integer"));
    }
    exact_token(&v.to_string()).ok_or_else(|| Fault::history("expected exact integer"))
}
pub fn jnum(n: &BigInt) -> Value {
    serde_json::from_str(&n.to_string()).expect("BigInt is valid JSON")
}
pub fn text(v: &Value) -> Result<&str> {
    v.as_str().ok_or_else(|| Fault::history("expected string"))
}
pub fn u64v(v: &Value) -> Result<u64> {
    v.as_u64()
        .ok_or_else(|| Fault::history("expected nonnegative bounded integer"))
}
pub fn normalized(v: &Value) -> Value {
    match v {
        Value::String(s) => Value::String(s.nfc().collect()),
        Value::Array(a) => Value::Array(a.iter().map(normalized).collect()),
        Value::Object(o) => {
            Value::Object(o.iter().map(|(k, v)| (k.clone(), normalized(v))).collect())
        }
        _ => v.clone(),
    }
}
pub fn canonical(v: &Value) -> String {
    normalized(v).to_string()
}
pub fn hash(v: &Value) -> String {
    hex::encode(Sha256::digest(canonical(v).as_bytes()))
}
pub fn now() -> i64 {
    Utc::now().timestamp_millis()
}
pub fn stamp(ms: i64) -> Result<String> {
    let dt = DateTime::<Utc>::from_timestamp_millis(ms).ok_or_else(Fault::overflow)?;
    let s = dt.to_rfc3339_opts(SecondsFormat::Millis, true);
    if s.len() != 24 {
        return Err(Fault::overflow());
    }
    Ok(s)
}
pub fn millis(s: &str) -> Result<i64> {
    let dt = DateTime::parse_from_rfc3339(s).map_err(|_| Fault::history("invalid timestamp"))?;
    let t = dt.timestamp_millis();
    if stamp(t)? != s {
        return Err(Fault::history(
            "timestamp must be canonical UTC milliseconds",
        ));
    }
    Ok(t)
}
pub fn add_seconds(t: i64, seconds: &Value) -> Result<i64> {
    let delta = number(seconds)? * BigInt::from(1000);
    let result = (BigInt::from(t) + delta)
        .to_string()
        .parse::<i64>()
        .map_err(|_| Fault::overflow())?;
    stamp(result)?;
    Ok(result)
}
pub fn id(prefix: &str, n: &BigInt) -> String {
    format!("{}_{:0>12}", prefix, n.to_string())
}
pub fn record_id(n: usize) -> String {
    format!("rec_{n:012}")
}
pub fn bid_id(n: usize) -> String {
    format!("bid_{n:012}")
}
