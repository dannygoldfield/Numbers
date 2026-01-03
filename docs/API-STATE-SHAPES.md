# API State Shapes — Numbers

This document defines the canonical JSON state shapes exposed by the Numbers API.

It is normative.

All API responses that represent system state must conform to these shapes.
Endpoints may differ, but shapes must not.

If a field is not defined here, it must not appear in API output.

If there is a conflict, STATE-MACHINE.md and DATA-MODEL.md take precedence.

---

## 1. Design Principles

1. Shapes represent what is known, not what is inferred
2. Absence is explicit, never implied
3. Uncertainty is preserved, not hidden
4. Time passing does not change meaning
5. Shapes are append-compatible but not mutable

---

## 2. Common Conventions

### 2.1 Field Conventions

All timestamps are ISO 8601 UTC strings.  
All numeric values are base-10.  
All identifiers are opaque strings.  
All optional fields are nullable, not omitted.

### 2.2 State Fields

All state objects include a `state` field.  
The value must correspond exactly to a state defined in STATE-MACHINE.md.

No derived or synthesized states are permitted.

---

## 3. Auction State Object

Represents the canonical state of a single auction.

```json
{
  "auction_id": "string",
  "number": 12345,
  "state": "Scheduled | Open | Closed | AwaitingSettlement | Finalized | Inscribing | Inscribed",
  "start_time": "ISO-8601 or null",
  "end_time": "ISO-8601 or null",
  "opened_at": "ISO-8601 or null",
  "closed_at": "ISO-8601 or null",
  "finalized_at": "ISO-8601 or null",
  "current_high_bid": "integer or null",
  "bid_count": "integer",
  "cap_reached": "boolean",
  "pause_blocked": "boolean"
}
```

## 4. Bid State Object

```json
{
  "bid_id": "string",
  "auction_id": "string",
  "amount": "integer",
  "timestamp": "ISO-8601",
  "status": "accepted | rejected"
}
```

## 5. Resolution State Object

```json
{
  "auction_id": "string",
  "resolved_at": "ISO-8601",
  "winning_bid_id": "string or null",
  "winning_amount": "integer or null"
}
```

## 6. Settlement State Object

Represents settlement progress after auction resolution.

```json
{
  "auction_id": "string",
  "status": "pending | settled | expired | not_required",
  "destination": "winner | null_steward",
  "settled_at": "ISO-8601 or null",
  "deadline": "ISO-8601 or null"
}
```

## 7. Inscription State Object

Represents inscription knowledge, not guarantees.

```json
{
  "auction_id": "string",
  "state": "none | attempting | ambiguous | inscribed",
  "txid": "string or null",
  "satpoint": "string or null",
  "attempted_at": "ISO-8601 or null",
  "observed_at": "ISO-8601 or null"
}
```

## 8. Pause State Object

Represents system-level pause state.

```json
{
  "system_state": "Running | Paused",
  "paused_at": "ISO-8601 or null",
  "pause_reason": "string or null"
}
```

## 9. Composite Auction View

A read-only aggregation for clients.

```json
{
  "auction": { "Auction State Object" },
  "resolution": { "Resolution State Object" },
  "settlement": { "Settlement State Object" },
  "inscription": { "Inscription State Object" }
}
```

## 10. Forbidden Representations

The API must not expose:

- Ownership claims
- Valuation or ranking
- Probabilistic outcomes
- Time-based inference
- Automatic certainty after delay

## 11. Backward Compatibility Rules

- New fields must be nullable
- Field meanings must never change
- Existing states must never be redefined
- New states require changes to STATE-MACHINE.md first


## 12. Final Rule

If the system does not know something with certainty,
it must represent that lack of knowledge explicitly.
