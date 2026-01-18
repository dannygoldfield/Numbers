# Key Management Policy — Numbers

This document defines how cryptographic keys are generated, stored,
rotated, and constrained by the Numbers system.

It is **normative**.

Keys are an authority surface.
Key handling **must** prevent authority reuse, retry, or reinterpretation.

If there is a conflict,
SECRETS-AND-CONFIG.md, PERSISTENCE.md, RESTART-RULES.md,
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

This policy applies only to cryptographic keys
controlled by the Numbers backend.

It does **not** govern:

- bidder or user wallets
- client-side key management
- third-party wallet software or services

Numbers **never** requires access to user private keys.

---

## 2. Key Classes

### 2.1 Authority-Bearing Keys (Critical)

Authority-bearing keys are any keys capable of:

- spending funds
- settling auction outcomes
- broadcasting inscription transactions
- exercising irreversible system authority

These keys **consume authority when used**.

Once authority is exercised,
it **must not** be recreated by key rotation,
backup restoration, or alternate signing context.

---

### 2.2 Operational Wallet Keys

The operational wallet exists to support system execution.

It is used to:

- receive bid payments
- pay transaction fees
- construct and broadcast inscriptions
- facilitate settlement

Constraints:

- balance **must** be limited to operational minimums
- balance **must** be monitored continuously
- long-term custody **must not** occur
- wallet usage **must** align with auction sequencing

The operational wallet is a hot wallet by necessity.
Exposure is reduced by balance limits and procedural constraints.

---

### 2.3 NullSteward Addresses

NullSteward addresses are destinations used when an auction
does not finalize to a bidder.

Properties:

- generated per occurrence
- intentionally unspendable
- no private keys exist
- no recovery material exists
- no signing capability exists

Funds sent to a NullSteward address are unrecoverable by design.

NullSteward addresses are **not controlled by**:
- bidders
- operators
- the Numbers system

They exist to permit sequence advancement
without retry or reinterpretation.

---

## 3. Key Storage Rules (Normative)

Operational keys **must** obey all of the following:

- stored using hardware-backed or OS-secured keystores where available
- never committed to source control
- never logged or emitted in output
- never exposed through API responses
- never persisted in application databases

Environment variables:

- **may** be used only for runtime provisioning
- **must not** be treated as durable storage
- **must not** survive process restarts unintentionally

---

## 4. Backup Policy (Normative)

Authority-bearing keys **may** be backed up offline
**only** to prevent accidental loss.

Backup rules:

- access **must** be more restricted than operational access
- backup existence **must** be auditable
- restoration procedures **must** be documented and tested

Backups **must not** be used to:

- retry inscription
- reattempt settlement
- recreate authority after ambiguity
- bypass restart rules

Backup restoration does not restore permission.

---

## 5. Key Rotation (Normative)

Key rotation **may** occur only under all of the following conditions:

- the system is paused at an auction boundary
- no authority-bearing action is in progress
- rotation is recorded as a system control event

Rotation rules:

- old keys **must** be retired immediately
- rotated keys **must not** be used to reattempt prior actions
- rotation **must not** alter completed or ambiguous outcomes
- rotation **must not** permit alternate execution paths

If rotation occurs after ambiguity,
ambiguity **remains permanent**.

---

## 6. Compromise Response (Normative)

If key compromise is suspected or confirmed:

1. auctions **must** pause at the next auction boundary
2. remaining funds **must** be swept to newly generated keys
3. compromised keys **must** be retired permanently
4. new keys **must** be installed before resume
5. the incident **must** be recorded durably

Compromise response **must not**:

- reopen auctions
- retry settlement
- retry inscription
- reinterpret outcomes

---

## 7. Restart Interaction (Normative)

On restart:

- persisted state defines authority consumption
- keys **must not** recreate permission
- missing or rotated keys **must not** justify retry

If a key required for a pending action is unavailable:

- the action **must** be treated as already attempted
- authority **must** be considered consumed
- execution **must** halt or observe only

---

## 8. Non-Custodial Guarantee

Numbers does not custody user funds as a service.

System-controlled keys exist **only** to:

- facilitate settlement
- construct and broadcast inscriptions
- preserve continuity of the sequence

They do not exist to:
- hold value
- provide recovery
- act as escrow
- compensate failure

---

## 9. Final Principle

Keys are authority.

If a key action could change history,
that action must be impossible to repeat.

Key rotation preserves continuity.
It does not restore permission.
