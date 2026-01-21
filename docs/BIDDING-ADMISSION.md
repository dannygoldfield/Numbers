# Bidding Admission — Numbers

This document defines **bid admission rules** for Numbers.

It is **normative**.

Bid admission governs whether a bid
**may be accepted or must be rejected**
at the moment it is received.

It does not define auctions, settlement, outcomes, or meaning.

If there is a conflict,
INVARIANTS.md, CORE-SEQUENCE.md, and STATE-MACHINE.md take precedence.

---

## Purpose

Bid admission exists to:

- prevent invalid bids from entering the system
- enforce irreversible exclusions deterministically
- keep downstream phases free of eligibility logic

Bid admission is a gate.
It does not negotiate, infer, or explain.

---

## Admission Scope (Normative)

Bid admission applies **only** when:

- the auction is in `Open`
- a bid submission is received

Bid admission does not apply:
- after auction close
- during settlement
- during inscription
- during observation or restart

---

## Admission Rules (Normative)

A bid **must be accepted** if and only if:

- the auction is `Open`
- the bid signature is valid
- the bid payload matches the auction number
- the bidding address is not excluded by persisted state

A bid **must be rejected** if:

- any required field is missing
- the signature is invalid
- the auction is not `Open`
- the bidding address is excluded by persisted state

No other criteria are permitted.

---

## Exclusion Semantics (Normative)

Exclusion status:

- is determined solely by persisted records
- is evaluated mechanically
- must not depend on operator judgment
- must not be inferred

If exclusion status cannot be determined with certainty,
the bid **must be rejected**.

---

## Non-Rules (Explicit)

Bid admission **must not** consider:

- bidder intent
- past behavior interpretation
- fairness
- probability of settlement
- auction value
- UI presentation

Bid admission is binary.
Accept or reject.

---

## Final Rule

If the system cannot prove that a bid
meets **all admission requirements**:

**The bid must be rejected.**
