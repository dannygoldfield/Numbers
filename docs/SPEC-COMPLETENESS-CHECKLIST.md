# Spec Completeness Checklist — Numbers

## Purpose

Track whether the documentation set is sufficient to produce a **correct implementation**
of Numbers by a human developer or a next-generation LLM **without clarification**.

This checklist is **normative**.

If an item is marked 🟡 PARTIAL, it is **not complete**.

---

## Legend

- ✅ EXISTS — exists and is adequate  
- 🟡 PARTIAL — exists but requires tightening or explicit completion  
- ❌ MISSING — must be created  

---

## 1. Orientation

- ✅ READ-THIS-FIRST.md  
  Defines how to approach the specification and establishes authority ordering.

- ✅ WHAT-IF.md  
  Conceptual framing only. Correctly non-normative.

- ✅ BIDDER.md  
  Minimal and sufficient definition of bidder role and expectations.

- ✅ GLOSSARY.md  
  Terminology lock. Remains frozen.

- ✅ NON-REQUIREMENTS.md  
  Explicit list of what Numbers will never do.  
  Prevents future scope drift.

---

## 2. System Definition (Core Semantics)

- ✅ CORE-SEQUENCE.md  
  Canonical sequencing rules and temporal invariants.

- ✅ ARCHITECTURE.md  
  Component boundaries, authority separation, and lifecycle description.

- ✅ STATE-MACHINE.md  
  Normative lifecycle definition including:
  - explicit states
  - allowed transitions
  - forbidden transitions
  - authority loss rules
  - restart semantics

- ✅ STATE-MACHINE-TABLE.md  
  Canonical transition table.

- ✅ STATE-MACHINE-ARTIFACT.json  
  Machine-readable backing artifact.

- ✅ TRANSITION-INVARIANTS.md  
  Cross-state safety properties enforced across subsystems.

- ✅ INVARIANTS.md  
  Global invariants governing correctness and irreversibility.

- ✅ AUTHORITY-CONSUMPTION.md  
  Defines:
  - what constitutes authority
  - when it is irreversibly consumed
  - how multiple records represent a single burn

- ✅ INVARIANT-INDEX.md  
  Canonical index of all invariants and authority rules.  
  Normative. Required to prevent invariant loss across documents.

- ✅ ENVIRONMENT-DETERMINED-RENDERING.md  
  Complete and correct. Explicitly non-normative.

---

## 3. Product and Interface

- ✅ PRD.md  
  Invariant guardrails. Not a roadmap.

- ✅ WEBSITE-PRD.md  
  Scope boundary is clear. No semantic leakage.

- ✅ UI-SPEC.md  
  Strong constraints. Presentation cannot alter meaning or authority.

---

## 4. Platform and Data

- ✅ PLATFORM.md  
  Separation of backend, chain interaction, and interfaces.

- ✅ DATA-MODEL.md  
  Canonical, append-only data definitions.

- ✅ API-SPEC.md  
  Entire external API surface defined.  
  Knowledge-only exposure rules are explicit.

- ✅ API-STATE-SHAPES.md  
  Canonical JSON shapes for:
  - auction
  - settlement
  - inscription
  - degraded state

- ✅ CATALOG.md  
  Explicit non-authoritative, derived-index stance.

---

## 5. Security and Risk

- ✅ THREAT-MODEL.md  
  Explicit trust boundaries and allowed responses.

- ✅ LIMITS-AND-CIRCUIT-BREAKERS.md  
  Bounded failure envelope. No authority leakage.

- ✅ KEY-MANAGEMENT-POLICY.md  
  Custody, rotation, and authority constraints.

- ✅ SECURITY-GOALS.md  
  Explicit positive security objectives.

- ✅ SECURITY-NON-GOALS.md  
  Explicit exclusions and non-promises.

- ✅ ERROR-TAXONOMY.md  
  Canonical error classes and escalation semantics.

- ✅ ERRORS.md  
  Execution-time error handling rules.

- ✅ FAILURE-MODES.md  
  Enumerated non-bug failure outcomes.

---

## 6. Operations

- ✅ DEPLOYMENT.md  
  Deployment constrained by auction and authority boundaries.

- ✅ SECRETS-AND-CONFIG.md  
  Secrets vs configuration authority separation.

- ✅ CONFIG-REFERENCE.md  
  Configuration surface area locked:
  - keys
  - types
  - defaults
  - valid ranges
  - validation rules

- ✅ CONFIG-LIMITS.md  
  Explicit operational limits.

- ✅ OBSERVABILITY.md  
  Required signals, alerts, and operator duties.

- ✅ OPERATIONAL-RUNBOOK.md  
  Human action constrained as an authority surface.

- ✅ RESTART-RULES.md  
  Restart behavior defined as reconstruction, not recovery.

- ✅ PERSISTENCE.md  
  Normative persistence guarantees tied directly to authority consumption.

---

## 7. Validation and Continuity

- ✅ TESTING.md  
  Clear definition of “tested enough.”

- ✅ LAUNCH-CHECKLIST.md  
  Launch gated on invariant verification.

- ✅ ROADMAP.md  
  Non-normative. Must never contradict PRD.

- ✅ CANONICAL-EXAMPLE.md  
  End-to-end worked examples:
  - clean auction
  - no-bid auction
  - failed settlement
  - pause and resume

- ✅ TEST-VECTORS.md  
  Deterministic inputs and expected outputs.

---

## 8. Appendices and Context

- ✅ POETICS.md  
  Explicitly non-binding.

- ✅ FAQ.md  
  Explanatory only.

- ✅ JOKE.md  
  Contained. No protocol leakage.

- ✅ BOOK-INTRO.md  
  Navigation and framing only.

- ✅ SUMMARY.md  
  Navigation index. Not authoritative.

- ✅ TARGET-AUDIENCE.md  
  Context only. Not binding.

---

## 9. Non-Text Artifacts (Non-Blocking)

The following are **implementation artifacts**, not specification gaps:

- 🟡 Reference code skeleton  
  Prototype exists but not yet a clean, minimal structural spine.

- 🟡 Concrete configuration files  
  Policy exists; example environment-specific files not yet locked.

---

## Status Summary

All normative specification documents required to implement Numbers
now exist and are complete.

The specification is:

- closed under restart
- closed under crash
- closed under ambiguity
- closed under authority reuse

Any change that affects system behavior
requires an explicit amendment to the specification.

All remaining work is implementation or non-authoritative exemplars.

