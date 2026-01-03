# Application Programming Interface Specification — Numbers

This document defines the external API exposed by the Numbers backend.

The API exists to expose canonical system state and allow permitted actions.
It does not interpret outcomes, infer meaning, or provide convenience abstractions.

If there is a conflict, `CORE-SEQUENCE.md`, `ARCHITECTURE.md`, and `PRD.md` take precedence.

---

## Design Goals

The API is designed to be:

- Read-only by default
- Minimal in surface area
- Deterministic in shape
- Explicit in failure
- Versioned from inception
- Limited to canonical system state

The API reflects what has happened.
It does not predict, summarize, or explain.

---

## Authority Model

The API is **not authoritative**.

- Bitcoin is the final record of inscriptions.
- The Numbers backend is authoritative for auction resolution and finalization.
- The API exposes the backend’s recorded state.

The API may be unavailable, delayed, or restarted
without altering system outcomes.

---

## Public Endpoints (Illustrative)

Endpoints below describe intent and structure.
Exact paths, parameters, and pagination may vary by version.

### GET /state

Returns the current system state.

Includes:
- current auction number
- auction open / closed state
- time remaining (if applicable)
- inter-auction pause status

This endpoint exposes **now**.
It does not expose predictions.

---

### GET /auction/history

Returns finalized auction outcomes in sequence order.

- Results are paginated
- Only canonical outcomes are returned
- Each entry corresponds to exactly one auction number

Includes, per entry:
- auction number
- final destination (winner address or NullSteward)
- inscription txid
- inscription satpoint
- timestamps

This endpoint does not:
- infer ownership
- collapse outcomes
- hide null or no-bid results

---

### POST /bid

Submits a bid for the currently open auction.

Requirements:
- request must prove control of the bidding wallet
- bid must target the current auction only

Bid submission is permitted **only** during an open auction window.

---

## Bid Validation

Bid requests are validated against the auction state as fixed at auction start.

A bid is invalid if:
- submitted outside the active auction window
- malformed or incomplete
- below the minimum valid bid
- not provably authorized by the submitting wallet

The minimum valid bid:
- is defined at auction start
- remains fixed for the duration of that auction
- does not change in response to other bids

Invalid bids:
- are rejected explicitly
- do not affect auction state
- do not alter timing or resolution

---

## Responses

All API responses share a common envelope:

- `status` — success or failure
- `data` — present only on success
- `error` — present only on failure

Errors are explicit.
Silent failure is forbidden.

Example error conditions:
- `400 Bad Request` — malformed or invalid bid
- `409 Conflict` — bid submitted outside active auction
- `503 Service Unavailable` — backend temporarily unavailable

The API does not retry actions on behalf of clients.

---

## Versioning

- The API is versioned from the first public release
- Breaking changes require a new version
- Versions do not change behavior retroactively

Old versions may be deprecated but must not silently change semantics.

---

## Non-Goals

The API does not:

- provide analytics or aggregation
- infer ownership or intent
- expose private or internal state
- decorate or reframe outcomes
- optimize for client convenience

Clients adapt to the system.
The system does not adapt to clients.

---

## Guiding Principle

The API exposes facts.

Interpretation happens elsewhere.
