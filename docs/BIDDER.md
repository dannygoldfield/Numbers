# Bidder — Numbers

This document defines the role of a bidder in the Numbers system.

This document is normative.

---

## Definition

A bidder is an external actor capable of submitting bids to the Numbers system.

A bid may be submitted **only** while an auction is in the `Open` state.
Bids submitted outside this state are invalid and rejected.

A bidder may be an individual, organization, automated agent, or software process.
Identity is not interpreted or verified beyond transaction validity.

---

## Eligibility and Participation

An entity becomes a bidder **only by submitting a bid during an open auction**.

By submitting a bid, the bidder:

1. targets the currently open auction only
2. provides a valid settlement address
3. accepts that outcomes are irreversible

Participation is voluntary and explicit.
No separate agreement is required.

---

## Acceptance of Outcomes

By submitting a bid, the bidder accepts that:

- each auction resolves exactly once
- settlement deadlines are final
- failure to settle results in routing to the NullSteward
- inscription authority may be permanently lost due to ambiguity
- the system does not retry, compensate, or reinterpret outcomes

System behavior follows published invariants only.

---

## Non-Entitlements

A bidder is not entitled to:

- refunds
- retries
- extensions
- reinterpretation of outcomes
- dispute resolution
- explanations beyond exposed system state

System silence is not an error.
Finality is not negotiable.

---

## External Responsibility

Any action taken by the operator outside the system
(e.g. reimbursement, communication, or goodwill gestures)
is external and non-authoritative.

Such actions do not:

- alter auction outcomes
- restore authority
- create precedent
- imply future guarantees

---

## Final Rule

If an entity requires guarantees,
it does not submit a bid.
