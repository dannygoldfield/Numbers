# Restart Rules — Numbers

This document defines mandatory behavior when the Numbers system restarts.

It is normative.

Authority precedence is defined exclusively in AUTHORITY-ORDER.md.

Restart handling ensures that:

- authority is never exercised twice
- outcomes are never recomputed after persistence
- ambiguity is never resolved by forgetting
- progress is never made without certainty

---

## 1. Principle

A restart is not a new execution.

A restart is continuation of the same execution,
with memory restored exclusively from persisted state.

Restart:

- does not introduce new states
- does not create authority
- does not resolve uncertainty
- does not alter lifecycle truth

If persisted state is incomplete, missing, or contradictory,
execution must halt.

---

## 2. Absolute Rules

### R-01. Persisted State Is Canonical

On restart:

- persisted records define reality
- all in-memory state is discarded
- logs are non-authoritative

If persisted state conflicts with expectation or operator belief,
persisted state prevails.

---

### R-02. No Recomputing After Persistence

On restart, the system must never recompute an outcome
if its canonical record already exists.

Specifically:

- if ResolutionRecord exists, resolution must not be recomputed
- if SettlementRecord exists, settlement must not be recomputed
- if FinalizationRecord exists, destination must not be recomputed
- if InscriptionRecord exists, inscription initiation must not be repeated

If an outcome record does not exist
but its prerequisite transition has occurred,
deterministic evaluation must complete and persist the missing record.

Unknown does not grant authority.

---

### R-03. Authority Is Never Recreated

Restart must not:

- restore consumed authority
- permit retries that were previously unsafe
- allow alternate irreversible actions
- change authority scope for an auction

Authority, once consumed, remains consumed permanently.

---

## 3. Restart Procedure (Normative)

On startup, the system must execute the following steps
in order.

---

### Step 1. Load Canonical Records

The system must load all canonical record types
defined in DATA-MODEL.md.

If any required canonical record is:

- missing
- unreadable
- malformed
- internally contradictory

Then execution must halt.
No authority may be exercised.

---

### Step 2. Reconstruct State Machines

For each auction number `N`:

- reconstruct auction state strictly from persisted records
- reconstruct inscription state strictly from persisted records
- reconstruct system control state strictly from persisted records

Inference is forbidden.

For auctions in state `Open`:

- load `base_end_time` from AuctionOpenRecord
- count ExtensionEventRecords
- derive:
```TEXT
current_end_time =
base_end_time +
(extension_increment_seconds * number_of_extension_events)
```

All time comparison must use authoritative `server_time`.

If reconstruction yields a state or transition
not permitted by STATE-MACHINE-TABLE.md:

Execution must halt.

---

### Step 3. Complete Deterministic Transitions

Only transitions explicitly permitted by reconstructed state
may be evaluated.

Permitted deterministic evaluations:

- `Open → Closed` if `server_time >= current_end_time`
- If AuctionCloseRecord exists and ResolutionRecord does not exist:
  - compute resolution deterministically
  - persist exactly one ResolutionRecord
- `AwaitingSettlement → Finalized` if settlement deadline expired

No other automatic transition is permitted.

Time must not:

- cause `Scheduled → Open`
- resolve ambiguity
- trigger inscription
- restore authority

---

### Step 4. Resume Eligibility by State

#### Scheduled
- Remains `Scheduled`
- No automatic advancement

#### Open
- If `server_time < current_end_time`, resume accepting bids
- If `server_time >= current_end_time`, persist AuctionCloseRecord

#### Closed
- If ResolutionRecord does not exist:
  - compute and persist ResolutionRecord
- If ResolutionRecord exists:
  - transition to `AwaitingSettlement`

#### AwaitingSettlement
- Resume observation only
- Deadlines must not be reset or modified

#### Finalized
- No auction transitions permitted
- Inscription lifecycle proceeds only if not yet initiated

#### NotStarted (Inscription)
- `Finalized → Inscribing` may occur once
- Only if InscriptionRecord does not exist

#### Inscribing
- If AmbiguityRecord exists → no action permitted
- If InscriptionRecord exists → no retry permitted
- Only observation permitted

#### Ambiguous
- Observation only
- No retry
- No alternate action

#### Inscribed
- No action permitted

---

### Step 5. Resume or Halt

If all reconstructed states are valid and consistent:

- resume execution only for transitions explicitly permitted
- authority-bearing transitions must satisfy all preconditions

If any state is invalid or incomplete:

- execution must halt
- operator inspection is required

---

## 4. Ambiguity Handling on Restart

Ambiguity survives restart.

If an auction is in `Ambiguous`:

- ambiguity remains
- inscription authority remains frozen
- retries remain forbidden
- observation remains the only permitted activity

Restart must never be used to escape ambiguity.

---

## 5. Forbidden Restart Behaviors

The following are forbidden:

- restarting to try again
- restarting to reclaim authority
- restarting to resolve ambiguity
- restarting to skip stalled auctions
- restarting to alter previously persisted outcomes

Restart is reconstruction, not repair.

---

## 6. Operator Obligations

If restart halts due to inconsistency:

- operator must not modify canonical records
- operator must not delete or fabricate history
- operator must not infer missing events

If authoritative history cannot be reconstructed,
the system must remain halted.

---

## Final Rule

On restart, any inscription attempt lacking explicit proof
that it did not occur
must be treated as already executed
and must not be retried.

Restart restores memory.
It does not grant permission.