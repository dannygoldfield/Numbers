# Test Vectors — Numbers

This document defines canonical test vectors for Numbers.

It is illustrative but binding.

Each vector represents a concrete scenario that must be handled exactly as described.
Implementations must produce the same state transitions, outputs, and prohibitions.

If a test vector conflicts with:
- STATE-MACHINE.md
- API-STATE-SHAPES.md
- ERROR-TAXONOMY.md

Those documents take precedence.

---

## 1. Purpose

Test vectors exist to:

- Lock system behavior
- Prevent regression
- Prevent interpretation drift
- Serve as fixtures for automated testing
- Serve as grounding examples for humans and LLMs

A test vector is not a suggestion.
It defines a required outcome.

---

## 2. Structure of a Test Vector

Each test vector must include the following sections:

- Initial State  
- Triggering Action  
- Observed Events (if any)  
- Expected Resulting State  
- Error Classification (if applicable)  
- Forbidden Actions  

Omitting any section is not permitted.

---

## 3. Vector — Bid Below Minimum

### Initial State

Auction is open and has an existing high bid.

Auction is open.

```json
{
  "auction_id": "auc_001",
  "state": "Open",
  "current_high_bid": 10000,
  "bid_count": 1
}
```

### Triggering Action

Describe the bid submission.

```json
{
  "bid_id": "bid_001",
  "auction_id": "auc_001",
  "amount": 9000
}
```

### Observed Events

Describe any internal or external observations.
- Bid validation runs
- Bid is rejected before persistence

### Expected Resulting State

Describe the state after handling the bid. Auction state is unchanged.

```json
{
  "auction_id": "auc_001",
  "state": "Open",
  "current_high_bid": 10000,
  "bid_count": 1
}
```

### Error Classification

Specify the error class from ERROR-TAXONOMY.md. Deterministic Error.

### Forbidden Actions

List actions the system must not take.
- Persisting the bid
- Updating bid count
- Updating current high bid
- Advancing auction state

---

## 4. Vector — Auction Closes Without Bids

### Initial State

Auction is open with zero bids.

```json
{
  "auction_id": "auc_002",
  "state": "Open",
  "bid_count": 0
}
```

### Triggering Action

Auction end time is reached.

### Observed Events

Clock advancement or scheduler trigger.
- Scheduler observes end_time

### Expected Resulting State

Auction finalizes with no settlement required.

```json
{
  "auction_id": "auc_002",
  "state": "Finalized",
  "bid_count": 0
}
```

### Forbidden Actions

List all prohibited behaviors.
- Creating a resolution with a winner
- Entering settlement
- Attempting inscription

---

## 5. Vector — Winning Bid, Settlement Pending

### Initial State

Auction has closed with at least one valid bid.

```json
{
  "auction_id": "auc_003",
  "state": "Closed",
  "current_high_bid": 15000,
  "bid_count": 3
}
```

### Triggering Action

Resolution process executes.

### Observed Events

Resolution record is written

### Expected Resulting State

Settlement enters pending state.

```json
{
  "auction_id": "auc_003",
  "settlement_status": "pending"
}
```

### Forbidden Actions

List prohibited actions prior to settlement completion.
- Inscription attempt before settlement
- Accepting new bids
- Reopening auction

---

## 6. Vector — Settlement Deadline Expires

### Initial State

Settlement pending with deadline set.

```json
{
  "auction_id": "auc_003",
  "settlement_status": "pending",
  "deadline": "2026-01-01T00:10:00Z"
}
```

### Triggering Action

Deadline passes without payment.

### Observed Events

Time observation only.

### Expected Resulting State

Settlement expires and destination becomes null steward.

```json
{
  "auction_id": "auc_003",
  "settlement_status": "expired",
  "destination": "null_steward"
}
```

### Forbidden Actions

- Accepting late payment
- Reopening settlement
- Assigning ownership

---

## 7. Vector — Inscription Broadcast Ambiguity

### Initial State

Settlement completed. No inscription attempt yet.

```json
{
  "auction_id": "auc_004",
  "inscription_state": "none"
}
```

### Triggering Action

Inscription broadcast is attempted.

### Observed Events

- Process crashes immediately after broadcast attempt
- No txid is persisted

### Expected Resulting State

Inscription state becomes ambiguous.

```json
{
  "auction_id": "auc_004",
  "inscription_state": "ambiguous"
}
```

### Error Classification

Ambiguous Error.

### Forbidden Actions

- Retrying broadcast
- Broadcasting a replacement transaction
- Marking inscription as failed

---

## 8. Vector — Observed Inscription After Ambiguity

### Initial State

Inscription state is ambiguous.

```json
{
  "auction_id": "auc_004",
  "inscription_state": "ambiguous"
}
```

### Triggering Action

Network observation detects confirmed inscription.

### Observed Events

External observation confirms inscription.

### Expected Resulting State

Inscription state updates to inscribed.

```json
{
  "auction_id": "auc_004",
  "inscription_state": "inscribed"
}
```

### Forbidden Actions

Any action implying ambiguity was resolved by retry.
- Claiming retry success
- Performing any additional broadcast
- Rewriting history

---

## 9. Vector — Invalid State Transition

### Initial State

Auction is finalized.

```json
{
  "auction_id": "auc_005",
  "state": "Finalized"
}
```

### Triggering Action

System attempts an invalid transition.

### Observed Events

- Invariant violation detected.
- State machine violation detected

### Expected Resulting State

Process halts immediately.

### Error Classification

Fatal Error.

### Forbidden Actions

- Continuing execution
- Auto-correcting state
- Silently ignoring the violation

---

## 10. Vector — Operator Pause Mid-Auction

### Initial State

Auction is open and accepting bids.

```json
{
  "auction_id": "auc_006",
  "state": "Open"
}
```

### Triggering Action

Operator initiates pause.

### Observed Events

Pause state recorded.

### Expected Resulting State

Auction remains open but blocked.

```json
{
  "auction_id": "auc_006",
  "state": "Open",
  "pause_blocked": true
}
```

### Forbidden Actions

- Advancing time-based state
- Auto-finalizing auction
- Accepting bids during pause

---

## 11. General Invariants Verified

This section lists invariants implicitly verified by all vectors, such as:

- No authority recovery after ambiguity
- No time-based certainty
- No hidden retries
- No inferred ownership
- No silent failure

---

## 12. Final Rule

If implementation behavior differs from a test vector:

The implementation is wrong.
