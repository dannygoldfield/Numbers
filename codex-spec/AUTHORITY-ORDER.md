# Authority Order — Numbers

The normative authority order of documents within `codex-spec` is:

1. core/INVARIANTS.md  
2. core/TRANSITION-INVARIANTS.md  
3. core/EVENT-TYPES.md  
4. core/STATE-MACHINE-TABLE.md  
5. core/AUTHORITY-CONSUMPTION.md  
6. core/STATE-MACHINE.md  
7. core/CORE-SEQUENCE.md  
8. settlement/SETTLEMENT.md  
9. inscription/INSCRIPTION-MACHINE.md  
10. inscription/INSCRIPTION-FORMAT.md  
11. chain/CHAIN-INTERACTION.md  
12. wallet/WALLET-SPEC.md  
13. data/DATA-MODEL.md  
14. data/PERSISTENCE.md  
15. data/RESTART-RULES.md  
16. errors/ERROR-TAXONOMY.md  
17. config/CONFIG-REFERENCE.md  
18. bidding/BIDDING-ADMISSION.md  
19. api/API-SPEC.md  
20. api/API-STATE-SHAPES.md  

If two documents conflict, the document higher in this list prevails.

Silence in a higher-authority document does not grant interpretive authority to a lower-authority document.  
Behavior must be explicitly defined to be valid.

`core/EVENT-TYPES.md` defines the canonical event records that form the append-only event log of the system.  
All lifecycle evaluation must derive exclusively from the ordered sequence of these events.

`core/STATE-MACHINE-TABLE.md` is the canonical definition of allowed lifecycle state transitions.  

`core/AUTHORITY-CONSUMPTION.md` defines irreversible authority boundaries.  

`inscription/INSCRIPTION-MACHINE.md` defines inscription lifecycle behavior but must not introduce new lifecycle states beyond those permitted by the state machine.  

`chain/CHAIN-INTERACTION.md` defines external truth recognition only and must not alter lifecycle semantics.  

`wallet/WALLET-SPEC.md` defines deterministic funding and signing behavior and must not alter lifecycle semantics.  

Documents outside `codex-spec` are non-authoritative with respect to execution semantics and must not override documents listed above.

This authority order governs conflict resolution while the specification is evolving and remains in force until an explicit freeze declaration is made.