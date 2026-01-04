# State Machine — Canonical Table

This document defines the authoritative state machine for Numbers.

It is normative.

All system behavior must conform to this table.
If any other document contradicts this table, **this table wins**.

This table defines:
- All valid states
- All permitted transitions
- All forbidden transitions
- Terminal states
- Authority boundaries

---

## State Definitions

| State | Description | Terminal |
|------|-------------|----------|
| Scheduled | Auction is known but not yet open | No |
| Open | Auction is accepting bids | No |
| Closed | Auction has closed to bids | No |
| AwaitingSettlement | Winning bid exists, settlement pending | No |
| Finalized | Auction resolved without inscription | Yes |
| Inscribing | Inscription attempt in progress | No |
| Inscribed | Canonical inscription observed | Yes |
| Paused | System pause overlay, not a lifecycle state | N/A |

---

## Allowed Transitions

| From State | Trigger | To State | Notes |
|-----------|--------|----------|------|
| Scheduled | start_time reached | Open | Automatic |
| Open | end_time reached | Closed | Automatic |
| Open | operator pause | Paused | Overlay only |
| Paused | operator resume | Open | No inference |
| Closed | resolution with winning bid | AwaitingSettlement | Deterministic |
| Closed | resolution with no bids | Finalized | No settlement |
| AwaitingSettlement | settlement completed | Inscribing | Authority exercised |
| AwaitingSettlement | settlement deadline expired | Finalized | Destination = NullSteward |
| Inscribing | inscription confirmed | Inscribed | Terminal |

---

## Ambiguity Rule (Non-Transition)

Ambiguity is **not** a state and does **not** produce a transition.

If ambiguity is detected during **Inscribing**:

- The system remains in the **current state**
- All execution authority is permanently frozen
- No retries, rebroadcasts, or alternate actions are permitted
- Observation is the only allowed activity

Ambiguity permanently constrains future behavior but does not advance or rewind the lifecycle.

---

## Forbidden Transitions

The following transitions are **never permitted**:

| Forbidden Transition | Reason |
|---------------------|--------|
| Open → Scheduled | Time reversal |
| Closed → Open | Bidding cannot reopen |
| Finalized → Any | Terminal state |
| Inscribed → Any | Terminal state |
| AwaitingSettlement → Open | Settlement does not reopen bidding |
| Inscribing → AwaitingSettlement | Authority cannot be reclaimed |
| Any → Inscribing without settlement | Authority violation |
| Paused → Any non-previous state | Pause does not advance state |
| Any transition caused by ambiguity | Ambiguity is not a transition |

---

## Authority Rules

- Authority is first exercised at **settlement**
- Authority may be further exercised during **inscription**
- Authority is permanently reduced upon **ambiguity**
- Authority is never restored by:
  - time passing
  - operator action
  - retries
  - observation delay

---

## Persistence Requirements

State **must** be persisted at:

- Entry to Open
- Entry to Closed
- Entry to AwaitingSettlement
- Before any inscription attempt
- Upon detection of ambiguity
- Upon reaching any terminal state

---

## Final Rule

If a transition is not listed as allowed:

**It is forbidden.**
