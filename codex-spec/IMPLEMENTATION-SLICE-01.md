# Implementation Slice 01: Demo 1

This document defines the first implementation slice for Numbers.

It is normative for Demo 1 implementation scope.

This document does not redefine core lifecycle semantics, authority consumption, canonical record semantics, restart rules, or protocol invariants.

If this document conflicts with higher-authority documents, the higher-authority document controls.

---

## 1. Purpose

`IMPLEMENTATION-SLICE-01.md` defines the minimum build target for the first Codex implementation pass.

The target is:

> Demo 1: deterministic single-machine browser auction demo without live Ordinals broadcast.

Demo 1 must demonstrate:

- auction lifecycle
- first valid bid opens auction
- deterministic bid admission and rejection
- auction close
- time-based auction close
- resolution
- simplified settlement outcome
- finalization
- append-only canonical event records
- restart reconstruction
- frontend-visible current state
- frontend-visible auction history
- sequence advancement after finalization and rhythm gap

Demo 1 must not require live Bitcoin, live Ordinals, wallet funding, or external chain observation.

---

## 2. Included Components

Demo 1 includes:

- local backend
- local browser frontend
- SQLite persistence
- append-only canonical event record storage
- deterministic state reconstruction from canonical event records
- auction lifecycle execution
- bid admission
- invalid bid recording
- auction close
- time-based auction close
- resolution
- simplified settlement
- finalization
- `InscriptionIntentRecord` persistence with deferred inscription adapter mode
- `GET /state`
- `POST /bid`
- `GET /auction/history`
- restart demonstration

---

## 3. Excluded Components

Demo 1 excludes:

- live Ordinals broadcast
- `ord`
- Bitcoin Core RPC
- wallet integration
- live settlement address derivation unless explicitly stubbed by fixed Demo 1 behavior
- inscription funding
- transaction construction
- transaction signing
- mempool checks
- confirmation observation
- public deployment
- production authentication
- production rate limiting
- generalized payments
- automatic retry behavior
- mainnet `NullSteward` burn implementation
- bid cap closure

Excluded components must not be required for Demo 1 auction correctness.

Excluded components must not block Demo 1 state reconstruction, API state generation, or frontend demonstration.

---

## 4. Storage

Demo 1 must use SQLite for local persistence.

SQLite storage must contain an append-only canonical event record table.

Canonical lifecycle truth must be derived from canonical event records only.

Mutable lifecycle state must not be stored as canonical truth.

Runtime state can be cached only if it is reconstructible from canonical event records.

Restart must reconstruct current state from canonical event records.

Restart must not repair, alter, infer, skip, or synthesize missing canonical records.

---

## 5. Auction Creation and Sequence

Auction numbers must begin from `auction.starting_number`.

Auction numbers must increase by exactly `1`.

Auction numbers must not be skipped, reused, or reordered.

`AuctionRecord` for `N + 1` must not be persisted until:

1. `FinalizationRecord` exists for `N`
2. `auction.inter_auction_gap_seconds` has elapsed after `FinalizationRecord.finalization_time`

The `auction.inter_auction_gap_seconds` interval is a rhythm gap only.

It must not be treated as:

- a recovery window
- a settlement window
- an inscription window
- an automatic auction-start trigger

Auction `N + 1` does not open automatically.

Auction `N + 1` opens only when the first valid bid for `N + 1` is accepted.

Inscription progress for `N` must not block auction availability for `N + 1` after finalization and rhythm-gap requirements are satisfied.

When `FinalizationRecord` exists for `N`, `auction.inter_auction_gap_seconds` has elapsed, and no active auction exists, the backend must persist `AuctionRecord` for `N + 1` at the next state-evaluation boundary.

Demo 1 state-evaluation boundaries are:

- backend startup after restart reconstruction completes
- `GET /state`
- `POST /bid`
- `GET /auction/history`

This persistence makes auction `N + 1` `Scheduled` only.

It must not open auction `N + 1`.

No background scheduler is required for Demo 1.

---

## 6. Scheduled Auction Behavior

A newly available auction begins in `Scheduled`.

While an auction is `Scheduled`:

- bid submissions can be evaluated
- invalid bids can be persisted as invalid `BidRecord` entries
- valid bid acceptance atomically opens the auction
- no countdown runs before the first valid bid is accepted
- no close time exists before the first valid bid is accepted

If no valid bid is accepted:

- the auction remains `Scheduled`
- no countdown starts
- no auction close occurs
- no resolution occurs
- no settlement occurs
- no finalization occurs
- no inscription process begins
- no `NullSteward` outcome is produced solely because no valid bid was accepted

---

## 7. Bid Admission

`POST /bid` must be accepted for evaluation only when:

- system control state is `Running`
- auction lifecycle state is `Scheduled` or `Open`

Bid admission must deterministically classify each evaluated bid as valid or invalid.

A bid accepted as valid must be persisted as a valid `BidRecord`.

A bid rejected as invalid must be persisted as an invalid `BidRecord` if it reached admission evaluation.

Invalid bids must not alter auction lifecycle state.

Invalid bids must not participate in resolution.

Bid acceptance means only that the bid is valid and persisted.

Bid acceptance must not imply:

- winning
- settlement
- finalization
- inscription
- transfer
- custody
- payment completion

---

## 8. First Valid Bid Opens Auction

When the first valid bid is accepted for a `Scheduled` auction:

- valid `BidRecord` must be persisted
- `AuctionOpenRecord` must be persisted
- auction lifecycle becomes `Open`
- countdown begins
- `base_end_time` and `current_end_time` are computed from the open time and configured auction duration

This operation must be atomic at the canonical record level.

There is no separate transition record unless explicitly defined elsewhere.

---

## 9. Open Auction Behavior

While auction lifecycle state is `Open`:

- valid bids can be accepted
- invalid bids can be recorded
- current leading bid must be derived from persisted valid `BidRecord` entries
- auction end time must be derived according to the specified timing rules
- bid display state must be reconstructible from canonical records

When `server_time >= current_end_time`, the auction must close according to the state machine.

Bid cap closure is excluded from Demo 1.

`cap_reached` must not be emitted in Demo 1.

---

## 10. Auction Close

Auction close must persist the canonical record required by the data model.

After auction close:

- no further valid bid may be accepted for that auction
- later bid submissions for that auction must not alter lifecycle state
- resolution becomes eligible

Close must not depend on live inscription, wallet state, Bitcoin Core, mempool observation, or confirmation observation.

Demo 1 implements time-based close only.

---

## 11. Resolution

Resolution must be deterministic.

Resolution must use only persisted valid `BidRecord` entries.

Invalid `BidRecord` entries must not participate in resolution.

Resolution must produce exactly one `ResolutionRecord`.

Resolution must not be recomputed after `ResolutionRecord` exists.

For Demo 1, a closed auction should have at least one valid bid because auctions open only on first valid bid.

A no-winner resolution path must not be implemented unless explicitly required by another active implementation slice.

---

## 12. Simplified Settlement

Demo 1 uses simplified settlement behavior.

The purpose of Demo 1 settlement is to demonstrate deterministic lifecycle progression, not live payment handling.

Demo 1 must not require:

- Bitcoin payment detection
- wallet-derived settlement address
- Bitcoin Core RPC
- mempool observation
- confirmation observation

For Demo 1, the implementation must provide a deterministic local way to produce a terminal settlement outcome after resolution.

The allowed Demo 1 terminal settlement outcomes are:

- `settled`
- `expired`

If settlement outcome is `settled`:

- final destination is the winner destination recorded through bid admission

If settlement outcome is `expired`:

- final destination is `NullSteward`

Settlement outcome must be persisted in exactly one `SettlementRecord`.

Settlement must not create or consume inscription authority.

Settlement must not create retry behavior.

Late payment behavior is outside Demo 1.

---

## 13. Finalization

Finalization occurs after terminal settlement outcome.

Finalization must persist exactly one `FinalizationRecord`.

`FinalizationRecord` must define the final destination for the number.

Allowed final destinations for Demo 1 are:

- winner destination
- `NullSteward`

After finalization:

- auction lifecycle state is terminal
- final destination must not change
- exactly one deferred `InscriptionIntentRecord` must be persisted for the finalized auction
- sequence advancement toward `N + 1` becomes eligible only after `auction.inter_auction_gap_seconds` has elapsed
- inscription lifecycle can proceed only as permitted by this implementation slice

For Demo 1, `FinalizationRecord` and the required deferred `InscriptionIntentRecord` must be persisted through the same serialized canonical commit path or immediately adjacent serialized commits before any later auction availability evaluation.

---

## 14. NullSteward

`NullSteward` is a protocol-visible final destination.

For Demo 1, `NullSteward` is a protocol label only.

Demo 1 must not implement:

- live `NullSteward` inscription
- burn address
- burn script
- Bitcoin transaction
- wallet funding for `NullSteward`

`NullSteward` must not be used as a recovery mechanism.

`NullSteward` must not be used to repair inscription ambiguity after inscription authority is consumed or frozen.

---

## 15. Inscription Behavior

Demo 1 must not perform live inscription broadcast.

Demo 1 must not simulate live inscription success.

Demo 1 must not simulate confirmation.

Demo 1 must persist exactly one deferred `InscriptionIntentRecord` for each finalized auction.

For Demo 1 inscription intent:

- `adapter_mode` must be `deferred_in_this_slice`
- no transaction is constructed
- no transaction is signed
- no transaction is broadcast
- no mempool state is checked
- no confirmation state is checked
- inscription status must remain visibly deferred or not started according to the API state shape

`InscriptionIntentRecord` does not consume inscription authority.

No `broadcast_commit` occurs in Demo 1.

---

## 16. API Endpoints

Demo 1 must implement:

- `GET /state`
- `POST /bid`
- `GET /auction/history`

Additional endpoints are excluded unless required to demonstrate restart or local settlement control.

API responses must use fixed response shapes where specified.

Unknown, unavailable, or not-yet-applicable values must be represented as `null` where the API state shape requires the field.

Fields defined in API state shapes must not be omitted.

---

## 17. GET /state

`GET /state` must return frontend-visible current state.

The state must be derived from canonical event records.

`GET /state` must expose enough information for the browser frontend to display:

- current auction number
- auction lifecycle state
- countdown state
- current leading bid
- bid history summary
- settlement status
- final destination when known
- inscription status
- sequence availability state
- server time

`GET /state` must not depend on live Bitcoin, wallet state, mempool observation, or confirmation observation.

---

## 18. POST /bid

`POST /bid` must evaluate bid admission.

`POST /bid` must support the first-valid-bid opening model.

When auction state is `Scheduled`, the first valid bid must atomically:

- persist a valid `BidRecord`
- persist `AuctionOpenRecord`
- open the auction
- start the countdown

When auction state is `Open`, a valid bid must persist a valid `BidRecord`.

Invalid bids that reach admission evaluation must persist invalid `BidRecord` entries.

`POST /bid` must return deterministic response fields sufficient for frontend display.

`POST /bid` must not imply winning, settlement, finalization, inscription, or transfer.

---

## 19. GET /auction/history

`GET /auction/history` must return canonical event-derived auction history.

History must be derived from canonical event records.

History must include enough information to inspect:

- auction creation
- valid bids
- invalid bids
- auction open
- auction close
- time-based auction close
- resolution
- settlement outcome
- finalization
- inscription intent or deferred inscription status when present

History must not be reconstructed from mutable lifecycle state.

---

## 20. Frontend Requirements

Demo 1 frontend must be browser-demonstrable on a single machine.

Frontend must display:

- current number
- auction lifecycle state
- countdown or scheduled state
- current leading bid
- bid submission form
- bid admission result
- settlement or finalization result when available
- inscription status as not started or deferred
- auction history

Frontend polish is not required.

Frontend behavior must not imply live Bitcoin payment, live inscription, or live transfer.

---

## 21. Restart Demonstration

Demo 1 must demonstrate restart reconstruction.

After backend restart:

- state must be reconstructed from canonical event records
- current auction state must match pre-restart canonical truth
- bid history must remain visible
- finalized auctions must remain finalized
- final destinations must remain unchanged
- deferred inscription state must remain deferred or not started
- no live inscription action may be triggered by restart
- no authority may be granted by restart
- no retry may be triggered by restart

---

## 22. Error Handling

Demo 1 must fail explicitly when required behavior is undefined or illegal.

The implementation must not silently repair:

- missing canonical records
- illegal transitions
- malformed persisted state
- authority violations
- sequence gaps
- duplicate auction numbers
- duplicate terminal records

Retry behavior is excluded unless explicitly defined by this implementation slice.

This slice defines no automatic retry behavior.

---

## 23. Completion Criteria

`IMPLEMENTATION-SLICE-01.md` is complete when a local browser demo can show:

1. auction starts as `Scheduled`
2. first valid bid opens auction
3. invalid bid is recorded without changing lifecycle state
4. valid bids update leading bid deterministically
5. auction closes when time condition is met
6. resolution produces a deterministic winner
7. simplified settlement produces a terminal outcome
8. finalization records final destination
9. next auction becomes available only after finalization and rhythm gap
10. next auction opens only on first valid bid
11. inscription status is not started or deferred, with no live broadcast
12. restart reconstructs state from canonical event records
13. auction history is visible in the frontend

---

## 24. Final Rule

Demo 1 implements only the behavior explicitly included in this document and the higher-authority specification documents.

Behavior excluded by this document must not be implemented for Demo 1.

If required implementation behavior is undefined, implementation must stop and the specification must be revised.