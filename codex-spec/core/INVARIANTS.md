# Invariants — Numbers

This document defines the invariants that govern the Numbers system.

It is normative.

Authority precedence is defined exclusively in AUTHORITY-ORDER.md.

An invariant is a property that must hold true at all times.
Violating an invariant permanently constrains or halts authority.
Some violations require immediate execution halt.

If a behavior depends on an assumption not stated here,
that assumption is invalid.

---

## Modal Language Rule (Normative)

In all normative documents:

- must / must not define obligations
- only / exactly once / at most once define bounds
- may is permitted only to describe observational uncertainty  
  and must never authorize action, imply permission, or introduce discretion

Any modal usage violating this rule is invalid.

---

## 1. Auction Identity and Order

### I-01. Auction Numbers Are Strictly Monotonic

Auction numbers must increase strictly by 1 when a new auction is created.

- Each auction number appears exactly once
- Auction numbers are never reused
- Auction numbers are never skipped
- Auction numbers are never reordered

---

### I-02. Only One Auction May Be Active

At most one auction may exist in a non-terminal lifecycle state at any time.

A terminal auction state is:

- `Finalized`

Auctions must not overlap.
Concurrent bidding windows are forbidden.

This invariant applies globally.

---

## 2. Resolution and Finality

### I-03. Each Auction Resolves Exactly Once

Each auction resolves exactly once.

- Resolution must be deterministic
- If resolution logic is invoked more than once,
  the resulting outcome must be identical
- Any second resolution attempt that would alter outcome is forbidden

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

Observation must not:
- trigger new authority-bearing transitions
- consume additional authority
- permit retries or substitutions

---

### I-09. Observation Is Knowledge Update Only

Observation is a deterministic evaluation of:

- persisted canonical records
- external authoritative systems
- confirmation depth rules defined in configuration

Observation may update knowledge.
Observation must not create permission.

Observation must not:
- restore authority
- justify retries
- substitute alternate actions
- alter historical records

Human interpretation or operator intent
must not change system state.

Any reimbursement or compensation performed by an operator
is external to the system and must not:
- alter outcomes
- restore authority
- substitute system behavior
- imply system guarantees

---

## 5. Time and Knowledge

### I-10. Time Passing Is Not Evidence

Time passing alone:

- does not resolve ambiguity
- does not imply success or failure
- does not permit retries
- does not grant certainty

Only explicit observation may change knowledge.

---

## 6. State Immutability

### I-11. Persisted History Is Immutable

Once the canonical records corresponding to a transition
are durably persisted:

- they must not be edited
- they must not be removed
- they must not be reinterpreted

History is append-only.

---

### I-12. Terminal States Are Globally Terminal

Once a terminal state is reached:

- no further lifecycle transitions are permitted
- no background process may mutate state
- no operator action may revive the auction

Terminality applies across all subsystems.

---

## 7. State Machine Enforcement

### I-13. Illegal State Transitions Halt Execution

Any transition not permitted by STATE-MACHINE-TABLE.md:

- must halt execution immediately
- must be logged
- must not be retried or auto-corrected

Silent correction is forbidden.

---

## 8. System Pause

### I-14. Pause Is Non-Semantic

System pause:

- does not advance state
- does not change lifecycle truth
- does not alter the meaning of any state
- does not imply outcomes

Pause is an overlay only.

While `system_state = Paused`:

- no new authority-bearing action may begin
- no new bids may be accepted

Time continues to advance while paused.

Deadlines and expiration conditions must evaluate
based on absolute persisted timestamps.

Pause must not extend deadlines,
settlement windows, or auction timing.

---

### I-15. Pause Cannot Restore or Create Authority

System pause:

- must not restore consumed authority
- must not justify retries
- must not authorize alternate actions

Pause does not retroactively affect actions
that were completed before the pause event was persisted.

---

## 9. Error Classification

### I-16. Errors Cannot Downgrade

Once an error escalates:

- it must not downgrade
- authority must not automatically return
- no automated or operator action may downgrade classification

Ambiguous and Fatal errors are terminal with respect to authority.

---

## Final Rule

If any behavior would violate an invariant in this document:

That behavior is forbidden.