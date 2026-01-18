# Reimbursement Operational Policy — Numbers

This document defines the operational policy for reimbursement within the Numbers system.

This policy is **non-normative**.
It does not grant authority, alter outcomes, or modify system truth.

If there is a conflict, PRD.md, CORE-SEQUENCE.md, STATE-MACHINE-TABLE.md, STATE-MACHINE.md, and INVARIANTS.md take precedence.

---

## 1. Purpose

The purpose of reimbursement is to optionally return funds to bidders
under narrowly defined conditions,
without affecting auction outcomes, finalization, or inscription authority.

Reimbursement exists to reduce bidder loss under defined failure conditions.
It is not a guarantee, entitlement, or correction mechanism.

---

## 2. Scope

This policy governs:

- When reimbursement eligibility may be evaluated
- What information may be considered
- What actions may be taken if reimbursement is approved

This policy does not govern:

- Auction resolution
- Settlement outcomes
- Inscription behavior
- Authority restoration
- System correctness

Reimbursement is external to system truth.

---

## 3. Timing of Evaluation

Reimbursement eligibility is evaluated **only after**:

- Auction N is finalized, and
- Inscription state for auction N has reached a terminal state (`Inscribed` or `Ambiguous`)

No reimbursement evaluation occurs before finalization.

No reimbursement evaluation may delay sequence advancement.

---

## 4. Evaluator

Reimbursement evaluation is performed by a **human operator** of Numbers.

- No automated process may approve reimbursement
- No on-chain process participates in evaluation
- No API endpoint triggers reimbursement

Operator judgment may approve or deny reimbursement,
but operator judgment does not modify system state.

---

## 5. Inputs Considered

Reimbursement evaluation may read:

- Final auction records
- Settlement records
- Inscription state (`Inscribed` or `Ambiguous`)
- Observed transaction data
- Bid records submitted during the auction

No inferred data may be used.

No probabilistic reasoning may be recorded as fact.

---

## 6. Eligibility Conditions

A bidder may be considered eligible for reimbursement only if:

- The bidder submitted a valid bid during an open auction, and
- Funds were successfully transferred to the system, and
- A defined reimbursement condition is met

Reimbursement conditions are defined outside this document
and may change without notice.

Eligibility does not imply approval.

---

## 7. Action on Approval

If reimbursement is approved:

- A discretionary payment may be sent to the bidder
- The payment is executed outside the Numbers protocol
- The payment has no on-chain linkage to the auction or inscription

Reimbursement does not:
- Reverse settlement
- Alter destinations
- Modify inscriptions
- Restore authority
- Create precedent

---

## 8. Action on Denial or Failure

If reimbursement is denied or fails:

- No retry is required
- No explanation is required
- No system behavior changes

Reimbursement failure has no effect on:
- Auction truth
- Sequence progression
- Authority boundaries

---

## 9. Authority Constraints

Reimbursement **cannot**:

- Reopen auctions
- Modify resolution
- Change finalization
- Affect inscription authority
- Override ambiguity
- Create new permissions

Reimbursement operates strictly after authority has been exhausted.

---

## 10. Records and Disclosure

Reimbursement actions may be logged externally.

- Logs are optional
- Logs are non-canonical
- Logs do not represent system truth

Absence of reimbursement records does not imply error.

---

## 11. No Guarantees

Reimbursement is:

- Optional
- Discretionary
- Non-guaranteed
- Revocable as a policy

Participation in Numbers implies acceptance that reimbursement may never occur.

---

## 12. Final Rule

If reimbursement would require violating any invariant or state-machine rule:

**Reimbursement must not occur.**
