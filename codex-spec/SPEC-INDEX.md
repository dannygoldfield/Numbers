# Numbers Codex Implementation Specification

This directory contains the current normative behavioral kernel
for backend implementation of Numbers.

The specification is evolving. Documents within this directory
may be revised until an explicit freeze declaration is made.

The following folders define the system:

- `core/` — state machine and invariants  
- `data/` — canonical records and restart semantics  
- `bidding/` — bid admission and settlement  
- `errors/` — error classification and escalation  
- `config/` — complete configuration surface  
- `api/` — external interface shapes and rules  

All backend behavior must be mechanically derivable from documents within this directory.

If required behavior is not specified here, it is undefined and must not be inferred.

No document outside this directory is authoritative for implementation.

Normative completeness is not assumed unless explicitly declared.