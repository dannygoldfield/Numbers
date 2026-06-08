# Data Model: Numbers

This document defines the canonical data recorded by Numbers.

It is normative.

Authority precedence is defined exclusively in `AUTHORITY-ORDER.md`.

It specifies:

- which canonical event records exist
- when canonical event records are written
- how lifecycle state is derived from canonical event records

Numbers uses append-only persistence.

Once written, canonical event records must never be mutated, reinterpreted, or deleted.

---

## Modal Language Rule

In this document and all normative specifications:

- `must` and `must not` define obligations
- `only`, `exactly once`, and `at most once` define bounds
- `may` is permitted only to describe uncertainty of knowledge

The following terms are forbidden in normative contexts:

- `possibly`
- `likely`
- `eventually`
- `for now`
- `TBD`

Any normative statement using forbidden modal language is invalid.

---

# 1. Data Model Principle

The Numbers data model is an append-only ordered log of canonical event records.

A canonical event record is the same persisted object as a canonical record.

The terms `event`, `event record`, `canonical record`, and `persisted record` refer to the same append-only object when used for system truth.

There is no separate event model and record model.

Record type names are the canonical event names.

Lifecycle state is derived from canonical event record presence, order, and contents.

Lifecycle state must never be stored as mutable truth.

---

# 2. Common Record Envelope

Every canonical event record must contain:

- `record_id`
- `record_type`
- `auction_id`
- `number`
- `sequence_index`
- `server_time`
- `payload_json`
- `payload_hash`

## Field Rules

### `record_id`

- must uniquely identify one canonical event record
- must never be reused

### `record_type`

- must equal one of the canonical record types defined in this document
- must equal one of the canonical event record types defined in `core/EVENT-TYPES.md`

### `auction_id`

- must identify the auction associated with the record

### `number`

- must identify the number associated with the auction

### `sequence_index`

- must define the total order of canonical event records
- must increase monotonically
- must never be reused
- must never be rewritten

### `server_time`

- must equal authoritative server time at the record boundary
- must not be supplied by the client

### `payload_json`

- must contain the record-specific fields defined for the record type

### `payload_hash`

- must commit to the canonical serialized payload
- must be stable for the persisted record
- must not change after persistence

## Demo 1 Identifier Generation

For Demo 1, identifiers must be generated deterministically at the canonical record persistence boundary.

Rules:

- `record_id` must equal `rec_` followed by the zero-padded 12-digit `sequence_index`
- `auction_id` must equal `auc_` followed by the zero-padded 12-digit auction `number`
- `bid_id` must equal `bid_` followed by the zero-padded 12-digit `sequence_index` of the `BidRecord`

The `sequence_index` must be allocated before computing any identifier derived from it.

Identifier generation must not depend on randomness, wall-clock time, process memory, database row IDs, or external systems.

## Canonical Payload Serialization

`payload_json` must be serialized for hashing as canonical UTF-8 JSON.

Canonical JSON serialization rules:

- object keys must be sorted lexicographically by Unicode code point
- insignificant whitespace must be omitted
- strings must be normalized to Unicode NFC before JSON encoding
- strings must be escaped using standard JSON string escaping
- quotation mark and reverse solidus must be escaped
- control characters U+0000 through U+001F must be escaped
- non-ASCII Unicode characters must be emitted as UTF-8 characters, not `\u` escapes, after NFC normalization
- solidus `/` must not be escaped
- integers must be encoded as JSON numbers without leading zeros
- booleans must be encoded as JSON booleans
- `null` must be encoded as JSON `null`
- timestamps must be encoded as UTC ISO-8601 strings with millisecond precision and `Z` suffix
- timestamp format must be `YYYY-MM-DDTHH:MM:SS.mmmZ`
- arrays must preserve their explicitly specified order

`payload_hash` must equal lowercase hexadecimal SHA-256 of the canonical payload JSON bytes.

`payload_hash` commits to `payload_json` only.

The common record envelope fields are not included in `payload_hash`.

Canonical serialization must produce identical bytes for identical payload values across restart.

---

# 3. Canonical Record Set

The entire canonical data model consists of:

1. `AuctionRecord`
2. `BidRecord`
3. `AuctionOpenRecord`
4. `ExtensionEventRecord`
5. `AuctionCloseRecord`
6. `ResolutionRecord`
7. `SettlementRecord`
8. `FinalizationRecord`
9. `InscriptionIntentRecord`
10. `InscriptionBroadcastRecord`
11. `InscriptionConfirmationRecord`
12. `AmbiguityRecord`
13. `PauseEventRecord`

No other record type can be persisted as canonical system truth.

Each canonical event record:

- must be immutable once written
- must have a stable schema
- must be written only at the boundary defined by the governing specification document
- must not encode mutable derived lifecycle state

---

# 4. AuctionRecord

Represents the existence of an auction for number `N`.

## Payload Fields

- `created_at`
- `duration_seconds`
- `extension_window_seconds`
- `extension_increment_seconds`
- `max_extensions`
- `minimum_bid_sats`
- `minimum_increment_sats`
- `maximum_bid_sats`

## Rules

- exactly one `AuctionRecord` must exist per number
- the first `AuctionRecord.number` must equal `auction.starting_number`
- each later `AuctionRecord.number` must equal the previous finalized auction number plus exactly `1`
- must be written only after the previous auction reaches `Finalized`, except for the first auction
- does not open the auction
- does not store lifecycle state
- must capture the fixed auction and bid configuration values listed in the payload fields
- captured configuration values must be used for all later bid admission, timing, extension, close, and restart evaluation for that auction
- captured configuration values must not be replaced by later configuration changes

---

# 5. BidRecord

Represents a bid submission attempt as evaluated at authoritative server receipt time.

## Payload Fields

- `bid_id`
- `bidder_id`
- `bidder_address`
- `amount_sats`
- `destination_address`
- `validation_profile`
- `nonce`
- `signature`
- `validity`
- `rejection_reason`

## Field Rules

### `validation_profile`

Must be one of:

- `demo_local`
- `cryptographic`

For Demo 1, `validation_profile` must be `demo_local`.

### `bidder_id`

For Demo 1, `bidder_id` must be non-null.

For live cryptographic validation, `bidder_id` can mirror the verified wallet identity only when that behavior is defined by an active implementation slice.

### `bidder_address`

For Demo 1, `bidder_address` can be `null` or non-null.

For Demo 1, `bidder_address` must not be used as proof of wallet control and must not alter bid validity.

### `nonce`

For Demo 1, `nonce` can be `null` or non-null.

For Demo 1, nonce replay protection is not required and `nonce` must not alter bid validity.

### `signature`

For Demo 1, `signature` can be `null` or non-null.

For Demo 1, `signature` must not be used as proof of wallet control and must not alter bid validity.

### `validity`

Must be one of:

- `valid`
- `invalid`

### `rejection_reason`

- must be `null` when `validity = valid`
- must be non-null when `validity = invalid`
- must use a stable rejection reason code defined in `bidding/BIDDING-ADMISSION.md` when `validation_profile = demo_local`

## Rules

- every bid submission attempt that reaches admission evaluation must produce exactly one `BidRecord`
- `server_time` must equal authoritative receipt time
- validity must be evaluated deterministically at submission
- validity must never change after persistence
- `BidRecord` does not consume auction authority
- `BidRecord` does not consume inscription authority

Only a `BidRecord` with `validity = valid` can:

- open a `Scheduled` auction
- trigger an extension
- participate in winner resolution

A `BidRecord` with `validity = invalid`:

- must not open an auction
- must not trigger an extension
- must not participate in winner resolution
- must not alter lifecycle state except as append-only audit truth

`current_high_bid` is the valid `BidRecord` with the highest `amount_sats` for the auction.

If more than one valid `BidRecord` has the same highest `amount_sats`, `current_high_bid` is the one with the lowest canonical `sequence_index`.

---

# 6. AuctionOpenRecord

Represents transition `Scheduled` to `Open`.

## Payload Fields

- `opening_bid_id`
- `opened_at`
- `base_end_time`

## Rules

- exactly one `AuctionOpenRecord` can exist per auction
- must be written atomically with the first valid `BidRecord`
- `opening_bid_id` must reference the first valid `BidRecord`
- `opened_at` must equal authoritative `server_time`
- `base_end_time` must equal `opened_at + auction.duration_seconds`
- `base_end_time` must never change

Presence of `AuctionOpenRecord` proves the auction entered `Open`.

---

# 7. ExtensionEventRecord

Represents one extension increment.

## Payload Fields

- `trigger_bid_id`
- `extension_increment_seconds`
- `extension_index`

## Rules

- one `ExtensionEventRecord` must exist per extension increment
- `trigger_bid_id` must reference a valid `BidRecord`
- records are append-only
- `extension_index` must increase by one for each extension on the same auction
- total extension records for an auction must not exceed configured `max_extensions`
- extension records must not modify `base_end_time`
- extension records must not create new lifecycle states
- extension records do not consume authority

`current_end_time` is derived as:

```text
current_end_time =
base_end_time +
(extension_increment_seconds * number_of_extension_events)
```

---

# 8. AuctionCloseRecord

Represents transition `Open` to `Closed`.

## Payload Fields

- `closed_at`
- `reason`

## Field Rules

### `reason`

Must be one of:

- `duration_expired`
- `cap_reached`

## Rules

- exactly one `AuctionCloseRecord` must exist per opened auction
- must be written when close trigger fires
- `closed_at` must equal authoritative `server_time`
- no further valid bids are permitted after `AuctionCloseRecord`

A valid `BidRecord` before `AuctionCloseRecord` can participate in resolution.

A `BidRecord` after `AuctionCloseRecord` must be invalid.

---

# 9. ResolutionRecord

Represents deterministic winner resolution.

## Payload Fields

- `winning_bid_id`
- `winning_amount_sats`
- `resolution_time`
- `settlement_deadline`
- `resolution_inputs_hash`

## Field Rules

### `winning_bid_id`

- must reference the winning valid `BidRecord`
- must not be `null` in Demo 1
- `null` is reserved for a future implementation slice that explicitly defines a no-winner resolution path

### `winning_amount_sats`

- must equal the amount of the winning valid bid
- must not be `null` in Demo 1
- must be `null` only if `winning_bid_id = null` under a future explicit no-winner slice

### `settlement_deadline`

- must be computed exactly once at resolution
- must equal `resolution_time + settlement.deadline_seconds`
- must not be `null` in Demo 1
- must be `null` only if `winning_bid_id = null` under a future explicit no-winner slice
- must never be recomputed after persistence
- must never be extended

### `resolution_inputs_hash`

`resolution_inputs_hash` must equal lowercase hexadecimal SHA-256 of the canonical resolution input bytes.

Canonical resolution input bytes are the canonical UTF-8 JSON serialization of an array containing one object per valid `BidRecord` for the auction.

The array must be ordered by canonical `sequence_index` ascending.

Each object must contain exactly:

- `bid_id`
- `sequence_index`
- `amount_sats`
- `bidder_id`
- `destination_address`
- `validation_profile`
- `server_time`

Invalid `BidRecord` entries must not be included.

No fields outside this list can be included.

The canonical JSON serialization rules in this document govern `resolution_inputs_hash`.

## Winner Selection Rule

The winning bid is the valid `BidRecord` with the highest `amount_sats` for the auction.

If more than one valid `BidRecord` has the same highest `amount_sats`, the winning bid is the one with the lowest canonical `sequence_index`.

The tie rule is mandatory even though valid equal leading bids should not occur under bid admission increment rules.

## Rules

- exactly one `ResolutionRecord` must exist per closed auction
- resolution must be computed exactly once
- resolution must not be recomputed
- resolution must use only persisted valid `BidRecord` entries for the auction
- invalid `BidRecord` entries must not participate in resolution
- presence of `ResolutionRecord` proves auction resolution occurred
- presence of `ResolutionRecord` moves auction state to `AwaitingSettlement`

---

# 10. SettlementRecord

Represents terminal settlement determination.

## Payload Fields

- `status`
- `settlement_source`
- `confirmation_txid`
- `settlement_time`

## Field Rules

### `status`

Must be one of:

- `settled`
- `expired`

`not_required` is reserved for a future implementation slice that explicitly defines a no-winner settlement path.

### `settlement_source`

Must be one of:

- `demo_local`
- `chain_confirmed`

For Demo 1, `settlement_source` must be `demo_local`.

### `confirmation_txid`

- must be `null` when `settlement_source = demo_local`
- must be non-null when `settlement_source = chain_confirmed` and `status = settled`
- must be `null` when `status = expired`

## Rules

- exactly one `SettlementRecord` must exist per resolved auction
- must follow `ResolutionRecord`
- must record terminal settlement outcome only
- must not represent a pending settlement state
- does not create inscription authority
- does not consume inscription authority
- settlement outcome determines whether final destination is the winning destination or `NullSteward`
- Demo 1 settlement records must use `settlement_source = demo_local`

---

# 11. FinalizationRecord

Represents transition to `Finalized`.

## Payload Fields

- `finalization_time`
- `destination_address`
- `finalization_reason`

## Field Rules

### `destination_address`

- must equal the winning destination when settlement status is `settled`
- must equal `NullSteward` when settlement status is `expired`

A no-valid-bid condition does not produce `FinalizationRecord` in Demo 1 because the auction remains `Scheduled`.

### `finalization_reason`

For Demo 1, `finalization_reason` must be one of:

- `settled_to_winner`
- `expired_to_nullsteward`

`finalization_reason` must equal `settled_to_winner` when `SettlementRecord.status = settled`.

`finalization_reason` must equal `expired_to_nullsteward` when `SettlementRecord.status = expired`.

No other `finalization_reason` value is permitted in Demo 1.

## Rules

- exactly one `FinalizationRecord` must exist per auction
- must follow `SettlementRecord`
- finalization is irreversible
- destination is fixed once persisted
- sequence advancement to `N + 1` is permitted only after `FinalizationRecord`

---

# 12. InscriptionIntentRecord

Represents persisted inscription intent.

## Payload Fields

- `intent_time`
- `destination_address`
- `destination_script_pubkey`
- `inscription_payload_hash`
- `inscription_content_type`
- `adapter_mode`
- `settlement_reference`
- `intent_id`

## Field Rules

### `adapter_mode`

Must be one of:

- `deferred_in_this_slice`
- `testnet_ordinals`

### `destination_script_pubkey`

For Demo 1, when `adapter_mode = deferred_in_this_slice`, `destination_script_pubkey` must be `null`.

For live inscription modes, `destination_script_pubkey` must be derived solely from the finalized destination address according to `inscription/INSCRIPTION-MACHINE.md`.

### `settlement_reference`

For Demo 1, `settlement_reference` must equal the `record_id` of the `SettlementRecord` that caused finalization.

### `intent_id`

For Demo 1, `intent_id` must use the deterministic derivation rule defined in `inscription/INSCRIPTION-MACHINE.md`.

## Rules

- must exist before any inscription broadcast attempt
- must follow `FinalizationRecord`
- does not consume inscription authority
- does not prove broadcast
- does not prove confirmation
- must not create auction lifecycle changes

For Demo 1, `adapter_mode` must be `deferred_in_this_slice` unless live testnet inscription has been explicitly moved into Demo 1 by a later scope revision.

---

# 13. InscriptionBroadcastRecord

Represents the classified result of an inscription broadcast attempt.

## Payload Fields

- `broadcast_time`
- `candidate_txid`
- `broadcast_outcome`
- `broadcast_reason`

## Field Rules

### `broadcast_outcome`

Must be one of:

- `committed`
- `pre_commit_rejected`
- `ambiguous`

### `candidate_txid`

- must be non-null when `broadcast_outcome = committed`
- must be nullable when `broadcast_outcome = pre_commit_rejected`
- must be nullable when `broadcast_outcome = ambiguous`

## Rules

- must follow `InscriptionIntentRecord`
- must not exist when inscription adapter mode is `deferred_in_this_slice`
- `committed` corresponds to the `broadcast_commit` boundary
- inscription authority is consumed at `broadcast_commit`
- `pre_commit_rejected` does not consume inscription authority
- `ambiguous` freezes inscription authority permanently
- after `committed`, no semantically distinct inscription attempt is permitted
- after `ambiguous`, no further inscription attempt is permitted

`broadcast_commit` occurs only when:

1. the broadcast RPC succeeds
2. the authoritative node reports the transaction present in its mempool

`pre_commit_rejected` can be recorded only when the system can determine that `broadcast_commit` did not occur.

`ambiguous` must be recorded when the system cannot determine whether `broadcast_commit` occurred.

---

# 14. InscriptionConfirmationRecord

Represents observed canonical inscription confirmation.

## Payload Fields

- `confirmation_time`
- `confirmed_txid`
- `block_height`
- `block_hash`

## Rules

- must exist only after an `InscriptionBroadcastRecord` with `broadcast_outcome = committed`
- `confirmed_txid` must equal the committed `candidate_txid`
- does not consume additional authority
- must not remove or alter prior records
- terminal for inscription lifecycle

---

# 15. AmbiguityRecord

Represents detected ambiguity.

## Payload Fields

- `ambiguity_time`
- `authority_scope`
- `reason`
- `related_record_id`

## Field Rules

### `authority_scope`

Must be one of:

- `auction`
- `settlement`
- `inscription`
- `system`

## Rules

- must be written immediately upon ambiguity detection
- irreversible
- permanently freezes affected authority
- must not be removed by restart, observation, operator action, or time passing

---

# 16. PauseEventRecord

Represents system-level pause or resume.

## Payload Fields

- `event_type`
- `timestamp`
- `reason`

## Field Rules

### `event_type`

Must be one of:

- `pause`
- `resume`

## Rules

- must be persisted
- must not alter auction lifecycle state
- must not alter inscription lifecycle state
- must not restore authority
- must not recreate authority
- must not modify deadlines
- is a system control record, not a lifecycle state

## System Control Derivation

If no `PauseEventRecord` exists, system control state must derive as `Running`.

If the latest `PauseEventRecord.event_type = pause`, system control state must derive as `Paused`.

If the latest `PauseEventRecord.event_type = resume`, system control state must derive as `Running`.

---

# 17. Cross-Record Invariants

- absence of a required record must halt execution
- duplicate authority-bearing records are forbidden
- records must appear in an order consistent with `STATE-MACHINE-TABLE.md`
- lifecycle state must be derived strictly from canonical event records
- restart reconstruction must rely exclusively on records defined above
- lower-authority documents must not introduce canonical record types not defined here
- `DATA-MODEL.md` must remain consistent with `core/EVENT-TYPES.md`

---

# Final Rule

If a required canonical event record is missing, ambiguous, contradictory, or violates ordering constraints:

The system must halt rather than guess.