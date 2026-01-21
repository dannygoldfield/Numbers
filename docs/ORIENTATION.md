# Orientation — Numbers

This repository defines **Numbers**.

Numbers is a specification-driven system.
The written specification is authoritative.

This documentation is designed to be sufficient for:
- a human developer, or
- a large language model

to produce a **correct implementation without clarification**.

If behavior is not specified in the normative documents,
it does not exist.

---

## What This Document Is

This document is **non-normative**.

It exists to:
- orient new readers
- explain how to approach the specification
- set expectations about rigor and discipline

It does **not** define behavior.
It does **not** grant authority.
It must not be used to infer rules.

---

## How to Read the Specification

Not all documents have equal authority.

Some documents define:
- invariants
- irreversible rules
- state transitions
- persistence requirements

Other documents provide:
- context
- explanation
- structure
- motivation

Every document clearly states whether it is **normative** or **non-normative**.

If a document does not explicitly grant authority,
it does not grant authority.

The authoritative reading order and ingestion rules
are defined in:

- `SPECIFICATION-ENTRY.md`

This document must defer to that contract.

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

Those may exist elsewhere.
They do not exist here.

---

## Implementation Guidance (Non-Normative)

Implementation obligations are defined elsewhere.

This document offers only orientation.

If the specification is insufficient,
the correct action is to stop.

---

## Attribution

Written by Danny Goldfield  
Operating as *123456789 and 0*  
Contact: dannygoldfield@gmail.com
