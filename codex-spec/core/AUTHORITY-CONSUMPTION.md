# Authority Consumption — Numbers

This document defines authority consumption rules for the Numbers system.

It is normative.

Authority precedence is defined exclusively in AUTHORITY-ORDER.md.

Authority governs when permission to perform an irreversible action
is exercised and when it is permanently lost.

Authority is not intent.
Authority is not success.
Authority is the right to attempt an irreversible action exactly once.

Authority is consumed by attempt or by uncertainty.

---

## 1. Purpose

Authority ensures that:

- irreversible actions occur at most once
- outcomes are never duplicated
- retries do not recreate permission
- uncertainty reduces power rather than increasing it

---

## 2. Definitions

- **Authority**  
  The system’s permission to perform a defined irreversible external action.

- **Authority Consumption**  
  The irreversible use or loss of authority.

- **Authority Exhaustion**  
  A terminal condition where authority can no longer be exercised,
  regardless of outcome or intent.

Authority is consumed by attempt or by uncertainty,
not by confirmation of success.

---

## 3. Authority Scope (Normative)

Numbers defines authority in exactly one scope:

1. Inscription authority

No other authority exists.

Deterministic lifecycle evaluation (e.g., resolution, settlement)
does not constitute authority.

---

## 4. General Authority Rules (Normative)

### A-01. Authority Is Finite

For each auction number `N`:

- inscription authority exists at most once
- authority is never renewed
- authority is never recreated by time, restart, or operator action

---

### A-02. Authority Is Consumed at Explicit Boundaries

Inscription authority is consumed only at:

`Finalized → Inscribing`

when `InscriptionRecord` is durably persisted.

Authority must never be consumed implicitly.

---

### A-03. Persistence Precedes Authority

The canonical record defined in DATA-MODEL.md
must be durably persisted
before the authority-bearing action is attempted.

If required persistence fails:

- authority must not be exercised
- execution must halt

Authority without durable record is forbidden.

---

### A-04. Uncertainty Consumes Authority

If the system cannot prove
that an inscription attempt did not occur:

- inscription authority must be treated as consumed
- retry is forbidden
- alternate action is forbidden

Uncertainty reduces authority.
It never restores it.

---

## 5. Inscription Authority

Inscription authority:

- exists at most once per auction
- permits exactly one inscription attempt
- is permanently exhausted by ambiguity

Inscription authority is consumed when:

- `InscriptionRecord` is durably persisted

---

### 5.1 Pre-Initiation Failure

If inscription construction fails before
`InscriptionRecord` is durably persisted:

- inscription authority has not yet been consumed
- retry is permitted

This is the only retry condition.

---

### 5.2 Post-Initiation Ambiguity

If `InscriptionRecord` exists and the system:

- broadcasts a transaction, or
- cannot prove that no broadcast occurred

Then:

- an `AmbiguityRecord` must be durably persisted
- inscription authority is permanently exhausted
- retry is forbidden
- replacement is forbidden
- override is forbidden
- observation is the only permitted activity

Time passing does not restore authority.

---

## 6. Authority and Restart (Normative)

On restart:

- authority state must be reconstructed exclusively from persisted records
- absence of required persistence must halt execution
- ambiguous states remain ambiguous

Restart must never be used to:

- retry an authority-bearing action
- escape ambiguity
- recreate permission

Restart restores memory.
It does not grant authority.

---

## 7. Forbidden Authority Patterns

The following are forbidden:

- retrying after uncertainty
- retrying after timeout
- retrying after restart
- retrying after operator intervention
- retrying after partial success
- retrying to “complete” a prior attempt

If authority status is unclear,
authority must be treated as exhausted.

---

## Final Rule

If the system cannot prove
that an inscription attempt did not occur:

It must behave as though it did.