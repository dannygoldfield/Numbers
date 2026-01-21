# State Machine — Numbers

This document defines the executable state machines for Numbers.

It is **normative**.

This document assumes familiarity with:
- STATE-MACHINE-TABLE.md
- CORE-SEQUENCE.md
- AUTHORITY-CONSUMPTION.md
- PERSISTENCE.md

It specifies:
1. All valid states
2. All allowed transitions
3. Transition triggers
4. Persistence requirements
5. Restart semantics
6. Pause semantics
7. Illegal transitions
8. State legality boundaries

If behavior is not explicitly permitted here, it is forbidden.

If a conflict exists between this document and any higher-authority document,
execution **must halt**.
This document must not be used to reconcile or infer intent.

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
2. Inscription State Machine (per number, post-finalization)
3. System Control State Machine (global)

These state machines interact **only** at explicitly defined boundaries.

This document defines **state legality only**.

- Authority semantics are defined exclusively in `AUTHORITY-CONSUMPTION.md`
- Timing guarantees are defined exclusively in `CORE-SEQUENCE.md`
- Persistence guarantees are defined exclusively in `PERSISTENCE.md`

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

Once an auction reaches `Finalized`,
its lifecycle is complete.

---

### 3.2 Auction State Definitions

#### Scheduled

**Meaning**  
Auction `N` exists but is not accepting bids.

**Entry conditions**
- Auction `N−1` has finalized
- Inter-auction gap timer is active

**Allowed actions**
- No bids accepted

**Persistence**
- Auction record exists
- Scheduled start time recorded

**Exit trigger**
- Inter-auction gap expires

**Guard**
- System state is `Running`

---

#### Open

**Meaning**  
Auction `N` is actively accepting bids.

**Entry conditions**
- Inter-auction gap has expired
- System state is `Running`

**Allowed actions**
- Accept valid bids
- Reject invalid bids

**Persistence**
- Auction open timestamp recorded
- Bid records appended

**Exit triggers**  
Exactly one of:
1. Auction duration expires
2. Auction bid cap is reached

---

#### Closed

**Meaning**  
Auction `N` is closed to new bids and resolving.

**Entry conditions**
- An `Open` exit trigger fires

**Allowed actions**
- Determine winning bid, if any
- Persist resolution outcome

**Persistence**
- Resolution record written **exactly once**

**Exit trigger**
- Resolution record exists

**Idempotence**
- Resolution must never be recomputed

---

#### AwaitingSettlement

**Meaning**  
Auction `N` has resolved but settlement is incomplete.

**Entry conditions**
- Resolution record exists

**Allowed actions**
- Observe settlement
- Track settlement deadline

**Persistence**
- Settlement intent recorded
- Settlement deadline recorded

**Exit triggers**  
Exactly one of:
1. Settlement succeeds before deadline
2. Settlement deadline expires
3. Resolution contained no bids

---

#### Finalized

**Meaning**  
Auction `N` has a final destination.

**Entry conditions**
- Settlement outcome determined

**Final destinations**
- Winning destination address
- `NullSteward`

**Persistence**
- Finalization record written
- Destination address recorded

Finalization is irreversible.

No auction action is permitted beyond this state.

---

### 3.3 Auction State Transition Table

| From | To | Trigger | Persistence Required |
|----|----|--------|----------------------|
| Scheduled | Open | Inter-auction gap expires | Auction open record |
| Open | Closed | Duration expires or cap reached | Auction close timestamp |
| Closed | AwaitingSettlement | Resolution record exists | Resolution record |
| AwaitingSettlement | Finalized | Settlement success | Finalization record |
| AwaitingSettlement | Finalized | Settlement failure or no bids | Finalization record |

No other auction transitions are valid.

---

### 3.4 Illegal Auction Transitions

The following transitions are forbidden:

- `Open → Scheduled`
- `Closed → Open`
- `AwaitingSettlement → Open`
- `Finalized → Any`
- Any transition that reopens bidding

Illegal transitions are fatal.

---

### 3.5 Auction Restart Semantics

On restart, the system **must reconstruct state exclusively from persisted records**.

Restart **must not** consume authority.

- **Scheduled**  
  Resume inter-auction gap timer

- **Open**  
  If auction end condition not met, resume bidding  
  Otherwise transition to `Closed`

- **Closed**  
  Resolution record **must already exist**  
  Transition to `AwaitingSettlement`

- **AwaitingSettlement**  
  Observe settlement only

- **Finalized**  
  No auction action permitted

If required persistence records are missing:
- execution **must halt**

---

## 4. Inscription State Machine

The inscription state machine begins **only after auction finalization**.

Auction finalization occurs only after:
- settlement outcome is determined
- destination address is persisted

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
- Initiate inscription

---

#### Inscribing

**Meaning**  
The inscription attempt has been initiated.

**Entry conditions**
- Inscription initiation record persisted

**Allowed actions**
- Construct payload
- Select UTXO
- Sign transaction
- Attempt broadcast

**Persistence**
- Inscription initiation record written
- Txid recorded if known

---

#### Ambiguous

**Meaning**  
An inscription transaction may exist and its outcome
cannot be determined with certainty.

**Entry conditions**
- Broadcast occurred or cannot be ruled out
- Observation is inconclusive

**Allowed actions**
- Observation only

**Persistence**
- Ambiguity record **must** be written immediately

This state is non-terminal and may persist indefinitely.

---

#### Inscribed

**Meaning**  
The canonical inscription is known.

**Entry conditions**
- Inscription transaction observed and accepted

**Persistence**
- Txid recorded
- Satpoint recorded
- Canonical flag set

This is a terminal state.

---

### 4.3 Inscription Transition Table

| From | To | Trigger |
|----|----|--------|
| NotStarted | Inscribing | Inscription initiation persisted |
| Inscribing | Inscribed | Inscription observed |
| Inscribing | Ambiguous | Ambiguity detected |
| Ambiguous | Inscribed | Inscription observed |

No other inscription transitions are valid.

---

### 4.4 Inscription Restart Semantics

On restart:

- **NotStarted**  
  Initiation permitted

- **Inscribing**  
  Retry permitted **only if**
  no broadcast occurred
  **and** no ambiguity record exists

- **Ambiguous**  
  Observation only  
  Retry forbidden

- **Inscribed**  
  No action permitted

---

## 5. System Control State Machine

### 5.1 System States

1. `Running`
2. `Paused`

---

### 5.2 Pause Rules

- The system may enter `Paused` **only between auctions**
- An open auction must never be interrupted
- Pausing prevents `Scheduled → Open`
- Pausing does not affect settlement or inscription

Pause events **must** be durably persisted.

---

### 5.3 Resume Rules

- Resume requires explicit operator action
- Resume is permitted only when persisted state is internally consistent
- Resume transitions the system to `Running`

---

## 6. Final Rule

If a transition is not listed as allowed:

**It is forbidden.**
