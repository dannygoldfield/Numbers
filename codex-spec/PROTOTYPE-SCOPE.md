# Prototype Scope

## Purpose

This document defines the active implementation scope for the current Numbers prototype.

The current implementation target is a deterministic, single-machine browser demo.

The prototype must demonstrate the auction lifecycle, append-only records, restart reconstruction, and frontend-visible state.

The prototype is not required to implement distributed operation, production-scale infrastructure, generalized fault tolerance, or public launch readiness.

---

## Current Prototype Target

The current prototype is:

- single-machine
- local-first
- deterministic
- browser-demonstrable
- append-only
- restart-reconstructible
- AI-assisted
- not production-hosted
- not distributed

The prototype should be demonstrable by one operator on one laptop.

---

## Core Demonstration Requirements

The prototype must demonstrate:

1. The current number in the sequence.
2. Auction opening by first valid bid.
3. Bid admission and deterministic bid rejection.
4. Countdown behavior derived from canonical persisted state.
5. Auction close.
6. Winner resolution.
7. Finalization.
8. NullSteward outcome when applicable.
9. Append-only canonical records.
10. Restart reconstruction from persisted records.
11. Browser-visible current state.
12. Browser-visible auction history.

---

## Prototype Demonstration Stages

The browser prototype is divided into two implementation stages.

### Demo 1: Browser Auction Demo Without Live Ordinals Broadcast

Demo 1 is the current implementation target.

Demo 1 must demonstrate:

1. Current number display.
2. First valid bid opens the auction.
3. Bid admission and deterministic bid rejection.
4. Countdown derived from canonical persisted state.
5. Auction close.
6. Winner resolution.
7. Settlement determination.
8. Finalization.
9. NullSteward outcome when applicable.
10. Append-only canonical records.
11. Restart reconstruction from persisted records.
12. Browser-visible current state.
13. Browser-visible auction history.
14. Inscription intent or inscription-deferred status.

Demo 1 must not require:

- live Ordinals broadcast
- `ord` availability
- Bitcoin Core RPC availability
- wallet availability
- mempool recognition
- confirmation observation
- external SSD availability

Auction correctness must remain demonstrable even when inscription execution is disabled, unavailable, or deferred.

If inscription execution is not active in Demo 1, the system must expose that status explicitly as `deferred_in_this_slice`, `not_broadcast`, or another explicitly specified non-authority-consuming status.

Demo 1 must not silently simulate a successful inscription.

### Demo 2: Browser Auction Demo With Testnet Inscription Adapter

Demo 2 is deferred until Demo 1 works.

Demo 2 may add:

1. Local Bitcoin Core Testnet integration.
2. Local wallet integration.
3. Inscription payload generation.
4. Testnet inscription broadcast attempt.
5. Broadcast outcome classification.
6. Inscription state display.
7. Confirmation observation if available.

Demo 2 may record inscription broadcast outcome as one of:

- `committed`
- `pre_commit_rejected`
- `ambiguous`

Demo 2 must still preserve the rule that Numbers canonical truth is the append-only Numbers record log.

Bitcoin Testnet, Ordinals, wallet state, mempool state, and confirmation state are external execution surfaces. They do not replace Numbers canonical records.

---

## Ordinals and Bitcoin Testnet Scope

The prototype may integrate with Bitcoin Testnet and Ordinals only through a bounded inscription adapter.

Bitcoin Testnet and Ordinals are external execution surfaces.

The canonical truth of Numbers remains the append-only Numbers record log.

The local Bitcoin Core Testnet node may be used as the authoritative chain node for prototype inscription behavior.

The location of Bitcoin Testnet data, including whether it is stored on an external SSD, is operational configuration and is not protocol semantics.

---

## Ordinals Behavior Included in Scope

The prototype may include:

- inscription payload generation
- inscription intent persistence
- interaction with a local Bitcoin Core Testnet node
- interaction with a local wallet
- optional testnet inscription broadcast
- broadcast commit recording
- confirmation observation
- inscription ambiguity recording
- frontend display of inscription state

Live testnet inscription is not required for Demo 1. Live testnet inscription belongs to Demo 2 unless explicitly moved into Demo 1 by a later scope revision.

---

## Ordinals Behavior Excluded From Scope

The prototype is not required to implement:

- Bitcoin mainnet inscription
- production wallet funding
- production custody design
- generalized Ordinals indexing
- distributed inscription workers
- automatic recovery from ambiguous inscription states
- speculative rebroadcast behavior
- production fee management
- public inscription service operation

---

## Inscription Dependency Rule

Auction correctness must not depend on live inscription success.

The auction lifecycle, bid records, winner resolution, finalization, and restart reconstruction must remain demonstrable even if live inscription broadcast is disabled, unavailable, or deferred.

If inscription behavior is unavailable in the current implementation slice, the system must expose that status explicitly rather than silently simulating production behavior.

---

## Authority Boundary

Inscription authority consumption must be defined by the inscription specification.

Prototype scope does not redefine authority consumption.

If the inscription specification declares a broadcast commit boundary, the prototype must not treat earlier operational steps as authority consumption.

---

## Configuration Boundary

Configuration may define local operational parameters such as:

- Bitcoin Core RPC URL
- wallet name
- network selection
- local data paths
- inscription adapter mode
- frontend server port

Configuration must not alter canonical truth, lifecycle rules, authority consumption, or restart reconstruction semantics.

---

## Deferred Work

The following work is deferred until after the browser demo:

- public deployment
- production wallet operations
- mainnet inscription
- multi-rail payments
- credit card payments
- USDC payments
- authentication
- rate limiting
- production monitoring
- contributor onboarding
- community features
- frontend polish beyond basic demo clarity

---

## Implementation Rule

The implementation must remain within this prototype scope unless the scope is explicitly revised.

If required behavior is outside this scope, it must be deferred, rejected, or represented as not implemented in the current slice.

It must not be silently invented.