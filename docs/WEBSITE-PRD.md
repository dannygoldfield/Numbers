# Website Product Requirements Document

This document defines normative constraints for the Numbers website.

## 1. Purpose

The Numbers website exists to expose the live auction sequence and allow participation.

It reflects canonical system state accurately, as reported by the system.
It does not define auction semantics or interpret ownership.

The website is an interface layer only.
It adapts to the system; it does not shape it.

---

## 2. Inputs and Authority

The Numbers website consumes the system as defined by:

- PRD.md (invariants)
- CORE-SEQUENCE.md
- ARCHITECTURE.md

If there is a conflict, PRD.md takes precedence.

The website must adapt to the system, never the reverse.

---

## 3. User Modes

The website supports interaction modes, not personas.

- **Observer** — views the live sequence and past outcomes
- **Active Bidder** — participates during open auctions
- **Past Winner** — verifies finalized inscription details
- **Archivist** — inspects historical auctions and inscriptions

No mode is visually privileged.

---

## 4. Core User Tasks

The website must allow users to:

1. See the current auction number
2. See time remaining in the auction
3. See the current high bid, if any
4. Place a bid during the open window
5. Observe auction closure
6. Observe settlement and finalization state
7. View inscription results once available
8. Browse completed numbers

No task may require interpretation.

---

## 5. System-Derived UI States

The UI must expose the following states:

- Open auction
- Closing / resolving
- Awaiting settlement
- Finalized (destination known)
- Inscribing
- Inscribed (canonical txid and satpoint visible)
- Degraded (partial or delayed data)

All states must be visible.
No state may be hidden to simplify presentation.

---

## 6. UX Constraints

The website must:

- Avoid implying ownership before finalization
- Avoid ranking or highlighting numbers
- Avoid decorating or stylizing the number itself
- Display no-bid and nonpayment outcomes plainly
- Distinguish delay from error

The website must not:

- Assign meaning to specific numbers
- Suggest rarity, value, or symbolism through design
- Add narrative or explanatory overlays by default

---

## 7. Tone, Rhythm, and Non-Semantic Motion

The website may express time and system rhythm, but not meaning.

Motion and transitions may indicate:
- Passage of time
- Auction closure
- State change

These cues must be:
- Outcome-agnostic
- Uniform across all resolutions
- Brief and non-accumulative
- Triggered only by state transitions

The website must not use motion, color, sound, or effects to:
- Celebrate outcomes
- Signal success or failure
- Create reward loops
- Assign emotional value

Temporal cadence may carry character.
Meaning must remain untouched by presentation.

---

## 8. Design Freedom

The following are permitted:

- Typography outside the number itself
- Non-semantic motion tied to time or state
- Optional sound without semantic meaning
- Information density choices
- Progressive disclosure
- Distinct live vs historical layouts

Design freedom ends where meaning begins.

---

## 9. Error and Degraded States

Degraded conditions must be visible.

Examples:
- RPC unavailable
- Delayed confirmations
- Settlement pending beyond expectation

Degraded does not mean broken.
Uncertainty must be communicated plainly.

---

## 10. Non-Goals

The website does not:

- Educate users about Bitcoin or Ordinals
- Explain why numbers matter
- Encourage bidding behavior
- Provide social features
- Optimize engagement

---

## 11. Success Criteria

The website succeeds if:

- System state is accurately reflected
- State transitions are legible
- No UI element contradicts system invariants
- The sequence can be observed without confusion

Aesthetic success is secondary to semantic correctness.
