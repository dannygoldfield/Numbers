# Invariants — Numbers

This document defines the invariants that govern the Numbers system.

It is normative.

An invariant is a property that must hold true at all times.
Violating an invariant permanently constrains or halts authority.
Some violations require immediate execution halt.

If a behavior depends on an assumption that is not stated here,
that assumption is invalid.

If there is a conflict, STATE-MACHINE.md takes precedence.

---

## 1. Auction Identity and Order

### I-01. Auction Numbers Are Monotonic

Auction numbers advance strictly forward.

- Each auction number appears exactly once
- Auction numbers are never reused
- Auction numbers are never skipped backward

Once auction `N` completes, the system advances to `N+1`.

---

### I-02. Only One Auction May Be Active

At most one auction may be in a non-terminal lifecycle state at any time.

- Auctions must not overlap
- No concurrent bidding windows are permitted
- No parallel settlement or inscription processes may exist

This invariant applies globally.

---

## 2. Resolution and Finality

### I-03. Each Auction Resolves Exactly Once

An auction may be resolved only once.

- Resolution is idempotent
- Re-running resolution must not alter outcome
- A second resolution attempt is forbidden

### I-04. Resolution Is Final

Once resolution occurs:

- The winner (or lack of winner) is fixed
- Resolution cannot be revised
- Resolution cannot be overridden

---

## 3. Settlement Authority

### I-05. Settlement Outcome Is Irreversible

Once settlement reaches a terminal state (`settled`, `expired`, `not_required`):

- The outcome cannot change
- Late payment cannot be accepted
- Authority does not return

### I-06. NullSteward Is a Valid Destination

Routing to `NullSteward` is a valid and normal outcome.

- It is not a failure
- It does not imply error
- It does not reduce system correctness

---

## 4. Inscription Authority

### I-07. Inscription Authority Is Exercised At Most Once

For each auction:

- At most one inscription attempt may occur
- Inscription authority cannot be retried or duplicated

### I-08. Ambiguity Permanently Consumes Authority

If inscription outcome is ambiguous:

- Authority is permanently reduced
- No retry, rebroadcast, or override is permitted
- Time passing does not restore certainty

Observation is the only permitted action.

---

### I-09. Observation Cannot Create Authority

Observation may update knowledge only.

- Observation must not permit new actions
- Observation must not restore authority
- Observation must not enable retries or substitutions

Knowledge change does not imply permission.

---

## 5. Time and Knowledge

### I-10. Time Passing Is Not Evidence

Time passing alone:

- Does not resolve ambiguity
- Does not imply success or failure
- Does not permit retries or inferred certainty

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

### I-13. Illegal State Transitions Are Fatal to Execution

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
- Does not delay or extend lifecycle timing
- Does not change meaning of any state
- Does not imply outcomes

Pause is an overlay only.

### I-15. Pause Cannot Interrupt Authority

System pause:

- Must not interrupt bidding, settlement, or inscription
- May occur only at safe boundaries
- Must not infer outcomes during pause

---

## 9. Representation Integrity

### I-16. APIs Represent Knowledge Only

API output must represent:

- What is known
- What is unknown

APIs must not represent:

- Inference
- Probability
- Assumed certainty

---

## 10. Error Classification

### I-17. Errors Cannot Downgrade

Once an error escalates:

- It must not downgrade
- Authority does not automatically return

Ambiguous and Fatal errors are terminal with respect to authority.

---

## 11. Final Rule

If any behavior would violate an invariant:

**That behavior is forbidden.**
