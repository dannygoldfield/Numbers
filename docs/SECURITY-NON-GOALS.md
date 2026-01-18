# Security Non-Goals — Numbers

This document defines what the Numbers system explicitly does **not** attempt to secure, prevent, or guarantee.

It is normative.

Security measures that are not listed as goals here must not be implied, assumed, or inferred by users, operators, or implementers.

If there is a conflict, PRD.md and INVARIANTS.md take precedence.

---

## Purpose

This document exists to prevent false assumptions.

Numbers is a constrained system.
Security is applied only where required to preserve system semantics and authority boundaries.

Anything outside those boundaries is intentionally out of scope.

---

## Core Principle

Numbers secures **correctness**, not **outcomes**.

The system guarantees:
- sequence integrity
- authority non-reuse
- irreversible outcomes

The system does **not** guarantee:
- user safety
- economic protection
- fairness beyond the auction rules
- protection from self-harm or poor decisions

---

## Explicit Non-Goals

The Numbers system does **not** attempt to:

### 1. Protect Users From Loss

- No prevention of overbidding
- No safeguards against economic harm
- No bid caps based on user profile
- No warnings about irreversible outcomes

Loss is a valid and expected outcome.

---

### 2. Prevent Strategic or Malicious Bidding

- No sybil resistance beyond wallet control
- No identity verification
- No intent analysis
- No collusion detection

The system evaluates bids mechanically only.

---

### 3. Guarantee Inscription Success

- No guarantee that an inscription will succeed
- No guarantee against fee spikes
- No guarantee against mempool eviction
- No guarantee against reorg-related ambiguity

Failure to inscribe is a valid outcome.

---

### 4. Guarantee Canonical Visibility

- No guarantee that inscriptions are visible in all wallets
- No guarantee of indexer support
- No guarantee of consistent rendering across environments

Recognition is procedural, not perceptual.

---

### 5. Prevent Duplicate or Lookalike Inscriptions

- The system does not prevent others from inscribing the same number
- The system does not attempt to police or suppress duplicates
- The system does not claim exclusivity over on-chain content

Numbers only recognizes its own procedural outcomes.

---

### 6. Provide Dispute Resolution

- No arbitration
- No appeals
- No operator override
- No corrective action after finalization

Outcomes are final by design.

---

### 7. Provide Privacy Guarantees

- No anonymity guarantees
- No mixing or obfuscation
- No protection against blockchain analysis

Participants are responsible for their own privacy practices.

---

### 8. Protect Against Operator Failure Beyond Defined Safeguards

- No guarantee against operator negligence
- No guarantee against misconfiguration
- No guarantee against hardware failure beyond persistence rules

The system limits damage through constraints, not trust.

---

### 9. Enforce Legal or Regulatory Compliance

- No jurisdiction-specific compliance logic
- No KYC or AML
- No regulatory interpretation

Compliance, if required, is external.

---

### 10. Secure Meaning or Value

- No protection of perceived value
- No defense against narrative framing
- No attempt to stabilize demand or interest

Numbers does not secure meaning.
Meaning is external.

---

## What Security *Is* Applied To

Security measures exist only to ensure:

- auctions do not rewind
- authority is not reused
- ambiguity does not create permission
- persistence prevents forgetting
- illegal transitions halt execution

Anything not required for those guarantees is out of scope.

---

## Final Rule

If a user, operator, or observer expects a protection not listed here:

**That expectation is invalid.**

Numbers is safe by being narrow.
Everything else is someone else’s problem.
