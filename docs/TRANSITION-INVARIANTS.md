# Transition Invariants — Numbers

This document defines **transition-level invariants** relied upon by the Numbers system.

These invariants are **enforced mechanically** by `STATE-MACHINE.json`.

This document is **derived** from the following authoritative sources:

- STATE-MACHINE.md / STATE-MACHINE-TABLE.md
- INVARIANTS.md
- ERROR-TAXONOMY.md

This document introduces **no new authority**.
It exists solely to make implicit guarantees explicit and mechanically verifiable.

If a transition violates any invariant listed here, execution **must halt immediately**.

If there is any conflict, **STATE-MACHINE.md / STATE-MACHINE-TABLE.md take precedence**.

---

## 1. Purpose

Transition invariants exist to:

- Bind state transitions to authority rules
- Prevent hidden or inferred transitions
- Prevent retries from masquerading as progress
- Ensure ambiguity never advances the lifecycle
- Provide a mechanical checklist for implementation, auditing, and verification

A transition invariant applies **at the exact moment a transition is evaluated**.

---

## 2. Lifecycle Transition Invariants

### TI-01. Only Listed Transitions May Occur

A state transition is permitted **only if** it appears in `allowed_transitions`
of the authoritative state machine.

Any attempted transition not listed is **forbidden and fatal**.

This prohibition includes:
- implicit transitions
- inferred transitions
- time-based assumptions
- operator-triggered shortcuts

---

### TI-02. Transitions Are Directional and Irreversible

Once a transition occurs:

- The prior state must never be re-entered
- No reverse transition is permitted
- No alternate branch may be explored

This holds even if:
- later information appears
- an error is discovered
- an operator intervenes

---

### TI-03. Terminal States Are Absorbing

If a state is marked `terminal = true`:

- No outgoing transitions are permitted
- No overlays may advance the lifecycle
- No retries or compensating actions are allowed

Terminal states include:
- Finalized
- Inscribed

---

## 3. Authority and Transition Coupling

### TI-04. Authority Is Exercised Only on Specific Transitions

Authority may be exercised **only** on the following transition:

- AwaitingSettlement → Inscribing

No other transition may:
- spend funds
- construct transactions
- broadcast inscriptions
- claim ownership

---

### TI-05. Authority Cannot Be Exercised More Than Once

For any auction:

- Each authority-exercising transition may occur **at most once**
- A second attempt is forbidden, regardless of outcome

This includes:
- retries
- rebroadcasts
- alternate construction paths

---

## 4. Ambiguity Transition Invariants

### TI-06. Ambiguity Is Not a Transition

Ambiguity:

- Is not a state
- Does not advance the lifecycle
- Does not rewind the lifecycle

When ambiguity is detected:
- The system remains in the current lifecycle state
- No new transition is permitted

---

### TI-07. Ambiguity Permanently Freezes Authority

Once ambiguity exists:

- No authority-exercising transition may occur
- No retry may be attempted
- No alternative action may be substituted

Observation is the **only** permitted activity.

---

### TI-08. Observation After Ambiguity Does Not Imply a Transition

External observation may:
- update knowledge
- confirm an outcome

But must **not**:
- justify a retry
- justify a compensating action
- justify rewriting history

Observation may update **representation**, not **authority**.

---

## 5. Time and Transition Invariants

### TI-09. Time Passing Does Not Cause Transitions Except Where Explicit

Time passing may cause a transition **only** when explicitly listed:

- Scheduled → Open
- Open → Closed
- AwaitingSettlement → Finalized (deadline expired)

Time passing must **not**:
- resolve ambiguity
- imply success or failure
- trigger inscription

---

## 6. Pause Overlay Invariants

### TI-10. Pause Is Not a Transition

Pause:

- Does not change lifecycle state
- Does not advance time inference
- Does not resolve pending actions

Pause may only:
- block external interaction
- resume to the **exact prior lifecycle state**

---

### TI-11. Pause Cannot Cross Authority Boundaries

Pause must not:
- interrupt settlement execution
- interrupt inscription broadcast
- straddle an authority-exercising transition

If authority execution has begun, pause is forbidden until a safe boundary is reached.

---

## 7. Error Escalation and Transitions

### TI-12. Errors May Escalate but Never Enable Transitions

Errors may:
- halt execution
- freeze authority
- block retries

Errors must **never**:
- unlock a transition
- justify a skipped state
- permit alternate progression

---

### TI-13. Fatal Errors Abort Transition Processing

If a fatal error occurs during transition evaluation:

- The transition must not complete
- No partial effects may persist
- The system must halt immediately

Silent recovery is forbidden.

---

## 8. Persistence and Transition Integrity

### TI-14. Persistence Is Required at Transition Boundaries

For every permitted transition:

- State must be persisted before authority execution
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

If a transition would violate **any** invariant in this document:

**That transition is forbidden.**
