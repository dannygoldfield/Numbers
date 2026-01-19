# Glossary — Numbers

This glossary defines terms used elsewhere in the project.

Definitions are **descriptive, not binding**.
Normative behavior is defined in specification documents.

---

## Ambiguity

Ambiguity means the system cannot determine whether an authority-bearing action occurred.

Ambiguity is resolved only by deterministic system observation.
Human belief, operator intent, or external knowledge does not resolve ambiguity.

---

## Auction

A fixed-duration process in which bids may be placed for the next number in the sequence.

Each auction resolves exactly once.

---

## Bitcoin

The Bitcoin network and blockchain used as the settlement and recording layer for Numbers.

Bitcoin is the sole durable record of transactions and inscriptions.

---

## Canonical (Numbers)

Recognized by the Numbers system as the outcome for a given number.

Canonical status is procedural.
It does not imply uniqueness, ownership, or meaning outside the system.

---

## Guess-Space

Guess-space is the set of behaviors an implementer would have to invent
due to missing, ambiguous, or underspecified requirements.

Guess-space exists when:
- behavior is implied but not stated
- edge cases are not explicitly resolved
- multiple reasonable interpretations are possible
- defaults are assumed rather than defined

Numbers treats guess-space as a correctness risk.

The specification is written to minimize guess-space by explicitly defining
states, transitions, authority boundaries, and failure outcomes.

If behavior is not specified, it is forbidden.

---

## Inscription

Data embedded in a Bitcoin transaction using the Ordinals protocol.

In Numbers:
- the inscription content is the number only
- inscriptions are recorded as witness data
- multiple inscriptions of the same number may exist on-chain

Only inscriptions produced by the Numbers procedure are system-recognized.

---

## NullSteward

A system-defined destination used when an auction produces no settled winner.

The NullSteward:
- is a provably unspendable address
- is not controlled by any participant
- allows the sequence to advance without retry

NullSteward is a destination, not an outcome.

---

## Number

An integer in the sequential series starting at 1.

Each number is auctioned exactly once.

---

## NumbersCatalog

A derived index of Numbers auction outcomes and their corresponding on-chain references
(txid, satpoint, timestamps).

The Catalog records what happened.
It does not grant authority or define validity.

Bitcoin remains the sole source of truth.

---

## Observation

Observation is the act of deterministically querying an external system of record
(e.g. a Bitcoin node or indexer) to update system knowledge.

Human judgment, interpretation, or intent does not constitute observation
and cannot change system state or restore authority.

---

## Ordinals

A protocol and indexing convention that associates data with specific satoshis.

Ordinals provide legibility and location.
They do not provide consensus, enforcement, or meaning.

---

## Resolution

The act of determining the provisional outcome of an auction at close.

Resolution identifies:
- a winning bidder, or
- no valid bids

Resolution occurs exactly once per auction.

---

## Satoshi

The smallest unit of Bitcoin.

---

## Satpoint

A reference that identifies a specific satoshi within a specific transaction output.

---

## Settlement

The process that determines the final destination for an auction’s inscription.

Settlement finalizes to:
- the winning bidder (if settlement succeeds), or
- the NullSteward (if settlement fails or no bids exist)

Settlement does not block progression of the sequence.

---

## System-Recognized

Designated by the Numbers system as the authoritative outcome for a given auction number.

There is exactly one system-recognized outcome per number.

Recognition is procedural, not semantic.

---

## Viewer-Determined Rendering

A property of Numbers in which presentation is defined by the viewing environment,
not by the system.

Numbers specify content only, not presentation.
