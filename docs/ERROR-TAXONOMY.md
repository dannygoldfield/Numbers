# Error Taxonomy — Numbers

This document defines how errors are classified and handled in Numbers.

It is **normative**.

Every error encountered by the system **must** be classified according to this taxonomy.  
Behavior not explicitly permitted here is forbidden.

If there is a conflict,
PRD.md, CORE-SEQUENCE.md, STATE-MACHINE-TABLE.md, STATE-MACHINE.md,
and INVARIANTS.md take precedence.

---

## Modal Language Rule (Normative)

In this document and all normative specifications:

- **must / must not** define obligations
- **only / exactly once / at most once** define bounds
- **may** is permitted **only** to describe uncertainty of knowledge,
  never to grant permission, select policy, or authorize action

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

- Prevent silent failure
- Prevent unsafe retries
- Prevent authority from being exercised after ambiguity
- Ensure failures are visible, bounded, and non-destructive

Errors are treated as **states of knowledge**, not merely runtime exceptions.

---

## 2. Error Classes

All errors fall into **exactly one** of the following classes:

1. **Deterministic Errors**
2. **Recoverable Errors**
3. **Ambiguous Errors**
4. **Fatal Errors**
5. **Operator Errors**

An error must never belong to more than one class at the same time.

---

## 3. Deterministic Errors

### Definition

Errors where:

- The cause is known
- The outcome is known
- No irreversible action has occurred
- Authority has not been exercised

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
- Must be logged
- Retry is permitted **only if** explicitly allowed by the current state machine

### Authority Impact

None.

Deterministic errors do not consume authority.

---

## 4. Recoverable Errors

### Definition

Errors where:

- The cause is transient
- Authority has not been exercised
- Retry is explicitly permitted by the state machine

### Examples

- Temporary RPC timeout
- Bitcoin Core unavailable
- Network interruption before broadcast
- Wallet locked but unlockable
- Pre-broadcast signing failure

### Handling Rules (Normative)

Retry is permitted **only when all of the following are true**:

- The current state explicitly permits retry
- No irreversible action has occurred
- No ambiguity has been introduced

Retry constraints:

- Retries **must** be bounded by configuration
- The retry bound **must** be explicit and finite
- The retry bound **must** be defined per action type
- Retries **must** be logged individually

If the retry bound is exceeded:

- The error **must** escalate to **Fatal**
- Execution **must** halt
- Authority **must not** be exercised further

### Authority Impact

None, unless escalation occurs.

If ambiguity is introduced at any point,
the error **must be reclassified immediately** as an Ambiguous Error.

---

## 5. Ambiguous Errors (Critical)

### Definition

Errors where:

- An irreversible action has occurred **or cannot be ruled out**
- The system cannot determine the outcome with certainty
- Authority has been partially or fully exercised

Ambiguous errors are **not failures**.  
They represent **loss of certainty**.

### Examples

- Transaction broadcast outcome unknown
- Node crash immediately after `sendrawtransaction`
- Network partition after broadcast
- Conflicting mempool observations
- Incomplete persistence following an irreversible action

### Handling Rules (Normative)

Once classified as Ambiguous:

- All retries are forbidden
- No competing or alternative action is permitted
- Authority must be treated as permanently consumed
- Ambiguity must be persisted durably
- Observation is the **only** permitted activity

### Authority Impact

Authority is permanently reduced.

Once ambiguity exists:

- Time passing does not restore authority
- Operator action does not restore authority
- External confirmation does not justify retries

This rule applies most critically to inscription broadcast.

---

## 6. Fatal Errors

### Definition

Errors where:

- A core invariant is violated
- Continuing execution risks corrupting history

### Examples

- State transition not permitted by STATE-MACHINE.md
- Attempt to reopen bidding
- Attempt to finalize more than once
- Attempt to broadcast a second inscription
- Database corruption
- Contradictory persisted state

### Handling Rules (Normative)

- Execution **must** halt immediately
- Error **must** be logged loudly and durably
- Operator intervention is required

### Authority Impact

Execution authority is suspended.

Fatal errors exist to protect history by stopping the system.

---

## 7. Operator Errors

### Definition

Errors introduced by human action outside the automated execution path.

### Examples

- Invalid configuration
- Incorrect key provisioning
- Manual pause at an unsafe boundary
- Attempted override of system rules

### Handling Rules (Normative)

- Must be rejected if they violate invariants
- Must be logged with operator identity and context
- Must never bypass or weaken system rules

### Authority Impact

None, unless the action introduces ambiguity or violates invariants.

In such cases, the error **must escalate** to Ambiguous or Fatal.

---

## 8. Error Escalation Rules

Errors **may escalate but must never downgrade**.

Permitted escalations:

- Deterministic → Recoverable
- Recoverable → Ambiguous
- Any → Fatal

Forbidden transitions:

- Ambiguous → Recoverable
- Ambiguous → Deterministic
- Fatal → Any

Once ambiguity or fatality exists,
authority does not return automatically.

---

## 9. Logging Requirements

All errors **must** be logged with:

- Error class
- Auction ID (if applicable)
- Current state
- Triggering action
- Timestamp

Ambiguous and Fatal errors **must additionally** include:

- What is known
- What is unknown
- What actions are now forbidden

Logs are append-only and non-authoritative.

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
| Deterministic | State-gated | No | No |
| Recoverable | State-gated and bounded | No | On bound exceed |
| Ambiguous | No | Yes | No |
| Fatal | No | N/A | Yes |
| Operator | State-gated | Depends | Depends |

---

## 12. Non-Goals

This taxonomy does not:

- Optimize throughput
- Minimize downtime
- Hide failure from users
- Resolve ambiguity heuristically

Its sole purpose is correctness.

---

## 13. Final Rule

If an error cannot be confidently classified:

**Classify it as Ambiguous.**
