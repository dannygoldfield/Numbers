# Core Sequence — Numbers

This document assumes familiarity with ARCHITECTURE.md.

This document defines the invariants of Numbers.

It describes what must always happen,
in what order,
and without exception.

It does not explain purpose or meaning.

If there is a conflict, PRD.md takes precedence.

---

## Sequence Invariants

Numbers advances through a monotonically increasing sequence of numbers.

For each number **N**, the system executes the following sequence exactly once.

The sequence never pauses, retries, or rewinds.

---

## Auction

For number **N**:

- An auction opens.
- The auction has a predefined end condition (time expiry or bid cap).
- Only bids that are valid at auction start are accepted.
- Bid validity remains fixed for the duration of the auction.

At auction close, bidding ends permanently.

---

## Resolution

At auction close, the auction resolves exactly once.

Resolution produces a provisional outcome:
- a winning bidder, or
- no valid bids

No further bids are accepted after resolution.

---

## Settlement

If a winning bidder exists, settlement begins.

- Settlement runs asynchronously.
- Settlement has a fixed deadline.
- Settlement success or failure does not affect subsequent auctions.

Finalization produces exactly one destination:
- settlement succeeds → winning address
- settlement fails → NullSteward
- no valid bids → NullSteward

---

## Inscription

After finalization, exactly one inscription is produced for number **N**.

- The inscription content is the number only.
- The inscription is recorded in a Bitcoin transaction.
- The destination is determined by finalization.

The inscription is constructed and broadcast.
Broadcast uncertainty corresponds to the **Ambiguous** inscription state defined in STATE-MACHINE.md and permanently revokes inscription authority.

The sequence then advances to **N + 1**.

---

## Guarantees

For every number **N**:

- The auction resolves exactly once.
- Finalization produces exactly one destination.
- An inscription is produced.
- The sequence advances.

These guarantees hold regardless of:
- bidder behavior
- settlement success
- inscription interpretation
