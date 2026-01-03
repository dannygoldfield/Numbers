# Architecture — Numbers

This document defines how Numbers behaves.

It describes structure, sequencing, authority boundaries, and lifecycle.
It is implementation-oriented and procedural.

It does not justify the premise.
See WHAT-IF.md and POETICS.md for conceptual framing.

---

## System Overview

Numbers runs a sequential series of auctions.
Each auction resolves exactly once.

The system advances monotonically from one number to the next.
It does not pause, rewind, or reinterpret past outcomes.

The system records outcomes.
It does not enforce meaning.

---

## Core Components

Numbers consists of four conceptual components:

1. Auction
2. Resolution and Settlement
3. Inscription
4. Catalog

Each component has a narrow responsibility.
No component interprets the others.

---

## Timing Policy

- **Auction duration:** fixed by configuration  
- **Inter-auction gap:** fixed and non-zero  

The inter-auction gap provides a boundary between auctions.

Auction timing is independent of settlement and inscription.
Settlement and inscription do not block progression.

See CORE-SEQUENCE.md for timing guarantees.

---

## 1. Auction

For each number **N**, the system opens an auction.

- The auction runs for a fixed duration.
- Bids may be submitted until the auction closes.
- Bids are compared strictly by value.
- Ties are resolved deterministically.

The auction does not:
- validate bidder intent
- assess bidder identity beyond transaction validity
- interpret the significance of the number

---

## 2. Resolution and Settlement

At auction close, the auction resolves exactly once.

Resolution produces a provisional outcome:
- a winning bidder, or
- no valid bids

Settlement runs asynchronously and has a deadline.

Finalization produces exactly one destination:
- **Winner settles before deadline → winning address**
- **Winner fails to settle → NullSteward**
- **No valid bids → NullSteward**

The **NullSteward** is a provably unspendable address.

Resolution and settlement do not delay subsequent auctions.

---

## 3. Inscription

After finalization, an inscription transaction is constructed and broadcast.

- Inscription content is the number only.
- The number is recorded as witness data in a Bitcoin transaction.
- Multiple inscriptions of the same number may exist on-chain.

An inscription is **recognized by the system** only if:
- it is produced by the system’s finalized outcome for that number
- its txid and satpoint are recorded in the NumbersCatalog

Content alone does not establish provenance.
Recognition is procedural, not visual.

Finalization to the NullSteward also produces an inscription.
Absence of a winner does not halt inscription.

---

## Canonical Recognition

Numbers distinguishes its own inscriptions
from arbitrary lookalike inscriptions on-chain.

Anyone can inscribe the number **N**.
That does not make it part of Numbers.

Recognition answers only:

> “Which inscription resulted from auction **N**?”

It does not answer:
- whether ownership is real
- whether value exists
- whether meaning should be assigned

Recognition is procedural.
Interpretation happens elsewhere.

---

## 4. Catalog

The NumbersCatalog is a derived index.

It records, for each auction number:
- final destination (winning address or NullSteward)
- inscription txid
- inscription satpoint
- associated timestamps

The catalog exists to support:
- observability
- retrieval
- inspection

It does not define ownership, meaning, or validity.

See CATALOG.md for schema and access patterns.

---

### Authority

The catalog is not authoritative.

Errors or omissions in the catalog do not alter outcomes recorded on-chain.
The blockchain is the only durable record.

The catalog may be corrected, rebuilt, or discarded
without changing system history.

---

### Derivation

All catalog entries are derived from Bitcoin data.

No catalog entry may depend on:
- private state
- operator memory
- off-chain metadata
- discretionary interpretation

Every entry must be reconstructible
from transactions and inscriptions alone.

---

### Rebuildability

The NumbersCatalog is not a source of truth.
It is a derived view over Bitcoin data.

Given:
- the Bitcoin blockchain
- public knowledge of the Numbers auction rules
- the ability to inspect transactions and inscriptions

the catalog can be deleted and rebuilt from scratch.

Rebuilding must converge on the same sequence of recognized outcomes.
No private data, off-chain state, or discretionary judgment is required.

If knowledge of the Numbers system itself is lost,
only the underlying Bitcoin data remains.

---

## Invariants

The following invariants hold at all times:

- Each auction resolves exactly once.
- The sequence advances monotonically.
- Every finalized outcome produces an inscription.
- Recognition depends on system provenance, not content.
- The catalog is derived, not authoritative.

---

## Non-Goals

The architecture explicitly does not:

- prevent duplicate inscriptions
- enforce ownership semantics
- guarantee economic value
- privilege interpretation
- provide dispute resolution

Those concerns exist outside the system.

---

## Summary

Numbers is a system that records outcomes
and refuses to explain them.

It advances.
It records.
It stops.

Everything else is external.
