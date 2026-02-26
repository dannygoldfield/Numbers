# Wallet Specification — Numbers

This document defines deterministic wallet behavior for:

- settlement address derivation
- inscription funding
- signing
- UTXO policy

It is normative.

No wallet behavior may occur outside the rules defined here.

---

## 1. Master Seed

The system must operate from a single HD master seed.

The seed must be:

- Generated once at initialization.
- Persisted encrypted on disk.
- Immutable for the lifetime of the deployment.
- Never derived from runtime entropy after initialization.

Restart must reconstruct wallet state solely from the persisted seed.

If the seed cannot be decrypted deterministically,
execution must halt.

The master seed must never be:

- Logged
- Exposed via API
- Exported without explicit operator action

---

## 2. Address Type

All settlement addresses must be:

P2WPKH (Native SegWit)

No other address types are permitted for settlement.

Legacy (P2PKH), P2SH, P2TR, or script-based addresses are forbidden for settlement.

---

## 3. Deterministic Settlement Address Derivation

Each auction must derive exactly one unique settlement address.

The derivation must use deterministic HD derivation with the following path:

m / 84' / coin_type' / 0' / 0 / auction_index

Where:

- 84' specifies BIP84 (P2WPKH).
- coin_type' corresponds to the active network.
- account' is fixed at 0'.
- change level is fixed at 0.
- auction_index equals the canonical number being auctioned.

auction_index must:

- Equal the canonical number value.
- Be immutable.
- Be unique across all auctions.
- Never be reused.

The settlement address for a given auction must be computable
from only:

- The master seed.
- The canonical number.

No runtime state may influence derivation.

---

## 4. Address Reuse

Settlement addresses must never be reused.

Because auction_index equals canonical number,
reuse is structurally forbidden.

---

## 5. Gap Scanning

The system must not use wallet gap scanning.

The settlement address for a given auction must be computed directly
from the canonical number.

The system must not scan unused address ranges to infer state.

---

## 6. Private Key Usage

The private key corresponding to a settlement address must:

- Never be used to sign outgoing transactions within Numbers.
- Exist only to allow operator recovery if necessary.

Settlement addresses are receive-only for protocol purposes.

---

## 7. Inscription Funding Policy

Inscription transactions must be funded from the operator wallet
derived from the master seed.

Settlement addresses must never fund inscription transactions.

UTXO selection must be:

- Deterministic
- Order-stable
- Independent of runtime randomness

UTXOs must be selected by:

- Sorting confirmed UTXOs by ascending txid,
- Then ascending vout,
- Selecting the minimal set required to satisfy output value, fee, and dust rules.

Randomized UTXO selection is forbidden.

---

## 8. UTXO Reservation

Before any inscription broadcast attempt:

- Selected UTXOs must be reserved in canonical persistence.
- Reserved UTXOs must not be reused by any other inscription attempt.

If authority is consumed:

- Reserved UTXOs remain permanently bound to that inscription attempt.

If broadcast is not committed:

- Reserved UTXOs may be released.

UTXO reservation must not introduce new lifecycle states.

---

## 9. Insufficient Funds

If confirmed wallet balance is insufficient to:

- fund inscription output,
- satisfy fee requirements,
- satisfy dust threshold,

then:

- Inscription must not proceed.
- The condition must be classified as insufficient_funds.
- Inscription remains NotStarted.

Partial funding is forbidden.

---

## 10. Dust Policy

All outputs must satisfy network dust thresholds.

If change output would be dust:

- Change output must be suppressed.
- The remainder must be absorbed into transaction fee.

Creation of dust outputs is forbidden.

---

## 11. Fee Ceiling Enforcement

Fee calculation must use:

- Authoritative node fee estimate,
- Deterministic transaction weight,
- Configured fee ceiling.

If required fee exceeds configured fee ceiling:

- Broadcast must not occur.
- Classification must be fee_ceiling_exceeded.
- Authority must not be consumed.

Fee ceiling must not alter lifecycle semantics.

---

## 12. Signing Surface

Signing must occur:

- Deterministically,
- Using private keys derived from the master seed,
- Without operator interaction.

If signing fails:

- Classification must be signer_unavailable.
- Authority must not be consumed.

External signers are not permitted in this deployment model.

---

## 13. Restart Behavior

On restart:

Wallet state must be reconstructed from:

- Master seed,
- Persisted UTXO reservation records,
- Authoritative node confirmed UTXO set.

If contradictions exist between persisted reservation records
and authoritative node UTXO set:

- Classification must be Fatal.
- Execution must halt.

Restart must not:

- Reselect different UTXOs,
- Modify fee calculation,
- Alter deterministic construction,
- Reconstruct alternate funding paths.

---

## 14. Final Rule

Wallet behavior must be:

- Deterministic,
- Reproducible,
- Independent of runtime history,
- Independent of operator memory.

If any wallet behavior requires guessing,
execution must halt.