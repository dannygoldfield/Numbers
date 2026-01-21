# Numbers — Specification Entry and Reading Order

This document defines the **reading order, authority structure, and ingestion rules**
for the Numbers specification.

It is **normative**.

This document introduces **no new system behavior**.
It defines **how behavior-defining documents must be read, ordered, and applied**.

The primary audience is:
- automated implementation agents
- LLM-based code generation systems

Human readability is secondary and non-authoritative.

---

## 1. Purpose

Numbers is specified in plain language but designed to be translated
mechanically into correct software.

The primary goal of the specification is:

> Minimize guess-space during implementation.

This document exists to:
- enforce strict authority ordering
- prevent accidental inference
- eliminate ambiguity about where truth is defined
- support deterministic translation into code, tests, and invariants
- define a single, machine-enforceable ingestion contract

This document governs **ingestion**, not navigation.
It is the behavioral companion to `SUMMARY.md` when used as an mdBook.

---

## 2. Scope and Precedence

This document is **normative** with respect to:

- reading order
- authority ordering
- implementation obligations
- refusal conditions

If a conflict exists:

- this document overrides all **non-normative** documents
- this document does **not** override:
  - `PRD.md`
  - `INVARIANTS.md`
  - `STATE-MACHINE.md`
  - any other explicitly normative specification

This document must not be used to infer system behavior.

---

## 3. Rules of Interpretation (Absolute)

### Rule 1: Authority Is Ordered

Documents earlier in the order have strictly higher authority.

If a contradiction is detected:
- the earlier document prevails
- execution must halt
- no reconciliation or inference is permitted

---

### Rule 2: Silence Forbids

If behavior is not explicitly permitted,
it is forbidden.

---

### Rule 3: Restart Is Reconstruction

State is reconstructed only from persisted records.

No recovery, repair, retry, or inference is implied.

---

## 4. Audience Model

This document is written **exclusively** for:

- automated code generation systems
- LLM-based implementation agents

It is not written for:
- human readers
- operators
- users
- reviewers

Human interpretation is explicitly out of scope.

The implementation agent is treated as a **deterministic executor**, not a designer.

---

## 5. Required Reading Order (Authority Ordering)

An implementation agent **must** ingest and apply the specification
in the following order.

Earlier documents are strictly higher authority.
Later documents must not contradict earlier ones.

1. `PRD.md`  
2. `INVARIANTS.md`  
3. `TRANSITION-INVARIANTS.md`  
4. `STATE-MACHINE.md`  
5. `STATE-MACHINE-TABLE.md`  
6. `STATE-MACHINE-ARTIFACT.json`  
7. `PERSISTENCE.md`  
8. `RESTART-RULES.md`  
9. `CORE-SEQUENCE.md`  
10. `SETTLEMENT.md`  
11. `CONFIG-REFERENCE.md`  
12. `API-STATE-SHAPES.md`  
13. `API-SPEC.md`  
14. `UI-SPEC.md`

If a contradiction is detected:
- the earlier document prevails
- the implementation agent **must halt**
- no attempt may be made to reconcile, guess, or infer intent

---

## 6. Prohibited Behaviors (Absolute)

An implementation agent **must not**:

- invent missing state
- infer authority or permission
- retry actions after ambiguity
- collapse uncertainty into success or failure
- optimize away safety checks
- merge, elide, or rename states
- introduce new transitions
- reinterpret or repair persisted history
- substitute heuristics for specification

If behavior is not explicitly permitted,
it is forbidden.

---

## 7. Allowed Assumptions (Exhaustive)

The implementation agent may assume **only** that:

- persistence is durable and authoritative
- clocks are monotonic within a single execution
- Bitcoin observation is fallible and non-authoritative
- all configuration is validated before startup
- missing records imply permanent loss of authority

No other assumptions are permitted.

In particular, the agent must not assume:
- network reliability
- user honesty
- operator correctness
- eventual consistency
- recoverability after failure

---

## 8. Implementation Obligations

The implementation agent **must** produce:

- code that enforces all invariants mechanically
- explicit fatal errors on illegal state transitions
- idempotent persistence boundaries
- restart-safe execution paths
- ambiguity-preserving behavior
- externally visible and testable state transitions

Silent recovery, automatic retries, or inferred repair are forbidden.

---

## 9. Refusal Condition

If any invariant, transition, or authority rule
cannot be implemented safely,
deterministically,
and without inference:

**The implementation agent must refuse to proceed.**

Partial implementation is not permitted.
Best-effort behavior is not permitted.

---

## 10. Completion Criteria

An implementation is considered complete **only** when:

- all states and transitions are explicit
- restart behavior is pure reconstruction
- ambiguity cannot trigger retries or alternative actions
- authority loss is irreversible
- missing data halts execution
- persisted history is append-only and never rewritten

Anything less is non-compliant.
