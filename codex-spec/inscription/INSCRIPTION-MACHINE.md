# Inscription Machine: Numbers

This document defines the inscription subsystem lifecycle, authority boundary, and permitted delivery behavior.

It is normative.

Authority precedence is defined exclusively in `AUTHORITY-ORDER.md`.

The inscription machine is separate from the auction lifecycle.

Auction correctness must not depend on inscription progress, inscription broadcast, inscription confirmation, or inscription failure.

If a behavior is not specified here, it is forbidden.

---

# 1. Purpose

The inscription machine exists to:

- construct an inscription payload for a finalized auction outcome
- persist inscription intent
- optionally broadcast an inscription transaction through the authoritative Bitcoin node
- classify broadcast outcome
- observe confirmation to the configured depth
- record outcomes without repair or concealment

The inscription machine must:

- prefer explicit failure over inferred success
- freeze authority permanently upon ambiguity
- preserve append-only truth
- avoid speculative recovery behavior

---

# 2. Prototype Scope Interaction

The current prototype is divided into demonstration stages as defined in `PROTOTYPE-SCOPE.md`.

## Demo 1

Demo 1 must not require:

- live Ordinals broadcast
- `ord` availability
- Bitcoin Core RPC availability
- wallet availability
- mempool recognition
- confirmation observation
- external SSD availability

For Demo 1:

- `InscriptionIntentRecord` may be persisted
- inscription adapter mode must be `deferred_in_this_slice`
- no `InscriptionBroadcastRecord` is required
- no `InscriptionConfirmationRecord` is required
- no live inscription success may be silently simulated

Auction lifecycle, winner resolution, finalization, sequence advancement, and restart reconstruction must remain demonstrable without live inscription execution.

## Demo 2

Demo 2 may add:

- local Bitcoin Core Testnet integration
- local wallet integration
- inscription transaction construction
- testnet inscription broadcast attempt
- broadcast outcome classification
- confirmation observation

Demo 2 must still preserve the rule that Numbers canonical truth is the append-only Numbers record log.

Bitcoin Testnet, Ordinals, wallet state, mempool state, and confirmation state are external execution surfaces. They do not replace Numbers canonical event records.

---

# 3. Definitions

## Authoritative Node

The authoritative Bitcoin Core node is defined in `chain/CHAIN-INTERACTION.md`.

All broadcast and observation must use only this node.

## Knowledge

Knowledge classification follows `chain/CHAIN-INTERACTION.md`.

## Confirmation Depth

`chain.confirmation_depth` is defined in `chain/CHAIN-INTERACTION.md` and `config/CONFIG-REFERENCE.md`.

## Canonical Number

The canonical number is the number associated with the finalized auction outcome.

---

# 4. Inscription States

The inscription machine has the following states per auction:

- `NotStarted`
- `Inscribing`
- `Inscribed`
- `Ambiguous`

## State Meanings

### `NotStarted`

No committed inscription broadcast exists and no terminal inscription ambiguity exists.

`InscriptionIntentRecord` alone does not move inscription state out of `NotStarted`.

### `Inscribing`

A committed inscription broadcast exists and confirmation has not yet been observed.

### `Inscribed`

The inscription transaction is Known Confirmed to the configured confirmation depth.

### `Ambiguous`

Inscription outcome is uncertain and inscription authority is permanently frozen.

## Terminal States

Terminal inscription states are:

- `Inscribed`
- `Ambiguous`

There is no canonical `Broadcasting` lifecycle state.

Broadcast attempt is an operation, not a lifecycle state.

---

# 5. Destination of Inscribed Output

The inscription transaction must deliver the inscribed output to the finalized destination address.

The finalized destination address must be:

- persisted in the auction finalization records
- immutable once persisted
- deterministically convertible to a valid scriptPubKey under permitted address types

The inscription machine must not:

- choose a destination
- modify the destination
- infer a fallback destination
- reinterpret settlement records
- reinterpret finalization records

If the persisted destination address:

- is missing
- is malformed
- is not permitted under the configured address policy
- cannot be deterministically converted to a scriptPubKey

then:

- inscription broadcast must not proceed
- the condition must be classified via `errors/ERROR-TAXONOMY.md`
- no inscription authority may be consumed

Destination resolution must occur before transaction construction.

---

# 6. Authority Boundary

## 6.1 Authority Consumption Event

Inscription authority is consumed at `broadcast_commit`.

`broadcast_commit` occurs only when both are true:

1. the system has broadcast a candidate inscription transaction using the authoritative node
2. the authoritative node reports the transaction present in its mempool

Before `broadcast_commit`, inscription authority is not consumed.

After `broadcast_commit`, inscription authority is consumed exactly once.

## 6.2 No Semantic Retries After Authority Consumption

After authority consumption, the system must not attempt any action that could create a semantically distinct inscription.

Delivery repricing is permitted only under the controlled RBF rules in this document and only when RBF behavior is included in the active implementation slice.

## 6.3 Ambiguity

If the system cannot determine whether `broadcast_commit` occurred, the result is `ambiguous`.

Ambiguity freezes inscription authority permanently.

Ambiguity must not be repaired by:

- time passing
- operator action
- restart
- later observation
- speculative rebroadcast

---

# 7. Canonical Records

The inscription machine must persist canonical event records in append-only form as defined in `data/PERSISTENCE.md`.

The inscription-related canonical event record types are:

1. `InscriptionIntentRecord`
2. `InscriptionBroadcastRecord`
3. `InscriptionConfirmationRecord`
4. `AmbiguityRecord`

No inscription-specific record type outside this set may be persisted as canonical system truth.

---

## 7.1 InscriptionIntentRecord

`InscriptionIntentRecord` represents persisted inscription intent.

It must be persisted before any broadcast attempt.

## Required Payload Fields

- `intent_time`
- `destination_address`
- `destination_script_pubkey`
- `inscription_payload_hash`
- `inscription_content_type`
- `adapter_mode`
- `settlement_reference`
- `intent_id`

## Field Rules

### `adapter_mode`

Must be one of:

- `deferred_in_this_slice`
- `testnet_ordinals`

### `inscription_content_type`

Must match the value defined in `inscription/INSCRIPTION-FORMAT.md`.

### `inscription_payload_hash`

Must be SHA-256 over the exact payload bytes defined in `inscription/INSCRIPTION-FORMAT.md` for the canonical number.

### `destination_script_pubkey`

Must be derived solely from the finalized destination address.

### `intent_id`

Must be deterministically derived from the other intent fields.

## Rules

- at most one `InscriptionIntentRecord` may exist per auction
- intent must be immutable once persisted
- intent persistence does not consume inscription authority
- intent persistence does not prove broadcast
- intent persistence does not prove confirmation
- intent persistence must not alter auction lifecycle state

For Demo 1, `adapter_mode` must be `deferred_in_this_slice` unless live testnet inscription has been explicitly moved into Demo 1 by a later scope revision.

---

## 7.2 InscriptionBroadcastRecord

`InscriptionBroadcastRecord` represents the classified result of an inscription broadcast attempt.

It may exist only when `InscriptionIntentRecord.adapter_mode = testnet_ordinals`.

It must not exist when `InscriptionIntentRecord.adapter_mode = deferred_in_this_slice`.

## Required Payload Fields

- `broadcast_time`
- `candidate_txid`
- `tx_hex_hash`
- `broadcast_outcome`
- `broadcast_reason`
- `rpc_result`
- `mempool_presence`
- `attempt_index`

## Field Rules

### `broadcast_outcome`

Must be one of:

- `committed`
- `pre_commit_rejected`
- `ambiguous`

### `rpc_result`

Must be one of:

- `success`
- `error`
- `unknown`

### `mempool_presence`

Must be one of:

- `present`
- `absent`
- `unknown`

### `candidate_txid`

- must be non-null when `broadcast_outcome = committed`
- may be null when `broadcast_outcome = pre_commit_rejected`
- may be null when `broadcast_outcome = ambiguous`

### `attempt_index`

- must start at `0`
- must increase by one for each permitted broadcast attempt for the same auction

## Classification Rules

`broadcast_outcome = committed` only if:

- `rpc_result = success`
- `mempool_presence = present`

`broadcast_outcome = pre_commit_rejected` only if:

- the system can determine that `broadcast_commit` did not occur

`broadcast_outcome = ambiguous` when:

- the system cannot determine whether `broadcast_commit` occurred

If `rpc_result = unknown`, then `mempool_presence` must be `unknown`.

## Authority Rules

- `committed` consumes inscription authority
- `pre_commit_rejected` does not consume inscription authority
- `ambiguous` freezes inscription authority permanently

## Retry Rule

This document does not permit automatic retry after `pre_commit_rejected`.

A later implementation slice may explicitly permit bounded retry behavior.

Without that explicit permission, `pre_commit_rejected` records the failed broadcast classification and no further broadcast attempt may be made automatically.

---

## 7.3 InscriptionConfirmationRecord

`InscriptionConfirmationRecord` represents observed canonical inscription confirmation.

It may exist only after an `InscriptionBroadcastRecord` with `broadcast_outcome = committed`.

## Required Payload Fields

- `confirmation_time`
- `confirmed_txid`
- `block_height`
- `block_hash`
- `confirmations`

## Rules

- `confirmed_txid` must equal the committed `candidate_txid` or an explicitly permitted equivalent replacement txid
- `confirmations` must be greater than or equal to `chain.confirmation_depth`
- confirmation does not consume additional authority
- confirmation must not remove or alter prior records
- confirmation is terminal for inscription lifecycle

---

# 8. Broadcast Procedure

Broadcast procedure is included only in implementation slices that enable `testnet_ordinals`.

## 8.1 Preconditions

Before attempting broadcast:

- auction state must be `Finalized`
- an `InscriptionIntentRecord` must exist
- `InscriptionIntentRecord.adapter_mode` must be `testnet_ordinals`
- required wallet funds must be available
- required UTXO reservations must be satisfied
- the constructed transaction must be derivable from `InscriptionIntentRecord`
- destination scriptPubKey must match the finalized destination address

If any precondition is not satisfied, broadcast must not proceed.

The failed condition must be classified via `errors/ERROR-TAXONOMY.md`.

No inscription authority may be consumed.

## 8.2 Steps

1. Construct candidate transaction deterministically from `InscriptionIntentRecord` and wallet state.
2. If computed fee exceeds fee ceiling, do not broadcast.
3. If broadcast does not proceed, persist `InscriptionBroadcastRecord` with `broadcast_outcome = pre_commit_rejected`.
4. If broadcast proceeds, call the authoritative node to broadcast the raw transaction.
5. If the node returns a txid, immediately check mempool presence for that txid via the authoritative node.
6. Persist `InscriptionBroadcastRecord` capturing `rpc_result`, `mempool_presence`, and `broadcast_outcome`.

## 8.3 State Effects

If `broadcast_outcome = committed`:

- inscription state becomes `Inscribing`
- inscription authority is consumed

If `broadcast_outcome = pre_commit_rejected`:

- inscription state remains `NotStarted`
- inscription authority is not consumed

If `broadcast_outcome = ambiguous`:

- inscription state becomes `Ambiguous`
- inscription authority is frozen permanently

---

# 9. Controlled RBF

Controlled RBF is not required for Demo 1.

Controlled RBF is not active unless explicitly included in the active implementation slice.

When controlled RBF is not active, no replacement transaction is permitted.

## 9.1 Equivalence Rule

A replacement transaction is permitted only if it is semantically equivalent to the originally committed transaction.

Two inscription transactions are semantically equivalent only if all of the following are identical:

- canonical number
- content type
- payload hash
- destination scriptPubKey
- inscription content semantics as defined in `inscription/INSCRIPTION-FORMAT.md`
- funding lineage as defined by the reserved UTXO set

Allowed differences are limited to:

- fee amount
- change output value
- sequence numbers required to enable RBF

Any other difference is forbidden.

## 9.2 Replacement Limits

Replacement limits must be defined in the active implementation slice before controlled RBF is enabled.

If no replacement limit is defined, controlled RBF is disabled.

## 9.3 Replacement Ambiguity

If during replacement the system cannot determine mempool presence for either the prior txid or the new txid, the inscription state becomes `Ambiguous`.

---

# 10. Confirmation Observation

Confirmation observation is included only in implementation slices that enable `testnet_ordinals`.

After authority consumption, the system may observe the authoritative node until either:

- a candidate txid becomes Known Confirmed to confirmation depth
- ambiguity is detected

## 10.1 Observed Candidate Set

The candidate set consists only of:

- the committed txid
- explicitly permitted equivalent replacement txids recorded in `InscriptionBroadcastRecord`

No other txids may be considered.

## 10.2 Confirmation Rule

When a candidate txid is Known Confirmed to confirmation depth:

- persist `InscriptionConfirmationRecord`
- inscription state becomes `Inscribed`

Mempool presence alone must not trigger `Inscribed`.

---

# 11. Restart Reconstruction

Restart handling must follow `data/RESTART-RULES.md`.

On startup, the system must reconstruct inscription state from canonical event records only.

## Reconstruction Rules

For a given auction:

1. If `InscriptionConfirmationRecord` exists:
   - inscription state is `Inscribed`.

2. Else if any `AmbiguityRecord` with `authority_scope = inscription` exists:
   - inscription state is `Ambiguous`.

3. Else if any `InscriptionBroadcastRecord` has `broadcast_outcome = ambiguous`:
   - inscription state is `Ambiguous`.

4. Else if any `InscriptionBroadcastRecord` has `broadcast_outcome = committed`:
   - inscription state is `Inscribing`.

5. Else:
   - inscription state is `NotStarted`.

`InscriptionIntentRecord` alone does not change inscription state.

## Post-Restart Chain Checks

Post-restart chain checks are included only in implementation slices that enable `testnet_ordinals`.

If inscription state reconstructs to `Inscribing`:

- the system may query the authoritative node for confirmation status of all candidate txids
- if any candidate is Known Confirmed to the configured confirmation depth, the system must persist `InscriptionConfirmationRecord`
- if node responses are contradictory relative to persisted records, the condition must be classified through `errors/ERROR-TAXONOMY.md`

The system must not attempt a new broadcast after restart unless a later implementation slice explicitly permits that behavior.

---

# 12. Failure Handling

All failures must be classified via `errors/ERROR-TAXONOMY.md`.

Rules:

- any loss of certainty after authority consumption is `Ambiguous`
- any contradiction between a persisted confirmation record and authoritative node truth is `Fatal`
- fee ceiling exceeded is a policy refusal, not a chain failure
- policy refusal before broadcast_commit does not consume inscription authority

---

# Final Rule

If a transition or action is not explicitly permitted by this document:

It is forbidden.

Ambiguity freezes inscription authority permanently.