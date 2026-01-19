# Implementation Entry — Numbers

This document defines how an implementation agent must ingest,
interpret, and apply the Numbers specification.

This document is **normative**.

If there is a conflict, this document takes precedence over all
**non-normative** documents.
It does not override PRD.md, INVARIANTS.md, STATE-MACHINE.md,
or any other normative specification.

---

## 1. Audience

This document is written exclusively for:

- automated code generation systems
- LLM-based implementation agents

It is not written for:
- human readers
- operators
- users
- reviewers

Human interpretation is explicitly out of scope.

The implementation agent is treated as a deterministic executor,
not a designer.

---

## 2. Required Reading Order (Authority Ordering)

An implementation agent **must** ingest and apply the specification
in the following order.

Earlier documents are strictly higher authority.
Later documents must not contradict earlier ones.

1. PRD.md  
2. INVARIANTS.md  
3. TRANSITION-INVARIANTS.md  
4. STATE-MACHINE.md  
5. STATE-MACHINE-TABLE.md  
6. STATE-MACHINE-ARTIFACT.json  
7. PERSISTENCE.md  
8. RESTART-RULES.md  
9. CORE-SEQUENCE.md  
10. SETTLEMENT.md  
11. CONFIG-REFERENCE.md  
12. API-STATE-SHAPES.md  
13. API-SPEC.md  
14. UI-SPEC.md  

If a contradiction is detected:
- the earlier document prevails
- the implementation agent **must halt**
- no attempt may be made to reconcile or infer intent

---

## 3. Prohibited Behaviors (Absolute)

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

## 4. Allowed Assumptions

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

## 5. Implementation Obligations

The implementation agent **must** produce:

- code that enforces all invariants mechanically
- explicit fatal errors on illegal state transitions
- idempotent persistence boundaries
- restart-safe execution paths
- ambiguity-preserving behavior
- externally visible and testable state transitions

Silent recovery, automatic retries, or inferred repair are forbidden.

---

## 6. Refusal Condition

If any invariant, transition, or authority rule
cannot be implemented safely,
deterministically,
and without inference:

**The implementation agent must refuse to proceed.**

Partial implementation is not permitted.
Best-effort behavior is not permitted.

---

## 7. Completion Criteria

An implementation is considered complete **only** when:

- all states and transitions are explicit
- restart behavior is pure reconstruction
- ambiguity cannot trigger retries or alternative actions
- authority loss is irreversible
- missing data halts execution
- persisted history is append-only and never rewritten

Anything less is non-compliant.
