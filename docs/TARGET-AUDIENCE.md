# Target Audience — Numbers

This document describes who Numbers is for.

It is **non-normative**.

Nothing in this document grants requirements, authority, or guarantees.
If there is any conflict, all other specification documents take precedence.

---

## 1. Purpose

The purpose of this document is to:

- Clarify intended audiences
- Prevent accidental optimization for unintended readers
- Provide context for design choices without justifying them

This document does **not** influence system behavior.

---

## 2. Primary Audience (Implementation Agents)

### Automated Implementation Agents

The primary audience for the Numbers specification is:

- automated implementation agents
- LLM-based code generation systems

These agents are expected to:

- ingest the specification mechanically
- obey explicit authority ordering
- refuse to proceed on ambiguity or contradiction
- prefer halting over inference

Authority, ingestion order, and refusal conditions are defined in:

- `SPECIFICATION-ENTRY.md`

---

## 3. Secondary Audience (Human Implementers and Reviewers)

### Human Developers and Reviewers

Human developers interacting with Numbers are expected to act as:

- implementers following the same constraints as machines, or
- reviewers verifying conformance

They are expected to:

- read the specification in full
- respect invariants and non-requirements
- avoid “improving” semantics
- prefer refusal over interpretation

Human developers are **not** granted discretionary authority.
They are bound by the same rules as automated agents.

---

## 4. Tertiary Audience (Observers and Readers)

### Observers, Readers, and Analysts

Numbers is also readable by people who:

- are curious about auctions as systems
- are interested in minimal protocols
- treat Numbers as a conceptual or cultural artifact

This audience may never interact directly with the system.

Read-only access is sufficient.
No operational understanding is required.

---

## 5. Bitcoin-Native Participants

Numbers is designed for participants who:

- understand Bitcoin at a conceptual level
- accept probabilistic finality and uncertainty
- are comfortable with public, append-only systems
- do not expect customer support, reversibility, or guarantees

This includes, but is not limited to:

- Bitcoin developers
- Ordinals-aware users
- technically literate collectors
- protocol-adjacent experimenters

---

## 6. Explicitly Not the Audience

Numbers is **not** designed for:

- retail users expecting consumer UX
- users requiring identity recovery
- users expecting refunds or reversibility
- users unfamiliar with public ledgers
- users expecting interpretation, guidance, or education

If a user requires these things, Numbers is not appropriate.

---

## 7. Institutional and Commercial Use

Numbers does not target:

- enterprises
- institutions
- regulated financial actors
- custodial services

Any such use is external, unsupported, and non-authoritative.

---

## 8. Misuse Expectations

Numbers assumes:

- users may misunderstand outcomes
- users may attribute meaning incorrectly
- external narratives may form

The system does not adapt to these interpretations.

---

## 9. Final Rule

If an audience expectation requires the system to:

- explain itself
- protect the user
- reinterpret outcomes
- soften loss

That audience is out of scope.
