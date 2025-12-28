# Roadmap

This document describes the intentional sequencing of the Numbers project.

This is not a feature wishlist.
It is a risk-reduction plan.

Progress is measured by reduced uncertainty.

---

## Phase 0: Specification and Alignment (Complete)

**Focus**
- Core system behavior defined
- UI behavior specified
- Security and operational constraints documented
- System boundaries clarified

**Exit Conditions**
- Specifications are internally consistent
- All known open questions are explicit and documented
- No undefined behavior remains in core flows

Phase 0 is complete only when ambiguity is intentional, not accidental.

---

## Phase 1: Testnet Production System

**Focus**
- Full auction lifecycle running on Bitcoin Testnet
- Caps and circuit breakers active
- Pause and resume exercised in practice
- Observability and logging in place

**Exit Conditions**
- Repeated Testnet auctions complete without manual intervention
- Pause and resume work as designed
- Failure scenarios are understood, bounded, and survivable
- No unresolved correctness questions remain

If Testnet behavior is surprising, this phase is not complete.

---

## Phase 2: Mainnet Pilot (Production with Constraints)

**Focus**
- Deployment on Bitcoin Mainnet
- Strict caps enforced
- Limited auctions per day
- Continuous manual oversight

**Exit Conditions**
- Multiple Mainnet auctions complete successfully
- No unresolved incidents or unexplained behavior
- Operational procedures are exercised and reliable
- Operator confidence is based on experience, not optimism

Mainnet Pilot is still an experimental phase.
Safety takes precedence over throughput.

---

## Phase 3: Expansion by Adjustment

**Focus**
- Gradual cap increases
- Increased auction frequency
- Improved tooling and automation
- Reduced manual overhead where justified

Expansion occurs only through adjustment of existing parameters.
No new core mechanisms are introduced casually.

**Preconditions for Adjustment**
- Review of incidents since last change
- Review of operational load and failure modes
- Reaffirmation of threat assumptions
- Explicit acceptance of increased exposure

---

## Explicit Exclusions

This roadmap does not promise:
- growth targets
- revenue targets
- user adoption targets
- timelines

Those outcomes emerge from system behavior, not planning.

---

## Design Principle

Progress is defined by reduced uncertainty,
not increased scope.
