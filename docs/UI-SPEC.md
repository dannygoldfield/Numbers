# User Interface Specification — Numbers

This document defines user interface behavior for Numbers.

It is **normative only where it constrains interaction, visibility,
and representation of recorded system state**.

It does not define visual style, branding, tone, or aesthetics.

If there is a conflict,
PRD.md, CORE-SEQUENCE.md, STATE-MACHINE.md,
API-SPEC.md, and API-STATE-SHAPES.md take precedence.

---

## Modal Language Rule (Normative)

In this document and all normative specifications:

- **must / must not** define obligations
- **only / exactly once / at most once** define bounds
- **may** is permitted **only** to describe uncertainty of knowledge,
  never permission, intent, or policy choice

The following terms are forbidden in normative contexts:

- possibly
- likely
- eventually
- for now
- TBD

Any normative statement using forbidden modal language is invalid.

---

## 1. Purpose of the Interface (Normative)

The Numbers interface exists to:

- reflect canonical backend state accurately
- permit user interaction **only** where explicitly allowed by the backend
- avoid narrative framing, implication, or interpretation

The interface reflects recorded knowledge.
It does not explain, justify, predict, or reassure.

The interface is **non-authoritative**.

---

## 2. Page Model (Normative)

The interface **must consist of exactly one primary page**.

Rules:

- There are no routes, modes, or application states
- Navigation alters **scroll position only**
- No navigation action may alter interaction eligibility

Content sections (About, Docs) are informational only.
They **must not** introduce modes or conditional behavior.

The page **must initialize** at the current auction.

Deep-linking to past or future auctions is forbidden.

---

## 3. Initialization and Refresh Behavior (Normative)

On page load or refresh:

- The interface **must** display the current auction
- Scroll position **must** reset to the current auction
- No UI state **may** persist across refresh

Clicking the “Numbers” logo **must** behave identically to refresh.

The interface always reasserts the present.

---

## 4. System State Representation (Normative)

The interface **must represent backend system state exactly**.

### Representable States

UI state labels must map exactly to backend states:

- `Scheduled`
- `Open`
- `Closed`
- `AwaitingSettlement`
- `Finalized`
- `Inscribing`
- `Inscribed`
- `Ambiguous`

Rules:

- UI states must not introduce new states
- UI states must not alias backend states
- UI states must not anticipate transitions
- UI states must not combine multiple backend states

---

## 6. Letline (System Voice) (Normative)

The Letline is a single ambient textual element.

Rules:

- It represents the system voice of “123456789 and 0”
- It **must not** respond to individual user actions
- It **must not** convey errors, confirmations, or diagnostics
- Message content **may** change without semantic implication

Typing or animation behavior is presentation-only and non-authoritative.

---

## 7. Wallet Connection (Normative)

Rules:

- Viewing the site **must not** require a wallet
- Wallet connection **must** be requested only when attempting to bid

Connection behavior:

- Wallet connection state may persist across auctions
  until explicitly disconnected by the user.

Disconnect:

- Available only via menu
- No emphasis, confirmation, or narrative framing

Wallet connection does not imply bid acceptance
or bid validity.

---

## 8. Bid Interaction (Normative)

Bid interaction represents an **attempt**, not an outcome.

### Successful Submission

- No confirmation message
- No animation
- No toast or banner
- Confirmation occurs only through reflected auction state

### Rejected Submission

- Error displayed inline at bid input
- Message is neutral, factual, and self-clearing
- Letline **must not** be used for errors

Bid UI **must not** imply:
- likelihood of winning
- priority
- acceptance beyond protocol rules

The interface may reveal the existence of competing bids through recorded outcomes,
but must not frame competition as urgency, encouragement, achievement, or loss.

---

## 9. Auction Transitions (Normative)

Auction transitions:

- **must** be immediate upon state change
- **must not** animate
- **must not** acknowledge the user
- **must not** scroll automatically
- **must not** notify

State changes are presented as facts, not events.

---

## 10. History and Scrolling (Normative)

Layout rules:

- The current auction **must** appear at the top
- Past auctions **must** appear below in strict sequence order
- Past auctions **must** be non-interactive
- Past auctions **must** be visually quieter

The page represents time vertically:

- Present at the top
- Past below

---

## 11. Menu (Normative)

The menu **must contain exactly five items**:

1. Jump to live auction
2. Jump to recent results
3. About / What is Numbers
4. Docs / How it works
5. Connect / Disconnect wallet

Menu behavior:

- Must not change application context
- Must not introduce modes
- Must not affect system state

Menu interaction is navigational only.

---

## 12. About and Docs Sections (Normative)

Rules:

- Content exists within the same page
- Menu scrolls to content position
- Disclosure expands inline
- No overlays, modals, or new windows

Scroll continuity **must** be preserved.

---

## 13. Metadata Display Policy (Normative)

The interface **must display only canonical system data**.

### Live Auction and Recent Results

- Addresses **must not** be displayed
- Ownership **must not** be implied

### Historical Auctions

- Addresses **may** be displayed in truncated form
- Addresses **must** be presented as data, not identity

---

## 14. Destination Semantics (Normative)

Every auction resolves to exactly one destination.

- Successful auction:
  - Destination = winning address

- No-bid or failed settlement:
  - Destination = `NullSteward`

Rules:

- `NullSteward` is unspendable
- No participant controls the destination

---

## 15. Transaction References (Normative)

Rules:

- `txid` and `satpoint` are distinct references
- They **must not** be merged conceptually
- Each links to its canonical external viewer
- They **must not** be embedded or interpreted

Transaction references are shown **only** for canonical outcomes.

---

## 16. Non-Goals (Normative)

The interface **does not** provide:

- user accounts
- personalization
- notifications
- rankings
- market data
- rarity indicators
- social features
- analytics

Any feature not specified here is forbidden.

---

## 17. Final Principle

The interface **must never** imply meaning
beyond recorded outcomes.

Numbers records.
The interface reflects.
Interpretation happens elsewhere.
