//! SE02 transport gates. The HTTP loop owns the only mutable Engine.
//!
//! Static files are presentation resources. Only the four specified API endpoints
//! reach the authoritative evaluator; malformed transport never does.
use crate::{store::Engine, values::command, Fault};
use num_bigint::BigInt;
use serde_json::Value;
use std::collections::BTreeMap;
use tiny_http::{Header, Request, Response, StatusCode};

pub struct Input {
    pub action: &'static str,
    pub limit: usize,
    pub offset: BigInt,
}

pub fn route(method: &str, url: &str, body: &[u8]) -> crate::Result<Input> {
    let (path, query) = url.split_once('?').unwrap_or((url, ""));
    let (required, action) = match path {
        "/state" => ("GET", "state"),
        "/auction/history" => ("GET", "history"),
        "/bid" => ("POST", "bid"),
        "/demo/settlement" => ("POST", "settlement"),
        _ => return Err(Fault::new("unknown_endpoint")),
    };
    if method != required {
        return Err(Fault::new("method_not_allowed"));
    }
    let mut input = Input {
        action,
        limit: 50,
        offset: BigInt::from(0),
    };
    if action == "history" {
        let mut parameters = BTreeMap::new();
        if !query.is_empty() {
            for pair in query.split('&') {
                let (key, value) = pair
                    .split_once('=')
                    .ok_or_else(|| Fault::new("invalid_read_parameters"))?;
                if !["limit", "offset"].contains(&key)
                    || parameters.insert(key, value).is_some()
                    || value.is_empty()
                    || !value.bytes().all(|b| b.is_ascii_digit())
                {
                    return Err(Fault::new("invalid_read_parameters"));
                }
            }
        }
        for (key, value) in parameters {
            if key == "limit" {
                input.limit = value
                    .parse::<usize>()
                    .map_err(|_| Fault::new("invalid_read_parameters"))?;
            } else {
                input.offset = value
                    .parse::<BigInt>()
                    .map_err(|_| Fault::new("invalid_read_parameters"))?;
            }
        }
        if !(1..=100).contains(&input.limit) {
            return Err(Fault::new("invalid_read_parameters"));
        }
    } else if !query.is_empty() {
        return Err(Fault::new(if required == "GET" {
            "invalid_read_parameters"
        } else {
            "malformed_request"
        }));
    }
    if required == "POST" {
        command(body)?;
    }
    Ok(input)
}

pub fn respond_json(request: Request, value: &Value) -> std::io::Result<()> {
    let status = if value["status"] == "success" {
        200
    } else {
        match value["error"]["code"].as_str().unwrap_or("") {
            "unknown_endpoint" => 404,
            "method_not_allowed" => 405,
            "storage_unavailable"
            | "history_invalid"
            | "clock_invalid"
            | "arithmetic_unrepresentable" => 503,
            _ => 400,
        }
    };
    request.respond(
        Response::from_string(value.to_string())
            .with_status_code(StatusCode(status))
            .with_header(header("Content-Type", "application/json; charset=utf-8"))
            .with_header(header("Cache-Control", "no-store")),
    )
}
fn header(name: &str, value: &str) -> Header {
    Header::from_bytes(name, value).expect("fixed ASCII header")
}

pub fn serve_request(mut request: Request, engine: &mut Engine) -> std::io::Result<()> {
    let resource = match request.url() {
        "/" | "/protocol" => Some((
            "text/html; charset=utf-8",
            include_str!("../static/index.html"),
        )),
        "/app.js" => Some((
            "text/javascript; charset=utf-8",
            include_str!("../static/app.js"),
        )),
        "/style.css" => Some((
            "text/css; charset=utf-8",
            include_str!("../static/style.css"),
        )),
        _ => None,
    };
    if let Some((mime, contents)) = resource {
        if request.method().as_str() != "GET" {
            return respond_json(request, &Fault::new("method_not_allowed").response(&[]));
        }
        return request.respond(
            Response::from_string(contents)
                .with_header(header("Content-Type", mime))
                .with_header(header("Cache-Control", "no-store")),
        );
    }
    let mut body = Vec::new();
    if request.as_reader().read_to_end(&mut body).is_err() {
        return respond_json(request, &Fault::new("malformed_request").response(&[]));
    }
    match route(request.method().as_str(), request.url(), &body) {
        Ok(input) => {
            // SE04: time is sampled here, after transport and exclusive writer entry.
            let result = engine.request(
                input.action,
                Some(&body),
                crate::values::now(),
                input.limit,
                input.offset,
            );
            // A lost HTTP response never causes a command to be replayed.
            respond_json(request, &result)
        }
        Err(fault) => respond_json(request, &fault.response(&[])),
    }
}
