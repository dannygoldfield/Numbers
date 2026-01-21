# Transition Invariants — Numbers

This document defines **transition-level invariants** enforced by the Numbers system.

These invariants are **mechanically enforced** by the authoritative state machine
(`STATE-MACHINE.md` and `STATE-MACHINE-TABLE.md`).

This document introduces **no new authority**.
It exists solely to make implicit guarantees explicit and mechanically verifiable.

If a transition violates any invariant listed here,
execution **must halt immediately**.

If there is a conflict,
`STATE-MACHINE.md` and `STATE-MACHINE-TABLE.md` take precedence.

---

## 1. Purpose

Transition invariants exist to:

- bind state transitions to authority boundaries
- prevent inferred, implicit, or speculative transitions
- prevent retries from masquerading as progress
- ensure ambiguity never advances lifecycle state
- provide a mechanical checklist for implementation and audit

A transition invariant applies **at the exact moment a transition is evaluated**.

---

## 2. Lifecycle Transition Invariants

### TI-01. Only Listed Transitions Are Permitted

A transition is permitted **only if** it appears in the
authoritative state machine.

Any transition not explicitly listed is **forbidden and fatal**.

This includes:
- implicit transitions
- inferred transitions
- time-derived assumptions
- operator-initiated shortcuts

---

### TI-02. Transitions Are Directional and Irreversible

Once a transition is committed and persisted:

- the prior state must never be re-entered
- no reverse transition is permitted
- no alternate branch is permitted

This rule applies even if:
- later information is discovered
- an error is identified
- an operator intervenes

---

### TI-03. Terminal States Are Absorbing

If a state is marked terminal:

- no outgoing transitions are permitted
- no overlays may advance the lifecycle
- no retries or compensating actions are permitted

Terminal states include:
- `Finalized` (auction)
- `Inscribed` (inscription)

---

## 3. Authority and Transition Coupling

### TI-04. Authority Is Exercised Only on Explicit Transitions

Authority-bearing actions **must occur only** on explicitly defined transitions.

In particular:

- inscription authority is exercised **only** on
  `Finalized → Inscribing`

No other transition may:
- spend funds
- construct transactions
- broadcast inscriptions
- substitute destinations

---

### TI-05. Authority Is Exercised At Most Once

For each auction number:

- an authority-exercising transition may occur **at most once**
- any subsequent attempt is forbidden, regardless of outcome

This includes:
- retries
- rebroadcasts
- alternate construction paths
- delayed reattempts

---

## 4. Ambiguity Invariants

### TI-06. Ambiguity Is a State, Not Progress

Ambiguity:

- is a real persisted state
- does not advance the auction lifecycle
- does not rewind the lifecycle
- does not permit alternative transitions

When ambiguity is entered:
- lifecycle position is frozen
- only observation is permitted

---

### TI-07. Ambiguity Permanently Freezes Authority

Once ambiguity is recorded:

- no authority-exercising transition is permitted
- no retry is permitted
- no substitute action is permitted

Observation is the **only** permitted activity.

---

### TI-08. Observation Cannot Justify Transitions

Observation may:
- update knowledge
- confirm visibility

Observation must **never**:
- justify a retry
- justify a compensating action
- justify rewriting history
- justify skipping a transition

Observation updates **knowledge only**, never **authority**.

---

## 5. Time and Transition Invariants

### TI-09. Time Alone Does Not Justify Transitions

Time passage causes a transition **only** when explicitly listed
in the authoritative state machine.

This includes:
- `Scheduled → Open`
- `Open → Closed`
- `AwaitingSettlement → Finalized` (deadline expiry)

Time passage must **never**:
- resolve ambiguity
- imply success or failure
- trigger inscription
- unlock authority

---

## 6. Pause Overlay Invariants

### TI-10. Pause Is Not a Lifecycle Transition

Pause:

- does not change lifecycle state
- does not imply progress
- does not resolve pending actions

Pause exists only as a control overlay and must resume
to the **exact prior lifecycle state**.

---

### TI-11. Pause Cannot Cross Authority Boundaries

Pause must not:
- interrupt inscription construction or broadcast
- interrupt settlement evaluation
- straddle an authority-exercising boundary

If authority execution has begun,
pause is forbidden until a safe boundary is reached.

---

## 7. Error and Transition Interaction

### TI-12. Errors Cannot Enable Transitions

Errors may:
- halt execution
- freeze authority
- block retries

Errors must **never**:
- unlock a transition
- justify skipping a state
- enable alternate progression

---

### TI-13. Fatal Errors Abort Transition Evaluation

If a fatal error occurs during transition evaluation:

- the transition must not complete
- no partial effects may persist
- execution must halt immediately

Silent recovery is forbidden.

---

## 8. Persistence and Transition Integrity

### TI-14. Persistence Is Required at Transition Boundaries

For every permitted transition:

- required records **must** be persisted
- persistence failure blocks the transition
- no transition may be assumed without durable records

---

### TI-15. Persisted Transitions Are Immutable

Once a transition is persisted:

- it must never be edited
- it must never be reinterpreted
- it must never be removed

History is append-only.

---

## Final Rule

If a proposed transition violates **any** invariant in this document:

**That transition is forbidden.**
