# Error Taxonomy — Numbers

This document defines how errors are classified and handled in Numbers.

It is normative.

Every error encountered by the system must be classified
according to this taxonomy.

Behavior not explicitly permitted here is forbidden.

If there is a conflict,
CORE-SEQUENCE.md, STATE-MACHINE-TABLE.md,
STATE-MACHINE.md, and INVARIANTS.md take precedence.

---

## Modal Language Rule (Normative)

In this document and all normative specifications:

- must / must not define obligations
- only / exactly once / at most once define bounds
- may is permitted only to describe uncertainty of knowledge,
  never to grant permission or authorize action

The following terms are forbidden in normative contexts:

- possibly
- likely
- eventually
- for now
- TBD

Any normative statement using forbidden modal language is invalid.

---

## 1. Purpose

This taxonomy exists to:

- prevent silent failure
- prevent unsafe retries
- prevent authority reuse
- preserve ambiguity when certainty is lost
- halt execution when invariants are violated

Errors are treated as states of knowledge,
not merely runtime exceptions.

---

## 2. Error Classes

All errors must belong to exactly one class:

1. Deterministic
2. Recoverable
3. Ambiguous
4. Fatal
5. Operator

An error must never belong to more than one class.

If classification is uncertain,
the error must be classified as Ambiguous.

---

## 3. Deterministic Errors

### Definition

An error is Deterministic when:

- the cause is known
- no irreversible action has occurred
- authority has not been exercised
- no ambiguity exists

### Examples

- Invalid bid format
- Bid below minimum
- Auction not open
- Invalid configuration at startup
- Transaction construction failure before signing
- Inscription payload serialization failure

### Handling Rules (Normative)

- The action must be rejected immediately
- Canonical state must not be altered
- The error must be logged
- Retry is permitted only if explicitly allowed
  by the current state machine

### Authority Impact

None.

Deterministic errors do not consume authority.

---

## 4. Recoverable Errors

### Definition

An error is Recoverable when:

- the cause is transient
- no irreversible action has occurred
- retry is explicitly permitted by the state machine

### Examples

- Temporary RPC timeout
- Bitcoin node unavailable
- Network interruption before broadcast
- Wallet locked but unlockable
- Pre-broadcast signing failure

### Handling Rules (Normative)

Retry is permitted only if:

- the current lifecycle state permits retry
- no irreversible action has occurred
- no ambiguity has been introduced

Retry constraints:

- retries must be bounded
- retry bounds must be defined in code
- retry bounds must be finite
- retry bounds must not be configurable
- each retry attempt must be logged

If the retry bound is exceeded:

- the error must escalate to Fatal
- execution must halt
- no further authority-bearing action is permitted

If at any point an irreversible action cannot be ruled out:

- the error must be reclassified immediately as Ambiguous

### Authority Impact

None, unless ambiguity is introduced.

---

## 5. Ambiguous Errors

### Definition

An error is Ambiguous when:

- an irreversible action has occurred,
  or cannot be ruled out
- the outcome cannot be determined with certainty
- authority must be treated as exercised

Ambiguity represents loss of certainty.

### Examples

- Broadcast outcome unknown
- Crash immediately after broadcast attempt
- Network partition after broadcast
- Conflicting mempool observations
- Persistence failure after irreversible action

### Handling Rules (Normative)

Retry is permitted only if:

- the current lifecycle state permits retry
- no irreversible action has occurred
- no ambiguity has been introduced

Retry constraints:

- retries must be deterministic
- retries must be finite
- retry bounds must be defined in code
- retry bounds must not be configurable
- each retry attempt must be logged

If the retry bound is exceeded:

- if no irreversible action has occurred,
  the error must escalate to Fatal only if forward progress is impossible
- if an irreversible action cannot be ruled out,
  the error must be reclassified immediately as Ambiguous

Execution must halt only when classified as Fatal.

### Authority Impact

Authority consumption under Ambiguous classification
is limited strictly to the authority scope
associated with the triggering irreversible action.

In Numbers, this scope is:

- inscription authority for the canonical number involved.

Ambiguity must not affect unrelated auctions
or unrelated authority scopes.

---

## 6. Fatal Errors

### Definition

An error is Fatal when:

- a core invariant is violated
- persisted state is contradictory
- continued execution risks corrupting history
- retry bounds have been exceeded

### Examples

- Invalid state transition
- Attempt to reopen bidding
- Duplicate authority-bearing record
- Attempt to consume authority twice
- Corrupted canonical records
- Inconsistent restart reconstruction

### Handling Rules (Normative)

- Execution must halt immediately
- The error must be logged durably
- No further transitions may be evaluated
- Operator intervention is required

### Authority Impact

Authority state remains as persisted.

Fatal errors protect history by halting execution.

### Canonical Contradiction Rule

If persisted canonical records contradict
authoritative chain truth
as defined in chain/CHAIN-INTERACTION.md,
the error must be classified as Fatal.

Examples include:

- Persisted confirmation record for a txid
  not present in the active best chain.
- Confirmation depth previously asserted
  no longer satisfied.

Contradictions must halt execution immediately.
Canonical records must not be rewritten to repair contradiction.

---

## 7. Operator Errors

### Definition

An error caused by external human action.

### Examples

- Invalid configuration
- Incorrect key provisioning
- Unsafe manual pause attempt
- Attempted override of lifecycle rules

### Handling Rules (Normative)

- Operator actions must be validated mechanically
- Violations of invariants must be rejected
- The error must be logged with operator context

If operator action introduces ambiguity:

- error must escalate to Ambiguous

If operator action violates invariants:

- error must escalate to Fatal

### Authority Impact

None, unless escalation occurs.

---

## 8. Escalation Rules

Errors may escalate.
Errors must never downgrade.

Permitted escalations:

- Deterministic → Fatal
- Recoverable → Ambiguous
- Recoverable → Fatal
- Operator → Ambiguous
- Operator → Fatal

Ambiguous must not downgrade.

Fatal must not downgrade.

Once ambiguity or fatal state exists,
authority does not return.

---

## 9. Logging Requirements

All errors must log:

- error class
- auction number (if applicable)
- lifecycle state
- triggering action
- timestamp

Ambiguous and Fatal errors must additionally log:

- what is known
- what is unknown
- which actions are now forbidden

Logs are append-only and non-authoritative.

---

## 10. Design Principles

- absence of certainty is not permission
- retry is not default
- time passing is not evidence
- safety overrides liveness
- history must not be rewritten

---

## 11. Summary Table

| Error Class   | Retry Allowed | Authority Lost | Requires Halt |
|--------------|--------------|----------------|----------------|
| Deterministic | State-gated | No | No |
| Recoverable   | State-gated and bounded | No | On bound exceed |
| Ambiguous     | No | Yes (scope-bound) | No |
| Fatal         | No | No additional loss | Yes |
| Operator      | State-gated | Escalation-dependent | Escalation-dependent |

---

## 12. Non-Goals

This taxonomy does not:

- optimize throughput
- minimize downtime
- conceal failure
- heuristically resolve ambiguity

Its purpose is correctness.

---

## Final Rule

If an error cannot be confidently classified:

It must be classified as Ambiguous.