# Event Types — Numbers

This document defines the canonical event records that form the
Numbers event log.

It is normative.

Events are append-only and totally ordered.

All lifecycle state transitions and deterministic evaluation must
be derived exclusively from the ordered sequence of persisted events.

If an event type is not defined in this document, it is forbidden.

Event records must be appended through the serialized canonical
commit path defined in CORE-SEQUENCE.md.

Event ordering defines system truth.

---

# 1. Event Log Principles

The Numbers system is defined by an append-only event log.

Events:

- are immutable once persisted
- are totally ordered
- must never be modified
- must never be deleted
- must never be rewritten

System state is always derived from the ordered sequence of events.

State must never be stored as mutable truth.

External observations have no authority until converted into a
canonical event record.

---

# 2. Auction Lifecycle Events

These events govern the auction lifecycle for number `N`.

---

## BidPlaced

Fields:

- `number`
- `bidder_address`
- `bid_amount`
- `server_time`

Meaning:

A valid bid has been accepted during the `Open` state.

Acceptance of the first `BidPlaced` event triggers the
`Scheduled → Open` lifecycle transition.

Each valid bid must produce exactly one `BidPlaced` event.

---

## AuctionOpened

Fields:

- `number`
- `opened_at`
- `base_end_time`

Meaning:

The auction lifecycle for number `N` has entered the `Open` state.

`opened_at` is the authoritative `server_time` when the first valid
bid was accepted.

`base_end_time` is defined as:
```Text
opened_at + auction.duration_seconds
```

This event must occur exactly once per auction.

---

## ExtensionRecorded

Fields:

- `number`
- `extension_index`
- `extension_seconds`
- `server_time`

Meaning:

An extension has been applied to the auction end time.

Extensions occur only when the extension window conditions defined
in CORE-SEQUENCE.md are satisfied.

Extensions:

- must not modify prior events
- must not modify `base_end_time`
- must not create new lifecycle states

---

## AuctionClosed

Fields:

- `number`
- `closed_at`

Meaning:

The auction lifecycle for number `N` has transitioned to `Closed`.

No further bids are permitted after this event.

Bid validity is determined solely by canonical event ordering.

---

## ResolutionRecorded

Fields:

- `number`
- `winning_bid_id`

Meaning:

The winning bid has been determined deterministically from the
persisted `BidPlaced` events.

Resolution must occur exactly once.

Resolution must never be recomputed after this event exists.

---

## SettlementRecorded

Fields:

- `number`
- `destination_address`
- `settlement_outcome`

Possible outcomes:

- `confirmed`
- `expired`
- `not_required`

Meaning:

The settlement outcome for the auction has been determined.

Destination must be either:

- the winning bidder address
- `NullSteward`

---

## FinalizationRecorded

Fields:

- `number`
- `destination_address`

Meaning:

The auction lifecycle has permanently finalized.

Destination is fixed and cannot be modified.

After this event:

- the auction lifecycle is complete
- sequence advancement to `N+1` is permitted

---

# 3. Inscription Lifecycle Events

These events govern the inscription lifecycle that begins after
auction state = `Finalized`.

---

## InscriptionStarted

Fields:

- `number`
- `commit_txid`

Meaning:

An inscription attempt has begun.

Inscription authority is consumed at this event boundary.

This event corresponds to the `broadcast_commit` boundary defined
in AUTHORITY-CONSUMPTION.md.

---

## InscriptionObserved

Fields:

- `number`
- `inscription_txid`

Meaning:

The inscription transaction has been observed and confirmed
according to the observation rules defined in the inscription
machine specification.

This event is terminal.

---

## InscriptionAmbiguous

Fields:

- `number`
- `reason`

Meaning:

The system cannot determine the outcome of the inscription attempt
with certainty.

Possible causes include:

- mempool observation failure
- contradictory authoritative node responses
- inability to determine confirmation status

This event permanently exhausts inscription authority.

No further inscription attempt is permitted.

---

# 4. System Control Events

These events affect system execution but do not alter auction
lifecycle state.

---

## SystemPaused

Fields:

- `paused_at`
- `operator_id`

Meaning:

The system has entered the `Paused` control state.

While paused:

- no new authority-bearing actions may begin
- deadlines continue to advance

---

## SystemResumed

Fields:

- `resumed_at`
- `operator_id`

Meaning:

The system has resumed execution and entered the `Running`
control state.

Resume must not restore or recreate authority.

---

# Final Rule

If an event is not defined in this document:

It is forbidden.