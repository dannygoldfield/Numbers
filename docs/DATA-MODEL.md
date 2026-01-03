# Data Model — Numbers

This document defines the canonical data recorded by Numbers.

It describes what is persisted, when records are written,
and what authority those records do and do not carry.

Numbers favors append-only records.
Once written, records are never mutated or reinterpreted.

---

## Design Principles

- All records are append-only
- Each record corresponds to a system event
- Records reflect procedure, not meaning
- Bitcoin remains the final authority

The database records *what the system did*,
not what the system claims is true.

---

## Canonical Record Types

The following record types constitute canonical system state:

1. Auction
2. Bid
3. Resolution
4. Settlement
5. Inscription
6. Pause Event

These records represent Numbers system behavior,
not arbitrary on-chain activity.

Each record is immutable once written.

---

## Auction

Represents a single auction in the sequence.

**Fields**
- `auction_id` (monotonic, sequential)
- `number`
- `start_time`
- `end_time`
- `status` (open, closed)

An Auction record is created at open and finalized at close.
It is never modified after close.

---

## Bid

Represents a bid submitted during an auction.

**Fields**
- `bid_id`
- `auction_id`
- `amount`
- `timestamp`
- `status` (valid, invalid, superseded)

Bid validity is determined at auction start
and remains fixed for the duration of that auction.

Bids are never deleted or rewritten.

---

## Resolution

Represents the outcome determined at auction close.

**Fields**
- `auction_id`
- `winning_bid_id` (nullable)
- `winning_amount` (zero if no valid bids)
- `resolution_time`

Resolution occurs exactly once per auction.

Resolution does not depend on settlement success.

---

## Settlement

Represents whether the provisional winner fulfilled settlement requirements.

**Fields**
- `auction_id`
- `destination` (winner address or NullSteward)
- `status` (settled, failed, no-bid)
- `finalization_time`

Settlement finalizes the inscription destination.
It does not delay or alter subsequent auctions.

---

## Inscription

Represents the on-chain inscription produced by a finalized outcome.

**Fields**
- `auction_id`
- `txid`
- `satpoint`
- `status` (pending, broadcast, confirmed)
- `recognized` (boolean)

Inscription content is the number only,
recorded as witness data in a Bitcoin transaction.

Multiple inscriptions of the same number may exist on-chain.
Only one may be recognized by the system.

---

## Pause Event

Represents an inter-auction gap.

**Fields**
- `from_auction_id`
- `to_auction_id`
- `start_time`
- `end_time`

Pause events exist to preserve sequencing guarantees.
They do not carry semantic meaning.

---

## Authority and Truth

- Bitcoin is the final authority for transactions and confirmations
- Numbers records system intent and observed outcomes
- Database records do not override on-chain data

If a database record conflicts with the blockchain,
the blockchain wins.

---

## Rebuildability

All records are reconstructible from:

- Bitcoin blockchain data
- Public knowledge of Numbers auction rules
- Inspection of inscriptions and transactions

No record depends on:
- private memory
- operator discretion
- off-chain metadata

The database may be deleted and rebuilt.
Reconstruction must converge on the same sequence of outcomes.

---

## Principle

Derived views may change.
Recorded events do not.

Numbers persists facts about procedure,
then steps aside.
