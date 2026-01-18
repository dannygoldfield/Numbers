# Implementation Entry — Numbers

This document defines how an implementation agent must ingest
and apply the Numbers specification.

This document is **normative**.

If there is any conflict, this document takes precedence
over all **non-normative** documents.
It does not override PRD.md, INVARIANTS.md, or STATE-MACHINE.md.

---

## 1. Audience

This document is written exclusively for:

- automated code generation systems
- LLM-based implementation agents

It is not written for human readers,
operators,
or users of the Numbers system.

Human interpretation is explicitly out of scope.

---

## 2. Required Reading Order (Authority Ordering)

An implementation agent **must** read and apply the specification
in the following order.

Earlier documents are strictly higher authority.
Later documents must not contradict earlier ones.

1. PRD.md  
2. INVARIANTS.md  
3. TRANSITION-INVARIANTS.md  
4. STATE-MACHINE.md  
5. STATE-MACHINE-TABLE.md  
6. PERSISTENCE.md  
7. RESTART-RULES.md  
8. CORE-SEQUENCE.md  
9. CONFIG-REFERENCE.md  
10. API-STATE-SHAPES.md  
11. API-SPEC.md  
12. UI-SPEC.md  

If a contradiction is detected:
- the earlier document prevails
- the implementation agent must halt

---

## 3. Prohibited Behaviors (Absolute)

An implementation agent **must not**:

- invent missing state
- infer authority or permission
- retry actions after ambiguity
- collapse uncertainty into success or failure
- optimize away safety checks
- merge or elide states
- introduce new transitions
- reinterpret persisted history

If behavior is not explicitly permitted,
it is forbidden.

---

## 4. Allowed Assumptions

The implementation agent may assume:

- persistence is durable and authoritative
- clocks are monotonic within a single execution
- Bitcoin observation is fallible and non-authoritative
- all configuration is validated before startup
- missing records imply loss of authority

No other assumptions are permitted.

---

## 5. Implementation Obligations

The implementation agent **must** produce:

- code that enforces all invariants mechanically
- explicit fatal errors on illegal state transitions
- idempotent persistence boundaries
- restart-safe execution paths
- ambiguity-preserving behavior
- testable, externally visible state transitions

Silent recovery is forbidden.

---

## 6. Refusal Condition

If any invariant cannot be implemented safely,
deterministically,
and without inference:

**The agent must refuse to proceed.**

Partial implementation is not permitted.

---

## 7. Completion Criteria

An implementation is considered complete **only** when:

- all state transitions are explicit
- restart behavior is deterministic
- ambiguity cannot trigger retries
- authority loss is irreversible
- missing data halts execution
- persisted history is never rewritten

Anything less is non-compliant.
