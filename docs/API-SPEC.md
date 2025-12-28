# Application Programming Interface Specification

This document defines the external API used by the website.

---

## Design Goals

- Read-only by default
- Minimal surface area
- Predictable shapes
- Versionable
- Exposes canonical system state only

---

## Public Endpoints (Example)

These endpoints illustrate shape and intent.
Exact paths and parameters may vary by version.

- GET /state
  - Current auction state
  - Pause status

- GET /auction/history
  - Canonical auction results (paged)

- POST /bid
  - Submit bid (request must prove wallet control)

---

### Bid Validation

Bid submission requests are validated against the current auction’s rules.

A bid is considered invalid if:
- it is submitted outside the active auction window
- it does not meet required format constraints
- it is below the current auction’s minimum valid bid

The minimum valid bid is fixed at auction start and does not change during the auction.

Invalid bids are rejected and do not affect auction state.

---

## Responses

All responses include:
- status
- data
- error (if any)

Errors are explicit.
No silent failures.

- 400 Bad Request: bid does not meet minimum valid bid

---

## Versioning

- API is versioned from day one
- Breaking changes require new version

---

## Principle

The UI reflects state.
It does not infer it.
