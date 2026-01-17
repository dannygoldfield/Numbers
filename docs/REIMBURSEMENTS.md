# Reimbursements — Numbers (Non-Normative)

This document records discretionary reimbursements made by the operator of Numbers.

This document is **not normative**.
It does **not** define system behavior, guarantees, or obligations.

Reimbursements recorded here are external human actions.
They do not alter, restore, substitute, or imply any auction, settlement,
inscription, or authority outcome within the Numbers system.

If there is any conflict, STATE-MACHINE.md, INVARIANTS.md, and CORE-SEQUENCE.md
take precedence.

---

## Purpose

This document exists to provide transparency and accountability
for discretionary reimbursements made by the operator.

Reimbursement is not part of the Numbers protocol.
It is not automatic, guaranteed, or implied by any system state.

---

## Scope

This document may record reimbursements related to:

- ambiguous inscription outcomes
- operational failures
- external disruptions
- discretionary goodwill actions

This document does **not**:

- define entitlement
- create precedent
- imply future reimbursement
- restore system authority
- alter recorded outcomes

---

## Authority Boundary

Reimbursement is an external human action performed by the operator.

It does not:

- create new authority
- restore consumed authority
- permit retries or substitutions
- modify auction or inscription state
- affect canonical recognition

Numbers remains correct regardless of whether reimbursement occurs.

---

## Recording Rules

Each reimbursement entry must:

- reference a specific auction number
- state a concrete reason
- record the amount and payment method
- include a payment reference if applicable
- avoid speculative language

Entries must describe **what happened**, not **what should have happened**.

---

## Reimbursement Entries

### Entry Template

```md
### YYYY-MM-DD — Auction N

- Auction number: N
- Recipient identifier: <address, masked wallet, or internal reference>
- Reason: <brief factual description>
- Amount: <amount and unit>
- Payment method: <BTC txid / fiat method / other>
- Date issued: <date>
- Notes: <optional>
