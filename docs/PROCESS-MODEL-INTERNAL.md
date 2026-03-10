# Language-First Software Development Model — Internal Discipline

Status: Normative  
Purpose: Enforce phase separation and eliminate guess-space in all future software projects.

---

# Foundational Rule

Specification outranks code.

Code never introduces behavior.

Ambiguity is a defect.

---

# Phase 1 — Exploration

Role of LLM: Collaborator

Allowed:

- Structural experimentation
- Conceptual reframing
- Terminology discovery
- Authority modeling
- Edge case enumeration

Forbidden:

- Silent assumption locking
- Premature implementation
- Implicit defaults

Exit Criteria:

- Clear system boundary
- Identified invariants
- Draft transition model
- Defined authority surfaces

---

# Phase 2 — Specification Hardening

Role of LLM: Adversarial Reviewer

Objectives:

- Eliminate synonym drift
- Collapse alias collisions
- Remove implicit equivalence
- Forbid undefined behavior
- Define restart semantics
- Map all error surfaces

Rules:

- One concept → One term → One definition
- No unlisted transitions
- No hidden defaults
- No implementation-dependent language
- No behavior outside normative documents

Semantic freeze occurs here.

If ambiguity exists, do not proceed.

---

# Phase 3 — Deterministic Implementation

Role of LLM: Compiler

Rules:

- No reinterpretation
- No feature expansion
- No inferred defaults
- No silent retries
- No semantic drift

If ambiguity appears:

Stop.
Return to Phase 2.

Implementation must feel mechanical and boring.

Drama indicates underspecification.

---

# Phase 4 — Audit

Role of LLM: Verifier

Validate:

- State machine fidelity
- Invariant enforcement
- Authority consumption
- Restart correctness
- Error mapping completeness

If drift is detected:

Amend specification first.
Then regenerate or correct code.

Never patch behavior solely in code.

---

# Phase Separation Rule

Phase leakage reintroduces guess-space.

Collaboration allowed only in Phase 1.
Adversarial review required in Phase 2.
Strict translation only in Phase 3.
Adversarial validation in Phase 4.

---

# Long-Term Objective

Prove that sufficiently precise natural language can produce deterministic systems.

If implementation requires invention, the language failed.

Correct the language.
Then rebuild.