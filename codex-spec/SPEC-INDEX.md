# Numbers Codex Implementation Specification

This directory contains the current normative behavioral kernel
for implementation of Numbers.

The specification is evolving. Documents within this directory
may be revised until an explicit freeze declaration is made.

Root-level documents define project-wide authority, scope, and specification structure:

- `AUTHORITY-ORDER.md`: defines authority order and conflict resolution.
- `PROTOTYPE-SCOPE.md`: defines the active implementation scope for the current deterministic, single-machine browser demo.
- `SPEC-INDEX.md`: describes the structure of this specification directory.

The following folders define the system:

- `core/`: state machine, invariants, event types, authority consumption, and core sequencing.
- `data/`: canonical records, persistence, and restart semantics.
- `bidding/`: bid admission and settlement.
- `errors/`: error classification and escalation.
- `config/`: complete configuration surface.
- `api/`: external interface shapes and rules.
- `chain/`: Bitcoin chain interaction rules.
- `wallet/`: wallet rules and constraints.
- `inscription/`: inscription format and inscription lifecycle rules.

All backend behavior must be mechanically derivable from documents within this directory.

If required behavior is not specified here, it is undefined and must not be inferred.

No document outside this directory is authoritative for implementation.

Normative completeness is not assumed unless explicitly declared.