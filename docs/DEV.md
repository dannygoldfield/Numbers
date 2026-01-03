# DEV — Development and Implementation Contract — Numbers

This document defines how Numbers is implemented and run in development.
It is procedural and binding.

This document:
- Describes how to run Numbers locally on Bitcoin Testnet
- Defines implementation constraints and boundaries
- Defines what developers must and must not implement

This document does not:
- Define system behavior or semantics
- Justify design decisions
- Describe UI behavior
- Describe future features

Authoritative behavior is defined in:
- CORE-SEQUENCE.md
- STATE-MACHINE.md
- PRD.md

If there is a conflict, those documents take precedence.

---

## 1. Supported Environment

### Operating Systems
- macOS
- Linux

Windows is not supported.

### Language and Tooling
- Rust 1.67 or later
- Cargo
- Bitcoin Core (Testnet enabled)

---

## 2. Execution Model

Numbers runs as a **single long-lived backend process** per environment.

The backend process:
- Maintains auction state
- Accepts bids
- Coordinates settlement
- Initiates inscriptions
- Persists all canonical records

Only **one instance** of the backend may run per environment.

Running multiple instances against the same datastore is forbidden.

---

## 3. Process Lifecycle

### Startup

On startup, the backend must:

1. Load configuration
2. Verify required secrets are present
3. Connect to Bitcoin Core
4. Load persisted auction state
5. Resume execution from persisted states
6. Never recompute resolution or finalization

If any step fails, the process must exit visibly.

---

### Shutdown

Shutdown must be graceful.

- No open auction may be interrupted
- Shutdown may occur only:
  - between auctions
  - or while auctions are paused

Forced shutdown during uncertainty must be treated as a crash.

---

## 4. Bitcoin Core (Testnet)

Numbers requires a local Bitcoin Core node running in Testnet mode.

### Start Bitcoin Core

```bash
bitcoind -testnet -datadir=/path/to/bitcoin-testnet
