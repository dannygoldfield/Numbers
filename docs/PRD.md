# Product Requirements Document

This document defines normative constraints for Numbers.

## Why This Is Not a Typical PRD

This document does not describe a roadmap, feature set, or user experience.

It defines constraints.

A typical Product Requirements Document exists to guide decisions about what to build next.  
This document exists to prevent accidental changes to what already exists.

Numbers is a system with fixed semantics. Once the sequence begins, outcomes cannot be revised, retried, or reinterpreted. Flexibility is therefore a liability, not a goal.

This PRD focuses on:
- Invariants rather than features
- Constraints rather than opportunities
- Correctness rather than optimization
- Permanence rather than iteration

If a change violates an invariant defined here, the result is no longer Numbers.

This document should be read as a protocol boundary, not a product plan.

---

## 1. Definition

Numbers is a system that auctions integers sequentially and inscribes each outcome onto Bitcoin.

Each auction resolves exactly once.  
Each resolution produces exactly one canonical inscription.  
The sequence advances without retry.

The inscription content is the number.

---

## 2. Invariants

These properties must remain true across all implementations, environments, and versions.

1. Numbers are auctioned strictly in order, starting from 1.
2. Each auction has a fixed duration defined by configuration.
3. Each auction resolves exactly once.
4. Each number produces exactly one canonical inscription.
5. There are no retries and no re-auctions.
6. Settlement does not block progression to the next number.
7. Nonpayment and no-bid outcomes are final and normal.
8. Inscription content is the number only.
9. Rendering is determined entirely by the viewer’s environment.

Violation of any invariant means the system is no longer Numbers.

---

## 3. Scope

Numbers concerns only:

- Auction sequencing
- Resolution and finalization
- Inscription of numbers onto satoshis
- Durable recording of outcomes

The system does not interpret, decorate, or add meaning to numbers.

---

## 4. Explicit Non-Goals

Numbers does not provide:

- Traits, attributes, or rarity mechanics
- Visual styling or customization
- Interpretive or narrative metadata
- Token abstractions or synthetic assets
- Buy-now pricing or secondary market mechanics
- Logic based on number symbolism or perceived value

Any such extensions must live outside the core system.

---

## 5. Operational Stages

Numbers operates in distinct stages.  
Auction semantics do not change between stages.

- **Prototype**  
  Local execution. Simulated settlement. Local persistence.

- **Testnet**  
  Bitcoin Testnet execution. Real inscriptions. Lightweight indexing.

- **Mainnet**  
  Bitcoin Mainnet execution. Production persistence and monitoring.

Differences between stages are environmental, not semantic.

---

## 6. Interface Constraints

Any interface exposing Numbers must:

- Display the current auction number
- Display time remaining in the current auction
- Display resolution and finalization state
- Display inscription results once available

Expressive behavior may apply to interface copy, not to auctioned numbers or inscriptions.

An interface must not:
- Imply ownership before finalization
- Add interpretation or aesthetic meaning
- Emphasize outcomes beyond their factual state

The interface exists to expose state, not to explain it.

---

## 7. Failure Handling Principles

Failures do not alter semantics.

- The sequence continues.
- Outcomes are not retried.
- Finalization occurs exactly once.
- Inscriptions either complete or fail visibly.

Correctness and traceability take priority over performance or convenience.

---

## 8. Authority and References

This document defines invariant behavior.

- Sequence rules are defined in `CORE-SEQUENCE.md`.
- System structure is defined in `ARCHITECTURE.md`.
- Rationale and motivation live in `WHAT-IF.md`.
- Implementation details live in `DEV.md`.

If there is a conflict, invariants defined here take precedence.
