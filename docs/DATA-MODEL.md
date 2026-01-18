# Data Model — Numbers

This document defines the canonical data recorded by Numbers.

It is **normative**.

It specifies:
- what records exist
- when records are written
- what authority those records do and do not carry

Numbers uses **append-only persistence**.
Once written, records **must never** be mutated, reinterpreted, or deleted.

If there is a conflict,
API-STATE-SHAPES.md, STATE-MACHINE.md, INVARIANTS.md,
TRANSITION-INVARIANTS.md, and ERROR-TAXONOMY.md take precedence.

---

## Modal Language Rule (Normative)

In this document and all normative specifications:

- **must / must not** define obligations
- **only / exactly once / at most once** define bounds
- **may** is permitted **only** to describe uncertainty of knowledge,
  never permission, intent, or policy choice

The following terms are forbidden in normative contexts:

- possibly
- likely
- eventually
- for now
- TBD

Any normative statement using forbidden modal language is invalid.

---

## 1. Design Principles

All persisted data **must** obey the following principles:

- all records are append-only
- each record corresponds to a single system event
- records describe procedure, not interpretation
- records never claim truth beyond what was observed
- Bitcoin remains the final authority

The database records **what the system did or observed**,
not what the system believes to be true.

---

## 2. Canonical Record Set (Normative)

The following record types constitute the **entire** canonical data model:

1. Auction  
2. Bid  
3. Resolution  
4. Settlement  
5. Inscription  
6. PauseEvent  

No other record type **may** be persisted as canonical state.

Each canonical record:
- **must** be immutable once written
- **must** have a stable schema
- **must** be written at a single, well-defined transition point

---

## 3. Auction Record

Represents a single auction in the global sequence.

### Fields

- `auction_id`  
  Monotonic identifier. Exactly one per auction.

- `number`  
  The number being auctioned. Exactly one per auction.

- `start_time`  
  Timestamp when the auction entered `Open`.

- `end_time`  
  Timestamp when the auction exited `Open`.

- `status`  
  One of: `open`, `closed`

### Rules

- an Auction record **must** be created exactly once
- creation **must** occur at auction open
- the record **must not** be modified after close
- auction records **must not** be synthesized retroactively

---

## 4. Bid Record

Represents a bid submission attempt.

### Fields

- `bid_id`  
  Unique identifier.

- `auction_id`  
  References the auction in which the bid was submitted.

- `amount`  
  Bid amount as submitted.

- `timestamp`  
  Submission time.

- `status`  
  One of: `valid`, `invalid`, `superseded`

### Rules

- bid validity **must** be evaluated against auction state fixed at auction start
- bid validity **must not** change during the auction
- bids **must never** be deleted or rewritten
- invalid bids **must** be persisted explicitly

Bid records record **attempted action**, not authority.

---

## 5. Resolution Record

Represents the deterministic outcome at auction close.

### Fields

- `auction_id`  
  References the resolved auction.

- `winning_bid_id`  
  Absent when no valid bids exist.

- `winning_amount`  
  Zero when no valid bids exist.

- `resolution_time`  
  Timestamp of resolution.

### Rules

- resolution **must** occur exactly once per auction
- resolution **must not** depend on settlement success
- resolution **must not** be delayed, recomputed, or retried
- resolution **must** be recorded even when no bids exist

Resolution consumes auction authority.

---

## 6. Settlement Record

Represents the settlement outcome derived from resolution.

### Fields

- `auction_id`  
  References the resolved auction.

- `destination`  
  Winner address or `NullSteward`.

- `status`  
  One of: `settled`, `failed`, `no-bid`

- `finalization_time`  
  Timestamp of settlement finalization.

### Rules

- settlement **must** be attempted exactly once
- settlement **must not** block subsequent auctions
- settlement **must** finalize to exactly one destination
- settlement failure **must** be explicit and recorded

Settlement fixes the inscription destination.
It does not confer inscription certainty.

---

## 7. Inscription Record

Represents inscription intent and observed chain outcome.

### Fields

- `auction_id`  
  References the finalized auction.

- `txid`  
  Transaction identifier, if known.

- `satpoint`  
  Satpoint, if known.

- `status`  
  One of: `pending`, `broadcast`, `confirmed`, `ambiguous`

- `recognized`  
  Boolean indicating whether this record represents
  the system’s procedurally-derived inscription for the auction.

### Rules

- at most one Inscription record **may** have `recognized = true` per auction
- multiple on-chain inscriptions **may** exist
- recognition **must** be explicit and immutable
- ambiguity **must** be recorded and must not be resolved by inference
- absence of confirmation **must not** be treated as failure

Inscription records reflect **attempt and observation**, not ownership or truth.

---

## 8. PauseEvent Record

Represents an inter-auction or system pause.

### Fields

- `from_auction_id`
- `to_auction_id`
- `start_time`
- `end_time`
- `reason`

### Rules

- PauseEvents **must** occur only at auction boundaries
- PauseEvents **must not** alter auction outcomes
- PauseEvents **must** be durably recorded

PauseEvents carry **no semantic meaning** beyond sequencing and control.

---

## 9. Authority and Truth (Normative)

- Bitcoin is authoritative for transaction existence and confirmation
- Numbers is authoritative only for:
  - auction sequencing
  - resolution determination
  - inscription intent and ambiguity tracking

Database records:

- **must not** override the blockchain
- **must not** assert ownership
- **must not** infer finality beyond observation

If a persisted record conflicts with on-chain data,
the blockchain **wins**.

---

## 10. Rebuildability (Normative)

The full system state **must** be reconstructible from:

- the Bitcoin blockchain
- public Numbers rules
- inspection of transactions and inscriptions

No canonical record **may** depend on:

- private memory
- operator discretion
- mutable off-chain metadata

The database **may** be deleted and rebuilt.
Reconstruction **must** converge on the same recorded outcomes
given the same observable data.

---

## 11. Final Principle

Derived views may change.  
Canonical records do not.

Numbers persists facts about procedure,
records loss and ambiguity without repair,
and then steps aside.
