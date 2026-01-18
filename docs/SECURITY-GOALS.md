# Security Goals — Numbers

This document defines the **only** security goals of the Numbers system.

It is normative.

Any security mechanism, assumption, or implementation effort
must exist solely to satisfy the goals listed here.

If a security concern does not map directly to one of these goals,
it is out of scope.

If there is a conflict, PRD.md and INVARIANTS.md take precedence.

---

## Core Security Objective

The sole security objective of Numbers is:

**Prevent incorrect reuse of authority.**

Everything else is secondary or irrelevant.

---

## Security Goals

### S-01. Prevent Auction Rewrites

- An auction must not be reopened once closed
- A resolution must not be recomputed
- A finalized destination must not be altered

Security exists to preserve irreversibility.

---

### S-02. Prevent Authority Reuse

- Resolution authority may be exercised exactly once
- Settlement authority may be exercised exactly once
- Inscription authority may be exercised at most once

No retry, replay, or substitution is permitted.

---

### S-03. Preserve Ambiguity

- Ambiguity must not be collapsed into certainty
- Missing information must not be inferred
- Forgetting must not restore permission

Security protects uncertainty from being overwritten.

---

### S-04. Enforce State Legality

- Illegal state transitions must be detected
- Illegal transitions must halt execution
- Silent correction is forbidden

Security is strict, not forgiving.

---

### S-05. Preserve Persistent Memory

- Persisted records must survive crashes and restarts
- Absence of memory must block action
- Memory loss must never result in progress

Security treats forgetting as a critical failure.

---

### S-06. Prevent Cross-Number Contamination

- Authority for auction N must not affect auction N+1
- Failure in one number must not rewrite another
- Sequence order must not be violated

Security preserves isolation between numbers.

---

## Non-Goals Reminder

Security does **not** exist to:

- protect users from loss
- guarantee inscription success
- prevent manipulation
- enforce fairness
- ensure value

Those are explicitly excluded by SECURITY-NON-GOALS.md.

---

## Final Rule

If a proposed security measure does not clearly support
one or more goals listed above:

**It must not be implemented.**

Security exists to protect correctness.
Nothing else.
