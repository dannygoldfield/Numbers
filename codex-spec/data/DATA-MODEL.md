# Data Model — Numbers

This document defines the canonical data recorded by Numbers.

It is normative.

Authority precedence is defined exclusively in AUTHORITY-ORDER.md.

It specifies:
- which records exist
- when records are written
- how lifecycle state is derived from records

Numbers uses append-only persistence.
Once written, records must never be mutated, reinterpreted, or deleted.

---

## Modal Language Rule (Normative)

In this document and all normative specifications:

- must / must not define obligations
- only / exactly once / at most once define bounds
- may is permitted only to describe uncertainty of knowledge

The following terms are forbidden in normative contexts:

- possibly
- likely
- eventually
- for now
- TBD

Any normative statement using forbidden modal language is invalid.

---

## 1. Design Principles (Normative)

All persisted data must obey:

- all records are append-only
- each record corresponds to a single irreversible system event
- records must represent observable system events
- records must not encode derived lifecycle state
- absence of a required record implies authority cannot be exercised

Lifecycle state is derived from record presence.
It is never stored as mutable truth.

---

## 2. Canonical Record Set (Normative)

The entire canonical data model consists of:

1. AuctionRecord
2. AuctionOpenRecord
3. AuctionCloseRecord
4. BidRecord
5. ExtensionEventRecord
6. ResolutionRecord
7. SettlementRecord
8. FinalizationRecord
9. InscriptionRecord
10. InscriptionConfirmationRecord
11. AmbiguityRecord
12. PauseEventRecord

No other record type may be persisted as canonical state.

Each canonical record:

- must be immutable once written
- must have a stable schema
- must be written only at the transition boundary defined in STATE-MACHINE-TABLE.md

---

## 3. AuctionRecord

Represents the existence of an auction number.

### Fields

- `auction_id`
- `number`
- `creation_time`

### Rules

- exactly one AuctionRecord must exist per number
- must be written only after previous auction reaches `Finalized`
- does not store lifecycle state

---

## 4. AuctionOpenRecord

Represents transition `Scheduled → Open`.

### Fields

- `auction_id`
- `opened_at`
- `base_end_time`

### Rules

- at most one AuctionOpenRecord per auction
- must be written atomically with first accepted valid bid
- `opened_at` must equal authoritative `server_time`
- `base_end_time = opened_at + auction.duration_seconds`
- `base_end_time` must never change

Presence proves auction is `Open`.

---

## 5. AuctionCloseRecord

Represents transition `Open → Closed`.

### Fields

- `auction_id`
- `closed_at`
- `reason` (`duration_expired`, `cap_reached`)

### Rules

- exactly one AuctionCloseRecord per opened auction
- must be written when close trigger fires
- `closed_at` must equal authoritative `server_time`

---

## 6. BidRecord

Represents a bid submission attempt.

### Fields

- `bid_id`
- `auction_id`
- `bidder_address`
- `amount_sats`
- `destination_address`
- `nonce`
- `signature`
- `server_time`
- `validity` (`valid`, `invalid`)

### Rules

- all bid attempts must be recorded
- `server_time` equals authoritative receipt time
- validity must be evaluated deterministically at submission
- validity must never change after evaluation
- BidRecord does not consume authority

---

## 7. ExtensionEventRecord

Represents one extension increment.

### Fields

- `auction_id`
- `trigger_bid_id`
- `server_time`

### Rules

- one record per extension increment
- records appended only
- `number_of_extension_events` equals record count
- must not exceed configured `max_extensions`

`current_end_time` is derived as:
```Text
current_end_time =
base_end_time +
(extension_increment_seconds * number_of_extension_events)
```

ExtensionEventRecord does not consume authority.

---

## 8. ResolutionRecord

Represents deterministic resolution.

### Fields

- `auction_id`
- `winning_bid_id` (null if none)
- `winning_amount_sats`
- `resolution_time`
- `resolution_inputs_hash`

### Rules

- exactly one ResolutionRecord per closed auction
- resolution must be computed exactly once
- resolution must not be recomputed
- presence of ResolutionRecord proves auction resolution occurred

---

## 9. SettlementRecord

Represents settlement determination.

### Fields

- `auction_id`
- `settlement_deadline`
- `status` (`settled`, `expired`, `not_required`)
- `confirmation_txid` (nullable)
- `settlement_time`

### Rules

- exactly one SettlementRecord per resolved auction
- must follow ResolutionRecord
- does not create inscription authority

---

## 10. FinalizationRecord

Represents transition `AwaitingSettlement → Finalized`.

### Fields

- `auction_id`
- `finalization_time`
- `destination_address`

### Rules

- exactly one FinalizationRecord per auction
- must follow SettlementRecord
- finalization is irreversible

---

## 11. InscriptionRecord

Represents transition `Finalized → Inscribing`.

### Fields

- `auction_id`
- `initiation_time`
- `intended_txid` (nullable)

### Rules

- at most one InscriptionRecord per auction
- once written, no additional InscriptionRecord may exist for the same auction
- record creation consumes inscription authority
- record must be written before broadcast attempt

Presence proves inscription initiation.
It does not prove confirmation.

---

## 12. InscriptionConfirmationRecord

Represents transition `Inscribing → Inscribed`.

### Fields

- `auction_id`
- `confirmation_time`
- `confirmed_txid`
- `block_height`

### Rules

- at most one InscriptionConfirmationRecord per auction
- does not consume additional authority
- must not remove or alter prior records

---

## 13. AmbiguityRecord

Represents transition `Inscribing → Ambiguous`.

### Fields

- `auction_id`
- `ambiguity_time`
- `reason`

### Rules

- must be written immediately upon detection
- irreversible
- permanently freezes inscription authority

---

## 14. PauseEventRecord

Represents system-level pause or resume.

### Fields

- `event_id`
- `event_type` (`pause`, `resume`)
- `timestamp`
- `reason`

### Rules

- must be persisted
- must not alter auction or inscription authority
- is not a lifecycle state

---

## 15. Cross-Record Invariants (Normative)

- absence of a required record must halt execution
- duplicate authority-bearing records are forbidden
- records must appear in an order consistent with STATE-MACHINE-TABLE.md
- lifecycle state must be derived strictly from record presence
- restart reconstruction must rely exclusively on records defined above

---

## Final Rule

If a required canonical record is missing,
ambiguous, contradictory,
or violates ordering constraints:

The system must halt rather than guess.