# Error Taxonomy: Numbers

This document defines how errors are classified and handled in Numbers.

It is normative.

Authority precedence is defined exclusively in `AUTHORITY-ORDER.md`.

Every error encountered by the system must be classified according to this taxonomy.

This taxonomy does not grant retry permission.

Behavior not explicitly permitted here or by the active implementation slice is forbidden.

If an error cannot be confidently classified, it must be classified as `Ambiguous`.

---

# 1. Purpose

This taxonomy exists to:

- prevent silent failure
- prevent unsafe retry
- prevent authority reuse
- preserve ambiguity when certainty is lost
- halt execution when invariants are violated
- distinguish rejection from corruption
- distinguish unknown from failure

Errors are treated as states of knowledge, not merely runtime exceptions.

---

# 2. Error Classes

All errors must belong to exactly one class:

1. `Deterministic`
2. `Operational`
3. `Ambiguous`
4. `Fatal`
5. `Operator`

An error must never belong to more than one class.

If classification is uncertain, the error must be classified as `Ambiguous`.

---

# 3. Retry Rule

Retry is not default behavior.

No automatic retry exists unless explicitly permitted by the active implementation slice.

Configuration must not create retry behavior.

Configuration must not turn an unspecified retry into a permitted retry.

A retry rule, if later permitted, must define:

- eligible operation
- maximum attempt count
- stopping condition
- persistence requirement
- authority effect
- error classification on exhaustion

Without those explicit rules, retry is forbidden.

---

# 4. Deterministic Errors

## Definition

An error is `Deterministic` when:

- the cause is known
- no irreversible external action has occurred
- no irreversible external action is suspected
- authority has not been consumed
- authority has not been frozen
- no ambiguity exists

## Examples

- invalid bid format
- bid below minimum
- bid submitted when auction state forbids admission
- bid destination address invalid
- invalid configuration at startup
- transaction construction failure before broadcast
- inscription payload serialization failure before broadcast

## Handling Rules

The action must be rejected explicitly.

The error must be surfaced through the governing interface.

The error must not consume authority.

The error must not freeze authority.

Canonical state must not be altered unless the governing specification explicitly requires a canonical event record.

For bid admission, if admission evaluation was reached, the deterministic rejection must be recorded as an invalid `BidRecord`.

Silent rejection is forbidden.

## Authority Impact

None.

Deterministic errors do not consume authority.

Deterministic errors do not freeze authority.

---

# 5. Operational Errors

## Definition

An error is `Operational` when:

- the cause is runtime or environmental
- no irreversible external action has occurred
- no irreversible external action is suspected
- authority has not been consumed
- authority has not been frozen
- the system can classify the failure without ambiguity

## Examples

- Bitcoin node unavailable before any broadcast attempt
- RPC timeout during read-only observation
- wallet unavailable before signing or broadcast
- storage unavailable before admission evaluation begins
- chain observation disabled in the active implementation slice
- live inscription unavailable in Demo 1

## Handling Rules

The operation must fail explicitly.

No lifecycle transition may be inferred.

No authority may be consumed.

No authority may be frozen unless uncertainty about an irreversible action is introduced.

Retry is forbidden unless explicitly permitted by the active implementation slice.

If the same operation is later invoked by a user or operator, it must be evaluated as a new operation only if doing so does not violate authority, lifecycle, persistence, or restart rules.

## Authority Impact

None, unless the error escalates to `Ambiguous`.

---

# 6. Ambiguous Errors

## Definition

An error is `Ambiguous` when:

- an irreversible external action has occurred, or
- an irreversible external action cannot be ruled out, or
- the system cannot determine whether an authority boundary was crossed, or
- certainty required for safe continuation has been lost

Ambiguity represents loss of certainty.

## Examples

- broadcast outcome unknown
- crash during or immediately after broadcast attempt
- RPC timeout after broadcast may have occurred
- network partition after broadcast attempt
- conflicting mempool observations after broadcast attempt
- persistence failure after an irreversible external action may have occurred
- inability to determine whether `broadcast_commit` occurred

## Handling Rules

Retry is forbidden.

Alternate inscription is forbidden.

Semantically distinct inscription is forbidden.

Affected authority must be frozen.

The ambiguity must be persisted if a canonical ambiguity record is required by the governing specification.

The system must preserve what is known and what is unknown.

The system must not repair ambiguity by:

- time passing
- restart
- operator action
- later observation
- speculative rebroadcast

## Authority Impact

Authority freeze is limited to the authority scope associated with the triggering irreversible action.

In Numbers, the authority scope is:

- inscription authority for the canonical number involved

Ambiguity must not affect unrelated auctions or unrelated authority scopes.

---

# 7. Fatal Errors

## Definition

An error is `Fatal` when:

- a core invariant is violated
- persisted canonical event records are contradictory
- canonical event record shape is malformed
- canonical event record order is invalid
- continued execution risks corrupting history
- implementation behavior would require invention
- chain interaction contradicts persisted canonical event records

## Examples

- invalid state transition
- attempt to reopen bidding
- duplicate authority-consuming record
- duplicate authority-freezing record
- attempt to consume authority twice
- corrupted canonical event record
- inconsistent restart reconstruction
- persisted confirmation record contradicted by authoritative chain truth
- canonical record type not defined in `core/EVENT-TYPES.md` or `data/DATA-MODEL.md`

## Handling Rules

Execution must halt immediately.

No further lifecycle transitions may be evaluated.

No authority may be exercised.

No canonical records may be rewritten to repair the condition.

Operator inspection is required.

The error must be logged durably.

Logs are non-authoritative.

## Authority Impact

Authority state remains as reconstructed from canonical event records.

Fatal errors do not restore authority.

Fatal errors do not consume additional authority unless the governing specification explicitly requires ambiguity classification first.

---

# 8. Operator Errors

## Definition

An error is caused by external human action.

## Examples

- invalid configuration
- incorrect key provisioning
- unsafe manual pause attempt
- attempted override of lifecycle rules
- attempted modification of canonical records
- attempted replay of a forbidden operation

## Handling Rules

Operator actions must be validated mechanically.

Violations of invariants must be rejected.

The error must be logged with operator context.

If operator action introduces uncertainty about an irreversible external action, the error must escalate to `Ambiguous`.

If operator action violates core invariants or threatens canonical history, the error must escalate to `Fatal`.

## Authority Impact

None, unless escalation occurs.

---

# 9. Escalation Rules

Errors may escalate.

Errors must never downgrade.

Permitted escalations are:

- `Deterministic → Fatal`
- `Operational → Ambiguous`
- `Operational → Fatal`
- `Operator → Ambiguous`
- `Operator → Fatal`

`Ambiguous` must not downgrade.

`Fatal` must not downgrade.

Once ambiguity or fatal state exists, authority does not return.

---

# 10. Logging Requirements

All errors must log:

- error class
- auction number if applicable
- lifecycle state if applicable
- triggering action
- timestamp
- error code
- human-readable message

`Ambiguous` and `Fatal` errors must additionally log:

- what is known
- what is unknown
- which actions are now forbidden

Logs are append-only and non-authoritative.

Logs must not substitute for canonical event records.

---

# 11. API Error Requirements

API errors must expose:

- `error_class`
- `error_code`
- `message`

API error codes must be machine-stable.

API error code meaning must not change within an API version.

API errors must not reveal private keys, wallet seed material, or sensitive operator secrets.

---

# 12. Summary Table

| Error Class | Retry Allowed | Authority Impact | Requires Halt |
|---|---|---|---|
| `Deterministic` | No, unless explicitly permitted by active implementation slice | None | No |
| `Operational` | No, unless explicitly permitted by active implementation slice | None unless escalated | No, unless specified |
| `Ambiguous` | No | Authority frozen within affected scope | No, unless required by governing spec |
| `Fatal` | No | No restoration, no additional authority by default | Yes |
| `Operator` | No, unless explicitly permitted by active implementation slice | Escalation-dependent | Escalation-dependent |

---

# 13. Design Principles

- absence of certainty is not permission
- retry is not default
- time passing is not evidence
- safety overrides liveness
- history must not be rewritten
- missing records do not create permission
- external observation does not replace canonical event records
- ambiguity is preserved, not repaired

---

# 14. Non-Goals

This taxonomy does not:

- optimize throughput
- minimize downtime
- conceal failure
- heuristically resolve ambiguity
- create retry behavior
- define lifecycle transitions
- define authority consumption boundaries

Its purpose is correctness.

---

# Final Rule

If an error cannot be confidently classified:

It must be classified as `Ambiguous`.

If continuing would require invented behavior:

Execution must halt.