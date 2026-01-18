# State Machine — Numbers

This document defines the executable state machines for Numbers.

It is **normative**.

This document assumes familiarity with:
- STATE-MACHINE-TABLE.md
- CORE-SEQUENCE.md

It specifies:
1. All valid states
2. All allowed transitions
3. Transition triggers and side effects
4. Persistence requirements
5. Restart semantics
6. Pause semantics
7. Illegal transitions
8. Authority loss rules

If behavior is not explicitly permitted here, it is forbidden.

If there is a conflict,
PRD.md, CORE-SEQUENCE.md, and STATE-MACHINE-TABLE.md take precedence.

---

## 1. Scope

This document governs:

- The auction lifecycle for a single number `N`
- The inscription lifecycle that follows auction finalization
- System-level pause behavior at auction boundaries

This document does **not** govern:

- UI presentation
- Wallet internals
- Bitcoin confirmation mechanics beyond required thresholds

---

## 2. State Model

The system consists of three distinct state machines:

1. Auction State Machine (per number)
2. Inscription State Machine (per number, post-auction)
3. System Control State Machine (global)

These state machines interact only at explicitly defined boundaries.

This document defines **state legality and authority boundaries**.
Timing guarantees are defined exclusively in CORE-SEQUENCE.md.

---

## 3. Auction State Machine

Each auction number `N` progresses through the following states
**exactly once and strictly in order**.

### 3.1 Auction States

1. `Scheduled`
2. `Open`
3. `Closed`
4. `AwaitingSettlement`
5. `Finalized`

No other auction states are valid.

Once an auction reaches `Finalized`, its lifecycle is complete.

---

## 3.2 Auction State Definitions

### Scheduled

**Meaning**  
Auction `N` exists but is not accepting bids.

**Entry conditions**
- Auction `N−1` has advanced
- Inter-auction gap timer is active

**Allowed actions**
- No bids accepted
- Auction timing visible but inactive

**Persistence**
- Auction record exists
- Start and end times recorded

**Exit trigger**
- Inter-auction gap expires

**Guard**
- System state is `Running`

---

### Open

**Meaning**  
Auction `N` is actively accepting bids.

**Entry conditions**
- Inter-auction gap has expired
- System state is `Running`

**Allowed actions**
- Accept valid bids
- Reject invalid bids

**Persistence**
- Bid records appended
- Auction open timestamp recorded

**Exit triggers**  
Exactly one of:
1. Auction duration expires
2. Auction bid cap is reached

---

### Closed

**Meaning**  
Auction `N` is closed to new bids and resolving.

**Entry conditions**
- An `Open` exit trigger fires

**Allowed actions**
- Determine winning bid, if any
- Record resolution outcome

**Persistence**
- Resolution record written exactly once

**Exit trigger**
- Resolution computation completes

**Idempotence**
- Resolution must never be recomputed

---

### AwaitingSettlement

**Meaning**  
Auction `N` has resolved but settlement is incomplete.

**Entry conditions**
- Resolution record exists

**Allowed actions**
- Observe settlement completion
- Track settlement deadline

**Persistence**
- Settlement intent recorded
- Settlement deadline recorded

**Exit triggers**  
Exactly one of:
1. Winning bidder settles before deadline
2. Settlement deadline expires
3. No bids were present

---

### Finalized

**Meaning**  
Auction `N` has a final destination.

**Entry conditions**
- Settlement success or failure determined

**Final destinations**
- Winning bidder address
- NullSteward address

**Persistence**
- Finalization record written
- Destination address recorded

Finalization is irreversible.

No auction action is permitted beyond this state.

---

## 3.3 Auction State Transition Table

| From | To | Trigger | Persistence Required |
|----|----|--------|----------------------|
| Scheduled | Open | Inter-auction gap expires | Auction start time |
| Open | Closed | Duration expires or cap reached | Close timestamp |
| Closed | AwaitingSettlement | Resolution written | Resolution record |
| AwaitingSettlement | Finalized | Settlement success | Finalization record |
| AwaitingSettlement | Finalized | Settlement failure or no bids | Finalization record |

No other auction transitions are valid.

---

## 3.4 Illegal Auction Transitions

The following transitions are forbidden:

- `Open → Scheduled`
- `Closed → Open`
- `AwaitingSettlement → Open`
- `Finalized → AwaitingSettlement`
- Any transition that reopens bidding

Illegal transitions are fatal and require operator inspection.

---

## 3.5 Auction Restart Semantics

On process restart:

- **Scheduled**  
  Resume inter-auction timer

- **Open**  
  Resume bidding if end time has not passed  
  Otherwise transition to `Closed`

- **Closed**  
  Resolution must already exist  
  Transition to `AwaitingSettlement`

- **AwaitingSettlement**  
  Resume settlement observation only

- **Finalized**  
  No auction action permitted  
  Inscription lifecycle proceeds independently

---

## 4. Inscription State Machine

The inscription state machine begins only after auction finalization.

Inscription state does not affect auction truth.

---

### 4.1 Inscription States

1. `NotStarted`
2. `Inscribing`
3. `Ambiguous`
4. `Inscribed`

No other inscription states are valid.

---

### 4.2 Inscription State Definitions

#### NotStarted

**Meaning**  
No inscription attempt has begun.

**Entry conditions**
- Auction `N` is finalized

**Allowed actions**
- Reserve wallet resources
- Initiate inscription

---

#### Inscribing

**Meaning**  
The inscription transaction is being constructed or broadcast.

**Entry conditions**
- Inscription process initiated

**Allowed actions**
- Construct payload
- Select UTXO
- Sign transaction
- Attempt broadcast

**Persistence**
- Inscription attempt recorded
- Txid recorded if known

---

#### Ambiguous

**Meaning**  
An inscription transaction exists or cannot be ruled out,
and its outcome cannot be determined with certainty.

**Entry conditions**
- Broadcast occurred or cannot be excluded
- Observation is inconclusive
- Competing inscription cannot be ruled out

**Allowed actions**
- Observe chain state only

**Observation (Normative)**  
Observation is performed exclusively by deterministic system processes
querying external sources of record.

Human judgment does not constitute observation
and must not change system state.

Observation updates knowledge only.
Observation must not restore authority or permit transitions.

This state is non-terminal and may persist indefinitely.

---

#### Inscribed

**Meaning**  
The canonical inscription is complete and known.

**Entry conditions**
- Inscription transaction is observed and accepted

**Persistence**
- Txid recorded
- Satpoint recorded
- Canonical flag set

This is a terminal state.

---

## 4.3 Authority Loss Rules (Normative)

Authority rules apply only to inscription.

**Pre-broadcast failure**
- Transaction construction or signing fails
- No transaction submitted
- Retry is permitted

**Post-broadcast ambiguity**
- A transaction was submitted or submission cannot be excluded

Once ambiguity exists:

- Inscription authority is permanently consumed
- Retry, replacement, or override is forbidden
- Time passing does not restore permission
- Observation is the only permitted activity

---

## 4.4 Inscription Transition Table

| From | To | Trigger |
|----|----|--------|
| NotStarted | Inscribing | Inscription initiated |
| Inscribing | Inscribed | Inscription observed |
| Inscribing | Ambiguous | Broadcast uncertainty detected |
| Ambiguous | Inscribed | Inscription observed |

No other inscription transitions are valid.

---

## 4.5 Inscription Restart Semantics

On restart:

- **NotStarted**  
  Initiation permitted

- **Inscribing**  
  Retry permitted only if a pre-broadcast failure is explicitly recorded

- **Ambiguous**  
  Retry forbidden  
  Observation only

- **Inscribed**  
  No action permitted

---

## 5. System Control State Machine

### 5.1 System States

1. `Running`
2. `Paused`

---

### 5.2 Pause Rules

- The system enters `Paused` only at auction boundaries
- An open auction must never be interrupted
- Pausing prevents `Scheduled → Open`
- Pausing does not affect settlement or inscription

Pause events are durably recorded with timestamp and reason.

---

### 5.3 Resume Rules

- Resume requires explicit operator action
- Resume is permitted only when system state is internally consistent
- Resume transitions the system to `Running`

---

## 6. Safety Properties Enforced

This state machine enforces invariants defined in:

- INVARIANTS.md
- TRANSITION-INVARIANTS.md

Including:

- Auctions resolve exactly once
- Auctions finalize exactly once
- At most one canonical inscription exists
- Bidding never reopens
- Ambiguity never grants authority

Violation of these properties is fatal.

---

## 7. Design Notes

- Absence of bids is a valid outcome
- Failure is treated as outcome, not exception
- Auction truth is independent of inscription success
- Time passing does not restore authority
- Certainty is recorded; uncertainty is preserved

---

## 8. External Assumptions

The following are defined elsewhere and must be stable at implementation time:

1. Settlement confirmation depth
2. Fee selection policy prior to broadcast
3. External observation mechanisms

If any assumption changes, this document must be revised before execution.
