# Data Model — Numbers

This document defines the canonical data recorded by Numbers.

It is **normative**.

It specifies:
- which records exist
- when records are written
- which authorities those records consume or preserve

Numbers uses **append-only persistence**.
Once written, records **must never** be mutated, reinterpreted, or deleted.

If there is a conflict,
PRD.md, INVARIANTS.md, STATE-MACHINE.md,
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
- each record corresponds to a single irreversible system event
- records describe procedure, not interpretation
- records never assert truth beyond what was observed
- absence of a required record implies loss of authority

The data model records **what the system did or observed**,
not what the system believes or intends.

---

## 2. Canonical Record Set (Normative)

The following record types constitute the **entire** canonical data model:

1. AuctionRecord
2. BidRecord
3. ResolutionRecord
4. SettlementRecord
5. InscriptionRecord
6. AmbiguityRecord
7. PauseEventRecord

No other record type **may** be persisted as canonical state.

Each canonical record:
- **must** be immutable once written
- **must** have a stable schema
- **must** be written at a single, well-defined transition point

---

## 3. AuctionRecord

Represents the existence and timing of a single auction in the global sequence.

### Fields

- `auction_id`  
  Monotonic identifier. Exactly one per auction.

- `number`  
  The number being auctioned. Exactly one per auction.

- `scheduled_start_time`

- `open_time`

- `close_time`

### Rules

- an AuctionRecord **must** be created exactly once
- creation **must** occur before bidding opens
- timestamps **must** reflect actual transitions
- AuctionRecords **must not** be synthesized retroactively

AuctionRecord existence proves auction identity, not outcome.

---

## 4. BidRecord

Represents a bid submission attempt.

### Fields

- `bid_id`
- `auction_id`
- `bidder_address`
- `amount`
- `destination_address`
- `nonce`
- `signature`
- `submission_time`
- `validity` (`valid`, `invalid`)

### Rules

- all bid attempts **must** be recorded
- bid validity **must** be evaluated using rules fixed at auction start
- bid validity **must not** change after evaluation
- BidRecords **must never** be deleted or rewritten

BidRecords record attempted participation.
They do **not** consume authority.

---

## 5. ResolutionRecord

Represents the deterministic outcome of an auction.

### Fields

- `auction_id`
- `winning_bid_id` (null if no valid bids)
- `winning_amount`
- `resolution_time`
- `resolution_inputs_hash`

### Rules

- exactly one ResolutionRecord **must** exist per auction
- resolution **must** be computed exactly once
- resolution **must not** depend on settlement
- resolution **must** be persisted immediately

ResolutionRecord existence **consumes auction resolution authority**.

---

## 6. SettlementRecord

Represents settlement outcome after resolution.

### Fields

- `auction_id`
- `settlement_deadline`
- `status` (`settled`, `failed`, `no-bid`)
- `destination_address`
- `confirmation_txid` (null if failed)
- `finalization_time`

### Rules

- exactly one SettlementRecord **must** exist per auction
- settlement outcome **must** be explicit
- settlement authority **must not** be reused
- settlement failure **must** be recorded durably

Settlement fixes destination.
It does not assert inscription success.

---

## 7. InscriptionRecord

Represents inscription attempt initiation and known outcomes.

### Fields

- `auction_id`
- `initiation_time`
- `txid` (null if unknown)
- `satpoint` (null if unknown)
- `status` (`initiated`, `broadcast`, `confirmed`)

### Rules

- at most one InscriptionRecord **may** be created per auction
- record creation **must** precede any broadcast attempt
- presence of this record **consumes inscription authority**
- absence of confirmation does not restore authority

InscriptionRecord does not resolve ambiguity.

---

## 8. AmbiguityRecord

Represents permanent uncertainty regarding inscription outcome.

### Fields

- `auction_id`
- `ambiguity_time`
- `reason`

### Rules

- AmbiguityRecord **must** be written immediately upon detection
- AmbiguityRecord **must** exist if broadcast cannot be ruled out
- AmbiguityRecord is irreversible
- Presence of this record permanently freezes inscription authority

Ambiguity is preserved, not resolved.

---

## 9. PauseEventRecord

Represents system-level pause or resume events.

### Fields

- `event_id`
- `event_type` (`pause`, `resume`)
- `timestamp`
- `reason`

### Rules

- Pause events **must** be persisted
- Pause records **must not** alter auction or inscription authority
- Pause does not create, consume, or restore authority

---

## 10. Cross-Record Invariants (Normative)

- absence of a required record **must** halt execution
- duplicate authority-bearing records are forbidden
- missing companion records **must not** be inferred
- restart reconstruction **must** rely exclusively on records above

---

## Final Rule

If a required canonical record is missing,
ambiguous, or contradictory:

**The system must halt rather than guess.**
