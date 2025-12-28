# Deployment

This document defines how Numbers is deployed and updated.

Deployments must preserve auction integrity.
No deployment may alter or interrupt an active auction.

---

## Deployment Principles

- Deployments must not rewrite history
- Deployments must not change auction semantics
- Deployments must respect auction boundaries
- Simplicity is preferred over automation complexity

If a deployment cannot meet these constraints, it must be deferred.

---

## Website Deployment

The Numbers website is an interface layer only.

- Built as static assets
- Deployed via CI or equivalent automation
- Rollback is immediate by redeploying a previous build
- No runtime configuration
- No authority over auction behavior

Website deployments may occur independently of backend deployment,
provided they do not misrepresent system state.

---

## Backend Deployment

The backend defines auction behavior and state progression.

- Deployed as a single binary or container
- One instance per environment
- Deployment is manual or via minimal automation
- Configuration is explicit and versioned

### Restart Rules

- Backend restarts are permitted only when no auction is active
- If a restart is required during uncertainty, auctions must be paused first
- Restarts must be restart-safe as defined in system architecture

A backend deployment must not create a second resolution for any auction.

---

## Bitcoin Core Deployment

Bitcoin Core is a critical dependency.

- Runs continuously
- Version upgrades are planned, infrequent, and deliberate
- All upgrades are tested on Testnet before Mainnet
- Configuration changes are version-controlled

Bitcoin Core must not be upgraded during an active auction unless required for critical security reasons.

---

## Rollback Strategy

Rollback restores a known-good version without altering past outcomes.

- Website: redeploy previous commit
- Backend: redeploy previous binary or container
- Bitcoin Core: rollback only for critical failures or security issues

Rollback must not reopen, re-resolve, or alter completed auctions.

---

## Deployment and Pausing

- No deployments occur during an active auction
- If deployment timing is uncertain, auctions must be paused at the next auction boundary
- Pause and resume behavior follows the defined pause semantics

---

## Design Rule

If a deployment cannot be performed safely at an auction boundary,
it must not be performed.
