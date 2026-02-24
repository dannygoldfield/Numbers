# Transition Invariants — Numbers

This document defines transition-level invariants enforced by the Numbers system.

It is normative.

Authority precedence is defined exclusively in AUTHORITY-ORDER.md.

These invariants are mechanically enforced by the authoritative state machine  
(STATE-MACHINE-TABLE.md and STATE-MACHINE.md).

This document introduces no new authority.  
It exists solely to make transition guarantees explicit and mechanically verifiable.

If a transition violates any invariant listed here,  
execution must halt immediately.

---

## 1. Purpose

Transition invariants exist to:

- bind transitions to explicit authority boundaries
- prevent inferred or speculative transitions
- prevent retries from masquerading as forward progress
- ensure ambiguity never advances lifecycle state
- ensure persistence and state legality remain mechanically enforceable

A transition invariant applies at the exact moment a transition is evaluated.

---

## 2. Lifecycle Transition Invariants

### TI-01. Only Listed Transitions Are Permitted

A transition is permitted only if it appears explicitly in  
STATE-MACHINE-TABLE.md.

Any transition not explicitly listed is forbidden and fatal.

This includes:

- inferred transitions
- speculative transitions
- operator shortcuts
- recovery shortcuts
- implicit time assumptions

---

### TI-02. Transitions Are Directional and Irreversible

Once the canonical record(s) associated with a transition are durably persisted:

- the prior state must never be re-entered
- no reverse transition is permitted
- no alternate branch is permitted

This rule applies even if:

- later information is discovered
- an error is identified
- an operator intervenes

History is append-only.

---

### TI-03. Terminal States Are Absorbing

If a state is defined as terminal in the authoritative state machine:

- no outgoing transitions are permitted
- no background process may advance lifecycle state
- no compensating action may reopen lifecycle state

Terminal states include:

- `Finalized` (auction)
- `Inscribed` (inscription)
- `Ambiguous` (inscription)

---

## 3. Authority and Transition Coupling

### TI-04. Authority Is Exercised Only on Explicit Authority Transitions

Authority-bearing transitions are limited to:

- `NotStarted → Inscribing`

No other transition may:

- construct an inscription transaction
- broadcast an inscription transaction
- substitute a destination
- reallocate funds
- retry a broadcast

---

### TI-05. Authority Is Exercised At Most Once Per Auction

For each auction number:

- the authority-bearing transition may occur at most once
- any subsequent attempt is forbidden

This includes:

- retries
- rebroadcasts
- alternate construction paths
- delayed reattempts after restart

Authority does not return once exercised.

---

## 4. Ambiguity Invariants

### TI-06. Ambiguity Is a Terminal Knowledge State

Ambiguity is a distinct persisted state in the inscription state machine.

Ambiguity:

- does not advance auction lifecycle state
- does not rewind lifecycle state
- does not permit alternate transitions

When ambiguity is entered:

- inscription authority is permanently consumed
- lifecycle position is frozen
- only observation is permitted

---

### TI-07. Ambiguity Freezes Authority Bound to That Action

Once `Inscribing → Ambiguous` occurs:

- no authority-bearing transition is permitted for that auction
- no retry is permitted
- no substitute inscription is permitted

Ambiguity consumes only the authority bound to that inscription attempt.  
It must not implicitly consume unrelated authority domains.

---

### TI-08. Observation Cannot Justify Transitions

Observation may:

- update knowledge
- confirm network visibility
- confirm inscription inclusion

Observation must never:

- justify a retry
- justify a compensating action
- justify rewriting history
- justify skipping a required transition
- restore authority

Observation updates knowledge only, never permission.

---

## 5. Time and Transition Invariants

### TI-09. Time Alone Does Not Justify Transitions

Time passage causes a transition only when explicitly listed  
in STATE-MACHINE-TABLE.md.

Time-driven transitions are limited to:

- `Open → Closed` when `server_time >= current_end_time`
- `AwaitingSettlement → Finalized` when settlement deadline expires

Time must never:

- cause `Scheduled → Open`
- create an auction
- reopen bidding
- resolve ambiguity
- trigger inscription
- restore authority

All time evaluation must use authoritative `server_time`.

---

## 6. System Control Overlay Invariants

### TI-10. System Control States Do Not Modify Lifecycle Truth

System control states such as `Running` and `Paused`:

- do not alter auction lifecycle state
- do not alter inscription lifecycle state
- do not modify `base_end_time`
- do not modify `current_end_time`
- do not alter persisted deadlines

System control states gate acceptance of actions only.

---

### TI-11. System Control Must Not Cross Authority Boundaries

System control transitions must not:

- partially execute an authority-bearing transition
- interrupt an atomic transition
- straddle a persistence boundary

Authority transitions must be atomic with respect to control overlay changes.

---

## 7. Error and Transition Interaction

### TI-12. Errors Cannot Enable Transitions

Errors may:

- halt execution
- freeze authority
- block retries

Errors must never:

- unlock a transition
- justify skipping a required state
- enable alternate progression
- restore authority

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

- the canonical records defined in DATA-MODEL.md for that transition must be durably persisted
- persistence failure blocks the transition
- absence of required canonical records invalidates the transition

---

### TI-15. Persisted Transitions Are Immutable

Once canonical records are persisted:

- they must not be edited
- they must not be removed
- they must not be reinterpreted

History is append-only.

---

## Final Rule

If a proposed transition violates any invariant in this document:

That transition is forbidden.