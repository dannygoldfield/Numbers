# Error Taxonomy — Numbers

This document defines how errors are classified and handled in Numbers.

It is normative.

Every error encountered by the system must be classified according to this taxonomy.
Behavior not permitted here is forbidden.

If there is a conflict, STATE-MACHINE.md and CORE-SEQUENCE.md take precedence.

---

## 1. Purpose

The purpose of this taxonomy is to:

- Prevent silent failure
- Prevent unsafe retries
- Prevent authority from being exercised after ambiguity
- Ensure failures are visible, bounded, and non-destructive

Errors are treated as **states of knowledge**, not merely exceptions.

---

## 2. Error Classes

All errors fall into exactly one of the following classes:

1. **Deterministic Errors**
2. **Recoverable Errors**
3. **Ambiguous Errors**
4. **Fatal Errors**
5. **Operator Errors**

No error may belong to more than one class.

---

## 3. Deterministic Errors

### Definition

Errors where:
- The cause is known
- The outcome is known
- No irreversible action has occurred

### Examples

- Invalid bid format
- Bid below minimum
- Auction not open
- Missing configuration at startup
- Transaction construction failure before signing
- Inscription payload serialization failure

### Handling Rules

- Must be rejected immediately
- Must not alter canonical state
- May be retried if state permits
- Must be logged

### Authority Impact

None.

Deterministic errors do not consume authority.

---

## 4. Recoverable Errors

### Definition

Errors where:
- The cause is transient
- No authority has been lost
- Retry is explicitly permitted by the state machine

### Examples

- Temporary RPC timeout
- Bitcoin Core temporarily unavailable
- Network interruption before broadcast
- Wallet locked but unlockable
- Pre-broadcast signing failure

### Handling Rules

- Retry is permitted only if:
  - State allows it
  - No ambiguity has been introduced
- Retries must be bounded
- Retries must be logged

### Authority Impact

None, unless ambiguity is introduced.

If ambiguity appears, reclassify immediately as Ambiguous Error.

---

## 5. Ambiguous Errors (Critical)

### Definition

Errors where:
- An irreversible action may have occurred
- The system cannot determine the outcome with certainty
- Authority may have been partially exercised

Ambiguous errors are **not failures**.
They are **loss of certainty**.

### Examples

- Transaction broadcast may have succeeded, but txid is unknown
- Node crash immediately after `sendrawtransaction`
- Network partition after broadcast
- Conflicting mempool observations
- Incomplete persistence after irreversible action

### Handling Rules

- **All retries are forbidden**
- **No competing action may be taken**
- System must assume the action *may* have occurred
- State must freeze authority
- Observation is the only allowed activity

### Authority Impact

Authority is permanently reduced.

Once ambiguity exists:
- It cannot be repaired by time
- It cannot be overridden by operators
- It cannot be retried away

This applies most critically to inscription broadcast.

---

## 6. Fatal Errors

### Definition

Errors where:
- A core invariant is violated
- Continuing execution risks corrupting history

### Examples

- State transition not permitted by STATE-MACHINE.md
- Attempt to reopen bidding
- Attempt to finalize twice
- Attempt to broadcast a second inscription
- Database corruption
- Inconsistent persisted state

### Handling Rules

- Process must halt immediately
- Error must be logged loudly
- Operator intervention is required

### Authority Impact

Execution authority is suspended until resolved.

Fatal errors protect history by stopping the system.

---

## 7. Operator Errors

### Definition

Errors introduced by human action.

### Examples

- Invalid configuration
- Incorrect key provisioning
- Manual pause at incorrect boundary
- Attempted unsafe override

### Handling Rules

- Must be rejected if they violate invariants
- Must be logged with operator context
- Must never bypass system rules

### Authority Impact

None, unless the action introduces ambiguity or violates invariants,
in which case the error escalates to Fatal.

---

## 8. Error Escalation Rules

Errors may escalate, but never downgrade.

Permitted escalations:

- Deterministic → Recoverable
- Recoverable → Ambiguous
- Any → Fatal

Forbidden transitions:

- Ambiguous → Recoverable
- Ambiguous → Deterministic
- Fatal → Any

Once ambiguity or fatality exists, authority does not return automatically.

---

## 9. Logging Requirements

All errors must be logged with:

- Error class
- Auction ID (if applicable)
- Current state
- Triggering action
- Timestamp

Ambiguous and Fatal errors must additionally include:
- What is known
- What is unknown
- What is now forbidden

Logs are append-only.

---

## 10. Design Principles

- Absence of certainty is not permission
- Time passing is not evidence
- Retrying is a privilege, not a default
- Safety is preferred over liveness
- History must not be rewritten

---

## 11. Summary Table

| Error Class | Retry Allowed | Authority Lost | Requires Halt |
|------------|---------------|----------------|----------------|
| Deterministic | Yes | No | No |
| Recoverable | Yes (bounded) | No | No |
| Ambiguous | No | Yes | No |
| Fatal | No | N/A | Yes |
| Operator | Depends | Depends | Depends |

---

## 12. Non-Goals

This taxonomy does not:
- Optimize for throughput
- Minimize downtime
- Hide failure from users
- Resolve ambiguity heuristically

Its sole purpose is correctness.

---

## 13. Final Rule

If an error cannot be confidently classified:

**Treat it as Ambiguous.**
