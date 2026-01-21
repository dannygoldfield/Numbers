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
PERSISTENCE.md, and AUTHORITY-CONSUMPTION.md take precedence.

---

## Principle

A restart is **not** a new execution.

A restart is a continuation of the same execution,
with memory restored **exclusively** from persisted state.

Restart does not introduce new states.
Restart does not create authority.
Restart does not resolve uncertainty.

If persisted state is incomplete, missing, or contradictory,
the system **must** halt.

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
- attempt a different irreversible action
  for the same authority scope

Authority, once consumed, remains consumed permanently.

---

## Restart Procedure (Normative)

On startup, the system **must** execute the following steps
in the order listed.

---

### Step 1. Load All Persisted Canonical Records

The system **must** load all canonical record types
defined in `DATA-MODEL.md`, including but not limited to:

- auction lifecycle records
- resolution records
- settlement records
- inscription records
- ambiguity records
- system control events

If any **required canonical record** is:

- missing
- unreadable
- malformed
- internally contradictory

Then:

- execution **must** halt immediately
- no authority **may** be exercised

---

### Step 2. Reconstruct State Machines

For each auction number **N**:

- Reconstruct auction state strictly from persisted transitions
- Reconstruct inscription state strictly from persisted transitions
- Reconstruct system control state strictly from persisted events

Inference is forbidden.

If reconstruction yields a state or transition
not permitted by `STATE-MACHINE.md` or
`STATE-MACHINE-TABLE.md`:

- execution **must** halt

---

### Step 3. Determine Resume Eligibility

Resume behavior is defined **only** by the reconstructed state.

#### Scheduled
- Resume inter-auction gap timer if it has not expired
- Advancement before expiry is forbidden

#### Open
- If the auction end condition has not been met, resume bidding
- If the auction end condition has been met, transition to `Closed`

#### Closed
- A resolution record **must already exist**
- Resolution **must not** be recomputed
- Transition to `AwaitingSettlement`

#### AwaitingSettlement
- Resume settlement observation only
- Deadlines **must not** be reset, extended, or inferred

#### Finalized
- No auction actions are permitted
- Inscription lifecycle proceeds only if not yet started

#### NotStarted (Inscription)
- Inscription initiation is permitted

#### Inscribing
- Resume **only if** a persisted record proves
  the inscription attempt failed **before broadcast**
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
- no authority-bearing action **may** occur
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
- restarting with modified configuration
  to alter previously determined outcomes

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
that it did **not** occur
is treated as already completed
and **must not** be retried.

Restart restores memory.
It does not grant permission.
