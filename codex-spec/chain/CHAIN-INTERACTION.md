# Chain Interaction — Numbers

This document defines how Numbers observes and interacts with the Bitcoin network.

It is normative.

No Bitcoin interaction may occur unless permitted and defined here.

If there is a conflict, documents higher in the authority order prevail.

---

## 1. Purpose

Chain interaction exists to support only these protocol needs:

1. Settlement success recognition
2. Inscription confirmation recognition
3. Recording chain-derived facts required by canonical records

Chain interaction must never:

- create or modify lifecycle semantics
- create or modify authority rules
- treat mempool presence as success
- rewrite or reinterpret canonical records

---

## 2. Authoritative Source

The only authoritative source of chain truth is a Bitcoin Core node
accessible via RPC.

No external indexer may be used to assert canonical truth.

External indexers may be used only for non-canonical operator visibility
and must not drive any state transition or canonical record creation.

If the authoritative Bitcoin Core node is unavailable or returns errors,
handling must follow ERROR-TAXONOMY.md.

---

## 3. Time Source

All timing decisions in chain interaction must use authoritative `server_time`.

Block timestamps must not be used for protocol deadlines.

Settlement deadline comparisons must use `server_time` and the persisted
`settlement_deadline` value.

---

## 4. Knowledge States

Chain interaction must classify knowledge about a transaction or fact as exactly one of:

1. Observed
2. Known
3. Unknown

Definitions:

- Observed  
  The authoritative node reports a candidate fact,
  but knowledge acceptance rules are not yet satisfied.

- Known  
  The fact satisfies the knowledge acceptance rules defined in this document.

- Unknown  
  The system cannot determine the fact with certainty,
  or authoritative node responses are unavailable,
  contradictory,
  or incomplete.

Unknown is conservative.
If classification is uncertain, it must be Unknown.

### 4.1 Mempool Presence Classification

Mempool presence must be classified as exactly one of:

- present
- absent
- unknown

Definitions:

- present  
  The authoritative node reports the transaction in its mempool.

- absent  
  The authoritative node reports the transaction not present in its mempool.

- unknown  
  The node cannot be queried,
  the response is malformed,
  or responses are contradictory.

Mempool presence does not imply confirmation.

---

### Knowledge Acceptance Rule (Normative)

A transaction is Known Confirmed only if:

- the authoritative node reports the transaction included in the active best chain, and
- the transaction's confirmation count is greater than or equal to `chain.confirmation_depth`

The active best chain is defined exclusively by the authoritative node’s current chain tip.

If the authoritative node cannot assert inclusion in the active best chain with certainty,
the transaction is not Known Confirmed.

---

## 6. Settlement Observation Rules

Settlement success recognition must follow SETTLEMENT.md.

Chain interaction rules:

1. Settlement success must be based only on Known Confirmed payment.
2. Settlement success must be determined using `server_time` compared to the persisted `settlement_deadline`.
3. If the payment transaction becomes Known Confirmed after the deadline, settlement must not be recognized as successful.
4. If no payment transaction is Known Confirmed by the deadline, settlement must be recognized as failed.

Broadcast does not count.
Mempool presence does not count.
Partial payments are forbidden.

### 6.1 Deterministic Settlement Address (Normative)

Settlement address derivation must follow wallet/WALLET-SPEC.md.

Each auction must have exactly one unique settlement address.

The settlement address must be deterministically derived from:

- The master seed.
- The canonical number being auctioned.

The settlement address must not be reused for any other auction.

The settlement address must be computable at any time
without reference to prior runtime state.

### 6.2 Confirmation Requirement

A qualifying settlement transaction must satisfy:

- inclusion in the active best chain, and
- confirmations >= chain.confirmation_depth

before `server_time >= settlement_deadline`.

Confirmation after the deadline does not retroactively validate settlement.

### 6.3 Reorg Handling for Settlement

Settlement reorg handling must follow Section 8 (Reorg Handling).

No settlement-specific override logic is permitted.

---

## 7. Inscription Observation Rules

Inscription confirmation recognition must follow the same knowledge acceptance rule:

Inscription is Known Confirmed only if:

- the authoritative node reports the inscription transaction in the active best chain, and
- confirmations are greater than or equal to `chain.confirmation_depth`

The system must not mark an inscription as confirmed based on mempool presence.

The inscription machine defines how candidate inscription txids are obtained.
This document defines only confirmation recognition.

---

## 8. Reorg Handling

Reorgs are treated as changes to authoritative chain truth.

If confirmation has not yet been persisted:

- the system must revert to observation.

If confirmation has already been persisted in a canonical record,
and later observation indicates the asserted txid is not in the active best chain
or does not satisfy confirmation depth,
the system is in contradictory state.

Contradictory state must be classified as Fatal and execution must halt.

No canonical record may be rewritten to repair reorg effects.

---

## 9. Mempool Usage Rules

Mempool data is non-authoritative.

Mempool may be used only for:

- operator visibility
- diagnostics
- logging

Mempool must not be used to:

- recognize settlement success
- recognize inscription success
- create or justify any state transition
- resolve ambiguity by inference

Conflicting mempool observations across sources must not be used to resolve truth.

If conflicting mempool observations contribute to uncertainty after an irreversible action,
classification must be Ambiguous per ERROR-TAXONOMY.md.

---

## 10. Node Failure Classifications

The following node interaction failures must be classified using ERROR-TAXONOMY.md:

- RPC timeout
- RPC connection failure
- node unavailable
- malformed RPC response
- contradictory responses across restart boundaries

Classification rules:

1. If no irreversible action has occurred and the failure is transient,
   retry is permitted only under a deterministic, finite retry bound defined in configuration.

2. If an irreversible action has occurred or cannot be ruled out,
   the error must be classified as Ambiguous and retries are forbidden.

3. If chain interaction contradicts persisted canonical records,
   the error must be classified as Fatal and execution must halt.

Retry bounds must be:

- finite
- deterministic
- logged
- non-branching

---

## 11. Final Rule

Only Known Confirmed facts may be used to create confirmation-bearing canonical records.

When certainty is lost, the system must prefer Unknown over inference.

No Bitcoin interaction may occur outside the rules of this document.