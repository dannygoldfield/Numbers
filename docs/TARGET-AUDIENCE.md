# Target Audience
# Target Audience — Numbers

This document describes who Numbers is for.

It is **non-normative**.

Nothing in this document grants requirements, authority, or guarantees.
If there is any conflict, all other specification documents take precedence.

---

## 1. Purpose

The purpose of this document is to:

- Clarify intended readership and users
- Prevent accidental optimization for unintended audiences
- Provide context for design decisions without justifying them

This document does not influence system behavior.

---

## 2. Primary Audience

### Bitcoin-Native Participants

Numbers is designed for people who:

- Understand Bitcoin at a conceptual level
- Accept probabilistic finality and uncertainty
- Are comfortable interacting with public, append-only systems
- Do not expect customer support, reversibility, or guarantees

This includes, but is not limited to:
- Bitcoin developers
- Ordinals-aware users
- Technically literate collectors
- Protocol-adjacent experimenters

---

## 3. Secondary Audience

### Observers and Readers

Numbers is also readable by people who:

- Are curious about auctions as a system
- Are interested in minimal protocols
- Treat Numbers as a conceptual or cultural artifact

This audience may never interact directly with the system.

Read-only access is sufficient.

---

## 4. Explicitly Not the Audience

Numbers is not designed for:

- Retail users expecting consumer UX
- Users requiring identity recovery
- Users expecting refunds or reversibility
- Users unfamiliar with public ledgers
- Users expecting interpretation, guidance, or education

If a user requires these things, Numbers is not appropriate.

---

## 5. Developer Audience

Developers interacting with Numbers are expected to:

- Read the specification in full
- Respect invariants and non-requirements
- Avoid “improving” semantics
- Prefer refusal over interpretation

This includes human developers and LLM-based implementers.

---

## 6. Institutional and Commercial Use

Numbers does not target:

- Enterprises
- Institutions
- Regulated financial actors
- Custodial services

Any such use is external and unsupported.

---

## 7. Misuse Expectations

Numbers assumes:

- Users may misunderstand outcomes
- Users may attribute meaning incorrectly
- External narratives may form

The system does not adapt to these interpretations.

---

## 8. Final Rule

If an audience expectation requires the system to explain itself,
protect the user, or reinterpret outcomes:

That audience is out of scope.
