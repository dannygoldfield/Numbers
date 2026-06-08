# Invariants: Numbers

This document defines the invariants that govern the Numbers system.

It is normative.

Authority precedence is defined exclusively in `AUTHORITY-ORDER.md`.

An invariant is a property that must hold true at all times.

Violating an invariant permanently constrains execution or requires execution halt.

If a behavior depends on an assumption not stated here, that assumption is invalid.

If behavior would violate an invariant in this document, that behavior is forbidden.

---

## Modal Language Rule

In all normative documents:

- `must` and `must not` define obligations
- `only`, `exactly once`, and `at most once` define bounds
- `may` is permitted only to describe observational uncertainty
- `may` must never authorize action, imply permission, or introduce discretion

Any modal usage violating this rule is invalid.

---

# 1. Auction Identity and Order

## I-01: Auction Numbers Are Strictly Monotonic

Auction numbers must begin at `auction.starting_number` and increase strictly by `1` when a new auction is created.

Rules:

- each auction number appears exactly once
- auction numbers are never reused
- auction numbers are never skipped
- auction numbers are never reordered

`AuctionRecord` for `N + 1` must not be persisted until `FinalizationRecord` exists for number `N` and `auction.inter_auction_gap_seconds` has elapsed.

The `auction.inter_auction_gap_seconds` interval is a rhythm gap only.

It must not be treated as:

- a recovery window
- a settlement window
- an inscription window
- an automatic auction-start trigger

Auction `N + 1` opens only when the first valid bid for `N + 1` is accepted.

When `FinalizationRecord` exists for `N`, `auction.inter_auction_gap_seconds` has elapsed, and no active auction exists, `AuctionRecord` for `N + 1` must be persisted at the next state-evaluation boundary defined by the active implementation slice.

This persistence makes auction `N + 1` `Scheduled`; it does not open auction `N + 1`.

Sequence advancement must not depend on live inscription broadcast, live inscription confirmation, inscription success, or inscription ambiguity.

---

## I-02: Only One Auction May Be Active

At most one auction may exist in a non-terminal auction lifecycle state at any time.

A terminal auction lifecycle state is:

- `Finalized`

Auctions must not overlap.

Concurrent bidding windows are forbidden.

This invariant applies globally.

---

# 2. Canonical Event Records

## I-03: Canonical Event Records Define Truth

System truth must be derived from canonical event records.

The following terms refer to the same persisted truth when used for system state:

- `event`
- `event record`
- `canonical record`
- `persisted record`

There is no separate event model and record model.

Lifecycle state must never be stored as mutable truth.

---

## I-04: Canonical Event Records Are Append-Only

Once a canonical event record is durably persisted:

- it must not be edited
- it must not be removed
- it must not be reordered
- it must not be reinterpreted
- it must not be replaced by external observation

History is append-only.

Corrections are forbidden unless an explicit correction record type is defined by the specification.

---

# 3. Auction Resolution and Finality

## I-05: Each Closed Auction Resolves Exactly Once

Each auction that reaches `Closed` must resolve exactly once.

Rules:

- resolution must be deterministic
- resolution must use only persisted valid `BidRecord` entries
- invalid `BidRecord` entries must not participate in resolution
- resolution must be persisted in exactly one `ResolutionRecord`
- resolution must not be recomputed after `ResolutionRecord` exists

Any second resolution attempt that would alter outcome is forbidden.

---

## I-06: Resolution Is Final

Once `ResolutionRecord` exists:

- the winner, or lack of winner, is fixed
- resolution must not be revised
- resolution must not be overridden
- resolution must not be recomputed

---

# 4. Settlement Finality

## I-07: Settlement Outcome Is Irreversible

Once `SettlementRecord` exists, settlement outcome is terminal.

Settlement status in the current first-valid-bid opening model must be one of:

- `settled`
- `expired`

`not_required` must not be emitted unless an active implementation slice explicitly defines a no-winner settlement path.

After settlement outcome is persisted:

- the outcome must not change
- late payment must not be accepted
- settlement must not be recomputed
- settlement must not restore authority
- settlement must not create authority

Settlement is not an authority scope.

---

## I-08: NullSteward Is a Valid Outcome

Routing to `NullSteward` is a valid and complete outcome.

`NullSteward`:

- is a protocol-visible final destination
- is not a system failure
- does not imply error
- does not reduce system correctness
- is not a configurable destination
- is not a universal recovery mechanism
- must not be used to repair inscription ambiguity after inscription authority is consumed or frozen

---

# 5. Inscription Authority

## I-09: Inscription Authority Exists At Most Once Per Auction

For each auction:

- inscription authority exists at most once
- inscription authority is never renewed
- inscription authority is never recreated by time, restart, or operator action
- inscription authority is consumed only at `broadcast_commit`
- inscription authority is frozen by ambiguity

Intent persistence does not consume inscription authority.

Transaction construction does not consume inscription authority.

Confirmation observation does not consume inscription authority.

---

## I-10: At Most One broadcast_commit Is Permitted

For each auction, at most one `broadcast_commit` is permitted.

After `broadcast_commit`:

- inscription authority is consumed
- no semantically distinct inscription attempt is permitted
- replacement is permitted only if explicitly allowed by the active implementation slice and inscription specification

A `pre_commit_rejected` outcome does not consume inscription authority.

A `pre_commit_rejected` outcome does not automatically permit retry.

Retry is forbidden unless explicitly permitted by the active implementation slice.

---

## I-11: Ambiguity Permanently Freezes Authority

If inscription authority becomes ambiguous:

- affected inscription authority is frozen
- frozen authority must be treated as exhausted
- retry is forbidden
- rebroadcast is forbidden
- replacement is forbidden unless already explicitly permitted and not made ambiguous
- override is forbidden
- semantically distinct inscription is forbidden
- the Numbers count must not be interrupted
- auction availability for later numbers must not be blocked after finalization and rhythm-gap requirements are satisfied

Time passing does not restore certainty or permission.

Restart does not restore certainty or permission.

Operator action does not restore certainty or permission.

Later observation must not repair ambiguity unless a higher-authority specification revision explicitly permits that behavior.

---

# 6. Observation and Knowledge

## I-12: Observation Is Knowledge Only

Observation is a deterministic evaluation of:

- canonical event records
- explicitly permitted external authoritative systems
- confirmation depth rules defined by configuration

Observation may produce a canonical event record only when explicitly permitted.

Observation must not:

- restore authority
- create authority
- justify retry
- substitute alternate action
- alter historical records
- replace canonical event records

Human interpretation and operator intent must not change system state.

Any reimbursement or compensation performed by an operator is external to the system and must not:

- alter outcomes
- restore authority
- substitute system behavior
- imply system guarantees

---

## I-13: Time Passing Is Not Evidence

Time passing alone:

- does not resolve ambiguity
- does not imply success
- does not imply failure
- does not permit retry
- does not grant certainty
- does not create authority
- does not restore authority

Only explicitly permitted deterministic evaluation may change knowledge or lifecycle state.

---

# 7. Lifecycle Terminality

## I-14: Terminal States Are Terminal Within Their Lifecycle

Terminality applies within the lifecycle machine where the terminal state occurs.

Auction terminal state:

- `Finalized`

Inscription terminal states:

- `Inscribed`
- `Ambiguous`

After auction state becomes `Finalized`:

- no further auction lifecycle transition is permitted
- final destination must not change
- `AuctionRecord` for `N + 1` must not be persisted until `auction.inter_auction_gap_seconds` has elapsed
- inscription lifecycle may begin if permitted by the active implementation slice

Inscription progress, inscription success, inscription failure, or inscription ambiguity for `N` must not block auction availability for `N + 1` after the finalization and rhythm-gap requirements are satisfied.

After inscription state becomes `Inscribed`:

- no further inscription lifecycle transition is permitted

After inscription state becomes `Ambiguous`:

- no further inscription attempt is permitted
- affected inscription authority remains frozen

Terminality must not be used to block unrelated lifecycle machines unless explicitly specified.

---

# 8. State Machine Enforcement

## I-15: Illegal State Transitions Halt Execution

Any transition not permitted by `core/STATE-MACHINE-TABLE.md`:

- must halt execution immediately
- must be logged
- must not be retried
- must not be auto-corrected
- must not be repaired by inference

Silent correction is forbidden.

---

# 9. System Pause

## I-16: Pause Is Non-Semantic

System pause:

- does not advance lifecycle state
- does not change lifecycle truth
- does not alter the meaning of any lifecycle state
- does not imply outcomes
- does not restore authority
- does not freeze authority by itself

Pause is an overlay only.

While system control state is `Paused`:

- no new bid may be accepted as valid
- no new authority-bearing action may begin
- time continues to advance
- deadlines are not modified
- auction end times are not modified
- settlement windows are not extended

Pause must not extend deadlines, settlement windows, or auction timing.

Pause does not retroactively affect actions completed before the pause event was persisted.

---

# 10. Error Classification

## I-17: Errors Cannot Downgrade

Once an error escalates:

- it must not downgrade
- authority must not automatically return
- no automated action may downgrade classification
- no operator action may downgrade classification

`Ambiguous` must not downgrade.

`Fatal` must not downgrade.

Once ambiguity exists, affected authority does not return.

---

# 11. Demo 1 Boundary

## I-18: Demo 1 Must Not Depend on Live Inscription

For Demo 1:

- live inscription broadcast is not required
- Bitcoin Core RPC is not required for auction correctness
- wallet interaction is not required for auction correctness
- mempool recognition is not required
- confirmation observation is not required
- external SSD availability is not required
- live inscription success must not be simulated
- confirmation must not be simulated
- exactly one deferred `InscriptionIntentRecord` must be persisted for each finalized auction
- `InscriptionIntentRecord.adapter_mode` must be `deferred_in_this_slice`

Auction lifecycle, finalization, sequence advancement, API state, and restart reconstruction must remain demonstrable without live inscription execution.

If no valid bid is accepted, the auction remains `Scheduled`.

No countdown, auction close, resolution, settlement, finalization, inscription process, or `NullSteward` outcome is produced solely because no valid bid has been accepted.

---

# Final Rule

If any behavior would violate an invariant in this document:

That behavior is forbidden.