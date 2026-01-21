# State Machine — Canonical Table

This document assumes familiarity with:
- CORE-SEQUENCE.md
- STATE-MACHINE.md
- AUTHORITY-CONSUMPTION.md

This document defines the authoritative lifecycle table for Numbers.

It is **normative**.

All system behavior **must** conform to this table.

If there is a conflict,
PRD.md, CORE-SEQUENCE.md, STATE-MACHINE.md,
INVARIANTS.md, and AUTHORITY-CONSUMPTION.md take precedence.

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
| Closed | Auction has closed to bids and is resolving | No |
| AwaitingSettlement | Resolution recorded, settlement window open | No |
| Finalized | Auction destination fixed | Yes |
| Inscribing | Inscription initiation persisted, attempt in progress | No |
| Ambiguous | Inscription outcome cannot be determined with certainty | Yes |
| Inscribed | Canonical inscription observed | Yes |

Notes:
- `Finalized` includes no-bid and failed-settlement outcomes.
- `Ambiguous` is a terminal state and **must be persisted**.
- Terminal states are irreversible.
- Pause is an overlay and is not a lifecycle state.

---

## Allowed Transitions

| From State | Trigger | To State | Notes |
|-----------|--------|----------|------|
| Scheduled | start_time reached | Open | Automatic |
| Open | end_time reached or cap reached | Closed | Automatic |
| Closed | resolution written | AwaitingSettlement | Resolution persisted |
| Closed | resolution with no bids | Finalized | Destination = NullSteward |
| AwaitingSettlement | settlement confirmed | Finalized | Destination fixed |
| AwaitingSettlement | settlement deadline expired | Finalized | Destination = NullSteward |
| Finalized | inscription initiation persisted | Inscribing | Authority consumed |
| Inscribing | inscription observed | Inscribed | Terminal |
| Inscribing | ambiguity detected | Ambiguous | Terminal |

No other transitions are permitted.

---

## Ambiguity Rules (Normative)

Ambiguity **is a terminal lifecycle state**.

When the system enters `Ambiguous`:

- Inscription authority is permanently consumed
- No retries, rebroadcasts, or alternate actions are permitted
- Observation is the only allowed activity
- The ambiguity record **must** be persisted immediately

Ambiguity:
- does not advance the sequence
- does not restore authority
- does not resolve into success

---

## Forbidden Transitions

The following transitions are **never permitted**:

| Forbidden Transition | Reason |
|---------------------|--------|
| Open → Scheduled | Time reversal |
| Closed → Open | Bidding cannot reopen |
| AwaitingSettlement → Open | Settlement does not reopen bidding |
| Finalized → Any | Terminal auction state |
| Inscribed → Any | Terminal inscription state |
| Ambiguous → Any | Terminal ambiguity |
| Any → Inscribing without Finalized | Authority violation |
| Any transition inferred from missing data | Guess-space forbidden |

---

## Authority Rules (Normative)

- Auction authority is consumed by resolution
- Settlement does **not** create authority
- Inscription authority is consumed at inscription initiation
- Inscription authority is exercised **at most once**
- Ambiguity permanently freezes remaining authority
- Authority is never restored by:
  - time passing
  - operator action
  - retries
  - observation delay

Authority semantics are defined exclusively in
AUTHORITY-CONSUMPTION.md.

---

## Persistence Requirements

State **must** be durably persisted at:

- Entry to `Open`
- Entry to `Closed`
- Resolution record creation
- Entry to `AwaitingSettlement`
- Settlement confirmation or deadline expiry
- Entry to `Finalized`
- Inscription initiation (authority consumption boundary)
- Ambiguity detection
- Entry to any terminal state

Absence of a required record **must** halt execution.

---

## Final Rule

If a transition is not listed above:

**It is forbidden.**
