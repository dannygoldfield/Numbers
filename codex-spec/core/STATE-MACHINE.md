# State Machine — Numbers

This document defines the executable state machines for Numbers.

It is normative.

Authority precedence is defined exclusively in AUTHORITY-ORDER.md.

If behavior is not explicitly permitted here, it is forbidden.

This document assumes familiarity with:
- STATE-MACHINE-TABLE.md
- CORE-SEQUENCE.md
- AUTHORITY-CONSUMPTION.md
- PERSISTENCE.md
- DATA-MODEL.md

It specifies:
1. All valid states
2. Allowed transitions
3. Transition triggers
4. Restart semantics
5. Pause semantics
6. Illegal transitions
7. State legality boundaries

State is always derived from persisted records.
State must never be stored as mutable truth.

---

# 1. Scope

This document governs:

- The auction lifecycle for a single number `N`
- The inscription lifecycle that begins after auction finalization
- The system-level control machine

This document does not govern:

- UI presentation
- Wallet internals
- Bitcoin confirmation mechanics beyond defined thresholds

---

# 2. State Model

The system consists of three distinct state machines:

1. Auction State Machine (per number)
2. Inscription State Machine (per number, post-finalization)
3. System Control State Machine (global)

These state machines interact only at explicitly defined authority boundaries.

Authority semantics are defined exclusively in AUTHORITY-CONSUMPTION.md.

Timing guarantees are defined exclusively in CORE-SEQUENCE.md.

Persistence guarantees are defined exclusively in PERSISTENCE.md.

---

# 3. Auction State Machine

Each auction number `N` progresses through the following states
exactly once and strictly in order:

1. `Scheduled`
2. `Open`
3. `Closed`
4. `AwaitingSettlement`
5. `Finalized`

No other auction states are valid.

Once an auction reaches `Finalized`,
its lifecycle is complete.

Auction state is derived from persisted records as defined in DATA-MODEL.md.

---

## 3.1 Scheduled

Meaning:  
Auction `N` exists but is not accepting bids.

Entry condition:
- AuctionRecord for `N` exists
- No AuctionOpenRecord exists

Invariants:
- `opened_at` is null
- `base_end_time` is null

Allowed actions:
- Reject all bids

Exit trigger:
- First valid bid accepted while system state = `Running`

---

## 3.2 Open

Meaning:  
Auction `N` is actively accepting bids.

Entry trigger:
- First valid bid accepted while in `Scheduled`

Atomic effects on entry:
- Persist BidRecord
- Persist AuctionOpenRecord with:
  - `opened_at = server_time`
  - `base_end_time = opened_at + auction.duration_seconds`

`server_time` is the authoritative system clock at the moment of bid acceptance.  
No client-provided timestamp may influence lifecycle timing.

Allowed actions:
- Accept valid bids
- Reject invalid bids
- Apply extension logic

---

### Extension Rule (Normative)

While state = `Open`:

Let:

- `number_of_extension_events` = count of ExtensionEventRecords
- `current_end_time` =
  `base_end_time + (auction.extension_increment_seconds * number_of_extension_events)`

Time source rule:
All time comparisons must use `server_time`.

If a valid bid is accepted and:

- `server_time >= current_end_time - auction.extension_window_seconds`
- and `number_of_extension_events < auction.max_extensions`

Then atomically:

- Persist exactly one ExtensionEventRecord
- Do not modify prior records
- Do not modify `base_end_time`

Extensions:
- Do not consume authority
- Do not create new lifecycle states
- Are bounded by `auction.max_extensions`

Exit trigger (exactly one):

- `server_time >= current_end_time`
- or configured bid cap reached

---

## 3.3 Closed

Meaning:  
Auction `N` is closed to new bids.

Entry trigger:
- AuctionCloseRecord persisted

Allowed actions:
- Compute resolution deterministically
- Persist ResolutionRecord as defined in DATA-MODEL.md

Resolution must occur exactly once.

Resolution must never be recomputed after ResolutionRecord exists.

Exit condition:
- ResolutionRecord exists

State is derived as Closed if:
- AuctionCloseRecord exists
- ResolutionRecord does not exist

---

## 3.4 AwaitingSettlement

Meaning:  
Resolution exists. Settlement outcome pending.

Entry condition:
- ResolutionRecord exists
- FinalizationRecord does not exist

Allowed actions:
- Observe settlement
- Determine settlement outcome
- Persist SettlementRecord as defined in DATA-MODEL.md

Exit trigger (exactly one):

- Settlement confirmed before deadline
- Settlement deadline expired
- ResolutionRecord indicates no valid bids

---

## 3.5 Finalized

Meaning:  
Auction `N` has a final destination.

Entry condition:
- FinalizationRecord exists

Destination:
- Winning address
- NullSteward

Finalization is irreversible.

No auction action is permitted beyond this state.

---

## 3.6 Auction Restart Semantics

On restart:

If AuctionOpenRecord exists:
- Reconstruct `base_end_time`
- Load all ExtensionEventRecords
- Compute `current_end_time`
- If `server_time < current_end_time`, state = Open
- If `server_time >= current_end_time`, persist AuctionCloseRecord if missing

If AuctionCloseRecord exists and ResolutionRecord does not exist:
- Compute resolution deterministically from BidRecords
- Persist exactly one ResolutionRecord

If ResolutionRecord exists and FinalizationRecord does not exist:
- Observe settlement only

If FinalizationRecord exists:
- No auction action permitted

If required canonical records are missing:
- Execution must halt

Restart must not consume authority.

---

# 4. Inscription State Machine

The inscription machine begins only after auction state = `Finalized`.

Inscription state does not alter auction state.

---

## 4.1 Inscription States

1. `NotStarted`
2. `Inscribing`
3. `Ambiguous`
4. `Inscribed`

No other inscription states are valid.

---

## 4.2 Inscription Transition Guards

Inscription initiation is permitted only if:

- Auction state = `Finalized`
- No InscriptionRecord exists
- No AmbiguityRecord exists

---

## 4.3 Inscription Transitions

| From | To | Trigger |
|------|----|---------|
| NotStarted | Inscribing | InscriptionRecord persisted |
| Inscribing | Inscribed | Canonical inscription observed |
| Inscribing | Ambiguous | Ambiguity detected |

No other inscription transitions are permitted.

---

## 4.4 Inscription Semantics

Inscription authority is consumed at transition:

`NotStarted → Inscribing`

If AmbiguityRecord exists:
- Retry is forbidden
- Observation only permitted

If Inscribed:
- Terminal
- No further action permitted

---

## 4.5 Inscription Restart Semantics

On restart:

If InscriptionRecord exists and neither Ambiguous nor Inscribed:
- Observation only
- Retry forbidden

If AmbiguityRecord exists:
- Terminal
- Observation only

If Inscribed:
- Terminal
- No action permitted

Restart must not restore inscription authority.

---

# 5. System Control State Machine

## 5.1 States

1. `Running`
2. `Paused`

---

## 5.2 Pause Rules

The system may enter `Paused` at any time.

While `Paused`:

- No new authority-bearing action may begin
- No new bids may be accepted
- Time continues to advance
- Deadlines and end times are not modified

Pause does not alter lifecycle truth.

Pause events must be persisted as defined in DATA-MODEL.md.

---

## 5.3 Resume Rules

Resume requires explicit operator action.

Resume is permitted only when persisted state is internally consistent.

Resume transitions system to `Running`.

Resume does not restore authority.

---

# Final Rule

If a transition is not explicitly listed as allowed:

It is forbidden.