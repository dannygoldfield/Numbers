# Catalog — Numbers

The system-maintained record of auction outcomes and their corresponding
on-chain inscription references (txid, satpoint, timestamps).

It records what happened.
It does not interpret meaning.

Bitcoin is the sole source of truth.

## Purpose

The Catalog exists to support:
- observability
- retrieval
- inspection

It does not define ownership, meaning, or validity.

## Authority

The Catalog is not authoritative.

Errors or omissions in the Catalog do not alter outcomes recorded on-chain.
The blockchain is the only durable record.

## Derivation

All Catalog entries are derived from Bitcoin transactions and inscriptions.

No Catalog entry may depend on:
- private state
- operator memory
- off-chain metadata
- discretionary interpretation

Every entry must be recoverable from the chain.

## Rebuildability

The Catalog can be deleted and rebuilt entirely from on-chain data.

Rebuilding the Catalog must converge on the same sequence of outcomes.
No private data or discretionary judgment is required.

## Mutability

The Catalog is append-only in normal operation.

Past entries are never edited or reinterpreted.
Corrections occur only by rebuilding from chain data.

## Scope Limits

The Catalog does not:
- confer ownership
- resolve disputes
- assign meaning
- override on-chain data
