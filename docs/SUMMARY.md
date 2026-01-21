# Summary — Numbers

This document defines the **reading order, authority structure, and compilation map**
of the Numbers specification.

It is **normative with respect to reading order only**.

This document introduces no new behavior.
It defines **how behavior-defining documents must be ingested and applied**.

The primary audience is:
- automated implementation agents
- LLM-based code generation systems

Human readability is secondary.

---

## Purpose

Numbers is specified in plain language but designed to be translated
mechanically into correct software.

The primary goal of this specification is to:

> Minimize guess-space during implementation.

This summary exists to:
- enforce strict authority ordering
- prevent accidental inference
- eliminate ambiguity about where truth is defined
- support deterministic translation into code, tests, and invariants
- serve as the table of contents for an LLM-focused mdBook

---

## Rules of Interpretation

**Rule 1: Authority is ordered**  
Documents earlier in the order have strictly higher authority.

If a contradiction is detected:
- the earlier document prevails
- execution must halt
- no reconciliation or inference is permitted

**Rule 2: Silence forbids**  
If behavior is not explicitly permitted, it is forbidden.

**Rule 3: Restart is reconstruction**  
State is reconstructed only from persisted records.
No recovery, repair, or retry is implied.

---

# Chapters

## Orientation (Non-Behavioral)

- [Read This First](READ-THIS-FIRST.md)
- [Target Audience](TARGET-AUDIENCE.md)
- [Why](WHY.md)
- [Protocol Core](PROTOCOL-CORE.md)
- [Poetics](POETICS.md)
- [Non-Requirements](NON-REQUIREMENTS.md)

## Product and Authority Foundations

- [Product Requirements](PRD.md)
- [Invariants](INVARIANTS.md)
- [Invariant Index](INVARIANT-INDEX.md)
- [Transition Invariants](TRANSITION-INVARIANTS.md)
- [Error Taxonomy](ERROR-TAXONOMY.md)
- [Errors](ERRORS.md)
- [Authority Consumption](AUTHORITY-CONSUMPTION.md)

## Core State and Execution Semantics

- [State Machine — Numbers](STATE-MACHINE.md)
- [State Machine — Canonical Table](STATE-MACHINE-TABLE.md)
- [State Machine Artifact](STATE-MACHINE-ARTIFACT.json)

## Persistence and Restart Semantics

- [Data Model](DATA-MODEL.md)
- [Persistence](PERSISTENCE.md)
- [Restart Rules](RESTART-RULES.md)

## Temporal and Procedural Flow

- [Core Sequence](CORE-SEQUENCE.md)
- [Settlement](SETTLEMENT.md)

## Configuration, Limits, and Safety Valves

- [Config Reference](CONFIG-REFERENCE.md)
- [Config Limits](CONFIG-LIMITS.md)
- [Limits and Circuit Breakers](LIMITS-AND-CIRCUIT-BREAKERS.md)

## External Interfaces

- [API State Shapes](API-STATE-SHAPES.md)
- [API Specification](API-SPEC.md)

## User Interface and Presentation

- [UI Specification](UI-SPEC.md)
- [Website PRD](WEBSITE-PRD.md)

## System Structure and Description

- [Architecture](ARCHITECTURE.md)
- [Catalog](CATALOG.md)
- [Platform](PLATFORM.md)
- [Environment-Determined Rendering](ENVIRONMENT-DETERMINED-RENDERING.md)

## Risk, Failure, and Security

- [Failure Modes](FAILURE-MODES.md)
- [Threat Model](THREAT-MODEL.md)
- [Security Goals](SECURITY-GOALS.md)
- [Security Non-Goals](SECURITY-NON-GOALS.md)

## Operations and Observability

- [Deployment](DEPLOYMENT.md)
- [Secrets and Configuration](SECRETS-AND-CONFIG.md)
- [Observability](OBSERVABILITY.md)
- [Operational Runbook](OPERATIONAL-RUNBOOK.md)

## Validation, Testing, and Continuity

- [Testing](TESTING.md)
- [Test Vectors](TEST-VECTORS.md)
- [Launch Checklist](LAUNCH-CHECKLIST.md)
- [Spec Completeness Checklist](SPEC-COMPLETENESS-CHECKLIST.md)
- [Roadmap](ROADMAP.md)

## Reference and Meta

- [Glossary](GLOSSARY.md)
- [Naming Convention](NAMING-CONVENTION.md)
- [Non-Text Artifacts Required](NON-TEXT-ARTIFACTS-REQUIRED.md)
- [Canonical Example](CANONICAL-EXAMPLE.md)
- [FAQ](FAQ.md)
