# Website Product Requirements Document

This document defines normative constraints for the Numbers website.

## 1. Purpose

The Numbers website exists to expose the live auction sequence and allow participation.

It does not define auction semantics or interpret ownership. 
It reflects canonical system state accurately and in real time.

The website is an interface layer only.  
It does not define auction semantics or ownership rules.

---

## 2. Inputs and Authority

The Numbers website consumes the system as defined by:

- PRD.md (invariants)
- CORE-SEQUENCE.md
- ARCHITECTURE.md

If there is a conflict, the system definition always wins.

The website must adapt to the system, never the reverse.

---

## 3. User Modes

The website supports different modes of interaction, not personas.

- **Observer**
  - Views the live count
  - Watches auctions progress
  - Browses past numbers

- **Active Bidder**
  - Views the current auction
  - Places bids during the open window
  - Monitors auction outcome

- **Past Winner**
  - Views finalized results
  - Confirms inscription details

- **Archivist**
  - Browses historical auctions
  - Inspects outcomes and inscriptions

No mode is privileged visually.

---

## 4. Core User Tasks

The website must allow users to:

1. See the current auction number
2. See time remaining in the auction
3. See the current high bid, if any
4. Place a bid during the open window
5. Observe when an auction closes
6. Observe settlement and finalization state
7. View inscription results once available
8. Browse previously completed numbers

No task should require interpretation.

---

## 5. System-Derived UI States

The UI must explicitly support the following states:

- Open auction
- Closing / resolving
- Awaiting settlement
- Finalized (destination known)
- Inscribing
- Inscribed (canonical txid and satpoint visible)
- Degraded (partial or delayed data)

All states must be visible and legible.  
No state should be hidden to simplify the interface.

---

## 6. UX Constraints

The website must:

- Never imply ownership before finalization
- Never rank or highlight numbers by perceived importance
- Never decorate or stylize the number itself
- Never suppress failure or no-bid outcomes
- Never reframe delays as errors unless they are errors

The website must not:

- Assign meaning to specific numbers
- Add narrative or explanatory overlays by default
- Suggest rarity, value, or symbolism through design

## 7. Tone, Rhythm, and Non-Semantic Motion

The website may express time and system rhythm, but not meaning.

Motion, transitions, and temporal cues may be used to signal:
- The passage of time
- The closing of an auction
- A change in system state

These expressions must be:
- Outcome-agnostic
- Identical across all resolution outcomes
- Brief and non-accumulative
- Non-repeatable outside their triggering state transition

The website must not use motion, color, sound, or effects to:
- Celebrate outcomes
- Signal success or failure
- Create reward loops
- Assign emotional value to specific numbers

Temporal cadence (e.g. fixed auction duration) may carry character.
State transitions may be perceptible.
Meaning must remain untouched by presentation.


---

## 8. Design Freedom

The following are explicitly allowed:

- Typography choices outside the number itself
- Motion and transitions that reflect time or state changes
- Sound (optional, non-semantic)
- Density of information
- Collapsible or progressive disclosure
- Different layouts for live vs historical views

Design choices must not alter meaning.

---

## 9. Error and Degraded States

The website must handle degraded conditions visibly.

Examples:
- RPC unavailable
- Delayed confirmations
- Settlement pending beyond expectation

Degraded does not mean broken.  
The UI should communicate uncertainty without drama.

---

## 10. Non-Goals

The website does not:

- Educate users about Bitcoin or Ordinals
- Explain why numbers matter
- Encourage specific bidding behavior
- Provide social features or commentary
- Optimize for engagement metrics

Those concerns live elsewhere, if at all.

---

## 11. Success Criteria

The website is successful if:

- System state is accurately reflected
- Transitions between states are clear
- No UI element contradicts system invariants
- Users can observe the sequence without confusion

Aesthetic success is secondary to semantic correctness.
