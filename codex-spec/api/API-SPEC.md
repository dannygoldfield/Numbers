# Application Programming Interface Specification: Numbers

This document defines the entire external API surface exposed by the Numbers backend.

It is normative.

Authority precedence is defined exclusively in `AUTHORITY-ORDER.md`.

This document assumes familiarity with:

- `core/INVARIANTS.md`
- `core/STATE-MACHINE-TABLE.md`
- `core/STATE-MACHINE.md`
- `core/EVENT-TYPES.md`
- `data/DATA-MODEL.md`
- `errors/ERROR-TAXONOMY.md`

If there is a conflict, higher-authority documents listed in `AUTHORITY-ORDER.md` prevail.

---

# 1. Purpose

The API exists to:

- expose canonical system state
- expose mechanically derived state
- accept explicitly permitted external actions
- surface recorded knowledge without speculative interpretation

The API does not:

- infer outcomes
- summarize meaning
- provide convenience abstractions
- grant authority
- restore authority
- repair missing records
- replace canonical event records

---

# 2. Design Goals

The API is:

- minimal
- enumerable
- deterministic in shape
- explicit in failure
- aligned strictly with canonical event records
- non-authoritative with respect to lifecycle truth

The API reflects what is recorded or mechanically derived from what is recorded.

It does not speculate.

---

# 3. API Authority Model

The API is not a source of lifecycle truth.

The API exposes backend state derived from canonical event records.

The Numbers backend is authoritative for:

- auction lifecycle reconstruction
- bid admission evaluation
- auction resolution
- settlement determination
- auction finalization
- inscription intent recording
- inscription ambiguity tracking

Bitcoin Testnet, Ordinals, wallet state, mempool state, and confirmation state are external execution surfaces.

External execution surfaces do not replace Numbers canonical event records.

API availability, delay, restart, or failure must not alter:

- canonical truth
- lifecycle state
- sequencing
- finalization
- authority consumption
- restart reconstruction

---

# 4. API Shape Rules

All endpoints must obey:

- strict input validation
- deterministic output shape
- stable field meaning within the API version
- explicit success or error envelope
- no silent failure
- no undocumented side effects

All fields defined by `api/API-STATE-SHAPES.md` must be present in responses.

Unknown, unavailable, or not-yet-applicable values must be represented as `null`.

Fields defined by `api/API-STATE-SHAPES.md` must not be omitted.

Additional undocumented fields must not appear.

The API must not expose inferred fields.

Mechanically derived fields are permitted only when they are strictly computed from canonical event records.

---

# 5. Response Envelope

All responses must use this envelope:

```json
{
  "status": "success",
  "data": {}
}
```

or:

```json
{
  "status": "error",
  "error": {}
}
```

## Success Responses

On success:

- `status` must equal `"success"`
- `data` must be present
- `error` must be absent

## Error Responses

On error:

- `status` must equal `"error"`
- `error` must be present
- `data` must be absent

Error responses must include:

- `error_class`
- `error_code`
- `message`

Error class must map to `errors/ERROR-TAXONOMY.md`.

Error code must be machine-stable.

Error code meaning must not change within the API version.

The API must not retry actions on behalf of clients.

---

# 6. Public Endpoint Surface

Endpoints listed here define the entire permitted API surface.

No undocumented endpoint may exist.

The permitted endpoints are:

1. `GET /state`
2. `GET /auction/history`
3. `GET /auction/{N}`
4. `POST /bid`

---

# 7. `GET /state`

Returns the current global system state.

The response must expose only:

- canonical event record values
- mechanically derived values computed from canonical event records
- explicit `null` values for unknown, unavailable, or not-yet-applicable fields

## Required Data Fields

`GET /state` data must include:

- `current_number`
- `auction_state`
- `system_control_state`
- `opened_at`
- `base_end_time`
- `number_of_extension_events`
- `current_end_time`
- `closed_at`
- `resolution_time`
- `settlement_status`
- `finalized_at`
- `final_destination`
- `current_high_bid`
- `bid_count`
- `inscription_state`
- `inscription_adapter_mode`
- `inscription_txid`
- `last_record_sequence_index`

## Derived Field Rules

`current_end_time` must be computed as:

```text
current_end_time =
base_end_time +
(extension_increment_seconds * number_of_extension_events)
```

`current_high_bid` must be derived only from valid `BidRecord` entries.

`bid_count` must be derived from `BidRecord` entries.

`inscription_state` must be reconstructed from inscription canonical event records.

`last_record_sequence_index` must equal the highest persisted canonical event record sequence index.

## Forbidden Behavior

`GET /state` must not:

- predict future transitions
- compute client-facing remaining time
- infer upcoming state changes
- interpolate missing temporal data
- hide ambiguity
- hide `NullSteward`
- expose mutable internal state

The frontend may compute a countdown display from `current_end_time`.

The backend must not expose speculative time-remaining projections.

---

# 8. `GET /auction/history`

Returns finalized auction outcomes in strict sequence order.

## Rules

- results must be paginated
- each entry must correspond to exactly one auction number
- only finalized auctions may be returned
- ordering must be canonical and immutable

## Required Entry Fields

Each history entry must include:

- `number`
- `auction_state`
- `winning_bid_id`
- `winning_amount_sats`
- `settlement_status`
- `final_destination`
- `finalization_time`
- `inscription_state`
- `inscription_adapter_mode`
- `inscription_txid`
- `inscription_satpoint`

Unknown, unavailable, or not-yet-applicable values must be represented as `null`.

## Forbidden Behavior

`GET /auction/history` must not:

- expose non-finalized auctions
- infer ownership
- collapse null outcomes
- hide `NullSteward` results
- rewrite history
- reinterpret history

---

# 9. `GET /auction/{N}`

Returns recorded and mechanically derived state for auction number `N`.

## Required Data Fields

The response must include:

- `number`
- `auction_state`
- `system_control_state`
- `opened_at`
- `base_end_time`
- `number_of_extension_events`
- `current_end_time`
- `closed_at`
- `resolution_time`
- `winning_bid_id`
- `winning_amount_sats`
- `settlement_status`
- `settlement_deadline`
- `finalized_at`
- `final_destination`
- `bid_count`
- `current_high_bid`
- `inscription_state`
- `inscription_adapter_mode`
- `inscription_txid`
- `ambiguity`

Unknown, unavailable, or not-yet-applicable values must be represented as `null`.

## Forbidden Behavior

`GET /auction/{N}` must not:

- infer missing records
- infer ownership
- reinterpret settlement
- reinterpret finalization
- hide ambiguity
- expose undocumented internal fields

---

# 10. `POST /bid`

Submits a bid for the current auction.

This is the only write-capable public endpoint.

`POST /bid` may be submitted only when auction state is:

- `Scheduled`
- `Open`

If auction state is `Scheduled` and the bid is valid, the bid atomically opens the auction.

If auction state is `Open` and the bid is valid, the bid is accepted into the open auction.

If auction state is `Closed`, `AwaitingSettlement`, or `Finalized`, the bid must be rejected explicitly.

---

## 10.1 Request Fields

A bid request must include:

- `auction_number`
- `bidder_address`
- `amount_sats`
- `destination_address`
- `nonce`
- `signature`

No additional request fields may alter bid semantics.

---

## 10.2 Preconditions

A bid request is valid only if all are true:

- system control state is `Running`
- auction state is `Scheduled` or `Open`
- the bid targets the current auction number
- the request proves control of the bidding wallet
- the bid amount satisfies the minimum bid rules
- the destination address satisfies configured address policy
- the request is well-formed

If any precondition fails, the bid must be rejected explicitly.

---

## 10.3 Bid Record Rule

Every bid submission attempt that reaches admission evaluation must produce exactly one `BidRecord`.

A valid bid must produce:

```text
BidRecord.validity = valid
```

An invalid bid must produce:

```text
BidRecord.validity = invalid
```

Invalid bids must include a non-null rejection reason.

Bid validity must be evaluated deterministically at authoritative server receipt time.

Bid validity must never change after persistence.

---

## 10.4 Scheduled Auction Behavior

If the current auction state is `Scheduled` and the bid is valid:

- persist `BidRecord`
- persist `AuctionOpenRecord`
- both records must be persisted atomically
- `AuctionOpenRecord.opened_at` must equal authoritative `server_time`
- `AuctionOpenRecord.base_end_time` must equal `opened_at + auction.duration_seconds`
- auction state becomes `Open`

If the current auction state is `Scheduled` and the bid is invalid:

- persist invalid `BidRecord`
- auction state remains `Scheduled`

---

## 10.5 Open Auction Behavior

If the current auction state is `Open` and the bid is valid:

- persist `BidRecord`
- evaluate whether an `ExtensionEventRecord` is required under bidding rules
- persist `ExtensionEventRecord` only if explicitly required by the governing bidding specification
- auction state remains `Open`

If the current auction state is `Open` and the bid is invalid:

- persist invalid `BidRecord`
- auction state remains `Open`

---

## 10.6 Forbidden Bid Behavior

Bid submission must not:

- consume inscription authority
- imply winning
- imply settlement
- imply inscription
- alter finalization
- reopen a closed auction
- alter `base_end_time`
- alter persisted records
- trigger inscription broadcast
- retry on behalf of the client

Bid acceptance does not imply winning.

Bid acceptance does not imply settlement.

Bid acceptance does not imply inscription.

---

# 11. Bid Validation

Bid validation is evaluated against authoritative server receipt time and persisted auction state.

A bid is invalid if:

- submitted for a non-current auction number
- submitted when system control state is not `Running`
- submitted when auction state is not `Scheduled` or `Open`
- submitted after `AuctionCloseRecord`
- malformed or incomplete
- below the minimum valid bid
- below required increment
- exceeds a configured maximum if such maximum is defined
- not provably authorized
- destination address violates configured address policy

The minimum opening bid:

- is defined by configuration
- must not change during the auction

The minimum increment:

- is fixed at auction open
- must not change during the auction

Minimum constraints must not change in response to other bids unless explicitly specified by `bidding/BIDDING-ADMISSION.md`.

Invalid bids:

- are rejected explicitly
- are recorded as invalid `BidRecord` entries if admission evaluation was reached
- do not alter auction lifecycle state
- do not affect timing
- do not participate in resolution
- do not consume authority

---

# 12. Versioning Rules

The API version for this prototype is:

```text
v1
```

Field meaning must never change within `v1`.

Breaking changes require a new version.

Older versions must not silently change semantics.

Deprecation:

- must be explicit
- must not alter historical responses

---

# 13. Security and Isolation

The API must not:

- expose private keys
- expose wallet seed material
- expose persistence internals
- expose operator controls unless explicitly specified
- allow clients to influence timing
- allow clients to influence sequencing
- allow clients to influence authority
- allow clients to modify canonical records
- allow clients to delete canonical records

Clients observe and submit bids.

They do not steer lifecycle truth.

---

# 14. Non-Goals

The API does not:

- provide analytics
- infer ownership
- infer intent
- expose internal logs
- explain outcomes
- optimize for convenience
- provide production authentication in Demo 1
- provide production rate limiting in Demo 1

---

# Final Rule

The API exposes canonical facts and mechanically derived state.

Interpretation happens elsewhere.

Authority lives elsewhere.