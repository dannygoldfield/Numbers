# Protocol Core — Numbers

This document defines the **core idea and operating philosophy** of the Numbers protocol.

It is **non-normative**.

This document exists to orient human readers.
It does not grant authority, define behavior, or override any specification.

If there is a conflict,
CORE-SEQUENCE.md, STATE-MACHINE.md, INVARIANTS.md,
and PERSISTENCE.md take precedence.

---

## What Numbers Is

Numbers is a sequential auction protocol that:

- advances one number at a time
- resolves each auction exactly once
- records outcomes immutably
- refuses to invent certainty when knowledge is incomplete

Each number represents a **single, irreversible opportunity**
to be resolved by procedure.

Numbers counts forward.
It never rewinds.

---

## What Numbers Is Not

Numbers is not:

- a marketplace
- a social system
- a fairness engine
- a trustless protocol
- a recovery-oriented system

Numbers does not optimize for:
- availability
- convenience
- throughput
- maximal participation

Correctness outweighs all of these.

---

## Core Commitments

Numbers makes the following commitments:

1. **Irreversibility**
   - Each auction resolves exactly once
   - Outcomes are final once recorded

2. **Containment**
   - Failure is allowed
   - Ambiguity is allowed
   - Authority reuse is not allowed

3. **Explicit Authority**
   - Every action that matters consumes authority
   - Authority is burned, not recycled

4. **Procedural Truth**
   - Only outcomes produced by the system’s full procedure, including settlement, are canonica
   - Identical on-chain artifacts created outside the system are irrelevant

---

## Loss as a First-Class Outcome

Numbers treats loss as valid.

Loss includes:
- no-bid auctions
- failed settlement
- inscription ambiguity
- funds routed to NullSteward

Loss is not an error condition.
It is the price of refusing to lie.

---

## Trust Model

Numbers separates trust explicitly:

- Bitcoin is trusted for transaction existence and immutability
- The operator is trusted to execute the procedure
- The system is trusted to refuse illegal actions

Numbers does not attempt to automate trust away.

---

## Why the System Stops Instead of Guessing

If the system cannot determine whether an authority-bearing action has already occurred, it assumes it has and refuses to proceed. This refusal applies to the affected authority, not to the overall sequence of future auctions.

Stopping is not failure.
Guessing is failure.

---

## Relationship to Other Documents

This document is intentionally incomplete.

For canonical definitions, see:

- CORE-SEQUENCE.md — temporal ordering
- STATE-MACHINE.md — legal states and transitions
- INVARIANTS.md — global correctness rules
- PERSISTENCE.md — authority preservation
- OPERATIONAL-RUNBOOK.md — constrained human action

This document explains *why*.
Those documents define *what*.

---

## Final Note

Numbers is a system that records outcomes,
not intentions.

Meaning is external.
Interpretation is external.

The protocol’s only obligation
is to refuse to lie.
