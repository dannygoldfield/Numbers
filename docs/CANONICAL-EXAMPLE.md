# Canonical Examples — Numbers

This document provides canonical, concrete examples of API state objects.

It is illustrative, not normative.
All examples must conform to the shapes defined in API-STATE-SHAPES.md.

Examples are intended to:
- Fix interpretation
- Reduce ambiguity
- Serve as fixtures for testing
- Serve as grounding input for LLMs

If an example conflicts with a state shape, the state shape is authoritative.

---

## 1. Auction — Scheduled

Example of an auction that is known but not yet open.

```json
{
  "auction_id": "auc_000123",
  "number": 123,
  "state": "Scheduled",
  "start_time": "2026-01-01T00:00:00Z",
  "end_time": "2026-01-01T00:01:00Z",
  "opened_at": null,
  "closed_at": null,
  "finalized_at": null,
  "current_high_bid": null,
  "bid_count": 0,
  "cap_reached": false,
  "pause_blocked": false
}
```

## 2. Auction — Open

Example of an auction actively accepting bids.

```json
{
  "auction_id": "auc_000124",
  "number": 124,
  "state": "Open",
  "start_time": "2026-01-01T00:02:00Z",
  "end_time": "2026-01-01T00:03:00Z",
  "opened_at": "2026-01-01T00:02:00Z",
  "closed_at": null,
  "finalized_at": null,
  "current_high_bid": 15000,
  "bid_count": 3,
  "cap_reached": false,
  "pause_blocked": true
}
```

## 3. Auction — Finalized (No Bids)

Example of an auction that completed without any bids.

```json
{
  "auction_id": "auc_000125",
  "number": 125,
  "state": "Finalized",
  "start_time": "2026-01-01T00:04:00Z",
  "end_time": "2026-01-01T00:05:00Z",
  "opened_at": "2026-01-01T00:04:00Z",
  "closed_at": "2026-01-01T00:05:00Z",
  "finalized_at": "2026-01-01T00:05:01Z",
  "current_high_bid": null,
  "bid_count": 0,
  "cap_reached": false,
  "pause_blocked": false
}
```

## 4. Resolution — Winning Bid Present

Example of a resolved auction with a winning bid recorded.

```json
{
  "auction_id": "auc_000124",
  "resolved_at": "2026-01-01T00:03:01Z",
  "winning_bid_id": "bid_789",
  "winning_amount": 15000
}
```

## 5. Settlement — Pending

Example of settlement awaiting completion by the winning bidder.

```json
{
  "auction_id": "auc_000124",
  "status": "pending",
  "destination": "winner",
  "settled_at": null,
  "deadline": "2026-01-01T00:10:00Z"
}
```

## 6. Settlement — Not Required

Example of settlement when no bids were placed.

```json
{
  "auction_id": "auc_000125",
  "status": "not_required",
  "destination": "null_steward",
  "settled_at": null,
  "deadline": null
}
```

## 7. Inscription — Ambiguous

Example where an inscription broadcast may have occurred, but outcome is unknown.

```json
{
  "auction_id": "auc_000124",
  "state": "ambiguous",
  "txid": null,
  "satpoint": null,
  "attempted_at": "2026-01-01T00:06:00Z",
  "observed_at": null
}
```

## 8. Inscription — Inscribed

Example where a canonical inscription is known and observed.

```json
{
  "auction_id": "auc_000124",
  "state": "inscribed",
  "txid": "abcd1234efgh5678",
  "satpoint": "abcd1234efgh5678:0:0",
  "attempted_at": "2026-01-01T00:06:00Z",
  "observed_at": "2026-01-01T00:07:30Z"
}
```

## 9. System Pause — Active

Example of the system in a paused state.

```json
{
  "system_state": "Paused",
  "paused_at": "2026-01-01T01:00:00Z",
  "pause_reason": "manual operator action"
}
```

## 10. Composite Auction View

Example of a read-only aggregation returned to clients.

```json
{
  "auction": {
    "auction_id": "auc_000124",
    "number": 124,
    "state": "Inscribed",
    "start_time": "2026-01-01T00:02:00Z",
    "end_time": "2026-01-01T00:03:00Z",
    "opened_at": "2026-01-01T00:02:00Z",
    "closed_at": "2026-01-01T00:03:00Z",
    "finalized_at": "2026-01-01T00:05:01Z",
    "current_high_bid": 15000,
    "bid_count": 3,
    "cap_reached": false,
    "pause_blocked": false
  },
  "resolution": {
    "auction_id": "auc_000124",
    "resolved_at": "2026-01-01T00:03:01Z",
    "winning_bid_id": "bid_789",
    "winning_amount": 15000
  },
  "settlement": {
    "auction_id": "auc_000124",
    "status": "settled",
    "destination": "winner",
    "settled_at": "2026-01-01T00:04:30Z",
    "deadline": "2026-01-01T00:10:00Z"
  },
  "inscription": {
    "auction_id": "auc_000124",
    "state": "inscribed",
    "txid": "abcd1234efgh5678",
    "satpoint": "abcd1234efgh5678:0:0",
    "attempted_at": "2026-01-01T00:06:00Z",
    "observed_at": "2026-01-01T00:07:30Z"
  }
}
```
