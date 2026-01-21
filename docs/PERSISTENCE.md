# Persistence — Numbers

This document defines persistence requirements for the Numbers system.

It is **normative**.

Persistence exists to prevent:
- recomputation
- reinterpretation
- accidental authority reuse
- mutation of recorded history

If a required state is not durably persisted,
the system **must treat that state as unknown**
and **must not consume authority**.

If there is a conflict,
PRD.md, CORE-SEQUENCE.md, STATE-MACHINE.md, INVARIANTS.md,
and AUTHORITY-CONSUMPTION.md take precedence.

---

## Purpose

Persistence ensures that:

- every authority consumption is durably recorded
- restarts do not change outcomes
- ambiguity is preserved rather than erased
- time passing does not recreate permission
- authority consumption is provable after the fact

Persistence is not an optimization.
It is a safety boundary.

---

## Authority Model (Normative)

Authority is defined and governed by **AUTHORITY-CONSUMPTION.md**.

Persistence is the mechanism by which authority consumption
is made irreversible and observable.

An authority-bearing action **does not exist**
unless its intent and boundary conditions
are recorded durably.

---

## Persistence Principles (Normative)

### P-01. Persist Before Authority

Any action that consumes authority **must** be preceded by
a durable persistence record that establishes intent.

If persistence fails:
- the action **must not** occur
- execution **must** halt or enter a safe paused state

Authority without memory is forbidden.

---

### P-02. Append-Only Persistence

Persisted records **must never** be:
- edited
- rewritten
- deleted
- compacted in a way that loses semantic meaning

History is strictly additive.

Corrections **must** be expressed as new records,
never as mutations of prior records.

---

### P-03. Persisted State Is Canonical

At runtime:

- persisted state defines reality
- in-memory state is a cache only
- on restart, all in-memory state **must** be discarded

If persisted state contradicts memory,
memory is incorrect.

---

## Canonical Record Dependency (Normative)

Persistence applies **only** to canonical record types
defined in **DATA-MODEL.md**.

Persistence **must not** introduce:
- new record categories
- implicit state
- inferred records

If a required canonical record is missing,
execution **must** halt.

---

## Authority Boundary Rule (Normative)

Any record whose absence could permit authority
to be consumed more than once
**must** be persisted **before**
that authority is exercised.

This includes, but is not limited to:
- resolution records
- finalization records
- settlement intent records
- inscription initiation records
- ambiguity detection records

Once such a record is persisted,
the corresponding authority is **irreversibly consumed**
as defined in **AUTHORITY-CONSUMPTION.md**.

Any subsequent attempt to consume the same authority
**must** be refused.

---

## Joint Authority Consumption (Normative)

Some authority consumptions are represented
by **multiple persisted records**.

Example:
- resolution record
- settlement outcome record
- inscription initiation record

Together, these records represent
**one and only one** authority burn.

Rules:

- partial persistence **must not** be treated as permission
- missing companion records **must** result in halt
- authority **must not** be re-consumed to “complete” a burn

Authority is consumed by the **first irreversible act**,
not by successful completion.

---

## Auction Lifecycle Persistence (Normative)

For each auction number **N**, the system **must** persist:

- auction number
- scheduled start time
- actual open timestamp
- actual close timestamp
- resolution record (exactly one)
- finalization record (exactly one)

Resolution and finalization records **must** include:
- timestamp
- deterministic inputs used
- resulting outcome

Resolution and finalization **must never** be recomputed.

---

## Settlement Persistence (Normative)

If settlement applies, the system **must** persist:

- settlement intent
- settlement deadline
- settlement outcome
- settlement evidence:
  - confirmed txid, or
  - explicit failure record

Settlement persistence **must** occur
before inscription authority is exercised.

Settlement failure **must** be explicit and durable.

---

## Inscription Persistence (Normative)

For each auction **N**, the system **must** persist:

- inscription initiation record
- inscription attempt metadata
- known txid, if available
- ambiguity detection record, if applicable
- inscription confirmation record, if applicable

If ambiguity is detected:

- the ambiguity record **must** be persisted immediately
- no further inscription actions **may** be attempted
- inscription authority is permanently exhausted

---

## System Control Persistence (Normative)

The system **must** persist all system-level events:

- pause events
- resume events
- fatal execution halts
- configuration snapshots active at the time of each event

System control records **must not**:
- grant authority
- restore authority
- advance lifecycle state

---

## Restart Semantics (Normative)

On process startup, the system **must**:

1. Load all persisted canonical records
2. Reconstruct state exclusively from persisted history
3. Resume only actions explicitly permitted by reconstructed state
4. Never recompute resolution, settlement, or finalization
5. Never infer missing records

If required records are missing or contradictory:

- execution **must** halt
- authority **must not** be consumed

Restarting is reconstruction, not recovery.

---

## Forbidden Persistence Patterns (Normative)

The following behaviors are explicitly forbidden:

- recomputing outcomes on restart
- treating logs as authoritative state
- inferring resolution from absence of data
- repairing missing records by assumption
- deleting ambiguous records to unblock progress
- re-consuming authority due to crash or timeout

If a required record is missing,
the only correct response is to stop.

---

## Idempotence Rules (Normative)

All persisted writes **must** be idempotent.

Re-applying the same write:

- **must not** change meaning
- **must not** advance state
- **must not** duplicate authority consumption

Duplicate records are acceptable.
Duplicate authority is not.

---

## Storage Independence

This document does not mandate a storage technology.

Permitted storage systems include:
- SQLite
- append-only files
- write-ahead logs
- embedded databases

Not permitted:
- volatile-only storage
- in-memory-only persistence
- best-effort caching without durability guarantees

Durability **must** survive:
- process crashes
- machine restarts
- power loss

---

## Final Rule

If the system cannot prove
that an authority-bearing action did **not** occur:

**It must assume it did, and refuse to repeat it.**

Persistence preserves memory.
Forgetting is equivalent to lying.
