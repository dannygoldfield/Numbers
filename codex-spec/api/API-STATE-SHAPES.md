# API State Shapes: Numbers

This document defines the canonical JSON shapes exposed by the Numbers API.

It is normative.

Authority precedence is defined exclusively in `AUTHORITY-ORDER.md`.

This document assumes familiarity with:

- `api/API-SPEC.md`
- `core/STATE-MACHINE-TABLE.md`
- `core/STATE-MACHINE.md`
- `core/INVARIANTS.md`
- `core/EVENT-TYPES.md`
- `data/DATA-MODEL.md`
- `data/PERSISTENCE.md`
- `data/RESTART-RULES.md`

All API responses that represent system state must conform exactly to these shapes.

If a field is not defined here, it must not appear in API output.

If there is a conflict, higher-authority documents listed in `AUTHORITY-ORDER.md` prevail.

---

# 1. Design Principles

API state shapes:

- represent canonical event record truth
- represent mechanically derived state
- preserve uncertainty
- expose absence as `null`
- must not infer missing facts
- must not create new lifecycle semantics
- must not expose undocumented fields

The API reflects what is recorded or mechanically derived from what is recorded.

---

# 2. Global Shape Rules

All API state responses must obey:

- all fields defined in a shape must be present
- unknown, unavailable, or not-yet-applicable values must be represented as `null`
- optionality is represented by `null`, never omission
- no additional fields are permitted
- field meanings must not change within the API version
- enum values are closed sets
- derived states not defined in `core/STATE-MACHINE-TABLE.md` are forbidden

Violation of these rules is fatal.

---

# 3. Common Conventions

## Demo 1 Error Codes

For Demo 1, API `error_code` must be one of:

- `malformed_request`
- `invalid_json`
- `unknown_endpoint`
- `method_not_allowed`
- `bid_precondition_failed`
- `bid_admission_failed`
- `settlement_precondition_failed`
- `invalid_settlement_outcome`
- `invalid_pagination`
- `storage_unavailable`
- `fatal_reconstruction_error`

No other `error_code` value is permitted for Demo 1.

## Primitive Types

- timestamps must be ISO 8601 UTC strings or `null`
- numeric values must be base-10 integers or `null`
- identifiers must be opaque strings or `null`
- boolean values must be `true`, `false`, or `null`
- enums must use the exact values defined in this document

## Auction State Enum

`auction_state` must be one of:

- `Scheduled`
- `Open`
- `Closed`
- `AwaitingSettlement`
- `Finalized`

## System Control State Enum

`system_control_state` must be one of:

- `Running`
- `Paused`

## Inscription State Enum

`inscription_state` must be one of:

- `NotStarted`
- `Inscribing`
- `Ambiguous`
- `Inscribed`

## Settlement Status Enum

`settlement_status` must be one of:

- `settled`
- `expired`
- `null`

`null` means no `SettlementRecord` exists.

`not_required` is reserved for a future implementation slice that explicitly defines a no-winner settlement path.

## Settlement Source Enum

`settlement_source` must be one of:

- `demo_local`
- `chain_confirmed`
- `null`

`null` means no `SettlementRecord` exists.

## Inscription Adapter Mode Enum

`inscription_adapter_mode` must be one of:

- `deferred_in_this_slice`
- `testnet_ordinals`
- `null`

`null` means no `InscriptionIntentRecord` exists.

---

# 4. Timestamp and Derived Time Rules

The following timestamp fields represent canonical history:

- `opened_at`
- `base_end_time`
- `current_end_time`
- `closed_at`
- `resolution_time`
- `settlement_deadline`
- `finalized_at`

## `opened_at`

Rules:

- must be `null` while auction state is `Scheduled`
- must be set exactly once by `AuctionOpenRecord`
- must never be overwritten
- must never be recalculated as stored truth

## `base_end_time`

Rules:

- must be `null` while auction state is `Scheduled`
- must be set exactly once by `AuctionOpenRecord`
- must equal `opened_at + auction.duration_seconds`
- must never be overwritten
- must never be recalculated as stored truth

## `current_end_time`

Rules:

- must be `null` while auction state is `Scheduled`
- must be mechanically derived from canonical event records
- must not be stored as mutable truth

Derived formula:

```text
current_end_time =
base_end_time +
(extension_increment_seconds * number_of_extension_events)
```

## `closed_at`

Rules:

- must be `null` while auction state is `Scheduled` or `Open`
- must be set exactly once by `AuctionCloseRecord`
- must never be overwritten
- must never be recalculated as stored truth

## `resolution_time`

Rules:

- must be `null` until `ResolutionRecord` exists
- must be set exactly once by `ResolutionRecord`
- must never be overwritten

## `settlement_deadline`

Rules:

- must be `null` if no settlement deadline exists
- must be derived from settlement rules and canonical event records
- must not be modified after the governing settlement boundary is persisted

## `finalized_at`

Rules:

- must be `null` until `FinalizationRecord` exists
- must be set exactly once by `FinalizationRecord`
- must never be overwritten

If any required timestamp is missing after its corresponding canonical event record exists, execution must halt.

---

# 5. `GET /state` Shape

`GET /state` returns the current global system state.

## Shape

```json
{
  "server_time": "ISO-8601",
  "current_number": "integer",
  "auction_state": "Scheduled | Open | Closed | AwaitingSettlement | Finalized",
  "system_control_state": "Running | Paused",

  "opened_at": "ISO-8601 or null",
  "base_end_time": "ISO-8601 or null",
  "number_of_extension_events": "integer",
  "current_end_time": "ISO-8601 or null",
  "closed_at": "ISO-8601 or null",
  "resolution_time": "ISO-8601 or null",
  "settlement_status": "settled | expired | null",
  "settlement_source": "demo_local | chain_confirmed | null",
  "settlement_deadline": "ISO-8601 or null",
  "finalized_at": "ISO-8601 or null",
  "final_destination": "string or null",

  "current_high_bid": "BidSummaryObject or null",
  "bid_count": "integer",

  "inscription_state": "NotStarted | Inscribing | Ambiguous | Inscribed",
  "inscription_adapter_mode": "deferred_in_this_slice | testnet_ordinals | null",
  "inscription_txid": "string or null",

  "last_record_sequence_index": "integer or null"
}
```

## Field Rules

- `server_time` must equal authoritative server time at response generation.
- `current_number` must identify the current auction number.
- `auction_state` must be reconstructed from canonical event records.
- `system_control_state` must be reconstructed from `PauseEventRecord` entries. If no `PauseEventRecord` exists, `system_control_state` must be `Running`.
- `number_of_extension_events` must equal the count of `ExtensionEventRecord` entries for the current auction.
- `current_high_bid` must be derived only from valid `BidRecord` entries using the winner-selection ordering rule.
- `bid_count` must equal the count of `BidRecord` entries for the current auction, including invalid bids.
- `settlement_deadline` must be derived from `ResolutionRecord` when present and must be `null` before resolution.
- `inscription_state` must be reconstructed from inscription canonical event records.
- `last_record_sequence_index` must equal the highest persisted canonical event record sequence index.

The backend must not expose speculative time-remaining projections.

The frontend can compute countdown display from `current_end_time`.

---

# 6. `GET /auction/history` Shape

`GET /auction/history` returns canonical event-derived auction history for Demo 1 browser inspection.

It is not limited to finalized auction outcome summaries.

## Shape

```json
{
  "entries": [
    {
      "number": "integer",
      "auction_id": "string",
      "auction_state": "Scheduled | Open | Closed | AwaitingSettlement | Finalized",
      "current_high_bid": "BidSummaryObject or null",
      "bid_count": "integer",
      "resolution": "ResolutionObject or null",
      "settlement": "SettlementObject or null",
      "finalized_at": "ISO-8601 or null",
      "final_destination": "string or null",
      "inscription": "InscriptionObject",
      "records": [
        "CanonicalEventRecordSummaryObject"
      ]
    }
  ],
  "pagination": {
    "limit": "integer",
    "offset": "integer",
    "next_offset": "integer or null"
  }
}
```

## Field Rules

- each entry must correspond to exactly one auction number with an `AuctionRecord`
- entries must be ordered by canonical number ascending
- non-finalized auctions can appear
- `auction_state` must be reconstructed from canonical event records
- `current_high_bid` must be derived from valid `BidRecord` entries using the winner-selection ordering rule
- `bid_count` must include valid and invalid `BidRecord` entries
- `resolution` must be `null` until `ResolutionRecord` exists
- `settlement` must be `null` until `SettlementRecord` exists
- `finalized_at` must be `null` until `FinalizationRecord` exists
- `final_destination` must expose `NullSteward` when `NullSteward` is the persisted final destination
- `inscription` must expose deferred inscription state when `InscriptionIntentRecord` exists
- `records` must include ordered canonical event record summaries for the auction
- `records` must include invalid `BidRecord` entries that reached admission evaluation
- history must not be reconstructed from mutable lifecycle state
- pagination `limit` must default to `50` when omitted
- pagination `limit` must be at least `1` and at most `100`
- pagination `offset` must default to `0` when omitted
- pagination `offset` must be greater than or equal to `0`
- invalid pagination parameters must produce an API error with `error_code = invalid_pagination`

---

## 6.1 Canonical Event Record Summary Object

`CanonicalEventRecordSummaryObject` represents one persisted canonical event record for API inspection.

## Shape

```json
{
  "record_id": "string",
  "record_type": "string",
  "auction_id": "string",
  "number": "integer",
  "sequence_index": "integer",
  "server_time": "ISO-8601",
  "payload_hash": "string",
  "payload_json": "object"
}
```

## Field Rules

- values must come from the canonical event record envelope
- `sequence_index` must be the canonical append order
- `payload_hash` must equal the persisted canonical payload hash
- `payload_json` must equal the persisted canonical payload object for the record
- this object must not infer, summarize, or reinterpret record meaning

---

# 7. Reserved Shape: `GET /auction/{N}`

`GET /auction/{N}` is reserved for a later implementation slice.

Demo 1 must not expose this shape.

Requests to `GET /auction/{N}` in Demo 1 must return the API error envelope with `error_code = unknown_endpoint`.

---

# 8. Bid Summary Object

`BidSummaryObject` represents the current high valid bid.

## Shape

```json
{
  "bid_id": "string",
  "amount_sats": "integer",
  "bidder_id": "string",
  "bidder_address": "string or null",
  "destination_address": "string",
  "validation_profile": "demo_local | cryptographic",
  "server_time": "ISO-8601"
}
```

## Field Rules

- must be derived only from valid `BidRecord` entries
- must use highest `amount_sats`, with lowest canonical `sequence_index` as tie-break
- must be `null` when no valid bid exists
- must not include invalid bids
- must not imply winning before `ResolutionRecord`

---

# 9. Bid State Object

`BidStateObject` represents a single bid submission attempt as evaluated at authoritative server receipt time.

## Shape

```json
{
  "bid_id": "string",
  "auction_id": "string",
  "amount_sats": "integer",
  "bidder_id": "string",
  "bidder_address": "string or null",
  "destination_address": "string",
  "validation_profile": "demo_local | cryptographic",
  "server_time": "ISO-8601",
  "validity": "valid | invalid",
  "rejection_reason": "string or null"
}
```

## Field Rules

- `server_time` must be set exactly once
- `server_time` must never change
- `server_time` must equal authoritative server receipt time
- `validity` must never change after persistence
- `rejection_reason` must be `null` when `validity = valid`
- `rejection_reason` must be non-null when `validity = invalid`

---

# 10. `POST /bid` Response Shape

`POST /bid` returns the deterministic result of bid admission.

## Shape

```json
{
  "accepted": "boolean",
  "bid": "BidStateObject",
  "auction_opened": "boolean",
  "auction_state": "Scheduled | Open | Closed | AwaitingSettlement | Finalized",
  "current_high_bid": "BidSummaryObject or null",
  "current_end_time": "ISO-8601 or null",
  "server_time": "ISO-8601"
}
```

## Field Rules

- `accepted` must be `true` when `bid.validity = valid`
- `accepted` must be `false` when `bid.validity = invalid`
- `auction_opened` must be `true` only when the bid atomically persists `AuctionOpenRecord`
- `auction_opened` must be `false` otherwise
- `auction_state` must be derived after bid admission persistence completes
- `current_high_bid` must be derived after bid admission persistence completes
- `server_time` must equal authoritative server response time


---


# 11. `POST /demo/settlement` Response Shape

`POST /demo/settlement` returns the deterministic result of Demo 1 local settlement control.

## Shape

```json
{
  "auction_id": "string",
  "outcome": "settled | expired",
  "settlement": "SettlementObject",
  "auction_state": "Finalized",
  "finalized_at": "ISO-8601",
  "final_destination": "string",
  "inscription_intent_record_id": "string",
  "server_time": "ISO-8601"
}
```

## Field Rules

- this response is returned only after successful settlement, finalization, and deferred inscription intent persistence
- `outcome` must equal the requested accepted settlement outcome
- `settlement` must be derived from the persisted `SettlementRecord`
- `auction_state` must be `Finalized`
- `finalized_at` must equal `FinalizationRecord.finalization_time`
- `final_destination` must equal the persisted final destination
- `inscription_intent_record_id` must reference the required deferred `InscriptionIntentRecord` persisted for the finalized auction
- `server_time` must equal authoritative server response time
- failed preconditions must use the error envelope, not this success shape

---

# 12. Resolution Object

`ResolutionObject` represents auction resolution.

## Shape

```json
{
  "auction_id": "string",
  "resolution_time": "ISO-8601",
  "winning_bid_id": "string or null",
  "winning_amount_sats": "integer or null"
}
```

## Field Rules

- `resolution_time` must be set exactly once
- `resolution_time` must never change
- if `winning_bid_id` is `null`, `winning_amount_sats` must be `null`
- if `winning_bid_id` is non-null, `winning_amount_sats` must be non-null
- in Demo 1, `winning_bid_id` and `winning_amount_sats` must be non-null after resolution
- winner selection must use highest valid `amount_sats`, with lowest canonical `sequence_index` as tie-break

---

# 13. Settlement Object

`SettlementObject` represents settlement determination.

## Shape

```json
{
  "auction_id": "string",
  "settlement_status": "settled | expired",
  "settlement_source": "demo_local | chain_confirmed",
  "confirmation_txid": "string or null",
  "settlement_deadline": "ISO-8601 or null",
  "settlement_time": "ISO-8601"
}
```

## Field Rules

- this object must exist only when `SettlementRecord` exists
- `settlement_status` must match `SettlementRecord.status`
- `settlement_source` must match `SettlementRecord.settlement_source`
- `confirmation_txid` must match `SettlementRecord.confirmation_txid`
- `confirmation_txid` must be `null` when `settlement_source = demo_local`
- `confirmation_txid` must be non-null when `settlement_source = chain_confirmed` and `settlement_status = settled`
- `settlement_time` must be set exactly once

---

# 14. Inscription Object

`InscriptionObject` represents inscription knowledge only.

## Shape

```json
{
  "auction_id": "string",
  "inscription_state": "NotStarted | Inscribing | Ambiguous | Inscribed",
  "inscription_adapter_mode": "deferred_in_this_slice | testnet_ordinals | null",
  "inscription_txid": "string or null",
  "inscription_satpoint": "string or null",
  "intent_time": "ISO-8601 or null",
  "broadcast_time": "ISO-8601 or null",
  "confirmation_time": "ISO-8601 or null"
}
```

## Field Rules

- `InscriptionIntentRecord` alone does not move inscription state out of `NotStarted`
- if `inscription_adapter_mode = deferred_in_this_slice`, then `broadcast_time` must be `null`
- if `inscription_adapter_mode = deferred_in_this_slice`, then `confirmation_time` must be `null`
- if `inscription_adapter_mode = deferred_in_this_slice`, then `inscription_txid` must be `null`
- if inscription state is `Ambiguous`, the shape must not imply that ambiguity will resolve
- terminal inscription states must not change

---

# 15. Ambiguity Object

`AmbiguityObject` represents recorded ambiguity.

## Shape

```json
{
  "authority_scope": "auction | settlement | inscription | system",
  "reason": "string",
  "related_record_id": "string or null",
  "ambiguity_time": "ISO-8601"
}
```

## Field Rules

- must be derived from `AmbiguityRecord`
- must not be inferred from missing data alone
- must not imply repair or future resolution

---

# 16. Pause State Object

`PauseStateObject` represents system-level pause knowledge.

## Shape

```json
{
  "system_control_state": "Running | Paused",
  "paused_at": "ISO-8601 or null",
  "pause_reason": "string or null"
}
```

## Field Rules

- if `system_control_state = Paused`, then `paused_at` must be non-null
- if `system_control_state = Running`, then `paused_at` must be `null`
- pause does not alter auction lifecycle state
- pause does not alter inscription lifecycle state
- pause does not restore authority
- when paused, the system must reject bid acceptance

---

# 17. Composite Auction View

Composite auction view is reserved for a later implementation slice.

Demo 1 must use `GET /state` and `GET /auction/history` for frontend-visible state and history.

---

# 18. Forbidden Representations

The API must not expose:

- ownership claims
- valuation
- ranking
- probabilistic outcomes
- speculative time remaining
- eventual certainty
- derived states not defined in `core/STATE-MACHINE-TABLE.md`
- fields not defined in this document

---

# 19. Backward Compatibility Rules

The API version for this prototype is `v1`.

Rules:

- new fields require a new API version unless already declared nullable
- existing field meanings must never change within `v1`
- existing enum values must never be redefined
- new states require changes to `core/STATE-MACHINE-TABLE.md` first
- removing fields requires a new API version

---

# Final Rule

If the system does not know something with certainty:

It must represent that lack of knowledge explicitly as `null`.

If a field is defined by a shape:

It must be present.