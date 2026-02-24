# Bidding Admission — Numbers

This document defines bid admission rules for Numbers.

It is normative.

Bid admission governs whether a bid must be accepted or rejected
at the moment it is received.

It does not define auctions, settlement, outcomes, or meaning.

If there is a conflict,
INVARIANTS.md, CORE-SEQUENCE.md, and STATE-MACHINE.md take precedence.

---

## 1. Purpose

Bid admission exists to:

- prevent invalid bids from entering the system
- enforce deterministic economic boundaries
- enforce irreversible exclusions
- isolate eligibility logic from downstream phases

Bid admission is a gate.
It does not negotiate, infer, or explain.

---

## 2. Admission Scope (Normative)

Bid admission applies only when:

- system state = `Running`
- auction state is `Scheduled` or `Open`
- a bid submission is received

Bid admission does not apply:

- after auction close
- during settlement
- during inscription
- during ambiguity
- during restart reconstruction

If system state is not `Running`,
all bids must be rejected.

---

## 3. Minimum Bid Rule (Normative)

Each auction has a configured value:

`bidding.minimum_bid_sats`

Rules:

- `bidding.minimum_bid_sats` must be a positive integer
- it must be fixed before the first accepted bid
- it must not change after the auction enters `Open`
- a bid amount must be greater than or equal to `bidding.minimum_bid_sats`

Zero is forbidden.

A bid with amount less than `bidding.minimum_bid_sats`
must be rejected.

---

## 4. Admission Conditions (Normative)

A bid must be accepted if and only if all of the following are true:

- system state = `Running`
- auction state is `Scheduled` or `Open`
- all required fields are present
- the bid signature is valid
- the bid payload references the correct auction number
- the bid amount ≥ `bidding.minimum_bid_sats`
- exclusion status is determinable
- the bidding address is not excluded by persisted state

A bid must be rejected if any of the above conditions fail.

No additional criteria are permitted.

---

## 5. First Bid Semantics (Normative)

If auction state = `Scheduled`
and a bid is accepted:

The following must occur atomically:

1. Persist `BidRecord`
2. Persist `AuctionOpenRecord` containing:
   - `opened_at = server_time`
   - `base_end_time = opened_at + auction.duration_seconds`
3. Persist the `Scheduled → Open` transition

No intermediate state is permitted.

If persistence fails,
the bid must be rejected
and the auction must remain `Scheduled`.

---

## 6. Open-State Admission (Normative)

If auction state = `Open`
and a bid is accepted:

- Persist `BidRecord`
- Evaluate extension rule as defined in CORE-SEQUENCE.md
- Persist `ExtensionEventRecord` if required

Extension evaluation must use authoritative `server_time`.

Admission must occur before extension evaluation.

---

## 7. Exclusion Semantics (Normative)

Exclusion status:

- must be derived exclusively from persisted records
- must be evaluated mechanically
- must not depend on operator interpretation
- must not be inferred from absence of data

If exclusion status cannot be determined with certainty,
the bid must be rejected.

---

## 8. Non-Rules (Explicit)

Bid admission must not consider:

- bidder intent
- historical interpretation
- fairness
- probability of settlement
- auction value
- UI presentation
- external economic conditions

Bid admission is binary.

Accept or reject.

---

## Final Rule

If the system cannot prove that a bid
meets all admission requirements:

The bid must be rejected.