# Persistence — Numbers

This document defines persistence requirements for the Numbers system.

It is normative.

Authority precedence is defined exclusively in AUTHORITY-ORDER.md.

Persistence prevents:

- recomputation
- reinterpretation
- accidental authority reuse
- mutation of recorded history

If a required state is not durably persisted,
the system must treat that state as unknown
and must not consume authority.

---

## 1. Purpose

Persistence ensures that:

- every authority consumption is durably recorded
- restarts do not change outcomes
- ambiguity is preserved rather than erased
- time passing does not recreate permission
- authority consumption is provable after the fact

Persistence is a safety boundary.

---

## 2. Relationship to Authority (Normative)

Authority is defined exclusively in AUTHORITY-CONSUMPTION.md.

An authority-bearing transition does not exist
unless its canonical boundary record
is durably persisted.

---

## 3. Persistence Principles (Normative)

### P-01. Persist Before Authority

Any authority-bearing transition must:

1. durably persist its canonical boundary record, and
2. complete atomically with respect to that record.

If persistence fails:

- the authority-bearing action must not occur
- execution must halt

Authority without durable record is forbidden.

---

### P-02. Append-Only Persistence

Persisted records must never be:

- edited
- rewritten
- deleted
- compacted in a way that loses semantic meaning

History is strictly additive.

Corrections must be expressed as new records.

---

### P-03. Persisted State Is Canonical

At runtime:

- persisted state defines reality
- in-memory state is a cache only

On restart:

- all in-memory state must be discarded
- state must be reconstructed exclusively from persisted canonical records

If persisted state contradicts memory,
memory is incorrect.

---

## 4. Canonical Record Dependency (Normative)

Persistence applies only to canonical record types
defined in DATA-MODEL.md.

Persistence must not introduce:

- implicit state
- inferred records
- undocumented record types

If a required canonical record is missing,
execution must halt.

---

## 5. Authority Boundary Persistence (Normative)

The following records represent authority boundaries
and must be durably persisted exactly once:

- InscriptionRecord (`Finalized → Inscribing`)
- AmbiguityRecord (`Inscribing → Ambiguous`)

Once such a record is persisted,
inscription authority is irreversibly consumed
as defined in AUTHORITY-CONSUMPTION.md.

Subsequent attempts to consume the same authority
must be refused.

---

## 6. Auction Lifecycle Persistence (Normative)

For each auction number `N`, the system must persist:

- AuctionRecord
- BidRecord(s)
- AuctionOpenRecord (exactly one if auction opens), containing:
  - `opened_at`
  - `base_end_time`
- ExtensionEventRecord(s), if any
- AuctionCloseRecord (exactly one if auction closes)
- ResolutionRecord (exactly one)
- SettlementRecord (exactly one)
- FinalizationRecord (exactly one)

AuctionRecord:

- must occur only after the previous auction reaches `Finalized`
- must not include a scheduled start time
- must not include a precomputed end time
- must not imply automatic opening

While state = `Scheduled`:

- `opened_at` must remain null
- `base_end_time` must remain null

`base_end_time` must be persisted exactly once,
when the first valid bid is accepted.

`current_end_time` must never be persisted.
It must always be derived as:
```Text
current_end_time =
base_end_time +
(extension_increment_seconds * number_of_extension_events
```

Presence of ResolutionRecord prohibits recomputation.

Presence of FinalizationRecord prohibits recomputation of destination.

---

## 7. Settlement Persistence (Normative)

If settlement applies, the system must persist:

- settlement deadline
- settlement outcome
- settlement evidence:
  - confirmed txid, or
  - explicit failure indicator

Settlement persistence must occur
before `AwaitingSettlement → Finalized`.

Settlement does not create inscription authority.

Settlement failure must be explicit and durable.

---

## 8. Inscription Persistence (Normative)

For each auction `N`, the system must persist:

- InscriptionRecord (`Finalized → Inscribing`)
- inscription attempt metadata
- known txid, if available
- AmbiguityRecord, if applicable
- InscriptionConfirmationRecord, if applicable

If AmbiguityRecord exists:

- InscriptionConfirmationRecord must not be written
- no further inscription actions are permitted

If ambiguity is detected:

- AmbiguityRecord must be persisted immediately
- inscription authority is permanently exhausted

Inscription confirmation must not remove or overwrite prior records.

---

## 9. System Control Persistence (Normative)

The system must persist system-level control events:

- pause events
- resume events
- fatal execution halts
- configuration snapshot active at time of each event

System control records:

- must not grant authority
- must not restore authority
- must not alter lifecycle truth

---

## 10. Restart Semantics (Normative)

On process startup, the system must:

1. Load all persisted canonical records
2. Reconstruct state exclusively from persisted history
3. Resume only transitions explicitly permitted by reconstructed state
4. Never infer missing records

If required records are missing or contradictory:

- execution must halt
- authority must not be consumed

Restart is reconstruction, not recovery.

---

## 11. Forbidden Persistence Patterns (Normative)

The following are forbidden:

- recomputing outcomes after canonical records exist
- treating logs as authoritative state
- inferring resolution from absence of data
- repairing missing records by assumption
- deleting ambiguity to unblock progress
- re-consuming authority due to crash or timeout

If a required record is missing,
execution must halt.

---

## 12. Idempotence Rules (Normative)

Non-authority records may be idempotently re-applied
without changing lifecycle meaning.

Authority-bearing records must be written exactly once.

Duplicate authority-bearing records are forbidden.

---

## 13. Storage Requirements

This document does not mandate a storage technology.

Permitted systems include:

- SQLite
- append-only files
- write-ahead logs
- embedded databases

Not permitted:

- volatile-only storage
- in-memory-only persistence
- best-effort caching without durability guarantees

Durability must survive:

- process crashes
- machine restarts
- power loss

---

## Final Rule

Persistence defines canonical truth.

If the system cannot prove
that an authority-bearing action did not occur:

It must assume it did and refuse to repeat it.