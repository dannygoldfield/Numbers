# Chain Interaction: Numbers

This document defines how Numbers observes and interacts with the Bitcoin network.

It is normative.

Authority precedence is defined exclusively in `AUTHORITY-ORDER.md`.

No Bitcoin interaction may occur unless permitted and defined here.

No Bitcoin interaction is required for Demo 1 unless explicitly added by a later scope revision or implementation slice.

If a behavior is not specified here, it is forbidden.

---

# 1. Purpose

Chain interaction exists only to support explicitly defined protocol needs:

1. Settlement success recognition.
2. Inscription broadcast commit classification.
3. Inscription confirmation recognition.
4. Recording chain-derived facts required by canonical event records.

Chain interaction must never:

- create lifecycle semantics
- modify lifecycle semantics
- create authority rules
- modify authority rules
- rewrite canonical event records
- reinterpret canonical event records
- substitute external observation for persisted Numbers truth

Bitcoin chain observation is an external execution surface.

Numbers canonical truth remains the append-only Numbers canonical event record log.

---

# 2. Prototype Scope Interaction

The current prototype stages are defined in `PROTOTYPE-SCOPE.md`.

## Demo 1

Demo 1 must not require:

- Bitcoin Core RPC availability
- wallet availability
- mempool recognition
- confirmation observation
- live settlement observation
- live Ordinals broadcast
- external SSD availability

For Demo 1:

- chain interaction is not part of the critical auction demo path
- auction correctness must remain demonstrable without live Bitcoin interaction
- no chain-derived success may be silently simulated
- no chain-derived confirmation may be silently simulated

## Demo 2

Demo 2 may add:

- local Bitcoin Core Testnet interaction
- wallet interaction
- settlement observation
- inscription broadcast commit classification
- inscription confirmation observation

Demo 2 must still preserve the rule that Numbers canonical truth is the append-only Numbers canonical event record log.

---

# 3. Authoritative Chain Source

The only authoritative source of chain truth is a Bitcoin Core node accessible via RPC.

No external indexer may be used to assert canonical truth.

External indexers are permitted only for non-canonical operator visibility and must not drive:

- lifecycle transition
- authority consumption
- canonical event record creation
- restart reconstruction
- settlement success
- inscription confirmation

If the authoritative Bitcoin Core node is unavailable or returns errors, handling must follow `errors/ERROR-TAXONOMY.md`.

---

# 4. Time Source

All timing decisions in chain interaction must use authoritative `server_time`.

Block timestamps must not be used for protocol deadlines.

Settlement deadline comparisons must use:

- authoritative `server_time`
- the persisted `settlement_deadline` value

---

# 5. Knowledge States

Chain interaction must classify knowledge about a transaction or chain fact as exactly one of:

1. `Observed`
2. `Known`
3. `Unknown`

## `Observed`

The authoritative node reports a candidate fact, but knowledge acceptance rules are not yet satisfied.

## `Known`

The fact satisfies the knowledge acceptance rules defined in this document.

## `Unknown`

The system cannot determine the fact with certainty.

A fact is `Unknown` if authoritative node responses are:

- unavailable
- malformed
- contradictory
- incomplete
- not safely classifiable

If classification is uncertain, classification must be `Unknown`.

---

# 6. Known Confirmed Rule

A transaction is `Known Confirmed` only if both are true:

1. the authoritative node reports the transaction included in the active best chain
2. the transaction confirmation count is greater than or equal to `chain.confirmation_depth`

The active best chain is defined exclusively by the authoritative node’s current chain tip.

If the authoritative node cannot assert inclusion in the active best chain with certainty, the transaction is not `Known Confirmed`.

Mempool presence does not imply confirmation.

---

# 7. Mempool Presence Classification

Mempool presence must be classified as exactly one of:

- `present`
- `absent`
- `unknown`

## `present`

The authoritative node reports the transaction in its mempool.

## `absent`

The authoritative node reports the transaction not present in its mempool.

## `unknown`

The node cannot be queried, the response is malformed, or responses are contradictory.

Mempool presence must not be used to recognize settlement success.

Mempool presence must not be used to recognize inscription confirmation.

Mempool presence is permitted only for `broadcast_commit` classification as defined in `inscription/INSCRIPTION-MACHINE.md` and `core/AUTHORITY-CONSUMPTION.md`.

---

# 8. Settlement Observation Rules

Settlement observation is included only in implementation slices that enable live chain settlement observation.

Settlement success recognition must follow `bidding/SETTLEMENT.md`.

When live chain settlement observation is enabled:

1. Settlement success must be based only on `Known Confirmed` payment.
2. Settlement success must be determined using `server_time` compared to persisted `settlement_deadline`.
3. If payment becomes `Known Confirmed` after the deadline, settlement must not be recognized as successful.
4. If no qualifying payment is `Known Confirmed` by the deadline, settlement must be recognized as failed.

Broadcast does not count.

Mempool presence does not count.

Partial payments are forbidden unless explicitly permitted by a later specification revision.

If live chain settlement observation is not included in the active implementation slice, the system must not silently simulate settlement success.

---

# 9. Settlement Address Rules

Settlement address derivation must follow `wallet/WALLET-SPEC.md`.

Each auction must have exactly one unique settlement address when live settlement observation is enabled.

The settlement address must be deterministically derived from:

- the configured wallet derivation material
- the canonical number being auctioned

The settlement address must not be reused for another auction.

The settlement address must be computable without reference to prior runtime memory.

Settlement address derivation is not required for Demo 1 unless live settlement observation is explicitly added to Demo 1 by a later scope revision or implementation slice.

---

# 10. Settlement Confirmation Requirement

A qualifying settlement transaction must satisfy:

- inclusion in the active best chain
- confirmations greater than or equal to `chain.confirmation_depth`
- confirmation recognized before `server_time >= settlement_deadline`

Confirmation after the deadline does not retroactively validate settlement.

Reorg handling for settlement must follow this document.

No settlement-specific override logic is permitted.

---

# 11. Inscription Observation Rules

Inscription observation is included only in implementation slices that enable `testnet_ordinals`.

Inscription confirmation recognition must follow the `Known Confirmed` rule.

An inscription transaction is `Known Confirmed` only if both are true:

1. the authoritative node reports the inscription transaction in the active best chain
2. confirmations are greater than or equal to `chain.confirmation_depth`

The system must not mark an inscription as confirmed based on mempool presence.

The inscription machine defines how candidate inscription txids are obtained.

This document defines only confirmation recognition and mempool presence classification.

---

# 12. Reorg Handling

Reorgs are treated as changes to authoritative chain truth.

If confirmation has not yet been persisted:

- the system must return to observation if observation is permitted by the active implementation slice

If confirmation has already been persisted in a canonical event record, and later observation indicates the asserted txid is not in the active best chain or does not satisfy confirmation depth:

- the system is in contradictory state
- the condition must be classified as `Fatal`
- execution must halt

No canonical event record may be rewritten to repair reorg effects.

---

# 13. Node Failure Classification

The following node interaction failures must be classified using `errors/ERROR-TAXONOMY.md`:

- RPC timeout
- RPC connection failure
- node unavailable
- malformed RPC response
- contradictory RPC response
- contradictory responses across restart boundaries

Classification rules:

1. If no irreversible action has occurred and the failure affects only read-only observation:
   - the result must be classified as `Unknown`
   - no lifecycle transition may be inferred

2. If an irreversible action has occurred or cannot be ruled out:
   - the condition must be classified as `Ambiguous`
   - retry is forbidden unless explicitly permitted by the governing inscription specification and active implementation slice

3. If chain interaction contradicts persisted canonical event records:
   - the condition must be classified as `Fatal`
   - execution must halt

---

# 14. Retry Rule

No automatic retry exists unless explicitly permitted by the active implementation slice.

Configuration must not create retry behavior.

Configuration must not alter authority semantics.

Configuration must not turn an unspecified retry into a permitted retry.

If a later implementation slice permits bounded retry, the retry rule must define:

- eligible operation
- maximum attempt count
- stopping condition
- authority effect
- persistence requirement
- error classification

Without those explicit rules, retry is forbidden.

---

# 15. Mempool Usage Rules

Mempool data is not confirmation truth.

Mempool data must not be used to:

- recognize settlement success
- recognize inscription confirmation
- infer finality
- resolve ambiguity by inference
- repair missing canonical event records

Mempool data is permitted for exactly one authority-relevant purpose:

- determining whether `broadcast_commit` occurred for an inscription broadcast attempt

If mempool presence cannot be determined with certainty after an inscription broadcast attempt, the broadcast outcome must be classified according to `inscription/INSCRIPTION-MACHINE.md`.

Conflicting mempool observations must not be used to resolve truth.

---

# 16. Configuration Boundary

Configuration may define operational parameters such as:

- Bitcoin Core RPC URL
- network
- confirmation depth
- wallet name
- chain observation enabled or disabled
- RPC timeout value

Configuration must not define:

- lifecycle semantics
- authority consumption
- retry permission
- settlement outcome rules
- inscription outcome rules
- restart reconstruction semantics

---

# Final Rule

Only `Known Confirmed` facts may be used to create confirmation-bearing canonical event records.

Mempool presence may be used only for `broadcast_commit` classification.

When certainty is lost, the system must prefer `Unknown` or `Ambiguous`, according to the governing error and authority rules.

No Bitcoin interaction may occur outside the rules of this document.