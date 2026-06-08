# State Machine Table: Numbers

This document defines the authoritative lifecycle tables for Numbers.

It is normative.

All system behavior must conform to these tables.

Authority precedence is defined exclusively in `AUTHORITY-ORDER.md`.

This document defines:

- all valid auction lifecycle states
- all valid inscription lifecycle states
- all permitted transitions
- all forbidden transitions
- terminal states
- authority boundaries

Auction lifecycle and inscription lifecycle are separate layered machines.

They may operate concurrently but must not interfere with each other’s authority domains.

---

# I. Auction Lifecycle Machine

## Auction States

| State | Description | Terminal |
|---|---|---|
| `Scheduled` | Auction exists but has not yet received its first valid bid. | No |
| `Open` | Auction is actively accepting bids. | No |
| `Closed` | Auction is closed to bids. Resolution is eligible and pending unless `ResolutionRecord` already exists. | No |
| `AwaitingSettlement` | Resolution exists. Settlement outcome pending or being determined. | No |
| `Finalized` | Auction destination permanently fixed. | Yes |

## Notes

- `Finalized` includes successful and failed-settlement outcomes.
- No valid bid means the auction remains `Scheduled`; it is not a no-bid finalization path.
- `Closed` is the pre-resolution auction lifecycle state derived from `AuctionCloseRecord` when no `ResolutionRecord` exists.
- Auction terminal state is `Finalized`.
- System control states such as `Paused` are overlays and are not lifecycle states.
- Auction state is derived from canonical event records.
- Auction state must never be stored as mutable truth.

---

## Auction Allowed Transitions

| From | Trigger | To | Required Canonical Event Records | Notes |
|---|---|---|---|---|
| `Scheduled` | First valid bid accepted | `Open` | `BidRecord`, `AuctionOpenRecord` | Atomic transition. `BidRecord.validity = valid`. `AuctionOpenRecord.opened_at = server_time`. `AuctionOpenRecord.base_end_time = opened_at + auction.duration_seconds`. |
| `Open` | `server_time >= current_end_time` | `Closed` | `AuctionCloseRecord` | `current_end_time = base_end_time + (extension_increment_seconds * number_of_extension_events)`. Demo 1 does not emit `cap_reached`. |
| `Closed` | `ResolutionRecord` persisted | `AwaitingSettlement` | `ResolutionRecord` | Resolution must occur exactly once and must not be recomputed. |
| `AwaitingSettlement` | Settlement confirmed before deadline | `Finalized` | `SettlementRecord`, `FinalizationRecord` | Destination = winning destination. |
| `AwaitingSettlement` | Settlement deadline expired | `Finalized` | `SettlementRecord`, `FinalizationRecord` | Destination = `NullSteward`. |

No other auction transitions are permitted.

---

## Auction Forbidden Transitions

| Forbidden | Reason |
|---|---|
| `Open → Scheduled` | Time reversal. |
| `Closed → Open` | Bidding cannot reopen. |
| `AwaitingSettlement → Open` | Settlement does not reopen bidding. |
| `Finalized → Any` | Terminal auction state. |
| `Scheduled → Finalized` because no valid bid was accepted | No valid bid leaves the auction `Scheduled`. |
| `Open → Closed` because bid cap was reached in Demo 1 | Bid cap closure is reserved for a later implementation slice. |
| Any transition inferred from missing records | Guess-space forbidden. |

---

# II. Inscription Lifecycle Machine

Inscription lifecycle begins only after auction state = `Finalized`.

Inscription lifecycle does not alter auction state.

Broadcast attempt is an operation, not a canonical lifecycle state.

There is no canonical `Broadcasting` lifecycle state.

---

## Inscription States

| State | Description | Terminal |
|---|---|---|
| `NotStarted` | No committed inscription broadcast exists and no terminal inscription ambiguity exists. | No |
| `Inscribing` | A committed inscription broadcast exists and confirmation has not yet been observed. | No |
| `Ambiguous` | Inscription authority is frozen because the broadcast or inscription outcome cannot be determined. | Yes |
| `Inscribed` | Canonical inscription confirmation has been observed. | Yes |

## Notes

- Inscription lifecycle authority is independent from auction lifecycle authority.
- Inscription progress for auction `N` must not block auction availability for `N + 1` after finalization and rhythm-gap requirements are satisfied.
- `Ambiguous` and `Inscribed` are terminal inscription states.
- Authority is consumed at `broadcast_commit`, not at intent persistence.
- `InscriptionIntentRecord` does not change inscription lifecycle state.
- `InscriptionBroadcastRecord` determines whether inscription state remains `NotStarted`, becomes `Inscribing`, or becomes `Ambiguous`.

---

## Inscription Allowed Transitions

| From | Trigger | To | Required Canonical Event Records | Notes |
|---|---|---|---|---|
| `NotStarted` | Inscription intent persisted | `NotStarted` | `InscriptionIntentRecord` | Intent persistence does not consume inscription authority and does not prove broadcast. |
| `NotStarted` | Broadcast classified as `pre_commit_rejected` | `NotStarted` | `InscriptionBroadcastRecord` | Authority not consumed. `pre_commit_rejected` may be recorded only when the system can determine that `broadcast_commit` did not occur. This table does not create retry behavior. |
| `NotStarted` | `broadcast_commit` | `Inscribing` | `InscriptionBroadcastRecord` | `InscriptionBroadcastRecord.broadcast_outcome = committed`. Authority consumed. |
| `NotStarted` | Broadcast classified as `ambiguous` | `Ambiguous` | `InscriptionBroadcastRecord` or `AmbiguityRecord` | Authority frozen permanently. |
| `Inscribing` | Canonical inscription observed to required confirmation depth | `Inscribed` | `InscriptionConfirmationRecord` | Terminal inscription state. |
| `Inscribing` | Ambiguity detected | `Ambiguous` | `AmbiguityRecord` | Persist immediately. Terminal inscription state. |

No other inscription transitions are permitted.

---

## Inscription Forbidden Transitions

| Forbidden | Reason |
|---|---|
| `NotStarted → Inscribed` | Cannot skip committed broadcast and confirmation. |
| `Inscribing → NotStarted` | Authority cannot be restored after `broadcast_commit`. |
| `Ambiguous → Any` | Terminal ambiguity. |
| `Inscribed → Any` | Terminal inscription state. |
| Any inscription transition before auction state = `Finalized` | Authority violation. |
| Any transition through `Broadcasting` | `Broadcasting` is not a canonical lifecycle state. |
| Any transition inferred from missing records | Guess-space forbidden. |

---

# Authority Rules

- Auction resolution occurs exactly once per auction.
- Settlement does not create inscription authority.
- Inscription authority is consumed only at `broadcast_commit`.
- `broadcast_commit` occurs when:
  - a broadcast RPC succeeds
  - the authoritative node reports the transaction present in its mempool
- Inscription authority may be consumed at most once per auction.
- After `broadcast_commit`, no semantically distinct inscription attempt is permitted.
- Controlled fee replacement, if implemented, is permitted only under the equivalence rules defined in `inscription/INSCRIPTION-MACHINE.md`.
- Ambiguity permanently freezes inscription authority.
- Ambiguity must not interrupt the Numbers count.
- Ambiguity must not authorize a second semantically distinct inscription.
- `NullSteward` must not be used to repair inscription ambiguity after inscription authority is consumed or frozen.
- Authority is never restored by:
  - time passing
  - operator action
  - restart
  - retry of a semantically distinct action
  - observation

Authority semantics are defined exclusively in `core/AUTHORITY-CONSUMPTION.md`.

---

# Final Rule

If a transition is not listed in the applicable lifecycle table:

It is forbidden.