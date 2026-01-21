# Invariants — Numbers

This document defines the invariants that govern the Numbers system.

It is **normative**.

This document assumes familiarity with:
- CORE-SEQUENCE.md
- STATE-MACHINE.md
- STATE-MACHINE-TABLE.md

An invariant is a property that must hold true at all times.
Violating an invariant permanently constrains or halts authority.
Some violations require immediate execution halt.

If a behavior depends on an assumption not stated here,
that assumption is invalid.

If there is a conflict,
PRD.md, CORE-SEQUENCE.md, STATE-MACHINE-TABLE.md, and STATE-MACHINE.md take precedence.

---

## Modal Language Rule (Normative)

In all normative documents:

- **must / must not** define obligations
- **only / exactly once / at most once** define bounds
- **may** is permitted **only** to describe observational uncertainty  
  and must never authorize action, imply permission, or introduce discretion

Any modal usage violating this rule is invalid.

---

## 1. Auction Identity and Order

### I-01. Auction Numbers Are Monotonic

Auction numbers advance strictly forward.

- Each auction number appears exactly once
- Auction numbers are never reused
- Auction numbers are never skipped or reordered

Once auction `N` completes, the system advances to `N+1`.

---

### I-02. Only One Auction May Be Active

At most one auction may exist in a non-terminal lifecycle state at any time.

- Auctions must not overlap
- Concurrent bidding windows are forbidden

This invariant applies globally.

---

## 2. Resolution and Finality

### I-03. Each Auction Resolves Exactly Once

Each auction resolves exactly once.

- Resolution is idempotent
- Re-running resolution must not alter outcome
- Any second resolution attempt is forbidden

---

### I-04. Resolution Is Final

Once resolution occurs:

- The winner, or lack of winner, is fixed
- Resolution must not be revised
- Resolution must not be overridden

---

## 3. Settlement Authority

### I-05. Settlement Outcome Is Irreversible

Once settlement reaches a terminal state
(`settled`, `expired`, or `not_required`):

- The outcome must not change
- Late payment must not be accepted
- Settlement authority must not return

---

### I-06. NullSteward Is a Valid Outcome

Routing to `NullSteward` is a valid and complete outcome.

- It is not a failure
- It does not imply error
- It does not reduce system correctness

---

## 4. Inscription Authority

### I-07. Inscription Authority Is Exercised At Most Once

For each auction:

- At most one inscription attempt is permitted
- Inscription authority must not be retried, duplicated, or substituted

---

### I-08. Ambiguity Permanently Consumes Authority

If an inscription outcome is ambiguous:

- Inscription authority is permanently consumed
- Retry, rebroadcast, replacement, or override is forbidden
- Time passing does not restore certainty or permission

Observation is the only permitted activity.
No state transition, authority consumption, or retry may occur as a result of observation.

---

### I-09. Observation Cannot Create Authority

Observation updates knowledge only.

Observation must not:
- permit new actions
- restore authority
- enable retries or substitutions

Observation is limited to deterministic system processes.

Human interpretation, operator intent, or subjective assessment
does not constitute observation
and must not change system state.

Knowledge change does not imply permission.

Any reimbursement or compensation performed by an operator
is an external human action and must not:
- alter outcomes
- restore authority
- substitute system behavior
- imply system guarantees

---

## 5. Time and Knowledge

### I-10. Time Passing Is Not Evidence

Time passing alone:

- Does not resolve ambiguity
- Does not imply success or failure
- Does not permit retries
- Does not grant certainty

Only explicit observation may change knowledge.

---

## 6. State Immutability

### I-11. Past States Are Immutable

Once a state is exited:

- It must not be edited
- It must not be reinterpreted
- It must not be rewritten

History is append-only.

---

### I-12. Terminal States Are Globally Terminal

Once a terminal state is reached:

- No further lifecycle transitions are permitted
- No background process may mutate state
- No operator action may revive the auction

Terminality applies across all subsystems.

---

## 7. State Machine Enforcement

### I-13. Illegal State Transitions Halt Execution

Any transition not permitted by STATE-MACHINE.md:

- Must halt execution immediately
- Must be logged
- Must not be retried or auto-corrected

Silent correction is forbidden.

---

## 8. System Pause

### I-14. Pause Is Non-Semantic

System pause:

- Does not advance state
- Does not extend or compress lifecycle timing
- Does not change meaning of any state
- Does not imply outcomes

Pause is an overlay only.

Pause must not extend deadlines, settlement windows, or auction timing.

---

### I-15. Pause Cannot Interrupt Authority

System pause:

- Must not interrupt bidding, settlement, or inscription
- Must occur only at safe boundaries
- Must not infer outcomes during pause

---

## 9. Error Classification

### I-16. Errors Cannot Downgrade

Once an error escalates:

- It must not downgrade
- Authority must not automatically return

Ambiguous and Fatal errors are terminal with respect to authority.

---

## 10. Final Rule

If any behavior would violate an invariant in this document:

**That behavior is forbidden.**
