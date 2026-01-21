# Catalog — Numbers

This document defines the Catalog subsystem of Numbers.

It is **normative**.

The Catalog is a system-maintained, derived index of auction outcomes
and their corresponding on-chain inscription references.

It records **what was observed and recorded by procedure**.
It does not interpret meaning.

Bitcoin is the sole source of transactional truth.

If there is a conflict,
API-STATE-SHAPES.md, DATA-MODEL.md, PERSISTENCE.md,
RESTART-RULES.md, and INVARIANTS.md take precedence.

---

## Modal Language Rule (Normative)

In this document and all normative specifications:

- **must / must not** define obligations
- **only / exactly once / at most once** define bounds
- **may** is permitted **only** to describe uncertainty of knowledge,
  never permission, intent, or authority

The following terms are forbidden in normative contexts:

- possibly
- likely
- eventually
- for now
- TBD

Any normative statement using forbidden modal language is invalid.

---

## 1. Purpose (Normative)

The Catalog exists solely to support:

- observability
- retrieval
- inspection

The Catalog:

- does not define ownership
- does not assert value
- does not confer meaning
- does not validate external claims

It is an index, not an authority.

---

## 2. Authority Boundary (Normative)

The Catalog is **not authoritative**.

Errors, omissions, corruption, or inconsistency in the Catalog:

- **must not** alter auction outcomes
- **must not** alter settlement destinations
- **must not** alter inscription authority
- **must not** justify retries, repair, or reinterpretation

The blockchain is the only durable authority
for transaction existence and confirmation.

---

## 3. Dependency Rule (Normative)

The Catalog is a **derived view**.

It is computed from:
- canonical persisted records defined in DATA-MODEL.md
- observable Bitcoin transactions and inscriptions

The Catalog **must never** be used as an input to:
- state transitions
- authority decisions
- persistence logic
- restart reconstruction
- legality checks

If the Catalog is unavailable, incorrect, empty, or stale,
the system **must** continue operating solely from:
- canonical persisted records
- direct blockchain observation

---

## 4. Derivation Rules (Normative)

Every Catalog entry **must** be derivable from:

- persisted Numbers records
- Bitcoin transactions and inscriptions
- deterministic inspection of chain data

No Catalog entry **must** depend on:

- private memory
- operator judgment
- off-chain annotations
- inferred intent
- discretionary interpretation

Catalog entries describe **procedure-linked observations only**.

---

## 5. Catalog Contents (Normative)

For each auction number **N**, the Catalog **must represent only known facts**.

The following fields **may be present only if known**:

- auction number
- finalized destination address
- inscription txid
- inscription satpoint
- canonical timestamps from persisted records
- ambiguity indicator

Absence of a field indicates **absence of knowledge**,
not failure, negation, or error.

The Catalog **must not**:
- synthesize missing fields
- infer intent or outcome
- normalize ambiguity away

---

## 6. Append-Only Behavior (Normative)

During normal operation:

- the Catalog **must** be append-only
- existing entries **must not** be edited, rewritten, or deleted
- derived meaning **must not** change over time

New entries represent new observations only.
They do not revise prior records.

---

## 7. Rebuild Semantics (Normative)

The Catalog **may** be deleted and rebuilt **only as a whole**.

Rebuilding:

- replaces the Catalog in its entirety
- **must not** modify historical meaning
- **must not** exercise authority
- **must not** change outcomes

Reconstruction **must** converge on the same ordered entries
given the same chain data and canonical Numbers rules.

Partial rebuilds are forbidden.

No private data, operator judgment,
or discretionary repair is permitted during rebuild.

---

## 8. Failure and Availability (Normative)

Catalog failure is **non-fatal**.

If the Catalog is unavailable, corrupted, incomplete, or stale:

- auctions **must** continue according to canonical state
- authority **must not** be paused solely due to Catalog failure
- outcomes **must not** be inferred from Catalog absence

Catalog recovery **must not**:
- modify persisted state
- justify retries
- alter authority consumption

---

## 9. Scope Limits (Normative)

The Catalog does **not**:

- confer ownership
- resolve disputes
- assert authenticity beyond procedure
- override on-chain data
- create, restore, or consume authority
- explain or justify outcomes

All interpretation occurs outside the system.

---

## 10. Final Rule

The Catalog is a lens, not a judge.

It shows what the system recorded and observed.
It does not decide what it means.
