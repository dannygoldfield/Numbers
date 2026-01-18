# State Machine — Canonical Table

This document assumes familiarity with CORE-SEQUENCE.md.

This document defines the authoritative state machine for Numbers.

It is **normative**.

All system behavior must conform to this table.
If there is a conflict,
PRD.md, CORE-SEQUENCE.md, INVARIANTS.md,
and AUTHORITY-CONSUMPTION.md take precedence.

This table defines:
- All valid lifecycle states
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
| AwaitingSettlement | Winning bid recorded, settlement window open | No |
| Finalized | Auction outcome fixed with no further authority | Yes |
| Inscribing | Inscription attempt in progress | No |
| Inscribed | Canonical inscription observed | Yes |
| Paused | System pause overlay, not a lifecycle state | N/A |

Notes:
- `Finalized` includes both clean no-bid outcomes and failed settlement outcomes.
- `Paused` is an overlay and does not represent lifecycle progression.

---

## Allowed Transitions

| From State | Trigger | To State | Notes |
|-----------|--------|----------|------|
| Scheduled | start_time reached | Open | Automatic |
| Open | end_time reached | Closed | Automatic |
| Open | operator pause | Paused | Overlay only |
| Paused | operator resume | Open | No inference |
| Closed | resolution with winning bid | AwaitingSettlement | Settlement authority is exercised |
| Closed | resolution with no bids | Finalized | No settlement, no inscription |
| AwaitingSettlement | settlement deadline reached (unpaid) | Finalized | Destination = NullSteward |
| AwaitingSettlement | settlement confirmed | Inscribing | Observational, authority already consumed |
| Inscribing | inscription confirmed | Inscribed | Terminal |

---

## Ambiguity Rule (Non-State)

Ambiguity is **not a lifecycle state**.

If ambiguity is detected during **Inscribing**:

- The system **remains in Inscribing**
- All remaining execution authority is permanently frozen
- No retries, rebroadcasts, or alternate actions are permitted
- Observation is the only allowed activity

Ambiguity constrains authority.
It does not advance, rewind, or terminate the lifecycle.

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
| Any → Inscribing without settlement resolution | Authority violation |
| Paused → Any non-previous state | Pause does not advance lifecycle |
| Any transition caused by ambiguity | Ambiguity does not advance lifecycle |

---

## Authority Rules

- Authority is **first consumed** when entering `AwaitingSettlement`
- Authority is **never created** by payment confirmation
- Inscription consumes remaining authority exactly once
- Ambiguity permanently reduces remaining authority
- Authority is never restored by:
  - time passing
  - operator action
  - retries
  - observation delay

---

## Persistence Requirements

State **must** be durably persisted at:

- Entry to `Open`
- Entry to `Closed`
- Entry to `AwaitingSettlement`
- Settlement confirmation or settlement deadline expiry
- Before any inscription attempt
- Upon detection of ambiguity
- Upon reaching any terminal state

---

## Final Rule

If a transition is not listed as allowed:

**It is forbidden.**
