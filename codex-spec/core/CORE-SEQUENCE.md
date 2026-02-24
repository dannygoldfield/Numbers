# Core Sequence — Numbers

This document defines the deterministic execution sequence of Numbers.

It is normative.

It describes what must occur, in what order, and without exception.

This document does not define:
- UI behavior
- authority semantics
- persistence mechanics
- error taxonomy

Those are defined elsewhere.

---

## 1. Global Sequence Invariants

Numbers advances through a monotonically increasing sequence of numbers.

For each number `N`, the system executes the auction lifecycle exactly once and strictly in order.

The sequence:

- never rewinds
- never skips a number
- never overlaps two active auctions
- never auto-advances based solely on time

If an auction remains in `Scheduled`,  
the sequence remains at `N`.

Sequence advancement is event-driven, not time-driven.

---

## 2. Auction Initialization

For each number `N`:

- An auction record exists in state `Scheduled`.
- The auction does not open automatically.
- The auction opens only when the first valid bid is accepted.
- No bid is accepted unless system state = `Running`.

If no valid bid is ever accepted,  
the auction remains `Scheduled`.

---

## 3. Auction Opening

When the first valid bid is accepted while state = `Scheduled`:

The following must occur atomically:

- Persist `BidRecord`
- Persist `AuctionOpenRecord`
- Set `opened_at = server_time`
- Set `base_end_time = opened_at + auction.duration_seconds`
- Persist Scheduled → Open transition

The auction enters state `Open`.

`server_time` is authoritative.  
Client timestamps are ignored.

---

## 4. Open Auction Timing

While state = `Open`:

Configuration parameters:

- `auction.duration_seconds = 45296`  (12:34:56)
- `extension_window_seconds = 83`     (1:23)
- `extension_increment_seconds = 83`  (1:23)
- `max_extensions = 3`

Definitions:

- `base_end_time` is persisted in `AuctionOpenRecord`
- `number_of_extension_events` equals the count of `ExtensionEventRecord` entries
- `current_end_time` is defined as:

```Text
current_end_time =
base_end_time +
(extension_increment_seconds * number_of_extension_events)
```

All time evaluation must use authoritative `server_time`.

---

## 5. Extension Rule

If a valid bid is accepted and:

- `server_time >= current_end_time - extension_window_seconds`
- `number_of_extension_events < max_extensions`

Then atomically:

- Persist `ExtensionEventRecord`

No prior record may be modified.

If `number_of_extension_events == max_extensions`:

- No further extensions are permitted.

Extension:

- does not create a new lifecycle state
- does not consume authority
- does not modify prior records

---

## 6. Auction Closing

The auction transitions `Open → Closed` when:

- `server_time >= current_end_time`, or
- bid cap reached

At closing:

- No further bids are accepted.
- Close timestamp must be persisted.

Closing is automatic and deterministic.

---

## 7. Resolution

In state `Closed`:

- The winning bid is determined deterministically.
- Exactly one `ResolutionRecord` must be persisted.
- Resolution must not be recomputed.

After resolution is persisted:

- State transitions to `AwaitingSettlement`.

---

## 8. Settlement

While state = `AwaitingSettlement`:

Exactly one of the following occurs:

1. Settlement confirmed before deadline  
   → Destination = winning address

2. Settlement deadline expires  
   → Destination = `NullSteward`

3. No valid bids (status = not_required)  
   → Destination = `NullSteward`

Settlement outcome must be persisted exactly once.

Settlement does not create inscription authority.

After settlement outcome is determined:

- State transitions to `Finalized`.

---

## 9. Finalization

In state `Finalized`:

- Destination is fixed.
- `FinalizationRecord` must be persisted exactly once.
- Auction lifecycle is complete.
- No further auction transitions are permitted.

Sequence advancement to `N+1` is permitted only after `Finalized`.

---

## 10. Inscription Sequence

After `Finalized`:

If inscription is initiated:

- Transition `NotStarted → Inscribing`
- Persist `InscriptionRecord`
- Inscription authority is consumed

While `Inscribing`:

- Transaction may be constructed
- Broadcast may be attempted
- Network may be observed

If inscription is observed:

- Transition `Inscribing → Inscribed`

If outcome cannot be determined with certainty:

- Transition `Inscribing → Ambiguous`

`Inscribed` and `Ambiguous` are terminal.

Ambiguity permanently consumes inscription authority.

---

## 11. Restart Behavior

On restart:

- State must be reconstructed exclusively from persisted records.
- Missing required records must halt execution.
- Authority must not be consumed during reconstruction.
- No transition may be inferred from absence of data.

Restart may cause deterministic evaluation of:

- `Open → Closed` if `server_time >= current_end_time`
- `AwaitingSettlement → Finalized` if deadline expired

No other automatic transition is permitted.

---

## Final Rule

If a step is not explicitly defined in this sequence:

It is forbidden.