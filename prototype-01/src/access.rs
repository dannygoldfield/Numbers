//! P01/P02/SE07. Host-trusted gateway assertions, outside the auction planner.
use crate::{Fault, Result};
use serde_json::{json, Value};
use tiny_http::Header;

pub const BODY_LIMIT: u64 = 4096;
pub const SPEC_REVISION: &str = "0.3";
#[derive(Clone, Debug)]
pub struct Access {
    pub origin: String,
    pub session_label: String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Principal {
    BidderA,
    BidderB,
    Facilitator,
}
impl Principal {
    pub fn bidder(self) -> Option<&'static str> {
        match self {
            Self::BidderA => Some("demo-1"),
            Self::BidderB => Some("demo-2"),
            Self::Facilitator => None,
        }
    }
    pub fn authorize(self, action: &str, body: &[u8]) -> Result<()> {
        if action == "settlement" && self != Self::Facilitator
            || action == "bid" && self == Self::Facilitator
        {
            return Err(Fault::new("forbidden"));
        }
        if action == "bid" {
            // route() has already validated the original JSON, including duplicate keys.
            let command: Value =
                serde_json::from_slice(body).map_err(|_| Fault::new("malformed_request"))?;
            if let Some(bidder) = command["bidder_id"].as_str() {
                if Some(bidder) != self.bidder() {
                    return Err(Fault::new("forbidden"));
                }
            }
        }
        Ok(())
    }
}
fn single<'a>(headers: &'a [Header], name: &'static str) -> Result<Option<&'a str>> {
    let mut values = headers.iter().filter(|h| h.field.equiv(name));
    let first = values.next().map(|h| h.value.as_str());
    if values.next().is_some() {
        return Err(Fault::new("malformed_request"));
    }
    Ok(first)
}
pub fn principal(headers: &[Header]) -> Result<Principal> {
    // No credential or caller-selected identity is parsed from the command or URL.
    match single(headers, "X-Numbers-Principal") {
        Ok(Some("bidder-a")) => Ok(Principal::BidderA),
        Ok(Some("bidder-b")) => Ok(Principal::BidderB),
        Ok(Some("facilitator")) => Ok(Principal::Facilitator),
        _ => Err(Fault::new("authentication_required")),
    }
}
impl Access {
    pub fn new(origin: String, session_label: String) -> std::result::Result<Self, String> {
        if session_label.is_empty()
            || session_label.len() > 64
            || !session_label
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
        {
            return Err(
                "session_label must contain 1–64 ASCII letters, digits, hyphens or underscores"
                    .into(),
            );
        }
        let authority = origin
            .strip_prefix("https://")
            .ok_or("allowed_origin must be an explicit HTTPS origin")?;
        let (host, port) = if let Some(rest) = authority.strip_prefix('[') {
            let (host, tail) = rest.split_once(']').ok_or("Invalid IPv6 origin")?;
            host.parse::<std::net::Ipv6Addr>()
                .map_err(|_| "Invalid IPv6 origin")?;
            (
                host,
                if tail.is_empty() {
                    None
                } else {
                    Some(tail.strip_prefix(':').ok_or("Invalid origin port")?)
                },
            )
        } else {
            let (host, port) = authority
                .split_once(':')
                .map_or((authority, None), |(h, p)| (h, Some(p)));
            if host.is_empty()
                || host.len() > 253
                || host.split('.').any(|part| {
                    part.is_empty()
                        || part.len() > 63
                        || part.starts_with('-')
                        || part.ends_with('-')
                        || !part
                            .bytes()
                            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
                })
            {
                return Err("allowed_origin must use a lowercase host with no path, wildcard, credentials, query or fragment".into());
            }
            (host, port)
        };
        let _ = host;
        if let Some(port) = port {
            if port.is_empty()
                || !port.bytes().all(|b| b.is_ascii_digit())
                || port.parse::<u16>().ok().filter(|p| *p > 0).is_none()
            {
                return Err("Invalid origin port".into());
            }
        }
        Ok(Self {
            origin,
            session_label,
        })
    }
    pub fn metadata(&self, principal: Principal) -> Value {
        json!({"status":"success","data":{"role": if principal.bidder().is_some() {"bidder"} else {"facilitator"}, "bidder_id":principal.bidder(),"session_label":self.session_label,"spec_revision":SPEC_REVISION}})
    }
    pub fn transport(&self, method: &str, headers: &[Header], length: Option<usize>) -> Result<()> {
        if length.is_some_and(|n| n as u64 > BODY_LIMIT) {
            return Err(Fault::new("request_too_large"));
        }
        let origin = single(headers, "Origin")?;
        if method == "POST" && origin != Some(self.origin.as_str())
            || origin.is_some_and(|o| o != self.origin)
            || matches!(
                single(headers, "Sec-Fetch-Site")?,
                Some("cross-site" | "same-site")
            )
        {
            return Err(Fault::new("forbidden"));
        }
        if method == "POST" {
            let media = single(headers, "Content-Type")?.unwrap_or("");
            let mut parts = media.split(';').map(str::trim);
            let json = parts
                .next()
                .is_some_and(|v| v.eq_ignore_ascii_case("application/json"));
            let charset = parts.next().is_none_or(|v| {
                v.split_once('=').is_some_and(|(k, v)| {
                    k.trim().eq_ignore_ascii_case("charset")
                        && (v.trim().eq_ignore_ascii_case("utf-8")
                            || v.trim().eq_ignore_ascii_case("\"utf-8\""))
                })
            });
            if !json
                || !charset
                || parts.next().is_some()
                || single(headers, "Content-Encoding")?
                    .is_some_and(|v| !v.eq_ignore_ascii_case("identity"))
            {
                return Err(Fault::new("unsupported_media_type"));
            }
        }
        Ok(())
    }
}
