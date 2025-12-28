# Key Management Policy

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

The operational wallet is used to facilitate system operations.

- Receives bid payments
- Pays transaction fees for inscriptions
- Holds only the minimum balance required for operation
- Balance is monitored continuously

The operational wallet is a hot wallet by necessity, but exposure is intentionally minimized.

---

### Null Steward Addresses

Null Steward addresses are used as final destinations when an auction does not result in successful settlement.

- Generated per occurrence
- No private keys are retained or stored
- The system does not possess spendable material for these addresses
- Funds sent to these addresses are intentionally unrecoverable

Null Steward addresses are not controlled by any participant, including the operator.

---

## Key Storage

- Operational keys are stored using hardware-backed or OS-secured keystores where available
- Plaintext private keys are never committed to source control
- Keys are never logged or exposed via application output
- Environment variables may be used for runtime configuration only, not long-term storage

---

## Backups

- Operational keys are backed up offline
- Backup access is strictly limited
- Backup procedures are documented and tested periodically
- Backup materials are stored separately from operational systems

Backups exist solely to prevent accidental loss, not to enable routine access.

---

## Rotation and Compromise Response

If key compromise is suspected or confirmed:

- Auctions are paused at the next auction boundary
- Remaining funds are swept to a newly generated wallet
- Compromised keys are retired
- New keys are generated and installed
- The incident and response are documented

Key rotation does not alter past auction outcomes.

---

## Design Principle

Numbers does not custody user funds long-term.

Keys exist only to:
- facilitate settlement
- construct and broadcast inscriptions
- maintain continuity of the auction sequence

Key exposure is minimized in time, balance, and authority.
