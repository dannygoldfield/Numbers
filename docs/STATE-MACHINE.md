# State Machine — Numbers

This document defines the executable state machine for Numbers.

It specifies:
1. All valid states
2. All allowed transitions
3. Transition triggers and side effects
4. Persistence requirements
5. Restart semantics
6. Pause semantics
7. Illegal transitions
8. Authority loss rules

If behavior is not permitted here, it is forbidden.

This document is normative.
If there is a conflict, CORE-SEQUENCE.md and PRD.md take precedence.

---

## 1. Scope

This state machine governs:

- The auction lifecycle for a single number N
- System-level pause behavior at auction boundaries

It does not govern:
- UI presentation
- Wallet internals
- Bitcoin confirmation mechanics beyond required thresholds

---

## 2. State Model

The system operates two coupled state machines:

1. Auction State Machine (per number)
2. System Control State Machine (global)

They interact only at explicitly defined boundaries.

---

## 3. Auction State Machine

Each auction number N progresses through the following states exactly once and in order.

### 3.1 Auction States

1. `Scheduled`
2. `Open`
3. `Closed`
4. `AwaitingSettlement`
5. `Finalized`
6. `Inscribing`
7. `Inscribed`

There are no other valid auction states.

---

## 4. Auction State Definitions

### 4.1 Scheduled

**Meaning**  
Auction N exists but is not yet accepting bids.

**Entry conditions**
- Auction N−1 has advanced
- Inter-auction gap timer is active

**Allowed actions**
- No bids accepted
- Auction timer visible but inactive

**Persistence**
- Auction record exists
- Start and end times recorded

**Exit trigger**
- Inter-auction gap expires
- System is not paused

---

### 4.2 Open

**Meaning**  
Auction N is actively accepting bids.

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

### 4.3 Closed

**Meaning**  
Auction N is closed to new bids and resolving.

**Entry conditions**
- Open auction exit trigger fires

**Allowed actions**
- Determine winning bid if any
- Record resolution outcome

**Persistence**
- Resolution record written exactly once

**Exit trigger**
- Resolution computation completes

**Idempotence rule**  
If re-entered after restart, resolution must not be recomputed.

---

### 4.4 AwaitingSettlement

**Meaning**  
Auction N has resolved, but settlement has not yet finalized.

**Entry conditions**
- Resolution record exists

**Allowed actions**
- Await settlement confirmation
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

### 4.5 Finalized

**Meaning**  
Auction N has a final destination.

**Entry conditions**
- Settlement success or failure determined

**Final destinations**
- Winning bidder address
- Null steward address

**Persistence**
- Finalization record written
- Destination address recorded

**Exit trigger**
- Inscription process initiated

Finalization is irreversible.

---

### 4.6 Inscribing

**Meaning**  
The canonical inscription for auction N is being constructed or has been broadcast.

**Entry conditions**
- Auction N is finalized
- Required wallet resources are reserved

**Allowed actions**
- Construct inscription payload
- Select UTXO
- Sign transaction
- Attempt broadcast

**Persistence**
- Inscription attempt recorded
- Txid recorded if known

---

#### Authority Loss Rules (Normative)

Inscribing encompasses two distinct phases:

**1. Pre-broadcast failure (safe)**
- Transaction construction or signing fails
- No transaction has been submitted to the network
- Retry is permitted

**2. Post-broadcast ambiguity (unsafe)**
- A transaction may have been broadcast
- Its fate cannot be determined with certainty
- The system must assume it may exist

Once post-broadcast ambiguity exists:

- Authority to create a competing inscription is permanently lost
- No retry, replacement, or override is permitted
- Time passing does not restore permission
- Observation is the only allowed action

Inscribing may persist indefinitely in this condition.

---

### 4.7 Inscribed

**Meaning**  
The canonical inscription for auction N is complete and known.

**Entry conditions**
- Inscription transaction is observed and accepted

**Persistence**
- Txid recorded
- Satpoint recorded
- Canonical flag set

This is a terminal state.

---

## 5. Auction State Transition Table

| From | To | Trigger | Persistence Required |
|----|----|--------|----------------------|
| Scheduled | Open | Inter-auction gap expires | Auction start time |
| Open | Closed | Duration expires or cap reached | Close timestamp |
| Closed | AwaitingSettlement | Resolution written | Resolution record |
| AwaitingSettlement | Finalized | Settlement success | Finalization record |
| AwaitingSettlement | Finalized | Settlement failure or no bids | Finalization record |
| Finalized | Inscribing | Inscription initiated | Inscription intent |
| Inscribing | Inscribed | Inscription observed | Txid and satpoint |

No other transitions are valid.

---

## 6. Illegal Transitions

The following transitions are explicitly forbidden:

- Open → Scheduled
- Closed → Open
- AwaitingSettlement → Open
- Finalized → AwaitingSettlement
- Inscribed → any other state
- Any transition that reopens bidding
- Any competing inscription attempt after ambiguity

Illegal transitions must trigger a fatal error and require operator intervention.

---

## 7. Restart Semantics

On process restart:

1. Load the latest persisted state for each auction
2. Resume from the recorded state
3. Never recompute resolution or finalization

Restart behavior by state:

- **Scheduled**  
  Resume inter-auction timer

- **Open**  
  Resume bidding if end time not passed  
  Otherwise transition to `Closed`

- **Closed**  
  Do not recompute resolution  
  Proceed to `AwaitingSettlement`

- **AwaitingSettlement**  
  Resume settlement monitoring

- **Finalized**  
  Resume inscription process

- **Inscribing**  
  If pre-broadcast failure is known, retry is permitted  
  If post-broadcast ambiguity exists, do not retry  
  Observe only

- **Inscribed**  
  No action

---

## 8. System Control State Machine

### 8.1 System States

1. `Running`
2. `Paused`

---

### 8.2 Pause Rules

- The system may enter `Paused` only at auction boundaries
- An open auction must never be interrupted
- Pausing prevents transition from `Scheduled` → `Open`
- Pausing does not affect settlement or inscription

Pause events are durably recorded with timestamp and reason.

---

### 8.3 Resume Rules

- Resume requires explicit operator action
- Resume is allowed only when system state is internally consistent
- Resume transitions system to `Running`

---

## 9. Invariants Enforced by This State Machine

1. Each auction resolves exactly once
2. Each auction finalizes exactly once
3. Each auction produces at most one canonical inscription
4. Auctions advance monotonically
5. Bidding never reopens
6. Past states are immutable
7. Ambiguity does not grant permission to retry

Violation of any invariant is a fatal system error.

---

## 10. Design Notes

- Absence of bids is a valid outcome
- Failure is treated as outcome, not exception
- Progression is prioritized over optimization
- Time passing does not restore authority
- Certainty is recorded; uncertainty is preserved

---

## 11. Open Assumptions

The following assumptions are external and must be defined elsewhere:

1. Confirmation depth required for settlement
2. Fee selection and inscription retry policy before broadcast
3. External observation mechanisms for inscription detection

If any assumption changes, this document must be revised before implementation.
