# Summary — Numbers

This document defines the reading order and authority structure of the Numbers specification.

Documents are grouped by role and authority.
If a document defines system behavior, it is listed under **System Law**.
If a document describes consequences, operations, or interpretation boundaries, it appears elsewhere.

This file introduces no new rules.

---

## Orientation

Start here if you are new.

- [Read This First](READ-THIS-FIRST.md)
- [What If](WHAT-IF.md)
- [Bidder](BIDDER.md)
- [Glossary](GLOSSARY.md)

---

## System Law (Normative)

These documents define the behavior, constraints, and authority boundaries of the Numbers system.

They are precedence-critical.

If there is a conflict between documents, precedence is defined within these files.

- [Product Requirements](PRD.md)
- [Core Sequence](CORE-SEQUENCE.md)
- [State Machine — Canonical Table](STATE-MACHINE-TABLE.md)
- [State Machine — Numbers](STATE-MACHINE.md)
- [Invariants](INVARIANTS.md)
- [Transition Invariants](TRANSITION-INVARIANTS.md)
- [Persistence](PERSISTENCE.md)
- [Errors](ERRORS.md)

---

## System Definition (Descriptive)

These documents describe the structure that emerges from the system law.
They do not redefine behavior.

- [Architecture](ARCHITECTURE.md)
- [Catalog](CATALOG.md)
- [Environment-Determined Rendering](ENVIRONMENT-DETERMINED-RENDERING.md)

---

## Product and Interface

These documents define how the system is presented and interacted with.
They adapt to the system; they do not shape it.

- [Product Requirements](PRD.md)
- [Website PRD](WEBSITE-PRD.md)
- [UI Specification](UI-SPEC.md)

---

## Platform and Data

These documents define data representation and access patterns.
They expose system state without interpretation.

- [Platform](PLATFORM.md)
- [Data Model](DATA-MODEL.md)
- [API Specification](API-SPEC.md)
- [API State Shapes](API-STATE-SHAPES.md)

---

## Risk and Failure (By Design)

These documents describe failure modes, limits, and boundaries that are expected outcomes of the system design.

Failure does not imply error.
Documentation here does not grant authority to correct outcomes.

- [Failure Modes](FAILURE-MODES.md)
- [Threat Model](THREAT-MODEL.md)
- [Limits and Circuit Breakers](LIMITS-AND-CIRCUIT-BREAKERS.md)
- [Security Goals](SECURITY-GOALS.md)
- [Security Non-Goals](SECURITY-NON-GOALS.md)

---

## Operations

These documents describe how the system is run and observed.
They do not alter system truth.

- [Deployment](DEPLOYMENT.md)
- [Secrets and Configuration](SECRETS-AND-CONFIG.md)
- [Observability](OBSERVABILITY.md)
- [Operational Runbook](OPERATIONAL-RUNBOOK.md)
- [Restart Rules](RESTART-RULES.md)

---

## Validation and Continuity

These documents support verification, launch readiness, and long-term continuity.

- [Testing](TESTING.md)
- [Test Vectors](TEST-VECTORS.md)
- [Launch Checklist](LAUNCH-CHECKLIST.md)
- [Roadmap](ROADMAP.md)
- [Poetics](POETICS.md)
- [FAQ](FAQ.md)

---

## Reference and Meta

These documents provide supporting structure and naming discipline.

- [Naming Convention](NAMING-CONVENTION.md)
- [Non-Requirements](NON-REQUIREMENTS.md)
- [Non-Text Artifacts Required](NON-TEXT-ARTIFACTS-REQUIRED.md)
- [Canonical Example](CANONICAL-EXAMPLE.md)
- [Spec Completeness Checklist](SPEC-COMPLETENESS-CHECKLIST.md)
