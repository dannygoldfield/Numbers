# Event Types — Numbers

This document defines the canonical event records that form the Numbers append-only event log.

It is normative.

Authority precedence is defined exclusively in `AUTHORITY-ORDER.md`.

---

# 1. Event Record Model

The Numbers system is defined by an append-only ordered log of canonical event records.

A canonical event record is the same persisted object as a canonical record.

The terms `event`, `event record`, `canonical record`, and `persisted record` refer to the same append-only object when used for system truth.

There is no separate event model and record model.

Record type names are the canonical event names.

All lifecycle state transitions and deterministic evaluations must be derived exclusively from the ordered sequence of canonical event records.

If an event record type is not defined in this document, it is forbidden.

---

# 2. Event Log Principles

Canonical event records:

- are immutable once persisted
- are totally ordered
- must never be modified
- must never be deleted
- must never be rewritten
- must never be reinterpreted after persistence

System state is always derived from the ordered sequence of canonical event records.

State must never be stored as mutable truth.

External observations have no authority until converted into a canonical event record.

Event ordering defines system truth.

---

# 3. Canonical Event Record Types

The canonical event record types are:

1. `AuctionRecord`
2. `BidRecord`
3. `AuctionOpenRecord`
4. `ExtensionEventRecord`
5. `AuctionCloseRecord`
6. `ResolutionRecord`
7. `SettlementRecord`
8. `FinalizationRecord`
9. `InscriptionIntentRecord`
10. `InscriptionBroadcastRecord`
11. `InscriptionConfirmationRecord`
12. `AmbiguityRecord`
13. `PauseEventRecord`

No other event record type may be persisted as canonical system truth.

---

# 4. Auction Lifecycle Event Records

## AuctionRecord

Represents the existence of an auction for number `N`.

Rules:

- exactly one `AuctionRecord` must exist per number
- `AuctionRecord` does not open the auction
- `AuctionRecord` does not store lifecycle state
- the auction remains `Scheduled` until the first valid `BidRecord` is accepted and an `AuctionOpenRecord` is atomically persisted

---

## BidRecord

Represents a bid submission attempt as evaluated at authoritative server receipt time.

A `BidRecord` may have validity:

- `valid`
- `invalid`

Rules:

- every bid submission attempt that reaches admission evaluation must produce exactly one `BidRecord`
- `server_time` must equal authoritative receipt time
- validity must be evaluated deterministically at submission
- validity must never change after persistence
- `BidRecord` does not consume auction authority
- `BidRecord` does not consume inscription authority

Only a `BidRecord` with `validity = valid` may:

- open a `Scheduled` auction
- trigger an extension
- participate in winner resolution

A `BidRecord` with `validity = invalid`:

- must not open an auction
- must not trigger an extension
- must not participate in winner resolution
- must not alter lifecycle state except as append-only audit truth

The first valid `BidRecord` for a `Scheduled` auction must be atomically persisted with `AuctionOpenRecord`.

---

## AuctionOpenRecord

Represents transition `Scheduled → Open`.

Rules:

- exactly one `AuctionOpenRecord` may exist per auction
- must be persisted atomically with the first valid `BidRecord`
- `opened_at` must equal authoritative `server_time`
- `base_end_time` must equal `opened_at + auction.duration_seconds`
- `base_end_time` must never change

Presence of `AuctionOpenRecord` proves the auction entered `Open`.

---

## ExtensionEventRecord

Represents one extension increment.

Rules:

- one `ExtensionEventRecord` must exist per extension increment
- extension records are append-only
- extension records must not modify `base_end_time`
- extension records must not create new lifecycle states
- extension records do not consume authority
- extension records must not exceed configured `max_extensions`

`current_end_time` is derived as:

```text
current_end_time =
base_end_time +
(extension_increment_seconds * number_of_extension_events)
```

---

## AuctionCloseRecord

Represents transition `Open → Closed`.

Rules:

- exactly one `AuctionCloseRecord` must exist per opened auction
- no further valid bids are permitted after `AuctionCloseRecord`
- close timestamp must equal authoritative `server_time`
- bid validity for resolution is determined by canonical event order and `BidRecord.validity`

A valid `BidRecord` before `AuctionCloseRecord` may participate in resolution.

A `BidRecord` after `AuctionCloseRecord` must be invalid.

---

## ResolutionRecord

Represents deterministic winner resolution.

Rules:

- exactly one `ResolutionRecord` must exist per closed auction
- resolution must be computed exactly once
- resolution must not be recomputed
- resolution must use only persisted valid `BidRecord` entries for the auction
- invalid `BidRecord` entries must not participate in resolution

Presence of `ResolutionRecord` proves auction resolution occurred.

---

## SettlementRecord

Represents settlement determination.

Settlement status must be one of:

- `settled`
- `expired`
- `not_required`

Rules:

- exactly one `SettlementRecord` must exist per resolved auction
- `SettlementRecord` must follow `ResolutionRecord`
- `SettlementRecord` does not create inscription authority
- settlement outcome must determine whether final destination is the winning destination or `NullSteward`

---

## FinalizationRecord

Represents transition to `Finalized`.

Rules:

- exactly one `FinalizationRecord` must exist per auction
- `FinalizationRecord` must follow `SettlementRecord`
- finalization is irreversible
- destination is fixed once persisted
- sequence advancement to `N + 1` is permitted only after `FinalizationRecord`

---

# 5. Inscription Lifecycle Event Records

Inscription lifecycle begins only after auction state = `Finalized`.

Inscription lifecycle does not alter auction state.

---

## InscriptionIntentRecord

Represents persisted inscription intent.

Rules:

- must exist before any inscription broadcast attempt
- does not consume inscription authority
- does not prove broadcast
- does not prove confirmation
- must not create auction lifecycle changes

---

## InscriptionBroadcastRecord

Represents the classified result of an inscription broadcast attempt.

A broadcast outcome must be one of:

- `committed`
- `pre_commit_rejected`
- `ambiguous`

Rules:

- `committed` corresponds to the `broadcast_commit` boundary
- inscription authority is consumed at `broadcast_commit`
- `pre_commit_rejected` does not consume inscription authority
- `ambiguous` freezes inscription authority permanently
- after `committed`, no semantically distinct inscription attempt is permitted
- after `ambiguous`, no further inscription attempt is permitted

`broadcast_commit` occurs only when:

1. the broadcast RPC succeeds
2. the authoritative node reports the transaction present in its mempool

`pre_commit_rejected` may be recorded only when the system can determine that broadcast_commit did not occur.

`ambiguous` must be recorded when the system cannot determine whether broadcast_commit occurred.

Ambiguity must not block auction finalization or sequence advancement unless another specification document explicitly says so.

---

## InscriptionConfirmationRecord

Represents observed canonical inscription confirmation.

Rules:

- may exist only after a committed `InscriptionBroadcastRecord`
- confirms inscription observation according to inscription rules
- does not consume additional authority
- terminal for inscription lifecycle

---

## AmbiguityRecord

Represents detected ambiguity.

Rules:

- must be persisted immediately upon ambiguity detection
- irreversible
- permanently freezes affected authority
- must not be removed by restart, observation, operator action, or time passing

---

# 6. System Control Event Records

## PauseEventRecord

Represents a system-level pause or resume event.

Rules:

- must not alter auction lifecycle state
- must not alter inscription lifecycle state
- must not restore authority
- must not recreate authority
- must not modify deadlines
- is a system control record, not a lifecycle state

---

# Final Rule

If an event record type is not defined in this document:

It is forbidden.