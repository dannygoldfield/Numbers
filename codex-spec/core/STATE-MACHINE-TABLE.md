# State Machine — Canonical Table

This document defines the authoritative lifecycle tables for Numbers.

It is normative.

All system behavior must conform to these tables.

Authority precedence is defined exclusively in AUTHORITY-ORDER.md.

This document defines:

- All valid auction lifecycle states
- All valid inscription lifecycle states
- All permitted transitions
- All forbidden transitions
- Terminal states
- Authority boundaries

Auction lifecycle and inscription lifecycle are separate layered machines.
They may operate concurrently but must not interfere with each other’s authority domains.

---

# I. Auction Lifecycle Machine

## Auction States

| State | Description | Terminal |
|-------|------------|----------|
| Scheduled | Auction exists but has not yet received its first valid bid. | No |
| Open | Auction is actively accepting bids. | No |
| Closed | Auction is closed to bids. Resolution must exist. | No |
| AwaitingSettlement | Resolution exists. Settlement outcome pending or being determined. | No |
| Finalized | Auction destination permanently fixed. | Yes |

Notes:

- `Finalized` includes successful, no-bid, and failed-settlement outcomes.
- Auction terminal state is `Finalized`.
- System control states such as `Paused` are overlays and are not lifecycle states.

---

## Auction Allowed Transitions

| From | Trigger | To | Notes |
|------|--------|----|-------|
| Scheduled | First valid bid accepted | Open | Atomic: persist BidRecord; persist AuctionOpenRecord with `opened_at = server_time` and `base_end_time = opened_at + auction.duration_seconds`. |
| Open | `server_time >= current_end_time` or bid cap reached | Closed | Persist AuctionCloseRecord. `current_end_time = base_end_time + (extension_increment_seconds * number_of_extension_events)`. |
| Closed | ResolutionRecord persisted | AwaitingSettlement | Presence of ResolutionRecord defines entry into AwaitingSettlement. Resolution must occur exactly once and must not be recomputed. |
| AwaitingSettlement | Settlement confirmed before deadline | Finalized | Persist SettlementRecord and FinalizationRecord. Destination = winning address. |
| AwaitingSettlement | Settlement deadline expired | Finalized | Persist SettlementRecord and FinalizationRecord. Destination = NullSteward. |
| AwaitingSettlement | ResolutionRecord indicates no valid bids | Finalized | Persist SettlementRecord (status = not_required) and FinalizationRecord. Destination = NullSteward. |

No other auction transitions are permitted.

Auction state is derived from persisted records.
Auction state must never be stored as mutable truth.

---

## Auction Forbidden Transitions

| Forbidden | Reason |
|-----------|--------|
| Open → Scheduled | Time reversal |
| Closed → Open | Bidding cannot reopen |
| AwaitingSettlement → Open | Settlement does not reopen bidding |
| Finalized → Any | Terminal auction state |
| Any transition inferred from missing records | Guess-space forbidden |

---

# II. Inscription Lifecycle Machine

Inscription lifecycle begins only after Auction state = Finalized.

Inscription lifecycle does not alter auction state.

## Inscription States

| State | Description | Terminal |
|-------|------------|----------|
| NotStarted | No inscription attempt has been initiated. | No |
| Inscribing | Inscription initiation persisted. Attempt in progress. | No |
| Ambiguous | Outcome cannot be determined with certainty. | Yes |
| Inscribed | Canonical inscription observed. | Yes |

Notes:

- Inscription lifecycle authority is independent from auction lifecycle authority.
- `Ambiguous` and `Inscribed` are terminal inscription states.

---

## Inscription Allowed Transitions

| From | Trigger | To | Notes |
|------|--------|----|-------|
| NotStarted | Inscription initiation persisted | Inscribing | Authority boundary. Persist InscriptionRecord. Inscription authority consumed. |
| Inscribing | Canonical inscription observed | Inscribed | Persist confirmation data. Terminal. |
| Inscribing | Ambiguity detected | Ambiguous | Persist AmbiguityRecord immediately. Terminal. |

No other inscription transitions are permitted.

---

## Inscription Forbidden Transitions

| Forbidden | Reason |
|-----------|--------|
| NotStarted → Inscribed | Cannot skip initiation |
| Inscribing → NotStarted | Authority cannot be restored |
| Ambiguous → Any | Terminal ambiguity |
| Inscribed → Any | Terminal inscription state |
| Any inscription transition before Auction state = Finalized | Authority violation |

---

# Authority Rules (Normative)

- Auction resolution occurs exactly once per auction.
- Settlement does not create inscription authority.
- Inscription authority is consumed at transition `NotStarted → Inscribing`.
- Inscription authority may be exercised at most once per auction.
- Ambiguity permanently freezes inscription authority.
- Authority is never restored by:
  - time passing,
  - operator action,
  - retries,
  - observation.

Authority semantics are defined exclusively in AUTHORITY-CONSUMPTION.md.

---

# Final Rule

If a transition is not listed in the applicable lifecycle table:

It is forbidden.