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

## 1. Design Principles (Normative)

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
- `auction_id`
- `amount`
- `timestamp`
- `status` (`valid`, `invalid`, `superseded`)

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
- `winning_bid_id` (absent if no valid bids)
- `winning_amount` (zero if no valid bids)
- `resolution_time`

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
- `destination`
- `status` (`settled`, `failed`, `no-bid`)
- `finalization_time`

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
- `txid` (if known)
- `satpoint` (if known)
- `status` (`pending`, `broadcast`, `confirmed`, `ambiguous`)
