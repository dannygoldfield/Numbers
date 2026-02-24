# Authority Order — Numbers

The normative authority order of documents in `codex-spec` is:

1. INVARIANTS.md  
2. STATE-MACHINE-TABLE.md  
3. STATE-MACHINE.md  
4. TRANSITION-INVARIANTS.md  
5. CORE-SEQUENCE.md  
6. AUTHORITY-CONSUMPTION.md  
7. DATA-MODEL.md  
8. PERSISTENCE.md  
9. RESTART-RULES.md  
10. CONFIG-REFERENCE.md  
11. ERROR-TAXONOMY.md  
12. BIDDING-ADMISSION.md  
13. SETTLEMENT.md  
14. API-SPEC.md  
15. API-STATE-SHAPES.md  

If two documents conflict, the one higher in this list prevails.

`STATE-MACHINE-TABLE.md` is the canonical definition of allowed state transitions.  
`STATE-MACHINE.md` is explanatory and must not introduce transitions not present in the table.

Documents outside `codex-spec` are non-authoritative with respect to execution semantics and must not override documents listed above.