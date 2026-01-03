# Glossary — Numbers

This glossary defines terms used elsewhere in the project.  
Definitions are descriptive, not binding.

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
It is not a claim about uniqueness, authority, or meaning on Bitcoin.

---

## Inscription
Data embedded in a Bitcoin transaction using the Ordinals protocol.

In Numbers:
- the inscription content is the number only
- inscriptions are recorded as witness data
- multiple inscriptions of the same number may exist on-chain

Only inscriptions produced by the Numbers system are treated as system-recognized outcomes.

---

## NullSteward
A system-defined destination used when an auction produces no settled winner.

The NullSteward:
- is a provably unspendable address
- is not controlled by any participant
- ensures the sequence advances without retry

NullSteward is a destination, not an outcome.

---

## Number
An integer in the sequential series starting at 1.

Each number is auctioned exactly once.

---

## NumbersCatalog
A derived index of Numbers auction outcomes and their corresponding on-chain inscription references (txid, satpoint, timestamps).

The Catalog records what happened.
It does not define ownership, meaning, or validity.

Bitcoin remains the sole source of truth.

---

## Ordinals
A protocol and indexing convention that associates data with specific satoshis.

Ordinals provide legibility and location.
They do not provide consensus, enforcement, or meaning.

---

## Satoshi
The smallest unit of Bitcoin.

---

## Satpoint
A reference that identifies a specific satoshi within a specific transaction output.

---

## Resolution
The act of determining the provisional outcome of an auction at close.

Resolution identifies:
- a winning bidder, or
- no valid bids

Resolution occurs exactly once per auction.

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
A property of Numbers in which the appearance of a number is defined by the viewing environment, not by the system.

Numbers specify content only, not presentation.
