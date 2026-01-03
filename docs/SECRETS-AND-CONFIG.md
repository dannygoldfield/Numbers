# Secrets and Configuration — Numbers

This document defines how secrets and configuration are handled by Numbers.

Secrets and configuration are distinct.  
They are managed, rotated, and audited differently.

---

## Scope

This document applies to all secrets and configuration values used by the Numbers system.

It does not govern:
- user wallet secrets
- bidder key management
- third-party client configuration
- external infrastructure not directly controlled by Numbers

---

## Secrets

Secrets are values that grant authority or access.

Secrets include:
- private keys
- Bitcoin Core RPC credentials
- API keys, if introduced

Secrets are subject to the following rules:

- Never committed to source control
- Never logged or exposed in application output
- Never transmitted to clients
- Never hard-coded in binaries

Any secret exposed outside its intended boundary must be treated as compromised.

---

## Secret Storage

- Secrets are provided at runtime via environment variables or OS-secured keystores
- Secrets are not persisted in application state or databases
- Access is limited to the operator and the running process
- Offline backups are permitted only where recovery requires them

Backup access is more restricted than operational access.

---

## Configuration

Configuration defines system behavior but does not grant authority.

Configuration includes:
- fee thresholds
- auction caps
- timeouts and deadlines
- environment identifiers

Configuration is:

- explicit
- human-readable
- version-controlled
- loaded at startup

Configuration changes must be intentional and reviewable.

---

## Configuration Boundaries

Configuration must not:
- redefine system invariants
- override auction sequencing rules
- bypass limits or circuit breakers

If a behavior change cannot be expressed safely through configuration,
it requires a code change.

---

## Rotation and Change Control

### Secret Rotation

- Secrets may be rotated without code changes
- Rotation requires pausing auctions at an auction boundary
- Old secrets are retired immediately
- Rotation events are documented

Key rotation does not alter past outcomes.

---

### Configuration Changes

- Configuration changes are applied at startup
- Changes that reduce limits require an auction-boundary pause
- Changes that increase limits may be applied without interruption

Configuration changes must never retroactively affect completed auctions.

---

## Audit and Traceability

The system must be able to determine:

- which configuration was active at any point in time
- when secrets were last rotated
- who performed the change
- why the change occurred

Secrets themselves are never logged.  
Metadata about changes is.

---

## Design Principle

Secrets confer power.  
Configuration constrains behavior.

If something cannot be rotated safely,
it is too powerful to exist.
