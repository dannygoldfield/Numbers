# Key Management Policy — Numbers

This document defines how cryptographic keys are generated, stored, and handled by Numbers.

Numbers minimizes key exposure by design.
Key custody is limited in scope, duration, and authority.

---

## Scope

This policy applies only to keys controlled by the Numbers system.

It does not govern:
- user wallets
- bidder key management
- third-party wallet software

Numbers never requires access to user private keys.

---

## Key Types

### Operational Wallet

The operational wallet supports system execution.

It is used to:
- receive bid payments
- pay transaction fees for inscriptions
- coordinate settlement

Constraints:
- holds only the minimum balance required for operation
- balance is monitored continuously
- funds are not custodied long-term

The operational wallet is a hot wallet by necessity.
Exposure is minimized by balance limits and operational discipline.

---

### NullSteward Addresses

NullSteward addresses are used when an auction does not finalize to a bidder.

Properties:
- generated per occurrence
- intentionally unspendable
- no private keys are retained or stored
- no spendable material exists within the system

Funds routed to NullSteward addresses are unrecoverable by design.

NullSteward addresses are not controlled by:
- bidders
- operators
- the Numbers system itself

They exist only to ensure the sequence advances without retry.

---

## Key Storage

Operational keys are handled with the following constraints:

- stored using hardware-backed or OS-secured keystores where available
- never committed to source control
- never logged or emitted in application output
- never exposed via API responses

Environment variables may be used for runtime configuration only.
They are not a long-term storage mechanism.

---

## Backups

Operational keys are backed up offline.

Backup policy:
- access is strictly limited
- procedures are documented and periodically tested
- backups are stored separately from operational systems

Backups exist to prevent accidental loss only.
They do not enable routine access.

---

## Rotation and Compromise Response

If key compromise is suspected or confirmed:

1. Auctions pause at the next auction boundary
2. Remaining funds are swept to newly generated keys
3. Compromised keys are retired
4. New keys are installed
5. The incident and response are recorded

Key rotation does not alter:
- past auction outcomes
- recorded inscriptions
- sequence history

---

## Design Principle

Numbers does not custody user funds as a service.

System-controlled keys exist only to:
- facilitate settlement
- construct and broadcast inscriptions
- preserve continuity of the sequence

Key exposure is minimized across:
- time
- balance
- authority
