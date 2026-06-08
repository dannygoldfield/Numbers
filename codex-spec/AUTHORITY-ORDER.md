# Authority Order: Numbers

The normative authority order of documents within codex-spec is:

1. AUTHORITY-ORDER.md
2. PROTOTYPE-SCOPE.md
3. core/INVARIANTS.md
4. core/TRANSITION-INVARIANTS.md
5. core/NULLSTEWARD.md
6. core/EVENT-TYPES.md
7. core/STATE-MACHINE-TABLE.md
8. core/AUTHORITY-CONSUMPTION.md
9. core/STATE-MACHINE.md
10. core/CORE-SEQUENCE.md
11. bidding/SETTLEMENT.md
12. inscription/INSCRIPTION-MACHINE.md
13. inscription/INSCRIPTION-FORMAT.md
14. chain/CHAIN-INTERACTION.md
15. wallet/WALLET-SPEC.md
16. data/DATA-MODEL.md
17. data/PERSISTENCE.md
18. data/RESTART-RULES.md
19. errors/ERROR-TAXONOMY.md
20. config/CONFIG-REFERENCE.md
21. bidding/BIDDING-ADMISSION.md
22. api/API-SPEC.md
23. api/API-STATE-SHAPES.md

If two documents conflict, the document higher in this list prevails.

Silence in a higher-authority document does not grant interpretive authority to a lower-authority document.

Behavior must be explicitly defined to be valid.

AUTHORITY-ORDER.md defines document authority and conflict resolution.

PROTOTYPE-SCOPE.md defines the active implementation scope for the current deterministic, single-machine browser demo.

PROTOTYPE-SCOPE.md may constrain what must be implemented in the current prototype, but it must not redefine lifecycle semantics, authority consumption, canonical records, or restart rules.

core/INVARIANTS.md defines system-wide invariants that all lower-authority documents must obey.

core/TRANSITION-INVARIANTS.md defines invariant rules that govern valid lifecycle transitions.

core/NULLSTEWARD.md defines NullSteward as a protocol-visible final destination and constrains its use.

core/EVENT-TYPES.md defines the canonical event records that form the append-only event log of the system.

All lifecycle evaluation must derive exclusively from the ordered sequence of canonical event records.

core/STATE-MACHINE-TABLE.md is the canonical definition of allowed lifecycle state transitions.

core/AUTHORITY-CONSUMPTION.md defines irreversible authority boundaries.

core/STATE-MACHINE.md describes lifecycle state behavior and must remain consistent with core/STATE-MACHINE-TABLE.md.

core/CORE-SEQUENCE.md defines canonical sequence progression.

bidding/SETTLEMENT.md defines settlement behavior.

inscription/INSCRIPTION-MACHINE.md defines inscription lifecycle behavior but must not introduce new lifecycle states beyond those permitted by the state machine.

inscription/INSCRIPTION-FORMAT.md defines inscription payload format and must not alter lifecycle semantics.

chain/CHAIN-INTERACTION.md defines external chain truth recognition only and must not alter lifecycle semantics.

wallet/WALLET-SPEC.md defines deterministic funding and signing behavior and must not alter lifecycle semantics.

data/DATA-MODEL.md defines canonical persisted record structure and must remain consistent with canonical event definitions.

data/PERSISTENCE.md defines persistence rules for canonical records.

data/RESTART-RULES.md defines reconstruction behavior after restart.

errors/ERROR-TAXONOMY.md defines error classification and escalation behavior.

config/CONFIG-REFERENCE.md defines permitted configuration parameters.

Configuration may tune declared parameters only and must not alter canonical truth, lifecycle semantics, authority consumption, or restart reconstruction.

bidding/BIDDING-ADMISSION.md defines bid admission rules.

api/API-SPEC.md defines external API behavior.

api/API-STATE-SHAPES.md defines frontend-visible API state shapes.

Documents outside codex-spec are non-authoritative with respect to execution semantics and must not override documents listed above.

This authority order governs conflict resolution while the specification is evolving and remains in force until an explicit freeze declaration is made.