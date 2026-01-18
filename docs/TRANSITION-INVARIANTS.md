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

- Bind state transitions to authority boundaries
- Prevent inferred, implicit, or speculative transitions
- Prevent retries from masquerading as progress
- Ensure ambiguity never advances the lifecycle
- Provide a mechanical checklist for implementation and audit

A transition invariant applies **at the exact moment a transition is evaluated**.

---

## 2. Lifecycle Transition Invariants

### TI-01. Only Listed Transitions Are Permitted

A transition is permitted **only if** it appears in
`allowed_transitions` of the authoritative state machine.

Any transition not explicitly listed is **forbidden and fatal**.

This prohibition includes:
- implicit transitions
- inferred transitions
- time-derived assumptions
- operator-initiated shortcuts

---

### TI-02. Transitions Are Directional and Irreversible

Once a transition is committed:

- The prior state must never be re-entered
- No reverse transition is permitted
- No alternate branch is permitted

This rule applies even if:
- later information is discovered
- an error is identified
- an operator intervenes

---

### TI-03. Terminal States Are Absorbing

If a state is marked `terminal = true`:

- No outgoing transitions are permitted
- No overlays may advance the lifecycle
- No retries or compensating actions are permitted

Terminal states include:
- `Finalized`
- `Inscribed`

---

## 3. Authority and Transition Coupling

### TI-04. Authority Is Exercised Only on Explicit Transitions

Authority is exercised **only** on the following transition:

- `AwaitingSettlement → Inscribing`

No other transition is permitted to:
- spend funds
- construct transactions
- broadcast inscriptions
- claim ownership

---

### TI-05. Authority Is Exercised at Most Once

For each auction:

- An authority-exercising transition may occur **at most once**
- Any subsequent attempt is forbidden, regardless of outcome

This prohibition includes:
- retries
- rebroadcasts
- alternate construction paths
- delayed reattempts

---

## 4. Ambiguity Invariants

### TI-06. Ambiguity Does Not Advance the Lifecycle

Ambiguity:

- Is not a lifecycle-advancing transition
- Does not advance the lifecycle
- Does not rewind the lifecycle

When ambiguity is detected:
- The system remains in the current lifecycle state
- No further lifecycle transitions are permitted

---

### TI-07. Ambiguity Permanently Freezes Authority

Once ambiguity exists:

- No authority-exercising transition is permitted
- No retry is permitted
- No alternative action is permitted

Observation is the **only** permitted activity.

---

### TI-08. Observation Cannot Justify Transitions

External observation may:
- update knowledge
- confirm visibility

Observation must **never**:
- justify a retry
- justify a compensating action
- justify rewriting history

Observation updates **representation only**, never **authority**.

---

## 5. Time and Transition Invariants

### TI-09. Time Alone Does Not Cause Transitions

Time passage causes a transition **only** when explicitly listed:

- `Scheduled → Open`
- `Open → Closed`
- `AwaitingSettlement → Finalized` (deadline expiration)

Time passage must **never**:
- resolve ambiguity
- imply success or failure
- trigger inscription
- unlock authority

---

## 6. Pause Overlay Invariants

### TI-10. Pause Is Not a Transition

Pause:

- Does not change lifecycle state
- Does not infer progress
- Does not resolve pending actions

Pause is permitted only to:
- block external interaction
- resume to the **exact prior lifecycle state**

---

### TI-11. Pause Cannot Cross Authority Boundaries

Pause must not:
- interrupt settlement execution
- interrupt inscription broadcast
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

- The transition must not complete
- No partial effects may persist
- Execution must halt immediately

Silent recovery is forbidden.

---

## 8. Persistence and Transition Integrity

### TI-14. Persistence Is Required at Transition Boundaries

For every permitted transition:

- State must be persisted **before** authority is exercised
- Persistence failure blocks the transition
- No transition may be assumed without persistence

---

### TI-15. Persisted Transitions Are Immutable

Once a transition is persisted:

- It must never be edited
- It must never be reinterpreted
- It must never be removed

History is append-only.

---

## 9. Final Rule

If a proposed transition violates **any** invariant in this document:

**That transition is forbidden.**
