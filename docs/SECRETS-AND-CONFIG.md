# Secrets and Configuration — Numbers

This document defines how secrets and configuration are handled by Numbers.

It is **normative**.

Secrets and configuration are distinct.
They are managed, rotated, and audited differently.

Secrets are an authority surface.
Configuration is a constraint surface.

If there is a conflict,
PERSISTENCE.md, RESTART-RULES.md, CONFIG-REFERENCE.md,
STATE-MACHINE.md, and INVARIANTS.md take precedence.

---

## Modal Language Rule (Normative)

In this document and all normative specifications:

- **must / must not** define obligations
- **only / exactly once / at most once** define bounds
- **may** is permitted **only** to describe uncertainty of knowledge
  or operator action outside automated execution

The following terms are forbidden in normative contexts:

- possibly
- likely
- eventually
- for now
- TBD

Any normative statement using forbidden modal language is invalid.

---

## 1. Scope

This document governs all secrets and configuration values
used by the Numbers backend.

It does **not** govern:

- bidder wallets or user-held keys
- client-side configuration
- third-party infrastructure not directly controlled by Numbers

---

## 2. Definitions

### Secret

A secret is any value that enables identity, access, or authority.

Secrets include, but are not limited to:
- private keys used for signing or spending
- Bitcoin Core RPC credentials
- authentication tokens for infrastructure services

Secrets are **authority-bearing** unless explicitly stated otherwise.

---

### Configuration

Configuration defines system parameters and limits.

Configuration includes, but is not limited to:
- auction timing parameters
- bid caps and limits
- deadlines and timeouts
- network identifiers

Configuration **does not grant authority**.

---

## 3. Secret Rules (Normative)

Secrets **must** obey all of the following:

- must never be committed to source control
- must never be logged or emitted in application output
- must never be transmitted to clients
- must never be hard-coded in binaries
- must never be persisted in application state or databases

Any secret exposed outside its intended boundary
**must** be treated as compromised.

---

## 4. Authority-Bearing Secrets (Critical)

Secrets that can exercise irreversible authority include:

- keys capable of broadcasting inscription transactions
- keys capable of settling auction outcomes
- credentials that allow direct wallet control

Rules:

- authority-bearing secrets **must** be treated as consumable capability
- loss, compromise, or uncertainty **must not** trigger retry
- rotation **must not** restore consumed authority
- rotation **must not** permit alternate execution paths

Changing a secret **does not** change history.

---

## 5. Secret Storage

Secrets **must** be provided at runtime via:

- environment variables
- OS-secured keystores

Secrets **must not** be:

- persisted to disk by the application
- stored in databases
- cached across restarts

Access to secrets **must** be limited to:
- the running process
- explicitly authorized operators

---

## 6. Secret Backup and Recovery

Offline backups of secrets:

- **may** exist only when recovery explicitly requires them
- **must** be more restricted than operational access
- **must** be auditable

Backup existence **must not** be used to justify:
- retrying authority
- reattempting inscription
- bypassing ambiguity

---

## 7. Secret Rotation (Normative)

Secret rotation **may** occur only under the following conditions:

- the system is paused at an auction boundary
- no authority-bearing action is in progress
- rotation is recorded as a system control event

Rotation rules:

- old secrets **must** be retired immediately
- rotated secrets **must not** be used to reattempt past actions
- rotation **must not** alter completed or ambiguous outcomes

If a secret is rotated after ambiguity,
ambiguity **remains**.

---

## 8. Configuration Rules (Normative)

Configuration **must** obey all of the following:

- must be explicit and human-readable
- must be version-controlled
- must be validated before startup
- must be snapshotted and persisted at startup

Configuration **must not**:

- redefine invariants
- alter state machine semantics
- bypass limits or circuit breakers
- permit retries forbidden by the specification

If a behavior change cannot be expressed safely through configuration,
it requires a code change.

---

## 9. Configuration Changes

Configuration changes:

- **must** be applied only at startup
- **must** be validated before execution begins
- **must** not retroactively affect completed auctions

Limit changes:

- increases **may** occur without interruption
- reductions **must** occur only at auction boundaries
- reductions **must not** invalidate in-flight authority

Configuration changes **must not** be used to:
- escape ambiguity
- reclaim authority
- reinterpret past outcomes

---

## 10. Restart Interaction (Normative)

On restart:

- persisted state defines reality
- secrets and configuration **must not** recreate permission
- missing or changed secrets **must not** justify retry

If a secret required for a pending action is unavailable:
- the action **must** be treated as already attempted
- authority **must** be considered consumed
- execution **must** halt or observe only

---

## 11. Audit and Traceability

The system **must** be able to determine:

- which configuration snapshot was active at any time
- when secrets were rotated
- who performed the change
- why the change occurred

Audit records:

- **must** be append-only
- **must** exclude secret material
- **must** be durable across restarts

---

## 12. Final Principle

Secrets confer power.  
Configuration constrains behavior.

If changing a value can restore authority,
that value is a secret and is too powerful to be treated as configuration.
