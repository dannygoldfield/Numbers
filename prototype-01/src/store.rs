//! PR03–PR06. One writer, whole groups, durable commit, pure startup reconstruction.
use crate::{model::*, values::*, Fault, Result};
use fs2::FileExt;
use num_bigint::BigInt;
use rusqlite::{params, Connection, TransactionBehavior};
use serde_json::{json, Value};
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

/// Owns the journal and its derived cache for the lifetime of the single writer.
pub struct Engine {
    pub state: State,
    pub config: Value,
    pub validated: usize,
    pub halted: Option<Fault>,
    db: Connection,
    path: PathBuf,
    _writer: File,
}
impl Engine {
    /// No evaluation occurs here: callers explicitly evaluate after reconstruction (PR04).
    pub fn open(path: &Path, config: Value) -> Result<Self> {
        let config = validate_config(config)?;
        let attempt = (|| -> Result<Self> {
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent).map_err(|e| Fault::storage(e.to_string()))?;
            }
            let file = OpenOptions::new()
                .create(true)
                .read(true)
                .write(true)
                .truncate(false)
                .open(path)
                .map_err(|e| Fault::storage(e.to_string()))?;
            // macOS file locks can conflict with SQLite's own locks on the database.
            // Use a separate cooperative lock, named from the canonical database path.
            drop(file);
            let canonical_path = path
                .canonicalize()
                .map_err(|error| Fault::storage(error.to_string()))?;
            let lock_path = format!("{}.writer.lock", canonical_path.display());
            let file = OpenOptions::new()
                .create(true)
                .read(true)
                .write(true)
                .truncate(false)
                .open(lock_path)
                .map_err(|error| Fault::storage(error.to_string()))?;
            file.try_lock_exclusive().map_err(|error| {
                Fault::storage(format!("Cannot acquire the single-writer lock: {error}"))
            })?;
            let mut db = Connection::open(path)
                .map_err(|e| Fault::history(format!("Cannot read journal: {e}")))?;
            db.busy_timeout(std::time::Duration::ZERO)
                .map_err(|e| Fault::storage(e.to_string()))?;
            db.execute_batch("PRAGMA synchronous=FULL; PRAGMA fullfsync=ON;")
                .map_err(|e| Fault::storage(e.to_string()))?;
            let tables: Vec<String> = {
                let mut q = db
                    .prepare("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name")
                    .map_err(|e| Fault::history(e.to_string()))?;
                let out = q
                    .query_map([], |r| r.get(0))
                    .map_err(|e| Fault::history(e.to_string()))?
                    .collect::<std::result::Result<_, _>>()
                    .map_err(|e| Fault::history(e.to_string()))?;
                out
            };
            if tables.is_empty() {
                let tx = db
                    .transaction_with_behavior(TransactionBehavior::Immediate)
                    .map_err(|e| Fault::storage(e.to_string()))?;
                tx.execute_batch(
                    "CREATE TABLE events (
                        sequence_index INTEGER PRIMARY KEY CHECK(sequence_index > 0),
                        record_json TEXT NOT NULL
                    );
                    CREATE TRIGGER events_no_update BEFORE UPDATE ON events
                    BEGIN SELECT RAISE(ABORT, 'append-only journal'); END;
                    CREATE TRIGGER events_no_delete BEFORE DELETE ON events
                    BEGIN SELECT RAISE(ABORT, 'append-only journal'); END;",
                )
                .map_err(|error| Fault::storage(error.to_string()))?;
                tx.commit().map_err(|e| Fault::storage(e.to_string()))?;
            } else if tables != ["events"] {
                return Err(Fault::history(
                    "Unexpected database schema; no repair was performed.",
                ));
            }
            let check: String = db
                .query_row("PRAGMA quick_check", [], |r| r.get(0))
                .map_err(|e| Fault::history(e.to_string()))?;
            if check != "ok" {
                return Err(Fault::history(check));
            }
            let rows = load(&db)?;
            let state = reconstruct(&rows, &config)?;
            let validated = state.records.len();
            Ok(Self {
                state,
                config,
                validated,
                halted: None,
                db,
                path: path.to_path_buf(),
                _writer: file,
            })
        })();
        match attempt {
            Ok(e) => Ok(e),
            Err(mut f) => {
                diagnostic(path, &mut f);
                Err(f)
            }
        }
    }
    fn stop(&mut self, mut f: Fault) -> Fault {
        diagnostic(&self.path, &mut f);
        self.halted = Some(f.clone());
        f
    }
    /// PR03: validate the complete effect, commit once, then replace the derived cache.
    pub fn commit(&mut self, group: &[Value]) -> Result<()> {
        let mut next = self.state.clone();
        next.fold(group)?;
        let encoded: Vec<(i64, String)> = group
            .iter()
            .map(|r| {
                Ok((
                    i64::try_from(u64v(&r["sequence_index"])?).map_err(|_| Fault::overflow())?,
                    canonical(r),
                ))
            })
            .collect::<Result<_>>()?;
        let attempt = (|| -> std::result::Result<(), rusqlite::Error> {
            let tx = self
                .db
                .transaction_with_behavior(TransactionBehavior::Immediate)?;
            let last: i64 = tx.query_row(
                "SELECT COALESCE(MAX(sequence_index),0) FROM events",
                [],
                |r| r.get(0),
            )?;
            if last != self.state.records.len() as i64 {
                return Err(rusqlite::Error::InvalidQuery);
            }
            for (seq, record) in encoded {
                tx.execute("INSERT INTO events VALUES (?1,?2)", params![seq, record])?;
            }
            tx.commit()
        })();
        if let Err(e) = attempt {
            return Err(self.stop(Fault::storage(format!("Journal write failed: {e}. Durable outcome may be uncertain. Restart to inspect before submitting a new command; no automatic retry or repair was performed."))));
        }
        self.state = next;
        Ok(())
    }
    /// SE04: ordered due facts precede the command, each in its own declared group.
    pub fn evaluate(&mut self, t: i64, appended: &mut Vec<Value>) -> Result<()> {
        if let Some(f) = &self.halted {
            return Err(f.clone());
        }
        self.state.clock_check(t)?;
        for action in ["initialize", "close", "resolve", "advance"] {
            let s = &self.state;
            let due = match action {
                "initialize" => s.records.is_empty(),
                "close" => s.phase() == "Open" && t >= s.end()?,
                "resolve" => s.phase() == "Closed",
                "advance" => s.phase() == "Finalized" && t >= s.next_time()?,
                _ => false,
            };
            if due {
                let group = plan(&self.state, action, t, None, &self.config)?;
                self.commit(&group)?;
                appended.extend(group);
            }
        }
        Ok(())
    }
    pub fn request(
        &mut self,
        action: &str,
        raw: Option<&[u8]>,
        t: i64,
        limit: usize,
        offset: impl Into<BigInt>,
    ) -> Value {
        let offset = offset.into();
        let mut evaluation = vec![];
        let result = (|| -> Result<Value> {
            if action == "bid" || action == "settlement" {
                command(raw.ok_or_else(|| Fault::new("malformed_request"))?)?;
            }
            self.evaluate(t, &mut evaluation)?;
            if action == "state" {
                return Ok(json!({
                    "status": "success",
                    "data": projection(&self.state, t, self.validated)?
                }));
            }
            if action == "history" {
                let offset_index = offset.to_string().parse::<usize>().unwrap_or(usize::MAX);
                let end = offset_index
                    .saturating_add(limit)
                    .min(self.state.records.len());
                let records = if offset_index >= end {
                    vec![]
                } else {
                    self.state.records[offset_index..end].to_vec()
                };
                let next_offset = (end < self.state.records.len()).then_some(end);
                return Ok(json!({
                    "status": "success",
                    "data": {
                        "records": records,
                        "pagination": {
                            "limit": limit,
                            "offset": jnum(&offset),
                            "next_offset": next_offset
                        }
                    }
                }));
            }
            let group = plan(&self.state, action, t, raw, &self.config)?;
            self.commit(&group)?;
            let mut data = json!({
                "accepted": true,
                "evaluation_records": evaluation,
                "command_records": group,
                "state": projection(&self.state, t, self.validated)?
            });
            if action == "bid" {
                data["bid_record"] = group[0].clone();
                data["accepted"] = json!(group[0]["payload_json"]["validity"] == "valid");
            }
            Ok(json!({
                "status": "success",
                "data": data
            }))
        })();
        match result {
            Ok(v) => v,
            Err(f) => {
                let fatal = [
                    "history_invalid",
                    "clock_invalid",
                    "arithmetic_unrepresentable",
                    "storage_unavailable",
                ]
                .contains(&f.code.as_str());
                let f = if fatal && self.halted.is_none() {
                    self.stop(f)
                } else {
                    f
                };
                f.response(&evaluation)
            }
        }
    }
    pub fn rows(&self) -> Result<Vec<(i64, String)>> {
        load(&self.db)
    }
}
fn load(db: &Connection) -> Result<Vec<(i64, String)>> {
    let mut q = db
        .prepare("SELECT sequence_index,record_json FROM events ORDER BY sequence_index")
        .map_err(|e| Fault::history(e.to_string()))?;
    let rows = q
        .query_map([], |r| Ok((r.get(0)?, r.get(1)?)))
        .map_err(|e| Fault::history(e.to_string()))?
        .collect::<std::result::Result<Vec<_>, _>>()
        .map_err(|e| Fault::history(e.to_string()))?;
    Ok(rows)
}
pub fn diagnostic(path: &Path, f: &mut Fault) {
    let attempt = (|| -> std::io::Result<()> {
        let mut log = OpenOptions::new()
            .create(true)
            .append(true)
            .open(format!("{}.diagnostic.log", path.display()))?;
        writeln!(
            log,
            "{} {}. No repair was performed.",
            stamp(now()).unwrap_or_default(),
            f
        )?;
        log.sync_all()
    })();
    if let Err(e) = attempt {
        f.message
            .push_str(&format!(" Diagnostic storage also unavailable: {e}"));
    }
}
