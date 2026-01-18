# Errors — Numbers

This document defines error classes and system responses.

Errors describe execution failures.  
Errors do not reinterpret outcomes.  
Errors do not create authority.

---

## Error Classification Rule

Every error must belong to exactly one error class.

If an error cannot be classified:
- execution must halt
- authority must not be exercised
- the system must not guess

---

## Error Classes

### 1. Transient

**Definition**  
A failure that occurs before authority is exercised.

**Examples**
- Network timeout before transaction broadcast
- Temporary RPC failure
- Fee estimation failure before signing
- Wallet lock unavailable prior to action

**System Response**
- Retry is permitted
- Authority is not consumed
- State does not advance
- Retries must not vary behavior except timing

---

### 2. Degraded

**Definition**  
A condition where the system remains correct but visibility, performance, or certainty is impaired.

**Examples**
- Indexer lag
- Delayed block propagation
- Monitoring or metrics unavailable
- Partial chain visibility

**System Response**
- Execution may continue
- Observation continues
- Operator alert is required
- No retries that alter outcomes
- No compensating actions

---

### 3. Ambiguous

**Definition**  
A condition where authority may have been exercised but the outcome cannot be determined with certainty.

**Examples**
- Uncertain transaction broadcast
- Conflicting or incomplete inscription visibility
- Loss of confirmation after broadcast
- Inability to rule out competing inscriptions

**System Response**
- Authority is permanently consumed
- No retries, rebroadcasts, or replacements permitted
- State freezes with respect to authority
- Observation only

---

### 4. Fatal

**Definition**  
A failure that violates invariants or produces an illegal state transition.

**Examples**
- Illegal state transition
- Duplicate resolution attempt
- Inscription authority exercised more than once
- State mutation after terminal state
- Invariant violation

**System Response**
- Immediate execution halt
- Manual operator intervention required
- No automatic recovery
- No authority exercised after detection

---

## Escalation Rule

Errors may escalate to a higher class.

Errors must never downgrade.

---

## Final Rule

If an error escalates to a higher class,
it must never downgrade.
