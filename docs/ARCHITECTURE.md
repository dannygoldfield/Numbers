# Architecture — Numbers

This document assumes familiarity with SUMMARY.md.

This document is **descriptive**, not normative.

It describes the system structure that emerges from the Numbers specification.
It documents structure, sequencing, authority boundaries, and lifecycle.

It does not justify purpose or meaning.
See POETICS.md for conceptual framing.

If there is a conflict, PRD.md and CORE-SEQUENCE.md take precedence.

---

## System Overview

Numbers executes a sequential series of auctions.
Each auction resolves exactly once.

The system advances monotonically from one number to the next.
Past outcomes are never revisited, revised, or reinterpreted.

Numbers records outcomes.
It does not assign meaning.

---

## Component Decomposition

Numbers consists of four conceptual components:

1. Auction
2. Resolution and Settlement
3. Inscription
4. Catalog

Each component has a narrow responsibility.
No component interprets the output of another.

The components together define a pipeline.
Authority is consumed as the pipeline advances.

---

## Timing Model

Auction timing parameters are defined in CORE-SEQUENCE.md and configuration.
This document assumes those guarantees and does not restate them.

The inter-auction gap provides a strict boundary between auctions.

Auction progression is independent of:
- settlement success
- settlement timing
- inscription broadcast or confirmation

Downstream activity never blocks the sequence.

---

## 1. Auction

For each number **N**, the system opens an auction.

- The auction runs for a fixed duration.
- Bids may be submitted until the auction closes.
- Bids are ordered strictly by value.
- Ties are resolved deterministically as defined in CORE-SEQUENCE.md.

The auction does not:
- validate bidder intent
- interpret bidder identity beyond submission validity
- assess meaning, rarity, or significance of the number

The auction’s only output is a closed bidding state.

---

## 2. Resolution and Settlement

At auction close, resolution occurs exactly once.

Resolution produces a provisional outcome:
- a highest valid bid, or
- no valid bids

Settlement executes asynchronously after resolution and has a deadline.

Finalization produces exactly one destination:
- winner settles before deadline → winning address
- winner fails to settle → NullSteward
- no valid bids → NullSteward

The **NullSteward** is a provably unspendable address.

Resolution and settlement do not delay, pause, or reorder subsequent auctions.

---

## 3. Inscription

After finalization, an inscription transaction is constructed and broadcast.

- Inscription content is the number only.
- The number is recorded as witness data in a Bitcoin transaction.
- Multiple inscriptions of the same number may exist on-chain.

An inscription is **recognized by the system** only when:
- it results from the finalized outcome of auction **N**
- its txid and satpoint are recorded by the system

Content alone does not establish provenance.
Recognition is procedural, not visual.

Finalization to the NullSteward follows the same inscription process.
Absence of a winner does not halt inscription attempts.

---

## Canonical Recognition

Numbers distinguishes its own inscriptions from arbitrary on-chain lookalikes.

Anyone may inscribe the number **N**.
That does not make it part of Numbers.

Recognition answers one question only:

> “Which inscription resulted from auction **N**?”

It does not answer:
- whether ownership is legitimate
- whether value exists
- whether meaning should be assigned

Recognition is procedural.
Interpretation occurs elsewhere.

---

## 4. Catalog

The NumbersCatalog is a derived index.

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

The catalog is not authoritative.

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

The architecture explicitly does not:

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
