# Inscription Machine — Numbers

This document defines the inscription subsystem lifecycle, authority boundary, and permitted delivery behavior.

It is normative.

The inscription machine is separate from the auction lifecycle. Auctions must not halt due to inscription progress or failure.

If a behavior is not specified here, it is forbidden.

---

## 1. Purpose

The inscription machine exists to:

- Construct an inscription transaction for a finalized auction outcome.
- Broadcast it to the Bitcoin network via the authoritative node.
- Observe confirmation to the configured depth.
- Record outcomes without repair or concealment.

The inscription machine must:

- Prefer explicit failure over inferred success.
- Freeze authority permanently upon ambiguity.
- Permit controlled delivery repricing without semantic change.

---

## 2. Definitions

### 2.1 Authoritative Node

The authoritative Bitcoin Core node is defined in `chain/CHAIN-INTERACTION.md`.

All broadcast and observation must use only this node.

### 2.2 Knowledge

Knowledge classification follows `chain/CHAIN-INTERACTION.md`.

### 2.3 Confirmation Depth

`chain.confirmation_depth` is defined in `chain/CHAIN-INTERACTION.md` and `config/CONFIG-REFERENCE.md`.

### 2.4 Canonical Number

The canonical number is the number being auctioned for the current auction lifecycle.

---

## 3. Inscription States

The inscription machine has the following states per auction:

- NotStarted
- Inscribing
- Inscribed
- Ambiguous

State meanings:

- NotStarted: No committed inscription broadcast has occurred.
- Inscribing: A committed broadcast exists and confirmation is pending.
- Inscribed: The inscription transaction is Known Confirmed to confirmation depth.
- Ambiguous: Inscription outcome is uncertain; authority is permanently frozen.

Terminal states:

- Inscribed
- Ambiguous

---

## 4. Destination of Inscribed Output (Normative)

The inscription transaction must deliver the inscribed output to the winner destination address.

The winner destination address must be:

- The address persisted as part of settlement outcome records.
- Immutable once persisted.
- Deterministically convertible to a valid scriptPubKey under permitted address types.

The inscription machine must not:

- Choose a destination,
- Modify the destination,
- Infer a fallback destination,
- Or reinterpret settlement records.

If the persisted destination address:

- Is missing,
- Is malformed,
- Is not permitted under the configured address policy,
- Or cannot be deterministically converted to a scriptPubKey,

then:

- The inscription attempt must not proceed.
- The condition must be classified via `errors/ERROR-TAXONOMY.md`.
- No authority may be consumed.

Destination resolution must occur before transaction construction.

---

## 5. Authority Boundary

### 5.1 Authority Consumption Event

Inscription authority is consumed at the first moment both are true:

1. The system has broadcast a candidate inscription transaction using the authoritative node, and
2. The authoritative node reports the transaction present in its mempool.

This event is called `broadcast_commit`.

Before `broadcast_commit`, authority is not consumed.

After `broadcast_commit`, authority is consumed exactly once.

### 5.2 No Semantic Retries

After authority consumption, the system must not attempt any action that could create a semantically distinct inscription.

Delivery repricing is permitted only under the controlled RBF rules in Section 8.

---

## 6. Canonical Records

The inscription machine must persist canonical records in append-only form as defined in `data/PERSISTENCE.md`.

Records listed here are required at minimum. Field names are normative.

### 6.1 InscriptionIntentRecord

An InscriptionIntentRecord must be persisted before any broadcast attempt.

Fields:

- record_type: "InscriptionIntentRecord"
- canonical_number: integer
- created_at_server_time: timestamp
- content_type: "text/plain; charset=utf-8"
- payload_hash: bytes32
- destination_script_pubkey: bytes
- settlement_reference: opaque identifier linking to finalized auction outcome
- funding_policy: object (see 6.4)
- fee_policy: object (see 6.5)
- rbf_policy: object (see 6.6)
- intent_id: bytes32

Rules:

- destination_script_pubkey must be derived solely from the persisted winner destination address.
- content_type must match the value defined in `inscription/INSCRIPTION-FORMAT.md`.
- payload_hash must be SHA-256 over the exact payload bytes defined in `inscription/INSCRIPTION-FORMAT.md` for the canonical_number.
- intent_id must be deterministically derived from all other fields.
- The intent must be immutable once persisted.
- There must be at most one InscriptionIntentRecord per canonical_number.

### 6.2 InscriptionBroadcastRecord

An InscriptionBroadcastRecord must be persisted after each broadcast attempt.

Fields:

- record_type: "InscriptionBroadcastRecord"
- canonical_number: integer
- attempt_index: integer (starts at 0)
- attempted_at_server_time: timestamp
- tx_hex_hash: bytes32
- txid: bytes32 or null
- rpc_result: "success" | "error" | "unknown"
- mempool_presence: "present" | "absent" | "unknown"
- classified_outcome: "committed" | "not_committed" | "ambiguous"
- error_code: optional string (must map to ERROR-TAXONOMY.md)

Rules:

- attempt_index must be strictly increasing.
- If rpc_result is unknown, mempool_presence must be unknown.
- classified_outcome must be:
  - committed only if rpc_result = success AND mempool_presence = present
  - not_committed only if rpc_result = error AND mempool_presence = absent
  - ambiguous otherwise

### 6.3 InscriptionConfirmationRecord

An InscriptionConfirmationRecord must be persisted when the inscription is Known Confirmed.

Fields:

- record_type: "InscriptionConfirmationRecord"
- canonical_number: integer
- confirmed_at_server_time: timestamp
- txid: bytes32
- block_height: integer
- confirmations: integer

Rules:

- txid must refer to the active inscription txid being tracked.
- confirmations must be >= chain.confirmation_depth at record creation time.

### 6.4 Funding Policy

funding_policy must specify:

- funding_source: "operator_wallet"
- utxo_selection_mode: "deterministic"
- utxo_reservation_required: true

UTXO reservation rules must be defined in `wallet/WALLET-SPEC.md`.

### 6.5 Fee Policy

fee_policy must specify:

- fee_mode: "adaptive"
- fee_ceiling_sats: integer
- fee_estimation_source: "authoritative_node"

Fee computation must be deterministic given:

- node fee estimate response
- configured caps
- transaction weight

If the required fee exceeds fee_ceiling_sats, the broadcast attempt must not proceed and must be recorded as not_committed with error_code = fee_ceiling_exceeded.

### 6.6 RBF Policy

rbf_policy must specify:

- rbf_allowed: true
- max_replacements: integer
- replacement_requires_equivalence: true

---

## 7. Broadcast Procedure

The broadcast procedure is the only permitted way to submit an inscription transaction.

### 7.1 Preconditions

Before attempting broadcast:

- The auction must be finalized.
- An InscriptionIntentRecord must exist for the canonical_number.
- Required wallet funds and UTXO reservations must be satisfied.
- The constructed transaction must be derivable from the InscriptionIntentRecord.
- The destination_script_pubkey must match the persisted winner destination address.

### 7.2 Steps

1. Construct tx candidate deterministically from InscriptionIntentRecord and wallet state.
2. If computed fee exceeds fee ceiling, do not broadcast. Persist InscriptionBroadcastRecord with not_committed and fee_ceiling_exceeded.
3. Call the authoritative node to broadcast the raw transaction.
4. If the node returns a txid, immediately check mempool presence for that txid via the authoritative node.
5. Persist InscriptionBroadcastRecord capturing rpc_result and mempool_presence.

### 7.3 Authority Consumption Classification

If InscriptionBroadcastRecord.classified_outcome = committed:

- Transition inscription state to Inscribing.
- Authority is consumed.

If classified_outcome = not_committed:

- Remain in NotStarted.
- Authority is not consumed.

If classified_outcome = ambiguous:

- Transition inscription state to Ambiguous.
- Authority is frozen permanently.

---

## 8. Controlled RBF

RBF is permitted only as a delivery adjustment after authority is consumed.

### 8.1 Equivalence Rule

A replacement transaction is permitted only if it is semantically equivalent to the originally committed transaction.

Two inscription transactions are semantically equivalent if all of the following are identical:

- canonical_number
- content_type
- payload_hash
- destination_script_pubkey
- inscription content semantics as defined in `inscription/INSCRIPTION-FORMAT.md`
- funding lineage (inputs derive from the same reserved UTXO set)

Allowed differences:

- fee amount
- change output value
- sequence numbers required to enable RBF

Any other difference is forbidden.

### 8.2 Replacement Limits

- The number of replacements must not exceed rbf_policy.max_replacements.
- Each replacement must be recorded as a new InscriptionBroadcastRecord with incremented attempt_index.

### 8.3 Replacement Ambiguity

If during replacement the system cannot determine mempool presence for either the prior txid or the new txid, the inscription must transition to Ambiguous.

---

## 9. Confirmation Observation

After authority consumption, the system must observe the authoritative node until either:

- A candidate txid becomes Known Confirmed to confirmation depth, or
- Ambiguity is detected.

### 9.1 Observed Candidate Set

The candidate set consists only of:

- The committed txid, and
- Any replacement txids recorded in InscriptionBroadcastRecords.

No other txids may be considered.

### 9.2 Confirmation Rule

When a candidate txid is Known Confirmed to confirmation depth:

- Persist InscriptionConfirmationRecord.
- Transition inscription state to Inscribed.

Mempool presence alone must not trigger Inscribed.

---

## 10. Restart Detection Logic

Restart handling must follow `data/RESTART-RULES.md`.

Additional inscription-specific reconstruction rules:

### 10.1 Reconstruction Inputs

On startup, the system must reconstruct inscription state from persisted records only.

### 10.2 Reconstruction Rules

For a given canonical_number:

1. If InscriptionConfirmationRecord exists:
   - State is Inscribed.

2. Else if any InscriptionBroadcastRecord has classified_outcome = ambiguous:
   - State is Ambiguous.

3. Else if any InscriptionBroadcastRecord has classified_outcome = committed:
   - State is Inscribing and the active candidate set is the committed txid plus any recorded replacements.

4. Else if InscriptionIntentRecord exists:
   - State is NotStarted.

5. Else:
   - State is NotStarted.

### 10.3 Post-Restart Chain Checks

If state reconstructs to Inscribing:

- The system must query the authoritative node for confirmation status of all candidate txids.
- If any candidate is Known Confirmed, transition to Inscribed.
- If node responses are contradictory relative to persisted records, classify Fatal and halt.

The system must not attempt a new broadcast after restart unless:

- No committed broadcast exists for the canonical_number, and
- No ambiguity record exists.

---

## 11. Failure Handling

All failures must be classified via `errors/ERROR-TAXONOMY.md`.

Rules:

- Any loss of certainty after authority consumption is Ambiguous.
- Any contradiction between persisted confirmation record and authoritative node truth is Fatal.
- Fee ceiling exceeded is not a chain failure; it is a policy refusal.

---

## 12. Final Rule

If a transition or action is not explicitly permitted by this document,
it is forbidden.

Ambiguity freezes authority permanently.