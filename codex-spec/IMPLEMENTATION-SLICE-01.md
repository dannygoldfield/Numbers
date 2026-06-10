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
- `POST /demo/settlement`
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

Each canonical event record must persist `payload_json` and `payload_hash`.

`payload_hash` must be computed using the canonical payload serialization and SHA-256 rules defined in `data/DATA-MODEL.md`.

Restart validation must recompute `payload_hash` for persisted records and halt on mismatch.

Canonical lifecycle truth must be derived from canonical event records only.

Mutable lifecycle state must not be stored as canonical truth.

Runtime state can be cached only if it is reconstructible from canonical event records.

Restart must reconstruct current state from canonical event records.

Restart must not repair, alter, infer, skip, or synthesize missing canonical records.

---

## 5. Auction Creation and Sequence

Auction numbers must begin from `auction.starting_number`.

For Demo 1, `auction.starting_number` defaults to `1`.

The first `AuctionRecord.number` must equal `auction.starting_number`.

On an empty canonical store, after configuration validation, the backend must persist exactly one `AuctionRecord` at the first state-evaluation boundary.

That initial `AuctionRecord` must use `number = auction.starting_number`.

Initial `AuctionRecord` persistence creates `Scheduled` only.

It must not open a countdown, persist `AuctionOpenRecord`, accept a bid by inference, resolve, settle, finalize, or create inscription intent.

Auction numbers must increase by exactly `1`.

Auction numbers must not be skipped, reused, or reordered.

Each `AuctionRecord` must capture the fixed configuration values needed for deterministic replay of that auction:

- `duration_seconds`
- `extension_window_seconds`
- `extension_increment_seconds`
- `max_extensions`
- `minimum_bid_sats`
- `minimum_increment_sats`
- `maximum_bid_sats`

Captured values must be used for later bid admission, extension, close, resolution, and restart reconstruction for that auction.

Later configuration changes must not alter an auction whose `AuctionRecord` already exists.

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
- `POST /demo/settlement`
- `GET /auction/history`

At each state-evaluation boundary, the backend must run one serialized deterministic state-evaluation step before forming a response or accepting an action.

The state-evaluation step must persist any required deterministic lifecycle records whose conditions are already satisfied, in this order:

1. initial `AuctionRecord` creation on empty canonical store
2. time-based `AuctionCloseRecord` for an open auction whose `current_end_time` has passed
3. `ResolutionRecord` for a closed auction without resolution
4. chain-confirmed settlement-deadline expiration records only when a later active implementation slice enables chain-confirmed settlement semantics
5. next `AuctionRecord` after finalization and rhythm gap when no active auction exists

Demo 1 state evaluation must not repair a finalized auction that is missing its required deferred `InscriptionIntentRecord`.

Demo 1 local settlement must not auto-expire on `settlement_deadline`.

Demo 1 `expired` settlement can be produced only by `POST /demo/settlement`.

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

If no `PauseEventRecord` exists, system control state derives as `Running`.

Demo 1 uses `validation_profile = demo_local`.

Demo 1 bid validation does not require:

- wallet proof-of-control
- cryptographic signature validation
- nonce replay protection
- Bitcoin address ownership proof

For Demo 1, `bidder_address`, `nonce`, and `signature` are optional and ignored for validity if supplied.

Bid admission must deterministically classify each evaluated bid as valid or invalid.

A bid accepted as valid must be persisted as a valid `BidRecord`.

A bid rejected as invalid must be persisted as an invalid `BidRecord` if it reached admission evaluation.

Invalid bids must not alter auction lifecycle state.

Invalid bids must not participate in resolution.

Bid acceptance means only that the bid is valid and persisted under the Demo 1 local validation profile.

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
- current leading bid must be derived from persisted valid `BidRecord` entries using highest `amount_sats`, with lowest canonical `sequence_index` as tie-break
- auction end time must be derived according to the specified timing rules
- bid display state must be reconstructible from canonical records

When `server_time >= current_end_time`, the auction must close according to the state machine.

Bid cap closure is excluded from Demo 1.

`cap_reached` must not be emitted in Demo 1.

---

## 10. Auction Close

Auction close must persist the canonical record required by the data model.

After auction close:

- no further valid bid must be accepted for that auction
- later bid submissions for that auction must not alter lifecycle state
- resolution becomes eligible

Close must not depend on live inscription, wallet state, Bitcoin Core, mempool observation, or confirmation observation.

Demo 1 implements time-based close only.

---

## 11. Resolution

Resolution must be deterministic.

Resolution must use only persisted valid `BidRecord` entries.

Invalid `BidRecord` entries must not participate in resolution.

The winning bid is the valid `BidRecord` with the highest `amount_sats`.

If more than one valid `BidRecord` has the same highest `amount_sats`, the winning bid is the one with the lowest canonical `sequence_index`.

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

For Demo 1, terminal settlement outcome is produced through `POST /demo/settlement`.

The allowed Demo 1 terminal settlement outcomes are:

- `settled`
- `expired`

If settlement outcome is `settled`:

- `SettlementRecord.status` must be `settled`
- `SettlementRecord.settlement_source` must be `demo_local`
- `SettlementRecord.confirmation_txid` must be `null`
- final destination is the winner destination recorded through bid admission

If settlement outcome is `expired`:

- `SettlementRecord.status` must be `expired`
- `SettlementRecord.settlement_source` must be `demo_local`
- `SettlementRecord.confirmation_txid` must be `null`
- final destination is `NullSteward`
- no settlement-deadline precondition is required in Demo 1 local settlement control

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

Allowed Demo 1 `FinalizationRecord.finalization_reason` values are:

- `settled_to_winner` when `SettlementRecord.status = settled`
- `expired_to_nullsteward` when `SettlementRecord.status = expired`

After finalization:

- auction lifecycle state is terminal
- final destination must not change
- exactly one deferred `InscriptionIntentRecord` must be persisted for the finalized auction
- sequence advancement toward `N + 1` becomes eligible only after `auction.inter_auction_gap_seconds` has elapsed
- inscription lifecycle can proceed only as permitted by this implementation slice

For Demo 1, `SettlementRecord`, `FinalizationRecord`, and the required deferred `InscriptionIntentRecord` must be persisted in one atomic canonical commit group by `POST /demo/settlement`.

If any record in this commit group cannot be persisted, none of the three records may persist and the request must fail explicitly.

A Demo 1 `FinalizationRecord` without the required deferred `InscriptionIntentRecord` is invalid persisted state and must halt reconstruction.

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
- `destination_script_pubkey` must be `null`
- `settlement_reference` must equal the `record_id` of the `SettlementRecord` that caused finalization
- `intent_id` must be derived according to `inscription/INSCRIPTION-MACHINE.md`
- no transaction is constructed
- no transaction is signed
- no transaction is broadcast
- no mempool state is checked
- no confirmation state is checked
- inscription status must remain visibly deferred or not started according to the API state shape

Demo 1 local destination identifiers and `NullSteward` do not need to be convertible to Bitcoin `scriptPubKey` values.

`InscriptionIntentRecord` does not consume inscription authority.

No `broadcast_commit` occurs in Demo 1.

---

## 16. API Endpoints

Demo 1 must implement:

- `GET /state`
- `POST /bid`
- `GET /auction/history`
- `POST /demo/settlement`

Additional endpoints are excluded except `POST /demo/settlement`, which is required for Demo 1 local settlement control.

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
- next auction availability time when applicable
- rhythm gap elapsed status when applicable
- server time

`GET /state` must use the sequence availability fields defined in `api/API-STATE-SHAPES.md`.

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

`POST /bid` must return the exact response shape defined in `api/API-STATE-SHAPES.md`.

`POST /bid` must not imply winning, settlement, finalization, inscription, or transfer.

---

## 19. GET /auction/history

`GET /auction/history` must return canonical event-derived auction history using the exact shape defined in `api/API-STATE-SHAPES.md`.

History must be derived from canonical event records.

History must include entries for auctions with an `AuctionRecord`, including non-finalized auctions.

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

History must include ordered canonical event record summaries for each auction.

History must not be reconstructed from mutable lifecycle state.

`GET /auction/history` is the Demo 1 inspection endpoint.

Pagination request parameters:

- `limit`: optional integer, default `50`, minimum `1`, maximum `100`
- `offset`: optional integer, default `0`, minimum `0`

History entries must be ordered by auction number ascending.

Invalid pagination parameters must return the API error envelope with `error_code = invalid_pagination`.

It must not be treated as a finalized-outcome-only summary endpoint.

---

## 19.1 POST /demo/settlement

`POST /demo/settlement` is Demo 1 local-only settlement control.

It must be accepted only when auction lifecycle state is `AwaitingSettlement`.

It accepts:

- `auction_id`
- `outcome`

`outcome` must be one of:

- `settled`
- `expired`

If `outcome = settled`, final destination must be the winner destination.

If `outcome = expired`, final destination must be `NullSteward`.

`POST /demo/settlement` must persist exactly one `SettlementRecord`, exactly one `FinalizationRecord`, and exactly one deferred `InscriptionIntentRecord` in one atomic canonical commit group.

If any of the three records cannot be persisted, none of the three records may persist and the request must fail explicitly.

`POST /demo/settlement` must return success only after all three records are durably persisted.

`POST /demo/settlement` must return the exact response shape defined in `api/API-STATE-SHAPES.md`.

`POST /demo/settlement` must not simulate live payment, mempool observation, or confirmation observation.

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
- a finalized Demo 1 auction missing its required deferred `InscriptionIntentRecord` must halt reconstruction
- restart must not repair or append a missing deferred `InscriptionIntentRecord`
- no live inscription action must be triggered by restart
- no authority must be granted by restart
- no retry must be triggered by restart

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

Demo 1 API error codes must use the stable vocabulary defined in `api/API-SPEC.md` and `api/API-STATE-SHAPES.md`.

`GET /auction/{N}` is reserved for a later implementation slice and must not be implemented in Demo 1.

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