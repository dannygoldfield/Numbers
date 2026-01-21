# Architecture — Numbers

This document describes the **logical architecture** of the Numbers system.

It assumes familiarity with SUMMARY.md.

This document is **descriptive**, not normative.

It defines:
- conceptual components
- lifecycle sequencing
- authority boundaries
- relationships between system stages

It does **not** define:
- deployment environments
- execution hosts
- infrastructure choices
- network topology
- operational concerns

Those concerns are defined exclusively in PLATFORM.md.

If there is a conflict, PRD.md and CORE-SEQUENCE.md take precedence.

---

## Role of This Document

This document explains **what exists** in the Numbers system
and **how authority flows** between conceptual stages.

It does not justify purpose or meaning.
Conceptual framing belongs in POETICS.md.

This document exists to prevent misinterpretation of structure,
not to introduce rules.

---

## System Overview

Numbers executes a strictly sequential series of auctions.

Each auction:
- corresponds to exactly one number `N`
- resolves exactly once
- produces an irreversible outcome

The system advances monotonically from one number to the next.

Past outcomes are never:
- revisited
- revised
- reinterpreted
- repaired

Numbers records outcomes.
It does not assign meaning.

---

## Conceptual Decomposition

Numbers consists of four **logical components**:

1. Auction
2. Resolution and Settlement
3. Inscription
4. Catalog

These components are **conceptual stages**, not deployable services.

Each component:
- has a narrow responsibility
- consumes authority exactly once
- does not interpret the output of another component

Together, they form a one-way pipeline.

Authority is consumed as the pipeline advances.
Authority is never restored.

---

## Timing Model

Auction timing guarantees are defined in CORE-SEQUENCE.md
and parameterized by configuration.

This document assumes those guarantees
and does not restate them.

The inter-auction gap provides a strict boundary between auctions.

Auction progression is independent of:
- settlement success
- settlement timing
- inscription broadcast
- inscription confirmation

Downstream activity never blocks the sequence.

---

## 1. Auction

For each number `N`, the system opens exactly one auction.

The auction:
- runs for a fixed duration
- accepts bids until closure
- orders bids strictly by value
- resolves ties deterministically as defined in CORE-SEQUENCE.md

The auction does **not**:
- validate bidder intent
- interpret bidder identity beyond submission validity
- assess meaning, rarity, or significance of the number

The auction’s only output is a closed bidding state.

No authority is exercised beyond bid acceptance and ordering.

---

## 2. Resolution and Settlement

At auction close, resolution occurs exactly once.

Resolution produces a provisional outcome:
- a highest valid bid, or
- no valid bids

Settlement executes asynchronously after resolution
and is bounded by a deadline.

Finalization produces exactly one destination:

- winner settles before deadline → winning address
- winner fails to settle → NullSteward
- no valid bids → NullSteward

The **NullSteward** is a provably unspendable address.

Resolution and settlement:
- do not delay subsequent auctions
- do not pause the sequence
- do not reorder numbers

Finalization consumes remaining auction authority.

---

## 3. Inscription

After auction finalization, an inscription attempt may occur.

The inscription process:
- constructs a Bitcoin transaction
- records the number as witness data
- may result in multiple on-chain lookalikes

An inscription is **recognized by the system** only if:

- it results from the finalized outcome of auction `N`
- its txid and satpoint are recorded by the system

Content alone does not establish provenance.

Recognition is procedural, not visual.

Finalization to the NullSteward follows the same inscription process.
Absence of a winner does not halt inscription attempts.

Inscription success or failure does not alter auction truth.

---

## Canonical Recognition

Numbers distinguishes its own inscriptions
from arbitrary on-chain inscriptions.

Anyone may inscribe the number `N`.
That does not make it part of Numbers.

Canonical recognition answers one question only:

> “Which inscription resulted from auction `N`?”

It does not answer:
- whether ownership is legitimate
- whether value exists
- whether meaning should be assigned

Recognition is mechanical.
Interpretation occurs elsewhere.

---

## 4. Catalog

The NumbersCatalog is a **derived index**.

For each auction number, it records:
- final destination (winning address or NullSteward)
- inscription txid
- inscription satpoint
- associated timestamps

The catalog exists to support:
- observability
- retrieval
- inspection

It does not define ownership, validity, or meaning.

See CATALOG.md for schema and access patterns.

---

### Authority

The catalog is **not authoritative**.

Errors, omissions, or corruption in the catalog do not alter:
- auction outcomes
- settlement destinations
- on-chain inscriptions

Bitcoin is the only durable authority.

The catalog may be corrected, rebuilt, or discarded
without changing system history.

---

### Derivation

All catalog entries are derived from Bitcoin data.

No entry depends on:
- private state
- operator memory
- off-chain metadata
- discretionary interpretation

Each entry is reconstructible
from transactions and inscriptions alone.

---

### Rebuildability

The NumbersCatalog is not a source of truth.
It is a derived view over Bitcoin data.

Given:
- the Bitcoin blockchain
- public knowledge of Numbers auction rules
- the ability to inspect transactions and inscriptions

the catalog can be deleted and rebuilt from scratch.

Rebuilding converges on the same sequence of recognized outcomes.
No private data or discretionary judgment is required.

If knowledge of Numbers itself is lost,
only the underlying Bitcoin data remains.

---

## Non-Goals

The architecture explicitly does **not**:

- prevent duplicate inscriptions
- enforce ownership semantics
- guarantee economic value
- privilege interpretation
- provide dispute resolution

These concerns exist outside the system.

---

## Summary

Numbers advances a sequence.
It records what occurs.
It refuses to explain.

It advances.
It records.
It stops.

Everything else is external.
