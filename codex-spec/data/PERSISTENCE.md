# Persistence: Numbers

This document defines persistence requirements for the Numbers system.

It is normative.

Authority precedence is defined exclusively in `AUTHORITY-ORDER.md`.

Persistence prevents:

- recomputation
- reinterpretation
- accidental authority reuse
- mutation of recorded history
- restart-driven invention
- silent repair of missing truth

Persistence defines canonical system truth.

If a required canonical event record is not durably persisted, the system must treat the corresponding fact as unknown and must not infer it.

---

# 1. Purpose

Persistence ensures that:

- canonical event records survive restart
- lifecycle state is reconstructible from durable history
- outcomes do not change after persistence
- ambiguity is preserved rather than erased
- authority consumption and authority freeze remain visible after restart
- time passing does not recreate permission

Persistence is a safety boundary.

---

# 2. Persistence Model

Numbers uses an append-only ordered log of canonical event records.

A canonical event record is the same persisted object as a canonical record.

The terms `event`, `event record`, `canonical record`, and `persisted record` refer to the same append-only object when used for system truth.

There is no separate event model and record model.

Persistence applies only to canonical event record types defined in:

- `core/EVENT-TYPES.md`
- `data/DATA-MODEL.md`

No other record type may be persisted as canonical system truth.

---

# 3. Common Persistence Envelope

Every canonical event record must contain the common envelope defined in `data/DATA-MODEL.md`:

- `record_id`
- `record_type`
- `auction_id`
- `number`
- `sequence_index`
- `server_time`
- `payload_json`
- `payload_hash`

## Rules

- `record_id` must uniquely identify one canonical event record
- `record_type` must equal a defined canonical event record type
- `sequence_index` must define total order
- `sequence_index` must never be reused
- `server_time` must be authoritative server time
- `payload_json` must contain the record-specific payload
- `payload_hash` must commit to the canonical serialized payload
- `payload_hash` must be computed using the canonical payload serialization and SHA-256 rules defined in `data/DATA-MODEL.md`
- persisted records must never be modified after persistence

## Canonical Payload Hash Validation

For each persisted canonical event record:

- `payload_json` must be serialized according to the canonical payload serialization rules defined in `data/DATA-MODEL.md`
- `payload_hash` must equal lowercase hexadecimal SHA-256 of the canonical payload JSON bytes
- restart validation must recompute `payload_hash`
- a mismatch between persisted `payload_hash` and recomputed `payload_hash` is fatal
- a record with invalid canonical payload serialization is fatal

The common record envelope fields are not included in `payload_hash`.

---

# 4. Append-Only Rule

Persisted canonical event records must never be:

- edited
- rewritten
- deleted
- reordered
- reinterpreted
- compacted in a way that loses semantic meaning

History is strictly additive.

Corrections must be expressed only as new canonical event records when such correction records are explicitly defined.

If no correction record is defined, correction is forbidden.

---

# 5. Persisted Truth Rule

At runtime:

- persisted canonical event records define reality
- in-memory state is a cache only
- API state is a projection only
- logs are non-authoritative
- operator belief is non-authoritative
- external chain observation is non-authoritative until recorded through a permitted canonical event record

On restart:

- all in-memory state must be discarded
- state must be reconstructed exclusively from persisted canonical event records

If persisted canonical event records contradict memory, memory is incorrect.

If persisted canonical event records contradict operator belief, operator belief is incorrect.

---

# 6. Relationship to Authority

Authority is defined exclusively in `core/AUTHORITY-CONSUMPTION.md`.

Persistence does not create authority.

Persistence does not restore authority.

Persistence does not repair ambiguity.

For inscription authority:

- `InscriptionIntentRecord` must be durably persisted before any inscription broadcast attempt
- `InscriptionIntentRecord` does not consume authority
- `InscriptionBroadcastRecord.broadcast_outcome = committed` records authority consumption
- `InscriptionBroadcastRecord.broadcast_outcome = ambiguous` records authority freeze
- `AmbiguityRecord` with `authority_scope = inscription` records authority freeze
- `InscriptionConfirmationRecord` does not consume additional authority

Authority without required durable pre-broadcast records is forbidden.

---

# 7. Demo 1 Persistence Boundary

Demo 1 must not require live Bitcoin or Ordinals execution.

For Demo 1:

- `InscriptionIntentRecord` may be persisted
- `InscriptionIntentRecord.adapter_mode` must be `deferred_in_this_slice`
- no `InscriptionBroadcastRecord` is required
- no `InscriptionConfirmationRecord` is required
- no live broadcast may be silently simulated
- no confirmation may be silently simulated

Auction correctness must remain demonstrable without:

- Bitcoin Core RPC
- `ord`
- wallet interaction
- mempool recognition
- confirmation observation
- external SSD availability

---

# 8. Auction Lifecycle Persistence

For each auction number `N`, the system must persist canonical event records as required by lifecycle progression.

## Required Auction Records

The auction lifecycle may include:

- `AuctionRecord`
- `BidRecord`
- `AuctionOpenRecord`
- `ExtensionEventRecord`
- `AuctionCloseRecord`
- `ResolutionRecord`
- `SettlementRecord`
- `FinalizationRecord`

## `AuctionRecord`

Rules:

- exactly one `AuctionRecord` must exist per number
- must occur only after the previous auction reaches `Finalized`, except for the first auction
- must not include a scheduled start time
- must not include a precomputed end time
- must not imply automatic opening

## `BidRecord`

Rules:

- every bid submission attempt that reaches admission evaluation must produce exactly one `BidRecord`
- valid and invalid bids are both canonical event records
- invalid bids must not alter auction lifecycle state
- invalid bids must not participate in winner resolution

## `AuctionOpenRecord`

Rules:

- exactly one `AuctionOpenRecord` may exist per auction
- must be persisted atomically with the first valid `BidRecord`
- must contain `opened_at`
- must contain `base_end_time`
- `base_end_time` must be persisted exactly once
- `base_end_time` must never be changed

While auction state is `Scheduled`:

- `opened_at` must be `null` in API projection
- `base_end_time` must be `null` in API projection

## `ExtensionEventRecord`

Rules:

- one `ExtensionEventRecord` must exist per extension increment
- extension records must be append-only
- extension records must not modify `base_end_time`

`current_end_time` must never be persisted as mutable truth.

It must always be derived as:

```text
current_end_time =
base_end_time +
(extension_increment_seconds * number_of_extension_events)
```

## `AuctionCloseRecord`

Rules:

- exactly one `AuctionCloseRecord` must exist per opened auction
- no valid bids may be accepted after `AuctionCloseRecord`

## `ResolutionRecord`

Rules:

- exactly one `ResolutionRecord` must exist per closed auction
- presence of `ResolutionRecord` prohibits recomputation of resolution
- `settlement_deadline` must be persisted in `ResolutionRecord`
- `settlement_deadline` must be computed exactly once at resolution

## `SettlementRecord`

Rules:

- exactly one `SettlementRecord` must exist per resolved auction
- `SettlementRecord` must record terminal settlement outcome only
- `SettlementRecord` must not represent pending settlement
- presence of `SettlementRecord` prohibits recomputation of settlement outcome

## `FinalizationRecord`

Rules:

- exactly one `FinalizationRecord` must exist per auction
- presence of `FinalizationRecord` prohibits recomputation of destination
- destination is immutable once finalized
- sequence advancement to `N + 1` is permitted only after `FinalizationRecord`

---

# 9. Settlement Persistence

Settlement persistence must follow `bidding/SETTLEMENT.md`.

The settlement deadline must be persisted exactly once in `ResolutionRecord`.

`SettlementRecord` records terminal settlement outcome only.

`SettlementRecord.status` must be one of:

- `settled`
- `expired`

`not_required` must not be emitted in Demo 1 unless an active implementation slice explicitly defines a no-winner settlement path.

Settlement persistence must occur before `FinalizationRecord`.

Settlement does not create inscription authority.

Settlement does not consume inscription authority.

Settlement failure must be explicit and durable.

If settlement fails, final destination must be `NullSteward`.

If no valid bid is accepted, the auction remains `Scheduled`.

No settlement, finalization, inscription intent, or `NullSteward` outcome is produced solely because no valid bid exists.

---

# 10. Inscription Persistence

Inscription persistence must follow `inscription/INSCRIPTION-MACHINE.md`.

The inscription-related canonical event records are:

- `InscriptionIntentRecord`
- `InscriptionBroadcastRecord`
- `InscriptionConfirmationRecord`
- `AmbiguityRecord`

## `InscriptionIntentRecord`

Rules:

- must exist before any inscription broadcast attempt
- must follow `FinalizationRecord`
- does not consume inscription authority
- does not prove broadcast
- does not prove confirmation
- must not alter auction lifecycle state

## `InscriptionBroadcastRecord`

Rules:

- must not exist when inscription adapter mode is `deferred_in_this_slice`
- records classified broadcast outcome when live inscription broadcast is included in the active implementation slice
- `broadcast_outcome = committed` records `broadcast_commit`
- `broadcast_outcome = pre_commit_rejected` does not consume authority
- `broadcast_outcome = ambiguous` freezes authority permanently

## `InscriptionConfirmationRecord`

Rules:

- may exist only after an `InscriptionBroadcastRecord` with `broadcast_outcome = committed`
- does not consume additional authority
- must not remove, repair, or overwrite prior records
- terminal for inscription lifecycle

## `AmbiguityRecord`

Rules:

- must be persisted immediately when ambiguity is detected
- permanently freezes affected authority
- must not be deleted to unblock progress
- must not be repaired by restart or observation

If ambiguity is detected:

- authority is frozen
- no semantically distinct inscription attempt is permitted
- restart must preserve ambiguity

---

# 11. System Control Persistence

The system may persist system-level control events only through canonical event record types defined in `data/DATA-MODEL.md`.

For the current model, system pause and resume are persisted through:

- `PauseEventRecord`

System control records:

- must not grant authority
- must not restore authority
- must not alter auction lifecycle state
- must not alter inscription lifecycle state
- must not modify deadlines
- must not rewrite canonical event records

If additional system control records are needed, they must first be defined in `core/EVENT-TYPES.md` and `data/DATA-MODEL.md`.

---

# 12. Restart Semantics

On process startup, the system must:

1. load all persisted canonical event records
2. validate record shape and ordering
3. reconstruct state exclusively from persisted canonical event records
4. resume only transitions explicitly permitted by reconstructed state
5. never infer missing records

If required records are missing, malformed, contradictory, or outside the canonical record set:

- execution must halt
- authority must not be exercised

Restart is reconstruction, not recovery.

---

# 13. Idempotence Rules

Non-authority deterministic evaluations may be completed after restart only when explicitly permitted by `data/RESTART-RULES.md`.

Existing outcome records must not be recomputed.

Authority-consuming records must not be duplicated.

Authority-freezing records must not be removed or bypassed.

Duplicate authority consumption is forbidden.

Duplicate authority freeze is forbidden unless an explicit later record type permits additional scope-specific ambiguity recording.

---

# 14. Storage Requirements

This document does not mandate a production storage technology.

Permitted storage systems for the prototype include:

- SQLite
- append-only files
- write-ahead logs
- embedded databases

For Demo 1, SQLite is required by `IMPLEMENTATION-SLICE-01.md`.

Not permitted:

- volatile-only storage
- in-memory-only persistence
- best-effort caching without durability guarantees

Durability must survive:

- process crash
- machine restart
- ordinary application restart

---

# 15. Forbidden Persistence Patterns

The following are forbidden:

- recomputing outcomes after canonical event records exist
- treating logs as authoritative state
- treating API responses as authoritative state
- inferring resolution from absence of data
- repairing missing records by assumption
- deleting ambiguity to unblock progress
- re-consuming authority due to crash or timeout
- storing mutable lifecycle state as canonical truth
- silently simulating inscription broadcast
- silently simulating confirmation
- persisting canonical record types not defined in `core/EVENT-TYPES.md` and `data/DATA-MODEL.md`

If a required record is missing, execution must halt.

---

# Final Rule

Persistence defines canonical truth.

If the system cannot prove that an authority-bearing action did not occur:

It must behave according to `core/AUTHORITY-CONSUMPTION.md`.

Missing records do not create permission.

Restart does not create permission.

# Demo 1 Deterministic Identifier Validation

For Demo 1, persistence must validate deterministic identifiers defined in `data/DATA-MODEL.md`:

- `record_id = rec_` plus zero-padded 12-digit `sequence_index`
- `auction_id = auc_` plus zero-padded 12-digit auction `number`
- `bid_id = bid_` plus zero-padded 12-digit `sequence_index` of the `BidRecord`

If a persisted identifier does not match the deterministic rule, restart validation must halt.

# Resolution Input Hash Validation

When `ResolutionRecord` exists, persistence validation must recompute `resolution_inputs_hash` using the rule in `data/DATA-MODEL.md`.

A mismatch is fatal.

# Captured Configuration Validation

`AuctionRecord` must persist the fixed configuration snapshot required by `data/DATA-MODEL.md`.

Restart must use the captured values for bid admission, timing, extension, close, and reconstruction for that auction.

Current configuration values must not override captured auction values.
