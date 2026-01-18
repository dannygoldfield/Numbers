# Operational Runbook — Numbers

This document defines how Numbers is operated in production.

It is **normative**.

Operational actions are an authority surface.
Human intervention **must** be constrained as strictly as code.

If something feels wrong, **pause first**.

If there is a conflict,
INVARIANTS.md, ERROR-TAXONOMY.md, PERSISTENCE.md,
RESTART-RULES.md, and LIMITS-AND-CIRCUIT-BREAKERS.md take precedence.

---

## Modal Language Rule (Normative)

In this document and all normative specifications:

- **must / must not** define obligations
- **only / exactly once / at most once** define bounds
- **may** is permitted **only** to describe uncertainty of knowledge,
  never permission, intent, or policy choice

The following terms are forbidden in normative contexts:

- possibly
- likely
- eventually
- for now
- TBD

Any normative statement using forbidden modal language is invalid.

---

## 1. Operating Assumptions (Normative)

The following assumptions **must always hold**:

- An open auction **must never** be interrupted
- Pauses **must occur only** at auction boundaries
- Settlement and inscription **must not** block auction sequencing
- Uncertainty **must** be treated as risk, not as permission
- Authority **must not** be reused to repair outcomes
- Absence of knowledge **must not** be treated as success

The default response to ambiguity is:

> **Stop starting new auctions.**

---

## 2. Continuous Monitoring (Normative)

The operator **must** continuously observe the following signals.

Failure to observe these signals
**is itself an operational failure**.

### 2.1 Chain and Network

- Bitcoin node availability
- Chain tip height
- Chain lag relative to peers
- Initial block download (IBD) status
- Observed mempool fee rates

### 2.2 Funds

- Operational wallet balance
- Reserved funds
- Safety margin relative to configured limits

### 2.3 Auctions

- Current auction number
- Auction lifecycle state (`open`, `closed`)
- Global system state (`running`, `paused`)

### 2.4 Settlement and Inscription

- Count of pending settlements
- Count of pending inscriptions
- Age of oldest pending settlement
- Age of oldest pending inscription

### 2.5 System Health

- Error rate by error class
- Repeated or escalating errors
- Forbidden or unexpected state transitions

---

## 3. Incident Classes and Required Responses (Normative)

For each incident class, the response is **mandatory**.
Improvisation, optimization, or deviation is forbidden.

---

### 3.1 High Fee Environment

**Condition**
- Observed fee levels exceed configured tolerance

**Required Response**
1. Schedule pause at the next auction boundary
2. Allow any open auction to resolve normally
3. Defer inscription broadcast if permitted by the state machine
4. Continue observing fee conditions
5. Resume auctions only after limits are satisfied

**Forbidden**
- Extending auctions
- Altering auction timing
- Retrying inscription after ambiguity

---

### 3.2 Settlement Backlog

**Condition**
- Pending settlements exceed configured limits
- Settlement age exceeds configured bounds

**Required Response**
1. Schedule pause at the next auction boundary
2. Allow existing settlements to finalize or fail naturally
3. Verify wallet balance and reserve requirements
4. Resume auctions only after backlog is within limits

**Forbidden**
- Skipping settlement records
- Reopening auctions
- Reassigning destinations

---

### 3.3 Inscription Failure or Ambiguity

**Condition**
- Broadcast failure
- Confirmation uncertainty
- Conflicting or incomplete observations

**Required Response**
1. **Do not retry**
2. Persist failure or ambiguity immediately
3. Classify the error per ERROR-TAXONOMY.md
4. Cease all further inscription attempts for the affected auction
5. Advance the sequence only if explicitly permitted by the state machine

**Forbidden**
- Heuristic repair
- Alternate inscription attempts after ambiguity
- Retrying broadcast without explicit authority

---

### 3.4 Node Failure or Desynchronization

**Condition**
- Node unavailable
- Node desynced
- Conflicting chain state observations

**Required Response**
1. Schedule pause at the next auction boundary
2. Restore or restart the node
3. Verify chain tip correctness
4. Verify wallet and persisted state consistency
5. Resume auctions only after verification completes

**Forbidden**
- Accepting bids on stale chain state
- Inferring correctness from partial sync
- Advancing auctions without verification

---

## 4. Pause Procedure (Normative)

A pause **must** follow these steps exactly:

1. Initiate pause (manual or automatic)
2. Confirm no auction is currently accepting bids
3. Verify paused status is externally visible
4. Persist pause event with timestamp and reason
5. Investigate and resolve the triggering condition

Pausing:

- **does not** alter past outcomes
- **does not** grant authority
- **does not** resolve ambiguity

---

## 5. Resume Procedure (Normative)

Resuming auctions **must** follow these steps exactly:

1. Verify all monitored signals are within defined limits
2. Verify no unresolved ambiguous state exists
3. Verify persisted state integrity
4. Verify configuration and secrets alignment
5. Resume auctions manually
6. Observe the first full auction cycle closely

If any verification fails,
resumption **must not** occur.

---

## 6. Restart Discipline (Normative)

On any restart:

- Persisted state **must** be treated as authoritative
- In-memory state **must** be discarded
- Missing or contradictory records **must** cause immediate halt
- No resolution, settlement, or inscription **may** be recomputed

Restarting is not recovery.  
Restarting is reconstruction.

---

## 7. Escalation Rule (Normative)

If the operator cannot confidently explain:

- the current auction number
- the current system state
- what authority has already been consumed

then:

> **Auctions must remain paused.**

Confusion is not a recoverable condition.

---

## 8. Design Principle

Humans are the highest-risk component.

This runbook exists to:

- reduce discretion
- prevent panic
- forbid “hero fixes”

There is no urgency premium.

Correctness compounds.  
Mistakes do not.
