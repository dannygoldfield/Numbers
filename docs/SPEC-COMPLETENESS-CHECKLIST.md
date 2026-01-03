# Spec Completeness Checklist — Numbers

Purpose  
Track whether the documentation set is sufficient to produce a correct implementation of Numbers  
by a human developer or a next-generation LLM **without clarification**.

Legend  
- ✅ EXISTS = exists and is adequate    
- ❌ MISSING = must be created  

This checklist is normative.  
If something is marked PARTIAL, it is not “good enough yet.”

---

## 1. Orientation

- ✅  READ-THIS-FIRST.md  
  Needs a clearer statement of authority ordering and what this book is.

- ✅ WHAT-IF.md  
  Conceptual framing only. Correctly non-normative.

- ✅ BIDDER.md  
  Minimal and sufficient.

- ✅ GLOSSARY.md  
  Terminology lock. Should remain frozen.

- ❌ NON-REQUIREMENTS.md  
  Explicit list of what Numbers will never do.  
  High leverage. Prevents future drift.

---

## 2. System Definition (Core Semantics)

- ✅ CORE-SEQUENCE.md  
  Invariants are clear and strong.

- ✅  ARCHITECTURE.md  
  Component boundaries are good.  
  Would benefit from explicit restart guarantees and concurrency notes.

- ✅ STATE-MACHINE.md  
  **Critical missing file.**  
  Must define:
  - explicit states
  - allowed transitions
  - illegal transitions
  - persistence points

- ✅ CATALOG.md  
  Clear non-authority stance.

- ✅ ENVIRONMENT-DETERMINED-RENDERING.md  
  Complete and correct.

---

## 3. Product and Interface

- ✅ PRD.md  
  Correctly framed as invariant guardrail, not roadmap.

- ✅ WEBSITE-PRD.md  
  Scope boundary is clear.

- ✅ UI-SPEC.md  
  Strong constraints. No semantic leakage.

---

## 4. Platform and Data

- ✅ PLATFORM.md  
  Clean separation of components.

- ✅ DATA-MODEL.md  
  Append-only intent is clear.  
  Sufficient for implementation.

- ✅  API-SPEC.md  
  Endpoints are defined, but canonical response shapes are not locked.

- ✅ API-STATE-SHAPES.md  
  Canonical JSON objects for:
  - auction
  - settlement
  - inscription
  - degraded state

---

## 5. Security and Risk

- ✅ THREAT-MODEL.md  
  Explicit trust boundaries. Strong.

- ✅ LIMITS-AND-CIRCUIT-BREAKERS.md  
  Very strong safety envelope.

- ✅ KEY-MANAGEMENT-POLICY.md  
  Clear custody and rotation rules.

- ✅ ERROR-TAXONOMY.md  
  Errors are described implicitly across docs.  
  Needs a single classification and response matrix.

---

## 6. Operations

- ✅ DEPLOYMENT.md  
  Correctly respects auction boundaries.

- ✅ SECRETS-AND-CONFIG.md  
  Policy is clear.

- ❌ CONFIG-REFERENCE.md  
  Policy exists, but no concrete schema yet  
  (keys, types, defaults, ranges).

- ✅ OBSERVABILITY.md  
  Signals and alerts are well defined.

- ✅ OPERATIONAL-RUNBOOK.md  
  Practical and realistic.

---

## 7. Validation and Continuity

- ✅ TESTING.md  
  Clear definition of “tested enough.”

- ✅  LAUNCH-CHECKLIST.md  
  Useful, but should explicitly reference invariant checks.

- ✅  ROADMAP.md  
  Acceptable as non-normative, but must never contradict PRD.

- ✅  CANONICAL-EXAMPLES.md  
  End-to-end worked examples:
  - clean auction
  - no-bid auction
  - failed settlement
  - pause and resume

- ✅  TEST-VECTORS.md  
  Deterministic inputs and expected outputs.

---

## 8. Appendices and Context

- ✅ POETICS.md  
  Clearly marked non-binding.

- ✅ FAQ.md  
  Fine as explanatory only.

- ✅ JOKE.md  
  Contained. No protocol leakage.

- ✅  BOOK-INTRO.md / SUMMARY.md  
  Navigation aids. Not authoritative.

- ✅  TARGET-AUDIENCE.md  
  Useful context, not binding.

---

## 9. Non-Text Artifacts Required

- 🟡 Reference code skeleton  
  Exists as prototype, but not as a clean, logic-free spine.

- ❌ State machine artifact  
  Table, JSON, or diagram backing STATE-MACHINE.md.

- ❌ Concrete config files  
