# Persistence — Numbers

This document defines persistence requirements for the Numbers system.

It is **normative**.

Persistence exists to prevent:
- recomputation
- reinterpretation
- accidental authority reuse
- mutation of recorded history

If state is not durably persisted,
it **must** be treated as **unknown**,
and authority **must not** be exercised.

If there is a conflict,
PRD.md, CORE-SEQUENCE.md, STATE-MACHINE.md, INVARIANTS.md,
and AUTHORITY-CONSUMPTION.md take precedence.

---

## Purpose

Persistence ensures that:

- every exercise of authority is durably recorded
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
unless its intent and outcome are recorded durably.

---

## Persistence Principles

### P-01. Write Before Authority

Any action that exercises authority **must** be preceded by
a durable write that records intent to perform that action.

If persistence fails:
- the action **must not** occur
- execution **must** halt or enter a safe paused state

No authority **may** be exercised without prior persistence.

---

### P-02. Persistence Is Append-Only

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

## Required Persisted Records

The following records **must** be persisted durably.

Absence of any required record is a **fatal error**.

---

### Authority Boundary Rule (Normative)

Any record whose absence would permit authority
to be exercised more than once
**must** be persisted **before** that authority is exercised.

This includes, but is not limited to:
- resolution records
- finalization records
- settlement intent records
- inscription initiation records
- ambiguity detection records

Once such a record is persisted,
the corresponding authority is **irreversibly consumed**
as defined in **AUTHORITY-CONSUMPTION.md**.

Any subsequent attempt to exercise the same authority
**must** be refused.

---

### Joint Authority Consumption (Normative)

Some authority burns are represented by **multiple records**.

Example:
- resolution record
- settlement finalization record
- inscription initiation record

Together, these records represent **one and only one**
authority consumption event.

Rules:

- partial persistence **must not** be treated as permission
- missing companion records **must** result in halt
- authority **must not** be re-exercised to “complete” a burn

Authority is consumed by the **first irreversible act**,
not by successful completion.

---

### Auction Lifecycle Persistence

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

### Settlement Persistence

If settlement applies, the system **must** persist:

- settlement intent
- settlement deadline
- settlement outcome
- settlement confirmation evidence  
  (txid or explicit failure record)

Settlement persistence **must** occur **before**
inscription authority is exercised.

Settlement failure **must** be explicit and durable.

---

### Inscription Persistence

For each auction **N**, the system **must** persist:

- inscription initiation record
- inscription attempt metadata
- known txid (if available)
- ambiguity detection record (if applicable)
- inscription confirmation record (if applicable)

If ambiguity is detected:

- the ambiguity record **must** be persisted immediately
- no further inscription actions **may** be attempted
- inscription authority is permanently exhausted

---

### System Control Persistence

The system **must** persist all system-level events:

- pause events
- resume events
- fatal error halts
- configuration snapshots active at the time of each event

Pause and resume records **must** include:
- timestamp
- triggering condition
- operator identity, if applicable

System control records **must not** grant or restore authority.

---

## Restart Semantics (Normative)

On process startup, the system **must**:

1. Load all persisted records
2. Reconstruct state machines exclusively from persisted history
3. Resume only actions explicitly permitted by reconstructed state
4. Never recompute resolution, settlement, or finalization
5. Never infer missing records

If required records are missing or contradictory:

- execution **must** halt
- authority **must not** be exercised

Restarting is reconstruction, not recovery.

---

## Forbidden Persistence Patterns

The following behaviors are explicitly forbidden:

- recomputing outcomes on restart
- treating logs as authoritative state
- inferring resolution from absence of data
- repairing missing records by assumption
- deleting ambiguous records to unblock progress
- reattempting authority due to crash or timeout

If a required record is missing,
the only correct response is to stop.

---

## Idempotence Rules

All persisted writes **must** be idempotent.

Re-applying the same write:

- **must not** change meaning
- **must not** advance state
- **must not** duplicate authority

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

If the system is unsure whether an authority-bearing action
has already occurred:

**It must assume it has, and refuse to repeat it.**

Persistence preserves memory.
Forgetting is equivalent to lying.
