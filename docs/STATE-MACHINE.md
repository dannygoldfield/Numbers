# State Machine — Numbers

This document defines the executable state machines for Numbers.

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

This document governs:

- The auction lifecycle for a single number N
- The inscription lifecycle that follows auction finalization
- System-level pause behavior at auction boundaries

It does not govern:
- UI presentation
- Wallet internals
- Bitcoin confirmation mechanics beyond required thresholds

---

## 2. State Model

The system operates three distinct state machines:

1. Auction State Machine (per number)
2. Inscription State Machine (per number, post-auction)
3. System Control State Machine (global)

They interact only at explicitly defined boundaries.

This document defines state legality and authority boundaries, not timing guarantees, which are defined in CORE-SEQUENCE.md.

---

## 3. Auction State Machine

Each auction number N progresses through the following states exactly once and in order.

### 3.1 Auction States

1. `Scheduled`
2. `Open`
3. `Closed`
4. `AwaitingSettlement`
5. `Finalized`

There are no other valid auction states.

Once an auction reaches `Finalized`, its lifecycle is complete.

---

## 3.2 Auction State Definitions

### Scheduled

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

**Guard**
- System is not paused

---

### Open

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

### Closed

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

### AwaitingSettlement

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

### Finalized

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

Finalization is irreversible.

Auction state does not progress beyond this point.

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

The following transitions are explicitly forbidden:

- Open → Scheduled
- Closed → Open
- AwaitingSettlement → Open
- Finalized → AwaitingSettlement
- Any transition that reopens bidding

Illegal transitions must trigger a fatal error and require operator intervention.

---

## 3.5 Auction Restart Semantics

On process restart:

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
  No auction action  
  Inscription may proceed independently

---

## 4. Inscription State Machine

The inscription state machine begins only after auction finalization.

Inscription state does not affect auction truth.

### 4.1 Inscription States

1. `NotStarted`
2. `Inscribing`
3. `Ambiguous`
4. `Inscribed`

---

### 4.2 Inscription State Definitions

#### NotStarted

**Meaning**  
No inscription attempt has begun.

**Entry conditions**
- Auction N is finalized

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
- Construct inscription payload
- Select UTXO
- Sign transaction
- Attempt broadcast

**Persistence**
- Inscription attempt recorded
- Txid recorded if known

---

#### Ambiguous

**Meaning**  
An inscription transaction may exist, but its fate cannot be determined with certainty.

**Entry conditions**
- Broadcast may have occurred
- Observation is inconclusive
- Competing inscription cannot be ruled out

**Allowed actions**
- Observe chain state only

**Observation definition (normative)**  
Observation is performed exclusively by deterministic system processes querying external sources of record (e.g. Bitcoin node or indexer).  
Human judgment does not constitute observation and cannot change state.

Observation does not permit new state transitions and does not restore authority.

This state may persist indefinitely.

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

### 4.3 Authority Loss Rules (Normative)

Authority rules apply only to inscription.

**Pre-broadcast failure (safe)**
- Transaction construction or signing fails
- No transaction submitted
- Retry is permitted

**Post-broadcast ambiguity (unsafe)**
- A transaction may have been broadcast
- Its fate cannot be determined

Once ambiguity exists:

- Authority to create a competing inscription is permanently lost
- No retry, replacement, or override is permitted
- Time passing does not restore permission
- Observation is the only allowed action

---

### 4.4 Inscription Transition Table

| From | To | Trigger |
|----|----|--------|
| NotStarted | Inscribing | Inscription initiated |
| Inscribing | Inscribed | Inscription observed |
| Inscribing | Ambiguous | Broadcast uncertainty detected |
| Ambiguous | Inscribed | Inscription later observed |

No other inscription transitions are valid.

---

### 4.5 Inscription Restart Semantics

On restart:

- **NotStarted**  
  Initiation permitted

- **Inscribing**  
  Retry permitted only if pre-broadcast failure is known

- **Ambiguous**  
  No retry permitted  
  Observe only

- **Inscribed**  
  No action

---

## 5. System Control State Machine

### 5.1 System States

1. `Running`
2. `Paused`

---

### 5.2 Pause Rules

- The system may enter `Paused` only at auction boundaries
- An open auction must never be interrupted
- Pausing prevents transition from `Scheduled` → `Open`
- Pausing does not affect settlement or inscription

Pause events are durably recorded with timestamp and reason.

---

### 5.3 Resume Rules

- Resume requires explicit operator action
- Resume is allowed only when system state is internally consistent
- Resume transitions system to `Running`

---

## 6. Safety Properties Enforced Here

This state machine enforces invariants defined in:
- INVARIANTS.md
- TRANSITION-INVARIANTS.md

Including but not limited to:
- Auctions resolve exactly once
- Auctions finalize exactly once
- At most one canonical inscription exists per auction
- Bidding never reopens
- Ambiguity does not grant authority

Violation of these properties is a fatal system error.

---

## 7. Design Notes

- Absence of bids is a valid outcome
- Failure is treated as outcome, not exception
- Auction truth is independent of inscription success
- Time passing does not restore authority
- Certainty is recorded; uncertainty is preserved

---

## 8. Open Assumptions

The following assumptions are external and must be defined elsewhere:

1. Confirmation depth required for settlement
2. Fee selection and retry policy prior to broadcast
3. External mechanisms for observing inscriptions

If any assumption changes, this document must be revised before implementation.
