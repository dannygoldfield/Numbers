# Restart Rules: Numbers

This document defines mandatory behavior when the Numbers system restarts.

It is normative.

Authority precedence is defined exclusively in `AUTHORITY-ORDER.md`.

Restart handling ensures that:

- authority is never exercised twice
- outcomes are never recomputed after persistence
- ambiguity is never resolved by forgetting
- progress is never made without certainty
- runtime memory is reconstructed from canonical event records only

---

# 1. Principle

A restart is not a new execution.

A restart is continuation of the same execution, with runtime memory restored exclusively from persisted canonical event records.

Restart:

- does not introduce new states
- does not create authority
- does not restore authority
- does not resolve uncertainty
- does not alter lifecycle truth
- does not repair missing records
- does not substitute external observation for canonical event records

If persisted canonical event records are incomplete, missing, malformed, or contradictory, execution must halt.

---

# 2. Absolute Rules

## R-01: Persisted Canonical Event Records Define Reality

On restart:

- persisted canonical event records define system truth
- all in-memory state is discarded
- logs are non-authoritative
- operator belief is non-authoritative
- external chain observation is non-authoritative until recorded through a permitted canonical event record

If persisted canonical event records conflict with expectation or operator belief, persisted canonical event records prevail.

---

## R-02: No Recomputing After Persistence

On restart, the system must never recompute an outcome if its canonical event record already exists.

Specifically:

- if `ResolutionRecord` exists, resolution must not be recomputed
- if `SettlementRecord` exists, settlement must not be recomputed
- if `FinalizationRecord` exists, destination must not be recomputed
- if any `InscriptionBroadcastRecord` has `broadcast_outcome = committed`, inscription broadcast must not be repeated
- if `InscriptionConfirmationRecord` exists, confirmation must not be recomputed

If an outcome record does not exist but its prerequisite transition has occurred, deterministic evaluation can complete only when explicitly permitted by this document and the governing lifecycle table.

Unknown does not grant authority.

Missing records do not grant authority.

Restart does not create permission.

---

## R-03: Authority Is Never Recreated

Restart must not:

- restore consumed authority
- restore frozen authority
- permit retry after ambiguity
- permit retry after committed broadcast
- allow alternate irreversible actions
- change authority scope for an auction

Authority, once consumed or frozen, remains unavailable permanently.

---

# 3. Restart Procedure

On startup, the system must execute the following steps in order.

---

## Step 1: Load Canonical Event Records

The system must load all canonical event records defined in `data/DATA-MODEL.md`.

If any required canonical event record is:

- missing
- unreadable
- malformed
- internally contradictory
- outside the canonical record set

then execution must halt.

No authority can be exercised.

---

## Step 2: Validate Record Order and Shape

The system must validate that:

- every record type is defined in `data/DATA-MODEL.md`
- every record type is defined in `core/EVENT-TYPES.md`
- `sequence_index` values define a total order
- no `sequence_index` is reused
- no canonical event record violates its required schema
- no duplicate authority-bearing record exists
- records appear in an order consistent with `core/STATE-MACHINE-TABLE.md`

If validation fails, execution must halt.

---

## Step 3: Reconstruct Auction State

For each auction number `N`, reconstruct auction state strictly from canonical event records.

Auction state must never be read from mutable stored state.

Auction reconstruction rules:

1. If `FinalizationRecord` exists:
   - auction state is `Finalized`.

2. Else if `SettlementRecord` exists without `FinalizationRecord`:
   - execution must halt.

3. Else if `ResolutionRecord` exists:
   - auction state is `AwaitingSettlement`.

4. Else if `AuctionCloseRecord` exists:
   - auction state is `Closed`.

5. Else if `AuctionOpenRecord` exists:
   - auction state is `Open`.

6. Else if `AuctionRecord` exists:
   - auction state is `Scheduled`.

7. Else:
   - auction does not exist.

If reconstruction yields a state or transition not permitted by `core/STATE-MACHINE-TABLE.md`, execution must halt.

---

## Step 4: Reconstruct Open Auction Time

For auctions reconstructed as `Open`:

- load `base_end_time` from `AuctionOpenRecord`
- count `ExtensionEventRecord` records for the auction
- derive `current_end_time`

```text
current_end_time =
base_end_time +
(extension_increment_seconds * number_of_extension_events)
```

All time comparison must use authoritative `server_time`.

`current_end_time` must not be stored as mutable truth.

If required time inputs are missing, malformed, or contradictory, execution must halt.

---

## Step 5: Complete Permitted Deterministic Auction Transitions

Only transitions explicitly permitted by reconstructed state can be evaluated.

Permitted deterministic evaluations after restart are:

0. If the canonical store contains no `AuctionRecord`:
   - persist exactly one `AuctionRecord` with `number = auction.starting_number`.
   - reconstruct that auction as `Scheduled`.
   - do not persist `AuctionOpenRecord`.
   - do not start a countdown.

1. If auction state is `Open` and `server_time >= current_end_time`:
   - persist exactly one `AuctionCloseRecord`.

2. If auction state is `Closed` and no `ResolutionRecord` exists:
   - compute resolution deterministically.
   - persist exactly one `ResolutionRecord`.

3. If auction state is `AwaitingSettlement`, settlement deadline expired, and the active implementation slice enables chain-confirmed settlement semantics:
   - persist exactly one `SettlementRecord`.
   - persist exactly one `FinalizationRecord`.

   This transition is outside Demo 1.

   Demo 1 local settlement does not auto-expire on settlement deadline during restart or runtime state evaluation.

   In Demo 1, `expired` settlement is produced only by `POST /demo/settlement`.

4. If the latest auction state is `Finalized`, no active auction exists, `auction.inter_auction_gap_seconds` has elapsed after `FinalizationRecord.finalization_time`, and no `AuctionRecord` exists for `N + 1`:
   - persist exactly one `AuctionRecord` for `N + 1`.
   - reconstruct auction `N + 1` as `Scheduled`.
   - do not persist `AuctionOpenRecord`.
   - do not start a countdown.

5. No-valid-bid restart path:

   No restart transition exists for a no-valid-bid condition in the current first-valid-bid opening model.

   If no valid bid has been accepted, the auction remains `Scheduled`.

   Restart must not create `AuctionCloseRecord`, `ResolutionRecord`, `SettlementRecord`, `FinalizationRecord`, inscription intent, or `NullSteward` outcome solely because no valid bid exists.

No other automatic auction transition is permitted.

Time must not:

- cause `Scheduled → Open`
- resolve ambiguity
- trigger inscription broadcast
- restore authority
- create missing bid records
- alter final destination

---

# 4. Inscription Lifecycle Reconstruction

Inscription state must be reconstructed strictly from canonical event records.

For a given auction:

1. If `InscriptionConfirmationRecord` exists:
   - inscription state is `Inscribed`.
   - no further inscription action is permitted.

2. Else if any `AmbiguityRecord` with `authority_scope = inscription` exists:
   - inscription state is `Ambiguous`.
   - inscription authority is frozen.
   - no further inscription attempt is permitted.

3. Else if any `InscriptionBroadcastRecord` has `broadcast_outcome = ambiguous`:
   - inscription state is `Ambiguous`.
   - inscription authority is frozen.
   - no further inscription attempt is permitted.

4. Else if any `InscriptionBroadcastRecord` has `broadcast_outcome = committed`:
   - inscription state is `Inscribing`.
   - inscription authority is consumed.
   - only confirmation observation explicitly permitted by the active implementation slice can occur.

5. Else:
   - inscription state is `NotStarted`.

`InscriptionIntentRecord` alone does not move inscription state out of `NotStarted`.

`InscriptionIntentRecord` alone does not consume inscription authority.

`InscriptionIntentRecord` alone does not permit broadcast after restart unless the active implementation slice explicitly includes broadcast behavior.

---

# 5. Demo 1 Restart Rule

For Demo 1:

- live inscription broadcast is not required
- exactly one deferred `InscriptionIntentRecord` must exist for each finalized auction
- `InscriptionIntentRecord.adapter_mode` must be `deferred_in_this_slice`
- no `InscriptionBroadcastRecord` is required
- no `InscriptionConfirmationRecord` is required
- restart must preserve the deferred inscription status
- restart must not silently simulate inscription broadcast or confirmation

If `FinalizationRecord` exists for a Demo 1 auction and the required deferred `InscriptionIntentRecord` is absent, execution must halt.

Restart must not complete, repair, synthesize, or append a missing deferred `InscriptionIntentRecord` after `FinalizationRecord` exists.

For Demo 1, `SettlementRecord`, `FinalizationRecord`, and the required deferred `InscriptionIntentRecord` must have been persisted as one atomic canonical commit group by `POST /demo/settlement`.

Demo 1 auction correctness must remain demonstrable without live inscription execution.

---

# 6. Post-Restart Chain Checks

Post-restart chain checks are included only in implementation slices that enable `testnet_ordinals`.

If inscription state reconstructs to `Inscribing` and the active implementation slice enables confirmation observation:

- the system can query the authoritative node for confirmation status of permitted candidate txids
- if a candidate txid is Known Confirmed to the configured confirmation depth, the system must persist `InscriptionConfirmationRecord`
- if node responses are contradictory relative to persisted canonical event records, the condition must be classified via `errors/ERROR-TAXONOMY.md`

The system must not attempt a new broadcast after restart unless a later implementation slice explicitly permits that behavior.

External chain observation must not erase, overwrite, or repair canonical event records.

---

# 7. Ambiguity Handling on Restart

Ambiguity survives restart.

If inscription state reconstructs as `Ambiguous`:

- ambiguity remains
- inscription authority remains frozen
- retries remain forbidden
- alternate inscription remains forbidden
- semantically distinct inscription remains forbidden

Restart must never be used to escape ambiguity.

A later observation must not repair ambiguity unless an explicit higher-authority specification revision permits that behavior.

---

# 8. Resume or Halt

If all reconstructed states are valid and consistent:

- resume execution only for transitions explicitly permitted
- authority-bearing transitions must satisfy all preconditions
- implementation must remain within the active prototype scope

If any state is invalid, contradictory, malformed, or incomplete:

- execution must halt
- operator inspection is required
- no authority can be exercised

---

# 9. Forbidden Restart Behaviors

The following are forbidden:

- restarting to try again
- restarting to reclaim authority
- restarting to resolve ambiguity
- restarting to skip stalled auctions
- restarting to alter previously persisted outcomes
- restarting to infer missing records
- restarting to silently simulate inscription success
- restarting to substitute chain observation for canonical records
- restarting to permit broadcast not included in the active implementation slice

Restart is reconstruction, not repair.

---

# 10. Operator Obligations

If restart halts due to inconsistency:

- operator must not modify canonical records
- operator must not delete history
- operator must not fabricate history
- operator must not infer missing events
- operator must not restart repeatedly to seek a different result

If authoritative history cannot be reconstructed from canonical event records, the system must remain halted.

---

# Final Rule

If any `InscriptionBroadcastRecord` exists with `broadcast_outcome = committed`, inscription authority must be treated as consumed.

If any `InscriptionBroadcastRecord` exists with `broadcast_outcome = ambiguous`, inscription authority must be treated as frozen and exhausted.

Restart restores runtime memory.

Restart does not grant permission.