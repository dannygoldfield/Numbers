# State Machine: Numbers

This document defines the executable state machines for Numbers.

It is normative.

Authority precedence is defined exclusively in `AUTHORITY-ORDER.md`.

If behavior is not explicitly permitted here, it is forbidden.

This document assumes familiarity with:

- `core/STATE-MACHINE-TABLE.md`
- `core/CORE-SEQUENCE.md`
- `core/AUTHORITY-CONSUMPTION.md`
- `data/PERSISTENCE.md`
- `data/DATA-MODEL.md`
- `data/RESTART-RULES.md`

It specifies:

1. all valid states
2. allowed transitions
3. transition triggers
4. restart constraints
5. pause semantics
6. illegal transitions
7. state legality boundaries

State is always derived from canonical event records.

State must never be stored as mutable truth.

Canonical event record ordering defines lifecycle precedence.

---

# 1. Scope

This document governs:

- the auction lifecycle for a single number `N`
- the inscription lifecycle that begins after auction finalization
- the system-level control machine

This document does not govern:

- UI presentation
- wallet internals
- Bitcoin confirmation mechanics beyond defined thresholds
- live Ordinals broadcast unless included in the active implementation slice

---

# 2. State Model

The system consists of three distinct state machines:

1. Auction State Machine, per number
2. Inscription State Machine, per number after finalization
3. System Control State Machine, global

These state machines interact only at explicitly defined boundaries.

Authority semantics are defined exclusively in `core/AUTHORITY-CONSUMPTION.md`.

Timing and sequence progression are defined by `core/CORE-SEQUENCE.md`.

Persistence guarantees are defined by `data/PERSISTENCE.md`.

Restart reconstruction is defined by `data/RESTART-RULES.md`.

---

# 3. Auction State Machine

Each auction number `N` progresses through the following auction states:

1. `Scheduled`
2. `Open`
3. `Closed`
4. `AwaitingSettlement`
5. `Finalized`

No other auction states are valid.

Once an auction reaches `Finalized`, its auction lifecycle is complete.

Auction state is reconstructed from canonical event records defined in `data/DATA-MODEL.md`.

---

## 3.1 Scheduled

Meaning:

Auction `N` exists but has not yet opened.

Entry condition:

- `AuctionRecord` for `N` exists
- no `AuctionOpenRecord` exists

Invariants:

- `opened_at` is `null` in API projection
- `base_end_time` is `null` in API projection
- no auction countdown exists

Allowed actions:

- evaluate bid submissions through `bidding/BIDDING-ADMISSION.md`
- persist invalid `BidRecord` entries when admission evaluation is reached
- atomically open the auction on the first valid bid

Exit trigger:

- first valid bid accepted while system control state = `Running`

Atomic effects on exit:

1. persist `BidRecord` with `validity = valid`
2. persist `AuctionOpenRecord`
3. derive auction state as `Open`

No intermediate lifecycle state is permitted.

---

## 3.2 Open

Meaning:

Auction `N` is actively accepting bid submissions.

Entry condition:

- `AuctionOpenRecord` exists
- no `AuctionCloseRecord` exists

Entry effects:

- `opened_at` is fixed by `AuctionOpenRecord`
- `base_end_time` is fixed by `AuctionOpenRecord`
- `current_end_time` is derived from `base_end_time` and `ExtensionEventRecord` count

Allowed actions:

- evaluate bid submissions through `bidding/BIDDING-ADMISSION.md`
- persist valid `BidRecord` entries
- persist invalid `BidRecord` entries when admission evaluation is reached
- apply extension logic if explicitly required
- close auction when close condition is satisfied

---

## 3.2.1 Bid Ordering Rule

When multiple valid bids are accepted during `Open`:

- each bid must be persisted as exactly one `BidRecord`
- `BidRecord` entries are ordered by canonical event record order
- canonical event record order defines bid precedence

Resolution must determine the winning bid using only persisted valid `BidRecord` entries.

Network arrival order, mempool propagation order, and block inclusion order must not influence bid precedence.

---

## 3.2.2 Extension Rule

While auction state = `Open`:

Let:

```text
number_of_extension_events = count of ExtensionEventRecord entries
```

and:

```text
current_end_time =
base_end_time +
(auction.extension_increment_seconds * number_of_extension_events)
```

All time comparisons must use authoritative `server_time`.

If a valid bid is accepted and:

- `server_time >= current_end_time - auction.extension_window_seconds`
- `number_of_extension_events < auction.max_extensions`

then the system must persist exactly one `ExtensionEventRecord`.

Extension records:

- must be append-only
- must not modify prior records
- must not modify `base_end_time`
- must not consume authority
- must not create new lifecycle states

---

## 3.2.3 Close Rule

Auction close is permitted when:

- `server_time >= current_end_time`
- or configured bid cap is reached

When close occurs:

- persist exactly one `AuctionCloseRecord`
- derive auction state as `Closed`
- no further valid bids are permitted

If `AuctionCloseRecord` exists before a bid is evaluated, the bid must be invalid.

If the system detects that `server_time >= current_end_time` before accepting a bid, the system must close the auction before accepting any further valid bid.

Bid admission and auction close must be serialized through canonical event record persistence.

---

## 3.3 Closed

Meaning:

Auction `N` is closed to new valid bids.

Entry condition:

- `AuctionCloseRecord` exists
- no `ResolutionRecord` exists

Allowed actions:

- compute resolution deterministically
- persist exactly one `ResolutionRecord`

Resolution must:

- occur exactly once
- use only persisted valid `BidRecord` entries
- exclude invalid `BidRecord` entries
- never be recomputed after `ResolutionRecord` exists

Exit condition:

- `ResolutionRecord` exists

Derived next state:

- `AwaitingSettlement`

---

## 3.4 AwaitingSettlement

Meaning:

Resolution exists and terminal settlement outcome has not yet been recorded.

Entry condition:

- `ResolutionRecord` exists
- no `SettlementRecord` exists
- no `FinalizationRecord` exists

Allowed actions:

- determine settlement outcome according to `bidding/SETTLEMENT.md`
- persist exactly one `SettlementRecord`
- persist exactly one `FinalizationRecord`

Exit triggers:

- settlement confirmed before deadline
- settlement deadline expired
- `ResolutionRecord` indicates no valid winning bid

Settlement state must not be represented by a pending `SettlementRecord`.

`SettlementRecord` records terminal settlement outcome only.

---

## 3.5 Finalized

Meaning:

Auction `N` has a permanently fixed final destination.

Entry condition:

- `FinalizationRecord` exists

Destination must be one of:

- winning destination address
- `NullSteward`

Finalization is irreversible.

No auction lifecycle action is permitted after `Finalized`.

Sequence advancement to `N + 1` is permitted only after `FinalizationRecord`.

---

# 4. Auction Restart Constraints

Restart behavior is governed by `data/RESTART-RULES.md`.

On restart:

- auction state must be reconstructed from canonical event records only
- mutable stored lifecycle state must be ignored
- missing required records must halt execution
- malformed records must halt execution
- contradictory records must halt execution
- existing outcome records must not be recomputed

Restart must not:

- open a scheduled auction
- create a bid
- alter `base_end_time`
- recompute resolution after `ResolutionRecord`
- recompute settlement after `SettlementRecord`
- recompute final destination after `FinalizationRecord`
- consume authority
- restore authority

---

# 5. Inscription State Machine

The inscription machine begins only after auction state = `Finalized`.

Inscription state does not alter auction state.

Auction correctness must not depend on inscription progress, broadcast, confirmation, or failure.

---

## 5.1 Inscription States

The valid inscription states are:

1. `NotStarted`
2. `Inscribing`
3. `Ambiguous`
4. `Inscribed`

No other inscription states are valid.

There is no canonical `Broadcasting` lifecycle state.

Broadcast attempt is an operation, not a lifecycle state.

---

## 5.2 NotStarted

Meaning:

No committed inscription broadcast exists and no terminal inscription ambiguity exists.

`InscriptionIntentRecord` alone does not move inscription state out of `NotStarted`.

Allowed actions:

- persist `InscriptionIntentRecord` after auction state = `Finalized`
- perform no live broadcast when adapter mode = `deferred_in_this_slice`
- perform broadcast only when adapter mode = `testnet_ordinals` and the active implementation slice explicitly permits live broadcast

---

## 5.3 Inscribing

Meaning:

A committed inscription broadcast exists and confirmation has not yet been observed.

Entry condition:

- `InscriptionBroadcastRecord.broadcast_outcome = committed`

Authority effect:

- inscription authority is consumed at `broadcast_commit`

Allowed actions:

- observe confirmation if the active implementation slice permits confirmation observation
- persist `InscriptionConfirmationRecord` if confirmation rules are satisfied
- persist `AmbiguityRecord` if ambiguity is detected

No semantically distinct inscription attempt is permitted.

---

## 5.4 Ambiguous

Meaning:

Inscription authority is frozen because the broadcast or inscription outcome cannot be determined.

Entry condition:

- `InscriptionBroadcastRecord.broadcast_outcome = ambiguous`
- or `AmbiguityRecord.authority_scope = inscription`

Rules:

- terminal inscription state
- authority remains frozen
- retry is forbidden
- alternate inscription is forbidden
- semantically distinct inscription is forbidden

Ambiguity must not be repaired by:

- restart
- operator action
- time passing
- later observation
- speculative rebroadcast

---

## 5.5 Inscribed

Meaning:

Canonical inscription confirmation has been observed and recorded.

Entry condition:

- `InscriptionConfirmationRecord` exists

Rules:

- terminal inscription state
- no further inscription action is permitted
- confirmation does not consume additional authority

---

## 5.6 Inscription Transitions

| From | Trigger | To | Required Canonical Event Records |
|---|---|---|---|
| `NotStarted` | Inscription intent persisted | `NotStarted` | `InscriptionIntentRecord` |
| `NotStarted` | Broadcast classified as `pre_commit_rejected` | `NotStarted` | `InscriptionBroadcastRecord` |
| `NotStarted` | `broadcast_commit` | `Inscribing` | `InscriptionBroadcastRecord` with `broadcast_outcome = committed` |
| `NotStarted` | Broadcast classified as `ambiguous` | `Ambiguous` | `InscriptionBroadcastRecord` with `broadcast_outcome = ambiguous` or `AmbiguityRecord` |
| `Inscribing` | Known confirmation observed | `Inscribed` | `InscriptionConfirmationRecord` |
| `Inscribing` | Ambiguity detected | `Ambiguous` | `AmbiguityRecord` |

No other inscription transitions are permitted.

---

## 5.7 Inscription Authority Rule

Inscription authority is consumed only at `broadcast_commit`.

Intent persistence does not consume authority.

Transaction construction does not consume authority.

Confirmation observation does not consume authority.

`pre_commit_rejected` does not consume authority.

`ambiguous` freezes authority permanently.

Authority semantics are governed by `core/AUTHORITY-CONSUMPTION.md`.

---

## 5.8 Demo 1 Inscription Rule

For Demo 1:

- live inscription broadcast is not required
- `InscriptionIntentRecord` may be persisted
- `InscriptionIntentRecord.adapter_mode` must be `deferred_in_this_slice`
- no `InscriptionBroadcastRecord` is required
- no `InscriptionConfirmationRecord` is required
- no live inscription success may be simulated
- no confirmation may be simulated

Auction lifecycle and sequence advancement must remain demonstrable without live inscription execution.

---

# 6. Inscription Restart Constraints

Restart behavior is governed by `data/RESTART-RULES.md`.

On restart:

- inscription state must be reconstructed from canonical event records only
- `InscriptionIntentRecord` alone reconstructs to `NotStarted`
- committed broadcast reconstructs to `Inscribing`
- ambiguous broadcast reconstructs to `Ambiguous`
- inscription ambiguity reconstructs to `Ambiguous`
- confirmation reconstructs to `Inscribed`

Restart must not:

- trigger live broadcast
- simulate broadcast
- simulate confirmation
- retry after committed broadcast
- retry after ambiguity
- restore authority
- repair ambiguity

---

# 7. System Control State Machine

## 7.1 States

The valid system control states are:

1. `Running`
2. `Paused`

No other system control states are valid.

System control state is derived from `PauseEventRecord` entries.

---

## 7.2 Running

Meaning:

The system is permitted to evaluate actions allowed by the current lifecycle state.

Allowed actions:

- evaluate bid submissions
- evaluate deterministic transitions
- perform implementation-slice-permitted operations

Running does not override lifecycle restrictions.

---

## 7.3 Paused

Meaning:

The system rejects new externally initiated lifecycle actions.

While `Paused`:

- no new bid may be accepted as valid
- no new authority-bearing action may begin
- time continues to advance
- deadlines and end times are not modified
- deterministic restart reconstruction remains governed by `data/RESTART-RULES.md`

Pause does not alter auction lifecycle truth.

Pause does not alter inscription lifecycle truth.

Pause does not restore authority.

Pause does not freeze authority by itself.

---

## 7.4 Pause and Resume Records

Pause and resume are represented by `PauseEventRecord`.

A pause event must not:

- alter auction state
- alter inscription state
- alter authority
- modify deadlines
- rewrite canonical event records

A resume event must not:

- alter auction state
- alter inscription state
- restore authority
- rewrite canonical event records

Resume is permitted only when persisted state is internally consistent.

---

# Final Rule

If a transition is not explicitly listed as allowed:

It is forbidden.