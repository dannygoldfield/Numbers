# API State Shapes — Numbers

This document defines the canonical JSON shapes exposed by the Numbers API.

It is normative.

This document assumes familiarity with:
- API-SPEC.md
- STATE-MACHINE.md
- INVARIANTS.md
- PERSISTENCE.md
- RESTART-RULES.md

All API responses that represent system state must conform exactly to these shapes.

If a field is not defined here, it must not appear in API output.

If there is a conflict,
CORE-SEQUENCE.md, STATE-MACHINE-TABLE.md,
STATE-MACHINE.md, and INVARIANTS.md take precedence.

---

## 1. Design Principles

1. Shapes represent recorded knowledge only.
2. Absence is explicit and represented as `null`.
3. Uncertainty is preserved, not resolved.
4. Time passing does not change meaning.
5. Shapes are append-compatible and never reinterpretive.

---

## 2. Global Shape Rules (Normative)

- All fields defined in a shape must be present.
- Optionality is represented by `null`, never omission.
- No additional fields are permitted.
- Field meanings must not change across versions.

Violation of these rules is fatal.

---

## 3. Common Conventions

### 3.1 Primitive Types

- All timestamps are ISO 8601 UTC strings.
- All numeric values are base-10 integers.
- All identifiers are opaque strings.
- All enums are closed sets.

### 3.2 State Fields

All state objects include a `state` field.

The value must exactly match a state defined in STATE-MACHINE.md.

Derived, synthesized, or alias states are forbidden.

---

## 4. Auction State Object

Represents the canonical lifecycle state of a single auction.

### Auction Timestamp Invariants (Normative)

The following timestamp fields are authoritative history:

- `opened_at`
  - Null while state = `Scheduled`
  - Must be set exactly once on transition `Scheduled → Open`
  - Must be persisted as part of that transition
  - Must never be overwritten or recalculated

- `base_end_time`
  - Null while state = `Scheduled`
  - Must be set exactly once on transition `Scheduled → Open`
  - Must be computed as `opened_at + auction.duration_seconds`
  - Must be persisted as part of that transition
  - Must never be overwritten or recalculated

- `current_end_time`
  - Null while state = `Scheduled`
  - Must equal:
```text
    current_end_time =
    base_end_time +
    (auction.extension_increment_seconds * number_of_extension_events)

  - Must be derived from persisted records
  - Must not be stored as mutable truth

- `closed_at`
  - Null while state ∈ {`Scheduled`, `Open`}
  - Must be set exactly once on transition `Open → Closed`
  - Must be persisted as part of that transition
  - Must never be overwritten or recalculated

- `finalized_at`
  - Null while state ∈ {`Scheduled`, `Open`, `Closed`, `AwaitingSettlement`}
  - Must be set exactly once on transition `AwaitingSettlement → Finalized`
  - Must be persisted as part of that transition
  - Must never be overwritten or recalculated

If any of these fields are:
- missing after the corresponding transition is persisted
- modified after being set
- recomputed on restart

execution must halt.

### Auction State Shape

```json
{
  "auction_id": "string",
  "number": "integer",
  "state": "Scheduled | Open | Closed | AwaitingSettlement | Finalized",

  "opened_at": "ISO-8601 or null",
  "base_end_time": "ISO-8601 or null",
  "current_end_time": "ISO-8601 or null",
  "closed_at": "ISO-8601 or null",
  "finalized_at": "ISO-8601 or null",

  "number_of_extension_events": "integer",

  "current_high_bid_sats": "integer or null",
  "bid_count": "integer",
  "cap_reached": "boolean"
}
```
### Field Semantics (Normative)

- current_high_bid_sats must be null when no accepted valid bids exist.
- bid_count must equal the total number of Bid State Objects for this auction, including those with validity = invalid.
- number_of_extension_events must equal the count of ExtensionEventRecords for this auction.

## 5. Bid State Object

Represents a single bid submission attempt as evaluated at time of submission.

### Bid Timestamp Invariants (Normative)

- server_time must be set exactly once.
- server_time must never change.
- server_time must equal authoritative server acceptance time.

### Bid State Shape

```json
{
  "bid_id": "string",
  "auction_id": "string",
  "amount_sats": "integer",
  "server_time": "ISO-8601",
  "validity": "valid | invalid"
}
```
## 6. Resolution State Object

Represents the outcome of auction resolution.

### Resolution Immutability (Normative)

- resolved_at must be set exactly once and must never change.

- winning_bid_id and winning_amount_sats must be set exactly once.

- If winning_bid_id is null, winning_amount_sats must be null.

- If winning_bid_id is non-null, winning_amount_sats must be non-null.

### Resolution State Shape

```JSON
{
  "auction_id": "string",
  "resolved_at": "ISO-8601",
  "winning_bid_id": "string or null",
  "winning_amount_sats": "integer or null"
}
```
## 7. Settlement State Object

Represents settlement knowledge after resolution.

### Settlement Immutability and Monotonicity (Normative)

- deadline must be set exactly once at resolution if settlement is applicable.
- deadline must never change.
- finalized_at must be set exactly once when settlement becomes terminal.
- Once status is terminal, it must never change.

Constraints:

- If status = not_required, then deadline must be null and finalized_at must be non-null.
- If status = pending, then finalized_at must be null.

## Settlement State Shape
``` JSON
{
  "auction_id": "string",
  "status": "pending | settled | expired | not_required",
  "finalized_at": "ISO-8601 or null",
  "deadline": "ISO-8601 or null"
}
```
## 8. Inscription State Object

Represents inscription knowledge only.

### Inscription Immutability and Authority Safety (Normative)

- attempted_at must be set exactly once when an inscription attempt begins.
- attempted_at must never change.
- If state = NotStarted, then attempted_at and txid and observed_at must be null.
- If txid is non-null, it must never change.
- If state is terminal, it must never change.

## Inscription State Shape
``` JSON
{
  "auction_id": "string",
  "state": "NotStarted | Inscribing | Ambiguous | Inscribed",
  "txid": "string or null",
  "attempted_at": "ISO-8601 or null",
  "observed_at": "ISO-8601 or null"
}
```
- Ambiguous is terminal.
- This shape must not imply that ambiguity will resolve.

## 9. Pause State Object

Represents system-level pause knowledge.

### Pause Immutability (Normative)

- If system_state = Paused, then paused_at must be non-null.
- If system_state = Running, then paused_at must be null.
- paused_at must never be overwritten.
- pause_reason is nullable; if non-null for a given pause event, it must not change for that pause event.

### Pause State Shape

```
{
  "system_state": "Running | Paused",
  "paused_at": "ISO-8601 or null",
  "pause_reason": "string or null"
}
```
When paused, the system must reject bid acceptance.
Auction lifecycle truth does not change solely due to pause.

## 10. Composite Auction View

A read-only aggregation for clients.
This is a packaging convenience only. It introduces no new semantics.
```
{
  "auction": { "Auction State Object" },
  "resolution": { "Resolution State Object" },
  "settlement": { "Settlement State Object" },
  "inscription": { "Inscription State Object" }
}
```
## 11. Forbidden Representations

The API must not expose:
- ownership claims
- valuation or ranking
- probabilistic outcomes
- time-based inference
- eventual certainty
- derived states not defined in STATE-MACHINE.md

## 12. Backward Compatibility Rules

- New fields must be nullable.
- Existing field meanings must never change.
- Existing enum values must never be redefined.
- New states require changes to STATE-MACHINE.md first.
- Removing fields requires a new API version.

## 13. Final Rule

If the system does not know something with certainty,
it must represent that lack of knowledge explicitly.
