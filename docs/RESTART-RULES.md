# Restart Rules — Numbers

This document defines mandatory behavior when the Numbers system restarts.

It is **normative**.

Restart handling exists to ensure that:
- authority is never exercised twice
- outcomes are never recomputed
- ambiguity is never resolved by forgetting
- progress is never made without certainty

If there is a conflict,
PRD.md, INVARIANTS.md, CORE-SEQUENCE.md, STATE-MACHINE.md,
and PERSISTENCE.md take precedence.

---

## Principle

A restart is **not** a new execution.

A restart is a continuation of the same execution,
with memory restored **exclusively** from persisted state.

If persisted memory is incomplete, missing, or contradictory,
the system **must** halt.

Restart does not introduce new states.

On restart, the system **must not** create, infer, or substitute any state
other than those explicitly defined in STATE-MACHINE.md and
STATE-MACHINE-TABLE.md.

Restart operates only by reconstructing and re-entering
existing lifecycle states.

---

## Absolute Rules

### R-01. Persisted State Is Canonical

On restart:

- Persisted records define reality
- All in-memory state is discarded
- Logs are non-authoritative and advisory only

If persisted state conflicts with expectation, intent,
or operator belief,
expectation, intent, and belief are wrong.

---

### R-02. No Recomputing

On restart, the system **must never**:

- recompute auction resolution
- recompute settlement outcomes
- recompute inscription decisions
- infer outcomes from missing or partial data

If an outcome is not durably persisted,
it **must** be treated as **unknown**.

Unknown does **not** grant authority.

---

### R-03. Authority Is Never Recreated

Restart **must not**:

- restore consumed authority
- permit retries that were previously unsafe
- allow alternate actions for the same auction
- “try again” after uncertainty or ambiguity

Authority, once consumed, remains consumed permanently.

---

## Restart Procedure (Normative)

On startup, the backend **must** execute the following steps
in the order listed.

---

### Step 1. Load All Persisted Records

The system **must** load:

- auction lifecycle records
- settlement records
- inscription records
- ambiguity detection records
- system control events

If any required record is missing, unreadable,
malformed, or contradictory:

- execution **must** halt immediately
- no authority **may** be exercised

---

### Step 2. Reconstruct State Machines

For each auction number **N**:

- Rebuild auction state strictly from persisted transitions
- Rebuild inscription state strictly from persisted transitions
- Rebuild system control state strictly from persisted events

Inference is forbidden.

If reconstruction yields an invalid or forbidden transition:

- execution **must** halt

---

### Step 3. Determine Resume Eligibility

For each reconstructed state:

#### Scheduled
- Resume the inter-auction timer if it has not expired
- Advancement before expiry is forbidden

#### Open
- If the auction end time has not passed, resume bidding
- If the auction end time has passed, transition to `Closed`

#### Closed
- A resolution record **must** already exist
- Resolution **must not** be recomputed
- Transition to `AwaitingSettlement`

#### AwaitingSettlement
- Resume settlement observation only
- Deadlines **must not** be reset, extended, or inferred

#### Finalized
- No auction actions are permitted
- Inscription lifecycle proceeds only if not yet started

#### Inscribing
- Resume **only** if a persisted Inscription record
  explicitly indicates a pre-broadcast failure
- Absence of a txid, absence of confirmation,
  or lack of observation **must not** be interpreted
  as pre-broadcast failure
- If broadcast occurred or cannot be ruled out,
  transition to `Ambiguous`

#### Ambiguous
- Observation only
- Retries are forbidden
- Alternate actions are forbidden

#### Inscribed
- No action is permitted

---

### Step 4. Resume or Halt

If **all** reconstructed states are valid and resumable:

- resume execution **only** for actions explicitly permitted
  by the reconstructed state machines
- no new authority-bearing action **may** occur
  unless its preconditions are fully satisfied

If **any** state is invalid, contradictory, or incomplete:

- execution **must** halt
- operator inspection is required

---

## Ambiguity Handling on Restart

Ambiguity survives restarts.

If an auction was ambiguous before restart:

- ambiguity remains
- authority remains frozen
- retries remain forbidden
- observation remains the only permitted activity

Restart **must never** be used to escape ambiguity.

---

## Forbidden Restart Behaviors

The following behaviors are explicitly forbidden:

- restarting to “try again”
- restarting to reclaim lost authority
- restarting to resolve ambiguity
- restarting to skip stalled auctions
- restarting with modified configuration to alter outcomes

Restart is not a repair mechanism.

---

## Operator Obligations

If restart halts due to inconsistency:

- the operator **must** determine whether authoritative history
  can be proven
- the operator **must not** modify, delete, fabricate,
  or reinterpret records
- the operator **must not** infer or invent missing events

If authoritative history cannot be reconstructed faithfully,
the system **must** remain halted.

---

## Final Rule

On restart, any action lacking explicit proof
of non-execution is treated as already completed
and **must not** be retried.

Restart restores memory.
It does not grant permission.
