# Authority Consumption — Numbers

This document defines **authority consumption rules** for the Numbers system.

It is **normative**.

Authority consumption governs **when permission is irreversibly used**
and **when it is permanently lost**.

If there is a conflict,
INVARIANTS.md, STATE-MACHINE.md, PERSISTENCE.md,
and RESTART-RULES.md take precedence.

---

## Purpose

Authority exists to ensure that:

- actions happen **at most once**
- outcomes are never duplicated
- retries do not recreate permission
- uncertainty reduces power rather than increasing it

Authority is not intent.
Authority is not success.
Authority is the **right to attempt an irreversible action exactly once**.

---

## Definitions

- **Authority**  
  The system’s permission to perform an irreversible action.

- **Authority Consumption**  
  The irreversible use or loss of authority.

- **Authority Exhaustion**  
  A terminal condition where authority can no longer be exercised,
  regardless of outcome or intent.

Authority is consumed by **attempt or uncertainty**, not by success.

---

## Authority Scope (Normative)

Numbers defines authority in the following scopes only:

1. Auction resolution authority
2. Inscription authority

No other authority exists.

Authority scopes are independent.
Consuming one does not restore or imply another.

---

## General Authority Rules (Normative)

### A-01. Authority Is Finite

For each auction number **N**:

- each authority scope exists **at most once**
- authority is never renewed
- authority is never recreated by time, restart, or operator action

---

### A-02. Authority Is Consumed Before Outcome Is Known

Authority is consumed when the system:

- commits to an irreversible action
- performs an action that cannot be proven not to have occurred

Success is irrelevant.
Failure still consumes authority.

---

### A-03. Persistence Precedes Authority

Any authority-bearing action **must** be preceded by
a durable persistence record.

If persistence does not occur:
- authority **must not** be exercised
- execution **must** halt or pause safely

Authority without memory is forbidden.

---

### A-04. Uncertainty Consumes Authority

If the system cannot prove that an authority-bearing action
did **not** occur:

- authority **must** be treated as consumed
- retries are forbidden
- alternate actions are forbidden

Uncertainty reduces authority.
It never restores it.

---

## Auction Resolution Authority

Auction resolution authority:

- exists exactly once per auction
- is consumed when resolution is recorded
- determines the winning bid or no-bid outcome

Rules:

- resolution **must** be recorded exactly once
- resolution **must not** be recomputed
- resolution authority **cannot** be reclaimed

Resolution authority is consumed regardless of downstream success.

---

## Settlement Phase (Non-Authority)

Settlement does **not** grant authority.

Settlement is a bounded observation window during which:

- payment **may** be observed
- absence of payment **may** be observed

Rules:

- settlement does not permit retries
- settlement does not grant permission
- settlement outcome must be recorded exactly once
- late payment must not be accepted

Settlement outcome does not create or restore inscription authority.

---

## Inscription Authority

Inscription authority:

- exists at most once per auction
- permits exactly one inscription attempt
- is permanently exhausted by ambiguity

### Pre-Initiation Failure (Normative)

If inscription construction fails **before** an inscription initiation
record is durably persisted:

- inscription authority is not yet consumed
- retry is permitted

This is the **only** retry condition.

---

### Post-Initiation Ambiguity (Normative)

If an inscription initiation record exists and the system:

- broadcasts a transaction, or
- cannot prove that no broadcast occurred

Then:

- inscription authority is **permanently exhausted**
- retries are forbidden
- replacement is forbidden
- override is forbidden
- observation is the only permitted activity

Time passing does not restore authority.

---

## Authority and Restart (Normative)

On restart:

- authority state is reconstructed **only** from persisted records
- missing records imply consumed authority
- ambiguous states remain ambiguous

Restart **must never** be used to:

- retry an authority-bearing action
- escape ambiguity
- recreate permission

Restart restores memory.
It does not grant power.

---

## Forbidden Authority Patterns

The following are explicitly forbidden:

- retrying after uncertainty
- retrying after timeout
- retrying after restart
- retrying after operator intervention
- retrying after partial success
- retrying to “complete” a previous attempt

If authority status is unclear,
the system **must assume authority is exhausted**.

---

## Final Rule

Authority is easier to lose than to prove.

If the system cannot prove
that an authority-bearing action did not occur:

**It must behave as though it did.**
