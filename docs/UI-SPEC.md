# User Interface Specification — Numbers

This document defines user interface behavior for Numbers.

It is **normative** where it constrains interaction, visibility,
and representation of system state.

It does not define visual style, branding, or aesthetics.

If there is a conflict,
PRD.md, CORE-SEQUENCE.md, STATE-MACHINE.md,
and API-STATE-SHAPES.md take precedence.

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

- Expose canonical system state accurately
- Allow user interaction **only** where the backend permits it
- Prevent narrative framing, interpretation, or implication

The interface reflects recorded knowledge.  
It does not explain, justify, predict, or reassure.

The interface is not authoritative.

---

## 2. Page Model (Normative)

The interface **must consist of exactly one primary page**.

Rules:

- There are no routes, modes, or application states
- Navigation alters **scroll position only**
- No navigation action may alter system interaction eligibility

Content sections (About, Docs) are informational only.
They **must not** introduce modes or conditional behavior.

The page **must always initialize** at the current auction.

Deep-linking to past or future auctions is forbidden.

---

## 3. Initialization and Refresh Behavior (Normative)

On page load or refresh:

- The interface **must** display the current auction
- Scroll position **must** reset to the current auction
- No UI state **may** persist across refresh

Clicking the “Numbers” logo **must** behave identically to refresh.

The interface always reasserts *now*.

---

## 4. System State Representation (Normative)

The interface **must represent backend system state explicitly**.

### Representable States

- `Open`
- `Closing`
- `AwaitingSettlement`
- `Finalized`
- `Inscribing`
- `Inscribed`
- `Degraded`

Rules:

- These states **must map directly** to backend state
- No UI-only states **may** be introduced
- UI state **must not** anticipate transitions

The `Inscribed` state refers **only** to inscriptions
recognized by the Numbers system.

---

## 5. Inter-Auction Pause (Normative)

Between auctions:

- The next auction number **must** be visible and inactive
- Bid input and controls **must** be visible but inert
- Keyboard **must** display `1234567890` in inactive form
- Timer **must** display `12:34:56` as a presentation placeholder

Interaction rules:

- Touching inactive elements **must have no effect**
- No wallet connection **may** be initiated
- Menu remains accessible

Displayed timing values during pause are **non-semantic**.
They do not define protocol behavior.

Liveness communication is limited to:

- Countdown
- Letline

---

## 6. Letline (System Voice) (Normative)

The Letline is a single, ambient textual element.

Rules:

- It represents the system voice of “123456789 and 0”
- It **must not** respond to individual user actions
- It **must not** convey errors, confirmations, or diagnostics
- Message content **may** change without semantic implication

Typing behavior is presentation-only and non-authoritative.

---

## 7. Wallet Connection (Normative)

Rules:

- Viewing the site **must not** require a wallet
- Wallet connection **must** be requested only when attempting to bid
- Wallet connection **must not** occur during inter-auction pause

Connection behavior:

- Connection persists across auctions
- Connection persists until explicit user disconnect

Disconnect:

- Available only via menu
- No emphasis, confirmation, or narrative framing

Wallet connection does not imply bid acceptance.

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

---

## 9. Auction Transitions (Normative)

Auction transitions:

- **must** be immediate
- **must not** animate
- **must not** acknowledge the user
- **must not** scroll automatically
- **must not** notify

State changes are presented as facts, not events.

---

## 10. History and Scrolling (Normative)

Layout rules:

- The current auction **must** appear at the top
- Past auctions **must** appear below in sequence order
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

### Historical Views

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
- A new address **must** be used per occurrence
- No participant controls the destination

Bid amount for no-bid outcomes **must** be recorded as `0`.

---

## 15. Transaction References (Normative)

Rules:

- `txid` and `satpoint` are distinct affordances
- They **must not** be merged conceptually
- Each links to its canonical external viewer
- They **must not** be embedded or interpreted

Transaction references are shown **only** for canonical outcomes.

---

## 16. Non-Goals (Normative)

The interface **does not** provide:

- User accounts
- Personalization
- Notifications
- Rankings
- Market data
- Rarity indicators
- Social features
- Analytics

Any feature not specified here is forbidden.

---

## 17. Final Principle

The interface **must never** imply meaning
beyond recorded outcomes.

Numbers records.  
The interface reflects.  
Interpretation happens elsewhere.
