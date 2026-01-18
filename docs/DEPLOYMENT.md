# Deployment — Numbers

This document defines how Numbers is deployed and updated.

It is **normative**.

Deployment is an authority surface.
Incorrect deployment can create ambiguity, duplicate authority,
or silently alter history.

If there is a conflict,
STATE-MACHINE.md, PERSISTENCE.md, RESTART-RULES.md,
INVARIANTS.md, and OPERATIONAL-RUNBOOK.md take precedence.

---

## Modal Language Rule (Normative)

In this document and all normative specifications:

- **must / must not** define obligations
- **only / exactly once / at most once** define bounds
- **may** is permitted **only** to describe uncertainty of knowledge,
  never permission, intent, or policy choice

The following terms are forbidden in normative contexts:

- possibly
- likely
- eventually
- for now
- TBD

Any normative statement using forbidden modal language is invalid.

---

## 1. Deployment Principles (Normative)

All deployments **must** satisfy the following principles:

- History **must never** be rewritten
- Auction semantics **must never** change retroactively
- An active auction **must never** be interrupted
- Deployments **must** respect auction boundaries
- Restart safety **must** be provable, not assumed
- Simplicity **must** be preferred over automation complexity

If a deployment cannot meet these constraints,
it **must not** be performed.

---

## 2. Deployment Boundaries (Normative)

Deployments **must occur only** at one of the following boundaries:

- When no auction exists yet
- Between two auctions
- While the system is explicitly paused at an auction boundary

Deployments **must not** occur:

- During an open auction
- During resolution
- During settlement finalization
- During inscription initiation or broadcast

If deployment timing is uncertain,
the system **must** be paused first.

---

## 3. Website Deployment (Normative)

The Numbers website is an interface layer only.

### Properties

- Built as static assets
- Deployed via CI or equivalent automation
- No runtime configuration
- No access to secrets
- No authority over auction execution

### Rules

Website deployments:

- **may** occur independently of backend deployment
- **must** reflect only persisted, canonical state
- **must not** imply guarantees not provided by the system
- **must not** influence auction behavior

Rollback:

- **must** be achievable by redeploying a previous build
- **must not** alter backend state

A website deployment **must never** alter:

- auction timing
- auction resolution
- settlement behavior
- inscription behavior

---

## 4. Backend Deployment (Normative)

The backend defines auction execution and authority progression.

### Properties

- Deployed as a single binary or container
- Exactly one active instance per environment
- Deployment is manual or minimally automated
- Configuration is explicit and version-controlled

### Rules

Backend deployment **must** satisfy all of the following:

- No second instance **may** run concurrently
- Persisted state **must** be preserved intact
- Restart behavior **must** conform to RESTART-RULES.md
- No authority **may** be recomputed, retried, or inferred

A backend deployment **must not**:

- Create a second resolution for any auction
- Alter auction timing guarantees
- Skip or repeat sequence steps
- Infer missing or ambiguous state

Deployment substitutes code.
It does not re-execute history.

---

## 5. Backend Restart Discipline (Normative)

Backend restarts are tightly constrained.

### Rules

- Restarts **must occur only** when no auction is active
- If restart timing is uncertain, auctions **must** be paused first
- Restart **must** discard all in-memory state
- Persisted state **must** be treated as authoritative

On startup, the backend **must**:

1. Load all persisted records
2. Reconstruct state exclusively from persistence
3. Refuse to recompute any completed authority
4. Halt if required records are missing or contradictory

Restarting is not recovery.  
Restarting is reconstruction.

---

## 6. Bitcoin Core Deployment (Normative)

Bitcoin Core is a critical dependency.

### Rules

- Bitcoin Core **must** run continuously
- Version upgrades **must** be deliberate and infrequent
- Upgrades **must** be tested on Testnet before Mainnet
- Configuration **must** be version-controlled

Bitcoin Core **must not** be upgraded:

- During an open auction
- During resolution
- During inscription initiation or broadcast
- While unresolved ambiguity exists

Exception:

- Critical security vulnerability requiring immediate action,
  followed by a pause at the next auction boundary if state safety
  cannot be proven

---

## 7. Rollback Strategy (Normative)

Rollback restores a known-good software version
without altering recorded outcomes.

### Permitted Rollbacks

- Website: redeploy previous static build
- Backend: redeploy previous binary or container
- Bitcoin Core: rollback only for critical failure or security risk

Rollback **must never**:

- Reopen an auction
- Re-resolve an outcome
- Retry settlement or inscription
- Modify persisted records
- Alter sequence history

Rollback changes software.
It does not change history.

---

## 8. Deployment and Pausing (Normative)

Deployment coordination rules:

- If a deployment risks violating auction boundaries,
  the system **must** be paused first
- Pause and resume **must** follow pause semantics exactly
- Pause is containment, not recovery

Deployment **must not** be used to:

- fix outcomes
- repair ambiguity
- normalize failure
- “clean up” history

---

## 9. Final Rule

If a deployment cannot be performed safely
at an auction boundary:

**It must not be performed.**

Correctness outweighs convenience.
