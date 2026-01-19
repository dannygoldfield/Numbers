# Read This First — Numbers

This repository defines **Numbers**.

Numbers is a specification-driven auction protocol.
The written specification is authoritative.

This documentation is designed to be sufficient for:
- a human developer, or
- a large language model

to produce a **correct implementation without clarification**.

If behavior is not specified here, it does not exist.

---

## How to Read This Specification

Not all documents have equal authority.

Some documents define **invariants** and **irreversible rules**.
Others define **procedures**, **constraints**, or **non-binding context**.

Every document clearly states whether it is **normative** or **non-normative**.

If a document does not explicitly grant authority, it does not grant authority.

---

## Authoritative Sources

The following documents are primary sources of truth:

- INVARIANTS.md  
- CORE-SEQUENCE.md  
- STATE-MACHINE.md  
- STATE-MACHINE-ARTIFACT.json  
- PERSISTENCE.md  
- SETTLEMENT.md  
- ERROR-TAXONOMY.md  

All behavior must be derivable from these documents.

If there is a conflict, the more restrictive rule applies.

---

## Design Discipline

Numbers is designed to reduce **guess-space** to zero.

This means:
- no inferred behavior
- no silent defaults
- no “reasonable assumptions”
- no recovery by interpretation

Ambiguity freezes authority.
Restart reconstructs state.
Loss is permitted.
Lying is not.

---

## What This Documentation Is Not

This is not:
- a roadmap
- a marketing document
- a UX guide
- a discussion of meaning or value

Intent, symbolism, and interpretation are explicitly out of scope.

---

## Implementation Rule

When implementing Numbers:

- Never invent behavior.
- Never improve semantics.
- Never optimize correctness away.
- If the specification is insufficient, stop and ask.

Correctness is the only success condition.

---

## Attribution

Written by Danny Goldfield  
Operating as *123456789 and 0*  
Contact: dannygoldfield@gmail.com
