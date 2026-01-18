# Invariant Index — Numbers

This document is **normative**.

It provides a canonical index of all invariants that govern the Numbers system
and maps each invariant to the documents that define, enforce, or depend on it.

This document introduces **no new behavior**.
It exists to eliminate ambiguity and prevent invariant loss across documents.

If there is a conflict,
the source documents referenced for each invariant take precedence.

---

## Purpose

The Invariant Index exists to ensure that:

- all invariants are explicitly enumerated
- no invariant is defined only implicitly
- implementers cannot miss an invariant due to document dispersion
- future changes cannot silently violate existing guarantees

This document is an index, not a restatement.

---

## Global Invariants

### I-01. Auction Numbers Are Monotonic

**Statement**  
Auction numbers advance strictly forward.
Each auction number appears exactly once and is never reused.

**Defined / Enforced In**
- CORE-SEQUENCE.md
- STATE-MACHINE.md
- STATE-MACHINE-TABLE.md

---

### I-02. Only One Auction May Be Active

**Statement**  
At most one auction may be in an active (open or resolving) state at any time.

**Defined / Enforced In**
- STATE-MACHINE.md
- TRANSITION-INVARIANTS.md
- ARCHITECTURE.md

---

### I-03. Each Auction Resolves Exactly Once

**Statement**  
Every auction produces exactly one resolution record.
Resolution is irreversible and must not be retried.

**Defined / Enforced In**
- STATE-MACHINE.md
- DATA-MODEL.md
- PERSISTENCE.md
- RESTART-RULES.md

---

### I-04. Authority Is Consumed Exactly Once

**Statement**  
Any authority to resolve, finalize, or inscribe is irreversibly consumed
once exercised and must never be reused.

**Defined / Enforced In**
- PERSISTENCE.md
- RESTART-RULES.md
- TRANSITION-INVARIANTS.md

---

### I-05. Ambiguity Permanently Freezes Authority

**Statement**  
If the system cannot determine whether an authority-bearing action
has already occurred, it must assume it has and refuse to retry.

Ambiguity reduces authority permanently.

**Defined / Enforced In**
- PERSISTENCE.md
- ERROR-TAXONOMY.md
- DATA-MODEL.md (Inscription ambiguity rules)

---

### I-06. Persistence Precedes Authority

**Statement**  
No authority-bearing action may occur unless the corresponding intent
or state transition has been durably persisted first.

**Defined / Enforced In**
- PERSISTENCE.md
- STATE-MACHINE.md

---

### I-07. Restart Does Not Recreate Permission

**Statement**  
System restart reconstructs state only.
It must not recompute, retry, or infer missing authority.

**Defined / Enforced In**
- RESTART-RULES.md
- PERSISTENCE.md
- STATE-MACHINE.md

---

### I-08. Settlement Does Not Block Sequencing

**Statement**  
Settlement success or failure must not delay or block
the progression of auction sequencing.

**Defined / Enforced In**
- ARCHITECTURE.md
- STATE-MACHINE.md
- DATA-MODEL.md

---

### I-09. Inscription Recognition Is Procedural

**Statement**  
Only inscriptions produced by the system’s finalized procedure
may be recognized as canonical, regardless of identical on-chain content.

**Defined / Enforced In**
- ARCHITECTURE.md
- DATA-MODEL.md
- CATALOG.md

---

### I-10. Catalog Is Non-Authoritative

**Statement**  
Catalog data must not grant, restore, or alter authority.
Catalog absence or corruption must not affect outcomes.

**Defined / Enforced In**
- CATALOG.md
- PERSISTENCE.md
- RESTART-RULES.md

---

### I-11. Pauses Occur Only at Auction Boundaries

**Statement**  
System pauses must not interrupt an active auction
and may occur only between auctions.

**Defined / Enforced In**
- CORE-SEQUENCE.md
- OPERATIONAL-RUNBOOK.md
- LIMITS-AND-CIRCUIT-BREAKERS.md

---

### I-12. UI and API Are Observational Only

**Statement**  
Interfaces must not introduce authority, interpretation,
or behavior not present in canonical state.

**Defined / Enforced In**
- UI-SPEC.md
- API-SPEC.md
- API-STATE-SHAPES.md

---

## Final Rule

If an invariant is not indexed here,
it is not an invariant.

All future specification changes
must update this index accordingly.
