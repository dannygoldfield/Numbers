# Deployment — Numbers

This document defines how Numbers is deployed and updated.

Deployments must preserve auction integrity.
No deployment may alter or interrupt an active auction.

---

## Deployment Principles

All deployments must satisfy the following:

- History is never rewritten
- Auction semantics are never changed
- Active auctions are never interrupted
- Deployments respect auction boundaries
- Simplicity is preferred over automation complexity

If a deployment cannot meet these constraints,
it must be deferred.

---

## Website Deployment

The Numbers website is an interface layer only.

Properties:
- built as static assets
- deployed via CI or equivalent automation
- rollback is immediate by redeploying a previous build
- no runtime configuration
- no authority over auction behavior

Website deployments:
- may occur independently of backend deployment
- must accurately reflect canonical system state
- must not imply behavior not guaranteed by the system

A website deployment must never alter auction execution.

---

## Backend Deployment

The backend defines auction execution and state progression.

Properties:
- deployed as a single binary or container
- exactly one instance per environment
- deployment is manual or minimally automated
- configuration is explicit and versioned

### Restart Rules

Backend restarts are constrained:

- restarts are permitted only when no auction is active
- if restart timing is uncertain, auctions must be paused first
- restarts must be restart-safe as defined in `ARCHITECTURE.md`

A backend deployment must not:
- create a second resolution for any auction
- alter timing guarantees
- skip or repeat sequence steps

---

## Bitcoin Core Deployment

Bitcoin Core is a critical dependency.

Rules:
- runs continuously
- version upgrades are deliberate and infrequent
- upgrades are tested on Testnet before Mainnet
- configuration changes are version-controlled

Bitcoin Core must not be upgraded during an active auction,
except in response to critical security issues.

---

## Rollback Strategy

Rollback restores a known-good version
without altering recorded outcomes.

Rollback methods:
- Website: redeploy a previous commit
- Backend: redeploy a previous binary or container
- Bitcoin Core: rollback only for critical failure or security risk

Rollback must never:
- reopen an auction
- re-resolve an outcome
- alter an inscription
- change sequence history

---

## Deployment and Pausing

Deployment coordination rules:

- No deployment occurs during an active auction
- If deployment timing is uncertain, auctions pause at the next auction boundary
- Pause and resume follow defined pause semantics

Pausing is a containment mechanism,
not a recovery mechanism.

---

## Design Rule

If a deployment cannot be performed safely
at an auction boundary,
it must not be performed.

Correctness outweighs convenience.
