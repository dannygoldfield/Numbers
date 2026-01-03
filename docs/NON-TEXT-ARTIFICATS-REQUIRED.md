# Non-Text Artifacts Required — Numbers

This document enumerates non-text artifacts required to make the Numbers
specification complete, implementable, and verifiable.

It is normative.

If an artifact listed here does not exist, the specification is incomplete.

---

## 1. Purpose

Text alone is insufficient to fully constrain implementation.

These artifacts exist to:

- Eliminate ambiguity that text cannot resolve
- Anchor interpretation across implementations
- Provide machine-checkable reference points
- Prevent “reasonable but incorrect” implementations

These artifacts do not add behavior.
They **lock** behavior already defined elsewhere.

---

## 2. Reference Code Skeleton

### Status
🟡 PARTIAL

### Description

A minimal, logic-free code skeleton that defines:

- Module boundaries
- Data flow direction
- Persistence boundaries
- Naming conventions
- Responsibility separation

This skeleton must:

- Compile
- Contain no business logic
- Contain no interpretation
- Contain no retries, heuristics, or inference
- Contain placeholders where logic must be filled in

### Purpose

The skeleton exists to answer:

- “Where does this logic live?”
- “What is allowed to talk to what?”
- “What must never be coupled?”

It prevents architectural drift while allowing internal freedom.

### Non-Goals

The skeleton must not:

- Implement auctions
- Implement settlement
- Implement inscription logic
- Encode policy decisions

---

## 3. State Machine Artifact

### Status
❌ MISSING

### Description

A canonical, machine-readable representation of the state machine defined in
STATE-MACHINE.md.

This artifact may be expressed as:

- A table
- JSON
- YAML
- A diagram with an accompanying machine-readable form

At minimum, it must define:

- All states
- All allowed transitions
- All forbidden transitions
- Transition triggers
- Persistence points

### Required Properties

- Deterministic
- Exhaustive
- Unambiguous
- Machine-parseable

Text in STATE-MACHINE.md must never contradict this artifact.
If a conflict exists, this artifact is authoritative.

### Purpose

This artifact exists so that:

- Implementations can validate transitions automatically
- LLMs cannot “invent” transitions
- Reviewers can reason about safety without prose

---

## 4. Concrete Configuration Files

### Status
❌ MISSING

### Description

Concrete, example configuration files that conform exactly to
CONFIG-REFERENCE.md.

These files must include:

- Default values
- Boundary values
- Explicit omissions
- Environment-specific variants (e.g. testnet)

Examples must be real, not schematic.

### Required Files (Minimum)

- `config.example.toml` (or equivalent)
- `config.testnet.toml`
- `config.invalid.toml` (demonstrates rejection)

### Purpose

These files exist to:

- Remove guesswork from configuration
- Prevent silent defaults
- Provide test fixtures
- Enable LLM grounding on real inputs

---

## 5. Artifact Authority Ordering

In case of conflict:

1. State Machine Artifact
2. Textual spec documents
3. Reference code skeleton
4. Example configuration files

No artifact may introduce behavior not defined elsewhere.

---

## 6. Final Rule

If an implementation decision cannot be justified by:

- A textual document **and**
- A non-text artifact

Then that decision is not permitted.
