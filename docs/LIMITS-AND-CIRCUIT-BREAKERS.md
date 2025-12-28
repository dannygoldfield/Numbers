# Limits and Circuit Breakers

This document defines the limits and circuit breaker framework governing Numbers.

Limits exist to bound risk.
Circuit breakers exist to contain failure.

They are safety mechanisms, not market features.

Auction sequencing, integrity, and timing are prioritized over settlement success, payment timing, and inscription cost efficiency.

---

## Why Limits Exist

Numbers operates in an adversarial and failure-prone environment.

Failures may arise from:
- software bugs
- wallet or node misconfiguration
- automated or malicious behavior
- unexpected fee spikes
- partial outages or degraded dependencies

Limits exist to ensure that no single auction, transaction, or failure mode can create unbounded financial or operational exposure.

The goal is not to prevent failure.
The goal is to **bound the impact of failure**.

Limits in Numbers take two forms: hard caps that bound exposure, and circuit breakers that constrain behavior under adverse conditions.

---

## Auction Caps

Unlike circuit breakers, caps are unconditional and always enforced.

Caps are expected to rise over time.
In normal operation, the cap should rarely, if ever, be reached.
Its purpose is to define a hard upper bound on exposure, not to influence typical bidding behavior.

Each auction enforces a **maximum bid cap**.

The cap bounds the system’s financial exposure per auction.
It exists to prevent a single auction from creating obligations the system cannot safely complete.

This protection applies regardless of intent.
It defends equally against bugs, automation errors, and adversarial behavior.

When the cap is reached:

- The auction resolves immediately
- No further bids are accepted
- The resolution is final and recorded
- Settlement proceeds as normal

Resolution via bid cap is a valid close condition.
It does not reopen or extend the auction.

After resolution:

- Settlement proceeds asynchronously
- An inter-auction pause may occur if configured
- The next auction begins normally

Caps exist to:

- Bound worst-case financial exposure per auction
- Limit the blast radius of faults or abuse
- Preserve system liveness under unexpected conditions
- Ensure failures remain local rather than systemic

---

## Minimum Valid Bid

Each auction enforces a **minimum valid bid**.

The minimum valid bid exists to prevent dust settlements, spam, and operational abuse.
It is a technical hygiene rule, not a valuation signal.

The minimum valid bid:
- is computed at auction start
- remains fixed for the duration of that auction
- may be derived from current network fee conditions with a fixed floor

Bids below the minimum valid bid are rejected.

Failure to meet the minimum valid bid does not alter auction sequencing.
If no valid bids are received, the auction resolves normally.

---

## Initial Limits (Subject to Change)

Initial values are intentionally conservative.

- Max bid per auction: TBD
- Max auctions per day: TBD
- Max concurrent pending settlements: TBD
- Max concurrent pending inscriptions: TBD

These limits apply globally across the running system.

Limits may be increased without interruption.
Limits may only be reduced at an auction boundary to avoid invalidating in-flight work.

---

## Circuit Breaker Philosophy

Circuit breakers protect the system under adverse conditions. Circuit breakers may degrade behavior without entering a paused state.

They are designed to keep auctions as independent as possible from:
- payment success
- settlement timing
- inscription cost or confirmation speed

Default rules:

- Never interrupt an open auction
- Prefer degrading downstream processes over stopping the sequence
- Avoid rewriting outcomes or timelines

Circuit breakers do not correct failures.
They contain them.

---

## Enforcement Points

Circuit breakers may act only at the following points, listed in order of preference:

1. **Inscription policy**
   - defer broadcast
   - use lower fee modes
   - queue work

2. **Settlement handling**
   - allow deadlines to expire naturally
   - finalize to Null Steward when required

3. **Bid intake**
   - temporarily disable new bids
   - expose degraded status in UI

4. **Auction boundary**
   - prevent the start of the next auction

An open auction is never paused, altered, or invalidated.

---

## Candidate Circuit Breakers

The following breakers define the system’s safety envelope.
They describe *what is monitored* and *where protection may be applied*.
They do not hardcode final thresholds.

| Breaker | Protects | Signal | Typical enforcement point | Notes |
|---|---|---|---|---|
| Fee pressure | Cost exposure | Fee rate (sat/vB) | Inscription policy | Auctions continue regardless of fees |
| Node lag | State correctness | headers − blocks, IBD | Bid intake, UI | Avoid bidding on stale state |
| Wallet low | Completion ability | available sats, reserved sats | Inscription policy | Auctions may outpace wallet |
| Error rate | System integrity | errors per unit time | Bid intake, auction boundary | Storage errors fail fast |
| Backlog growth | Operational bounds | pending count, oldest age | Inscription policy | Defines safe operating envelope |

---

## Threshold Design Method

Thresholds are chosen deliberately.
They reflect cost tolerance, risk posture, and operational maturity.

For each breaker:

### 1. Define the worst case

State explicitly:

> “If this goes wrong, the worst plausible outcome is ______.”

Examples:
- unbounded fee spend
- accepting bids on stale chain state
- inscriptions stalled due to insufficient funds
- compounding backlog growth

---

### 2. Choose a threshold class

| Class | Description |
|---|---|
| Absolute | fixed numeric ceiling or floor |
| Relative | percentage over baseline |
| Capacity | queue depth or age |
| Rate | events per unit time |

---

### 3. Add hysteresis

Every breaker must define:

- a **trip condition**
- a **clear condition** that is stricter than the trip

This prevents oscillation and flapping.

---

## Paused State

A paused state may occur only between auctions.

When paused:

- No new auctions begin
- No bids are accepted
- Existing auctions resolve normally
- Settlement and inscription may continue or drain safely
- UI reflects paused status and cause

Paused state does not alter past outcomes.

---

## Manual Controls

- Operator may manually pause the system between auctions
- Manual resume requires confirmation that blocking conditions are cleared
- All manual actions are durably logged with timestamp and reason

Manual controls cannot reopen or alter resolved auctions.

---

## Logging Requirements

All breaker evaluations must record:

- breaker name
- observed signal values
- threshold values
- enforcement state
- timestamp

Pause and resume events are immutable records.

---

## Design Principle

Limits are features.

They protect users.
They protect operators.
They protect the integrity of the sequence.

Safety mechanisms must never rewrite history.

## Non-Goals

Limits and circuit breakers are not intended to:
- shape bidding behavior
- enforce fairness or price discovery
- optimize market outcomes

