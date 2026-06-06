# Authority Consumption: Numbers

This document defines authority consumption rules for the Numbers system.

It is normative.

Authority precedence is defined exclusively in `AUTHORITY-ORDER.md`.

Authority governs when permission to perform an irreversible external action is exercised, frozen, or permanently lost.

Authority is not intent.

Authority is not success.

Authority is not confirmation.

Authority is the right to reach a defined irreversible boundary at most once.

---

# 1. Purpose

Authority ensures that:

- irreversible actions occur at most once
- outcomes are never duplicated
- retries do not recreate permission
- uncertainty reduces power rather than increasing it
- restart does not restore authority
- operator action does not restore authority

---

# 2. Definitions

## Authority

The system’s permission to perform a defined irreversible external action.

## Authority Consumption

The irreversible use of authority at a defined consumption boundary.

For inscription authority, the consumption boundary is `broadcast_commit`.

## Authority Exhaustion

A terminal condition where authority can no longer be exercised, regardless of whether the intended external action succeeded.

## Authority Freeze

A terminal authority condition caused by ambiguity.

Frozen authority must be treated as exhausted.

## broadcast_commit

The inscription authority consumption boundary.

`broadcast_commit` occurs only when both are true:

1. a candidate inscription transaction is broadcast via the authoritative node
2. the authoritative node reports the transaction present in its mempool

---

# 3. Authority Scope

Numbers defines authority in exactly one scope:

1. inscription authority

No other authority exists.

Deterministic lifecycle evaluation does not constitute authority.

The following do not consume authority:

- auction opening
- bid admission
- auction close
- resolution
- settlement determination
- finalization
- inscription intent persistence
- confirmation observation

---

# 4. General Authority Rules

## A-01: Authority Is Finite

For each auction number `N`:

- inscription authority exists at most once
- authority is never renewed
- authority is never recreated by time, restart, or operator action

---

## A-02: Authority Is Consumed Only at Explicit Boundary

Inscription authority is consumed only at:

```text
broadcast_commit
```

Authority must never be consumed implicitly.

Intent persistence does not consume authority.

Transaction construction does not consume authority.

Confirmation observation does not consume authority.

---

## A-03: Persistence Precedes Authority

`InscriptionIntentRecord` and all required pre-broadcast canonical records must be durably persisted before any broadcast attempt.

If required persistence fails:

- authority must not be exercised
- broadcast must not proceed
- execution must halt or reject according to the governing error specification

Authority without durable pre-broadcast record is forbidden.

---

## A-04: Ambiguity Freezes Authority

If the system cannot determine whether `broadcast_commit` occurred:

- inscription authority must be frozen
- frozen authority must be treated as exhausted
- retry is forbidden
- alternate inscription is forbidden
- semantically distinct inscription is forbidden

Ambiguity reduces authority.

Ambiguity never restores authority.

---

# 5. Inscription Authority

Inscription authority:

- exists at most once per auction
- permits at most one `broadcast_commit`
- is consumed by `broadcast_commit`
- is frozen by ambiguity
- is never restored after consumption or freeze

---

## 5.1 Pre-Commit Rejection

A pre-commit rejection occurs only when the system can determine that `broadcast_commit` did not occur.

The canonical broadcast outcome is:

```text
pre_commit_rejected
```

If broadcast outcome is `pre_commit_rejected`:

- inscription authority is not consumed
- inscription authority is not frozen
- inscription state remains `NotStarted`
- no confirmation may be inferred
- no success may be inferred

`pre_commit_rejected` does not by itself permit retry.

A later implementation slice may explicitly define bounded retry behavior.

If bounded retry behavior is not explicitly defined, no automatic retry is permitted.

---

## 5.2 Committed Broadcast

If broadcast outcome is:

```text
committed
```

then:

- `broadcast_commit` occurred
- inscription authority is consumed
- inscription state becomes `Inscribing`
- no semantically distinct inscription attempt is permitted

After `broadcast_commit`, only behavior explicitly permitted by the inscription specification may occur.

---

## 5.3 Ambiguous Broadcast

If broadcast outcome is:

```text
ambiguous
```

then:

- inscription authority is frozen
- frozen authority must be treated as exhausted
- inscription state becomes `Ambiguous`
- no further inscription attempt is permitted
- no semantically distinct inscription is permitted

Ambiguity must be recorded as canonical event record truth.

---

## 5.4 Controlled Fee Replacement

Controlled fee replacement is not required for Demo 1.

Controlled fee replacement is not active unless explicitly included in the active implementation slice.

If controlled fee replacement is active, it is permitted only under the equivalence rules defined in `inscription/INSCRIPTION-MACHINE.md`.

A replacement transaction must not create a semantically distinct inscription.

If equivalence cannot be proven, replacement is forbidden.

---

# 6. Authority and Restart

On restart:

- authority state must be reconstructed exclusively from canonical event records
- consumed authority remains consumed
- frozen authority remains frozen
- ambiguous state remains ambiguous
- missing required persistence must halt execution

Restart must never be used to:

- retry a committed broadcast
- escape ambiguity
- recreate permission
- infer success
- infer failure
- replace canonical records with external observation

Restart restores runtime memory.

Restart does not grant authority.

---

# 7. Authority and Observation

Observation does not create authority.

Observation does not restore authority.

Observation does not consume additional authority.

Observation may only produce canonical records when explicitly permitted by the governing specification.

A later observation must not erase or repair ambiguity.

---

# 8. Forbidden Authority Patterns

The following are forbidden:

- retrying after ambiguity
- retrying after restart unless explicitly permitted by a later implementation slice
- retrying after operator intervention unless explicitly permitted by a later implementation slice
- retrying to create a semantically distinct inscription
- modifying destination after authority consumption
- modifying payload after authority consumption
- treating inscription intent as authority consumption
- treating confirmation as authority consumption
- treating missing records as permission
- treating external chain observation as a substitute for canonical event records

If authority status is unclear, authority must be treated as frozen.

---

# Final Rule

If the system cannot prove that `broadcast_commit` did not occur:

It must behave as though authority is frozen and exhausted.