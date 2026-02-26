# Authority Order — Numbers

The normative authority order of documents within `codex-spec` is:

1. core/INVARIANTS.md  
2. core/TRANSITION-INVARIANTS.md  
3. core/STATE-MACHINE-TABLE.md  
4. core/AUTHORITY-CONSUMPTION.md  
5. core/STATE-MACHINE.md  
6. core/CORE-SEQUENCE.md  
7. settlement/SETTLEMENT.md  
8. inscription/INSCRIPTION-MACHINE.md  
9. chain/CHAIN-INTERACTION.md  
10. wallet/WALLET-SPEC.md  
11. data/DATA-MODEL.md  
12. data/PERSISTENCE.md  
13. data/RESTART-RULES.md  
14. errors/ERROR-TAXONOMY.md  
15. config/CONFIG-REFERENCE.md  
16. bidding/BIDDING-ADMISSION.md  
17. api/API-SPEC.md  
18. api/API-STATE-SHAPES.md  

If two documents conflict, the document higher in this list prevails.

Silence in a higher-authority document does not grant interpretive authority to a lower-authority document.  
Behavior must be explicitly defined to be valid.

`core/STATE-MACHINE-TABLE.md` is the canonical definition of allowed state transitions.  
`core/AUTHORITY-CONSUMPTION.md` defines irreversible authority boundaries.  
`inscription/INSCRIPTION-MACHINE.md` defines inscription lifecycle behavior but must not introduce new lifecycle states beyond those permitted by the state machine.  
`chain/CHAIN-INTERACTION.md` defines external truth recognition only and must not alter lifecycle semantics.  
`wallet/WALLET-SPEC.md` defines deterministic funding and signing behavior and must not alter lifecycle semantics.  

Documents outside `codex-spec` are non-authoritative with respect to execution semantics and must not override documents listed above.

This authority order governs conflict resolution while the specification is evolving and remains in force until an explicit freeze declaration is made.