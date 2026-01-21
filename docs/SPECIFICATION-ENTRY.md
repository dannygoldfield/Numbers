# Numbers — Specification Entry and Reading Order

```yaml
specification_entry:
  version: 1
  role: ingestion_contract
  normative: true
  introduces_behavior: false
  scope: specification_ingestion
  audience:
    - automated_implementation_agents
    - llm_based_code_generation_systems

authority_model:
  rule: earlier_document_wins
  on_conflict: halt
  silence_means: forbidden
  restart_semantics: reconstruction_only

required_reading_order:
  - PRD.md
  - INVARIANTS.md
  - TRANSITION-INVARIANTS.md
  - BIDDING-ADMISSION.md
  - STATE-MACHINE.md
  - STATE-MACHINE-TABLE.md
  - STATE-MACHINE-ARTIFACT.json
  - PERSISTENCE.md
  - RESTART-RULES.md
  - CORE-SEQUENCE.md
  - SETTLEMENT.md
  - CONFIG-REFERENCE.md
  - API-STATE-SHAPES.md
  - API-SPEC.md
  - UI-SPEC.md

refusal_conditions:
  - contradiction_detected
  - missing_required_document
  - unsafe_or_inferential_implementation
```