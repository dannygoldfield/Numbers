# Core Sequence: Numbers

This document defines the deterministic execution sequence of Numbers.

It is normative.

Authority precedence is defined exclusively in `AUTHORITY-ORDER.md`.

This document describes what must occur, in what order, and without exception.

This document does not define:

- UI behavior
- authority semantics
- persistence mechanics
- error taxonomy
- wallet internals
- Bitcoin confirmation mechanics

Those are defined elsewhere.

If behavior is not explicitly defined here, it is forbidden.

---

# 1. Global Sequence Invariants

Numbers advances through a monotonically increasing sequence of numbers.

For each number `N`, the system executes the auction lifecycle exactly once and strictly in order.

The sequence:

- never rewinds
- never overlaps two active auctions
- never auto-opens an auction based solely on time
- never persists `AuctionRecord` for `N + 1` until `FinalizationRecord` exists for `N` and `auction.inter_auction_gap_seconds` has elapsed

If an auction remains in `Scheduled`, the sequence remains at `N`.

The `auction.inter_auction_gap_seconds` interval is a rhythm gap only.

It is not:

- a recovery window
- a settlement window
- an inscription window
- an automatic auction-start trigger

Persisting `AuctionRecord` for `N + 1` makes auction `N + 1` `Scheduled`.

Auction `N + 1` opens only when the first valid bid for `N + 1` is accepted.

When `FinalizationRecord` exists for `N`, `auction.inter_auction_gap_seconds` has elapsed, and no active auction exists, the backend must persist `AuctionRecord` for `N + 1` at the next state-evaluation boundary.

State-evaluation boundaries for Demo 1 are:

- backend startup after restart reconstruction completes
- `GET /state`
- `POST /bid`
- `GET /auction/history`

This deterministic creation of `AuctionRecord` for `N + 1` does not open auction `N + 1`.

No background scheduler is required for Demo 1.

Canonical event records must be appended through a single serialized commit path that guarantees total ordering.

All lifecycle state must be derived from canonical event records.

Lifecycle state must never be stored as mutable truth.

---

# 2. Canonical Event Boundary

External observations have no lifecycle authority until converted into a permitted canonical event record.

External inputs include:

- client bid submissions
- Bitcoin node observations
- wallet observations
- mempool observations
- block observations
- operator observations

When a condition defined by this specification is satisfied, the system appends the corresponding canonical event record to persistent storage.

Persisted canonical event records form the authoritative event log.

External network state, mempool observations, client submissions, or block ordering have no authority once a canonical event record has been appended.

All lifecycle transitions and deterministic evaluation must be derived exclusively from the ordered sequence of canonical event records.

If a required canonical event record is absent, the corresponding event must be treated as not having occurred.

---

# 3. Auction Initialization

The first auction number must be `auction.starting_number`.

For each number `N`:

- exactly one `AuctionRecord` must exist
- the auction begins in `Scheduled`
- the auction does not open automatically
- the auction opens only when the first valid bid is accepted
- no bid may be accepted as valid unless system control state = `Running`

For `N > 1`, `AuctionRecord` for `N` must not be persisted until:

1. `FinalizationRecord` exists for `N - 1`
2. `auction.inter_auction_gap_seconds` has elapsed after `FinalizationRecord.finalization_time`

If no valid bid is ever accepted, the auction remains `Scheduled`.

No countdown exists while auction state is `Scheduled`.

No auction close, resolution, settlement, finalization, inscription intent, or NullSteward outcome is produced solely because no valid bid has been accepted.

---

# 4. Auction Opening

When the first valid bid is accepted while auction state = `Scheduled`:

The following must occur atomically:

1. persist `BidRecord` with `validity = valid`
2. persist `AuctionOpenRecord`
3. set `AuctionOpenRecord.opened_at = server_time`
4. set `AuctionOpenRecord.base_end_time = opened_at + auction.duration_seconds`

The auction state is then derived as `Open`.

No separate transition record exists.

No intermediate lifecycle state is permitted.

`server_time` is authoritative.

Client timestamps must not influence lifecycle timing.

If the bid is invalid:

- persist `BidRecord` with `validity = invalid`
- auction state remains `Scheduled`

---

# 5. Open Auction Timing

While auction state = `Open`:

Configuration parameters are defined by `config/CONFIG-REFERENCE.md`.

Timing values fixed for the auction must be captured at the applicable persistence boundary.

Definitions:

- `base_end_time` is persisted in `AuctionOpenRecord`
- `number_of_extension_events` equals the count of `ExtensionEventRecord` entries for the auction
- `current_end_time` is derived, not stored as mutable truth

`current_end_time` is derived as:

```text
current_end_time =
base_end_time +
(extension_increment_seconds * number_of_extension_events)
```

All time evaluation must use authoritative `server_time`.

---

# 6. Extension Rule

If a valid bid is accepted while auction state = `Open` and:

- `server_time >= current_end_time - auction.extension_window_seconds`
- `number_of_extension_events < auction.max_extensions`

then the system must persist exactly one `ExtensionEventRecord`.

No prior record may be modified.

If `number_of_extension_events == auction.max_extensions`:

- no further extensions are permitted

Extension records:

- do not create a new lifecycle state
- do not consume authority
- do not modify prior records
- do not modify `base_end_time`

---

# 7. Auction Closing

The auction transitions from `Open` to `Closed` when:

- `server_time >= current_end_time`

At closing:

- persist exactly one `AuctionCloseRecord`
- close timestamp must be persisted as `closed_at`
- no further valid bids are permitted

Auction close is deterministic once the close condition is satisfied.

For Demo 1, bid cap closure is excluded.

`cap_reached` is reserved for a later implementation slice and must not be emitted in Demo 1.

---

# 8. Canonical Ordering Rule

The ordered canonical event log defines the canonical sequence of actions.

If a valid `BidRecord` appears in the canonical event log before `AuctionCloseRecord`, the bid may participate in resolution.

If a `BidRecord` appears after `AuctionCloseRecord`, the bid must be invalid.

External observation order, mempool propagation order, block inclusion order, network timing, or client timing must never influence bid validity once canonical event records are persisted.

Deterministic replay of the canonical event log must always produce the same resolution outcome.

---

# 9. Resolution

In auction state `Closed`:

- the winning bid must be determined deterministically
- exactly one `ResolutionRecord` must be persisted
- resolution must use only persisted valid `BidRecord` entries
- invalid `BidRecord` entries must not participate in resolution
- resolution must not be recomputed after `ResolutionRecord` exists

After `ResolutionRecord` exists:

- auction state is derived as `AwaitingSettlement`

Under the current first-valid-bid opening rule, a normally opened auction has at least one valid bid.

A no-valid-bid condition does not close, resolve, settle, finalize, or route to `NullSteward`.

If no valid bid is accepted, the auction remains `Scheduled`.

---

# 10. Settlement

While auction state = `AwaitingSettlement`:

Exactly one terminal settlement outcome must be recorded.

Permitted settlement outcomes are:

- `settled`
- `expired`

`not_required` is not produced by the current first-valid-bid opening model.

A future implementation slice that creates a no-winner settlement path must specify that path explicitly before `not_required` is emitted.

Settlement outcome must be persisted exactly once in `SettlementRecord`.

Settlement outcome must not be recomputed after `SettlementRecord` exists.

Settlement observation, if enabled, is governed by `bidding/SETTLEMENT.md` and `chain/CHAIN-INTERACTION.md`.

For Demo 1, live chain settlement observation is not required.

If settlement succeeds:

- `SettlementRecord.status = settled`
- final destination is winning destination address

If settlement deadline expires:

- `SettlementRecord.status = expired`
- final destination is `NullSteward`

If no valid bid has been accepted:

- auction state remains `Scheduled`
- no countdown starts
- no `AuctionCloseRecord` is persisted
- no `ResolutionRecord` is persisted
- no `SettlementRecord` is persisted
- no `FinalizationRecord` is persisted
- no inscription process begins
- no `NullSteward` outcome is produced

Settlement does not create inscription authority.

Settlement does not consume inscription authority.

---

# 11. Finalization

After terminal settlement outcome is determined:

- persist exactly one `FinalizationRecord`
- destination is fixed
- auction lifecycle is complete

Destination must be one of:

- winning destination address
- `NullSteward`

`NullSteward` is a protocol-visible final destination.

It is not:

- a universal recovery mechanism
- an error state
- a retry mechanism
- a way to repair inscription ambiguity after inscription authority is consumed or frozen

Finalization is irreversible.

No auction lifecycle action is permitted after `FinalizationRecord`.

`AuctionRecord` for `N + 1` must not be persisted until `FinalizationRecord` exists for number `N` and `auction.inter_auction_gap_seconds` has elapsed.

Sequence advancement does not depend on live inscription broadcast, live inscription confirmation, inscription success, or inscription ambiguity.

Inscription progress for auction `N` must not block auction availability for `N + 1` after the finalization and rhythm-gap requirements are satisfied.

---

# 12. Inscription Sequence

Inscription lifecycle begins only after auction state = `Finalized`.

Inscription lifecycle does not alter auction lifecycle state.

Auction correctness must not depend on inscription progress, broadcast, confirmation, or failure.

## Demo 1

For Demo 1:

- live inscription broadcast is not required
- exactly one deferred `InscriptionIntentRecord` must be persisted for each finalized auction
- `InscriptionIntentRecord.adapter_mode` must be `deferred_in_this_slice`
- `InscriptionIntentRecord` must be persisted through the same serialized canonical commit path as finalization or immediately after `FinalizationRecord` before any later auction availability evaluation
- no `InscriptionBroadcastRecord` is required
- no `InscriptionConfirmationRecord` is required
- no live inscription success may be simulated
- no confirmation may be simulated

`InscriptionIntentRecord` does not consume inscription authority.

`InscriptionIntentRecord` does not move inscription state out of `NotStarted`.

## Demo 2

For Demo 2, if live testnet inscription behavior is explicitly included in the active implementation slice:

- inscription broadcast may be attempted only after `InscriptionIntentRecord`
- broadcast outcome must be classified in `InscriptionBroadcastRecord`
- broadcast outcome must be one of:
  - `committed`
  - `pre_commit_rejected`
  - `ambiguous`

If broadcast outcome is `committed`:

- `broadcast_commit` occurred
- inscription authority is consumed
- inscription state becomes `Inscribing`

If broadcast outcome is `pre_commit_rejected`:

- inscription authority is not consumed
- inscription state remains `NotStarted`

If broadcast outcome is `ambiguous`:

- inscription authority is frozen
- inscription state becomes `Ambiguous`
- the Numbers count must not be interrupted
- no second semantically distinct inscription is authorized

If canonical inscription confirmation is observed:

- persist `InscriptionConfirmationRecord`
- inscription state becomes `Inscribed`

`Inscribed` and `Ambiguous` are terminal inscription states.

---

# 13. Restart Behavior

On restart:

- state must be reconstructed exclusively from canonical event records
- missing required records must halt execution
- malformed records must halt execution
- contradictory records must halt execution
- authority must not be consumed during reconstruction
- live inscription broadcast must not be triggered by restart
- inscription success must not be simulated by restart

Restart deterministic evaluation is limited to transitions explicitly permitted by `data/RESTART-RULES.md`.

No other automatic transition is permitted.

Restart is reconstruction, not recovery.

---

# Final Rule

If a step is not explicitly defined in this sequence:

It is forbidden.