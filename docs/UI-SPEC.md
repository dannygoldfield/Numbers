# User Interface Specification

This document defines the user interface behavior for Numbers.

It constrains design decisions.
It does not describe visual style.

If there is a conflict between this document and any other specification,
`ARCHITECTURE.md` and `CORE-SEQUENCE.md` take precedence.

---

## 1. Purpose of the Interface

The Numbers interface exists to:

- Expose system state accurately
- Allow participation only where the system allows it
- Avoid interpretation, ranking, or narrative framing

The interface reflects outcomes.
It does not explain or justify them.

---

## 2. Page Model

- The site consists of **one continuous primary page**
- There are no routes or modes
- Navigation changes **scroll position**, not context

The page always initializes at **the current auction**

There is no deep-linking to past or future numbers.

---

## 3. Initialization and Refresh Behavior

- On page load, the interface shows the **current auction**
- On page refresh, the interface resets to the current auction
- Scroll position is not preserved
- Tapping the “Numbers” logo behaves identically to refresh

Numbers always reasserts “now.”

---

## 4. System States → UI States

The interface must represent all system states explicitly.

### States

- Open Auction
- Closing / Resolving
- Awaiting Settlement
- Finalized
- Inscribing
- Inscribed
- Degraded

The Inscribed state refers only to canonical inscriptions produced by the Numbers system.

No UI-only states may be introduced.

---

## 5. Inter-Auction Pause

Between auctions:

- The **next auction number** is visible but inactive
- Bid input, bid button, and keyboard are visible but inactive
- Keyboard reads `1234567890`
- Auction timer displays `12:34:56` in an inactive state
- Touching bidding elements has no effect
- Menu remains active

Liveness is communicated only through:
- Countdown
- Letline (system voice)

---

## 6. Letline (System Voice)

- The Letline is a single-line textual element
- It represents the system voice of “123456789 and 0”
- It is ambient and impersonal

Rules:
- Letline never provides individual user feedback
- Letline messaging may change over time
- Typing behavior (speed, pauses, reversal) is presentation-only

Example messages:
- “stand by for the next auction”
- “What if you could own a number?”

---

## 7. Wallet Connection

- Viewing the site never requires a wallet
- Wallet connection is required **only** to submit a bid
- Wallet connection is triggered only by attempting to place a bid

During inter-auction pauses:
- Wallet connection is unavailable

Wallet connections:
- Persist across auctions
- Persist until explicitly disconnected by the user

Disconnect:
- Available only via the menu
- No ceremony or emphasis

---

## 8. Bid Interaction

### Successful Bid
- No confirmation message
- No animation
- No toast
- Confirmation occurs only through reflected state

### Failed Bid
- Failure is indicated inline at the bid input
- Message is brief, neutral, and self-clearing
- Letline is not used for error messaging

---

## 9. Auction Transitions

- Auction start and end transitions are immediate
- No animation
- No acknowledgement layer
- No auto-scroll
- No notifications

State changes are presented as facts, not events.

---

## 10. History and Scrolling

- The current auction is always at the top
- Scrolling downward reveals prior auctions
- Prior auctions are non-interactive
- Prior auctions are visually quieter than the live auction

The home page is a vertical sequence:
- Present at the top
- Past below

---

## 11. Menu

The menu contains exactly five items:

1. Jump to live auction
2. Jump to recent results
3. About / What is Numbers
4. Docs / How it works
5. Connect / Disconnect wallet

The menu:
- Does not change context
- Does not introduce modes
- Is always accessible

---

## 12. About and Docs Sections

- About and Docs exist **within the same page**
- Menu items scroll to these sections
- Content expands inline
- No overlays, modals, or windows

Disclosure uses simple expansion.
Scroll continuity is preserved.

---

## 13. Metadata Display Policy

The interface displays only canonical inscriptions as defined by the system.

### Live Auction and Recent Results
- No addresses displayed
- No ownership implied

### Detailed History Views
- Addresses may be displayed in truncated form
- Addresses are presented as data, not identity

---

## 14. Destination Semantics

Every auction resolves to a destination.

- Successful auction:
  - Destination = winner’s address

- No-bid or failed-settlement auction:
  - Destination = Null Steward

Null Steward:
- Is a system-controlled destination
- Uses a new address per occurrence
- Is intentionally unspendable
- Is not controlled by any participant

Bid amount for no-bid outcomes is recorded as `0`.

---

## 15. Transaction References

- Transaction ID and satpoint are treated as **separate affordances**
- They are grouped visually but not merged conceptually
- Each links to its canonical external viewer
- They are never embedded or interpreted by Numbers

Transaction references are shown only for canonical system outcomes.

---

## 16. Non-Goals

The interface does not provide:

- User accounts
- Personalization
- Notifications
- Market data
- Rankings
- Rarity signals
- Social features
- Analytics dashboards

Any feature not explicitly described here is out of scope.

---

## 17. Guiding Principle

The interface must never imply meaning beyond recorded outcomes.

Numbers records.
Viewers interpret.
