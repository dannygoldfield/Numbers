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

For each number **N**, the system executes the following sequence
**at most once and strictly in order**.

The sequence never rewinds.
Previously completed steps are never repeated.

Temporary blocking of progression
due to pause, settlement, or observation
does not alter sequence order.

---

## Auction

For number **N**:

- An auction opens.
- The auction has a predefined end condition
  (time expiry or bid cap).
- Only bids that are valid at auction start are accepted.
- Bid validity remains fixed for the duration of the auction.

At auction close, bidding ends permanently.

No subsequent action may reopen bidding.

---

## Resolution

At auction close, the auction resolves exactly once.

Resolution produces a provisional outcome:

- a highest valid bid, or
- no valid bids

Resolution:

- is deterministic
- is independent of settlement success
- must not be recomputed
- must be durably persisted

No further bids are accepted after resolution.

---

## Settlement

If a winning bidder exists, settlement begins.

Settlement:

- executes asynchronously
- has a fixed deadline
- does not block subsequent auctions

Finalization produces exactly one destination:

- settlement succeeds → winning address
- settlement fails → NullSteward
- no valid bids → NullSteward

Finalization is irreversible
and consumes all remaining auction authority.

---

## Inscription

After finalization, the system performs **exactly one inscription attempt**
for number **N**.

The inscription attempt:

- has content equal to the number only
- targets the destination determined by finalization
- is constructed and broadcast as a Bitcoin transaction

Inscription authority is exercised **once**.

Possible outcomes include:

- confirmed inscription (`Inscribed`)
- unresolved broadcast or observation uncertainty (`Ambiguous`)

Ambiguity:

- permanently exhausts inscription authority
- forbids retries, replacement, or override
- permits observation only

The system **does not guarantee**
that an inscription will ever be confirmed.

---

## Sequence Advancement

After inscription authority is exercised
or permanently exhausted:

- the system advances to **N + 1**
- no further action is permitted for **N**

The sequence advances regardless of:

- bidder behavior
- settlement success
- inscription confirmation
- ambiguity persistence

---

## Guarantees

For every number **N**:

- the auction opens at most once
- the auction resolves exactly once
- finalization produces exactly one destination
- inscription authority is exercised at most once
- the sequence advances monotonically

Loss, ambiguity, and uncertainty
are valid terminal outcomes.

They do not block progression.
