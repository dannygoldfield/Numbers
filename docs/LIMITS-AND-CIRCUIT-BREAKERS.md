# Limits and Circuit Breakers — Numbers

This document defines the limits and circuit breaker framework governing Numbers.

It is **normative**.

Limits exist to bound risk.  
Circuit breakers exist to contain failure.

They are safety mechanisms, not market features.

Auction sequencing, integrity, and timing are prioritized over:
- settlement success
- payment timing
- inscription cost efficiency

If there is a conflict,
PRD.md, CORE-SEQUENCE.md, STATE-MACHINE.md, INVARIANTS.md,
ERROR-TAXONOMY.md, and RESTART-RULES.md take precedence.

---

## Modal Language Rule (Normative)

In this document and all normative specifications:

- **must / must not** define obligations
- **only / exactly once / at most once** define bounds
- **may** is permitted **only** to describe uncertainty of knowledge
  or operator action outside automated execution

The following terms are forbidden in normative contexts:

- possibly
- likely
- eventually
- for now
- TBD

Any normative statement using forbidden modal language is invalid.

---

## 1. Purpose

Limits and circuit breakers exist to:

- bound worst-case financial exposure
- prevent cascading failure
- preserve auction correctness under stress
- ensure failures remain local, visible, and non-authoritative

They do not exist to optimize outcomes.
They exist to prevent the system from lying.

---

## 2. Why Limits Exist

Numbers operates in an adversarial and failure-prone environment.

Failures arise from:
- software defects
- wallet or node misconfiguration
- automated or malicious behavior
- fee volatility
- partial outages or degraded dependencies

Limits ensure that no single auction, transaction, or failure mode
can create unbounded financial, operational, or authority exposure.

The goal is not to prevent failure.  
The goal is to **bound the impact of failure**.

---

## 3. Limit Categories

Limits in Numbers take exactly two forms:

1. **Caps**  
   Hard, unconditional bounds that are always enforced.

2. **Circuit Breakers**  
   Conditional constraints that activate under adverse conditions.

Limits constrain behavior.  
They do not reinterpret outcomes, recreate authority, or infer certainty.

---

## 4. Auction Caps (Normative)

Auction caps are unconditional and always enforced.

Each auction enforces a **maximum bid cap**.

### Properties

- The cap bounds maximum financial exposure per auction
- The cap applies regardless of bidder intent
- The cap applies equally to bugs, automation errors, and adversarial behavior
- The cap is evaluated in real time during bidding

### Cap Reach Behavior

When the cap is reached:

- The auction **must** resolve immediately
- No further bids **must** be accepted
- The resolution **must** be recorded exactly once
- Settlement **must** proceed as defined by the state machine

Resolution via bid cap:
- is a valid close condition
- does not reopen or extend the auction
- does not alter sequencing

After resolution:

- settlement proceeds asynchronously
- inter-auction timing proceeds normally
- the next auction begins according to configuration

Caps exist to:
- bound worst-case exposure
- limit blast radius
- preserve global liveness
- prevent systemic failure

---

## 5. Minimum Valid Bid (Normative)

Each auction enforces a **minimum valid bid**.

The minimum valid bid exists to:
- prevent dust settlements
- prevent spam
- bound operational cost

It is a hygiene rule, not a valuation signal.

### Rules

The minimum valid bid:

- **must** be computed at auction start
- **must** remain fixed for the duration of that auction
- **must** be persisted as part of the auction record
- **may** be derived from observed fee conditions with a fixed floor

Bids below the minimum valid bid:

- **must** be rejected
- **must not** alter auction state
- **must not** affect timing or resolution

If no valid bids are received:
- the auction resolves normally
- the destination finalizes to `NullSteward`

---

## 6. Global Operational Limits (Normative)

The system enforces global limits to bound concurrent exposure.

The following limits **must** exist and be validated by configuration:

- maximum bid per auction
- maximum auctions per unit time
- maximum concurrent pending settlements
- maximum concurrent pending inscriptions

### Rules

- all limits apply globally
- limits **must** be explicit and finite
- limits **must not** be inferred
- exceeding a limit **must** trigger a defined enforcement action

Limit changes:

- increases **may** occur without interruption
- reductions **must** occur only at auction boundaries
- reductions **must not** invalidate in-flight authority

---

## 7. Circuit Breaker Philosophy

Circuit breakers protect the system under adverse conditions.

They do not correct failures.  
They **contain** failures.

Circuit breakers are designed to keep auctions independent from:
- settlement success
- payment timing
- inscription confirmation speed
- fee volatility

### Global Rules

- an open auction **must never** be interrupted
- authority **must never** be retried due to breaker activation
- breakers **must not** rewrite outcomes or timelines
- breakers **must not** infer certainty
- breaker activation **must not** downgrade error classification

---

## 8. Enforcement Points (Normative)

Circuit breakers **may act only** at the following points,
listed in order of preference:

1. **Inscription policy**
   - defer broadcast
   - queue work
   - reduce fee selection aggressiveness

2. **Settlement handling**
   - allow deadlines to expire naturally
   - finalize to `NullSteward` when required

3. **Bid intake**
   - temporarily reject new bids
   - expose degraded status externally

4. **Auction boundary**
   - prevent the start of the next auction

An open auction:
- must not be paused
- must not be altered
- must not be invalidated

---

## 9. Defined Circuit Breakers

The following breakers define the required safety envelope.

They specify:
- what is observed
- what is protected
- where enforcement may occur

They do not define thresholds.

| Breaker | Protects | Observed Signal | Enforcement Point |
|---|---|---|---|
| Fee pressure | Cost exposure | observed fee rate | Inscription policy |
| Node lag | State correctness | headers vs blocks, IBD | Bid intake |
| Wallet depletion | Completion ability | available vs reserved funds | Inscription policy |
| Error rate | System integrity | error count per interval | Bid intake, auction boundary |
| Backlog growth | Operational bounds | queue depth and age | Inscription policy |

---

## 10. Threshold Design Method (Normative)

Every circuit breaker **must** define:

### 1. Worst-Case Statement

An explicit statement of worst plausible outcome.

Examples:
- unbounded fee spend
- accepting bids on stale chain state
- compounding backlog growth

### 2. Threshold Class

Exactly one of the following:

| Class | Description |
|---|---|
| Absolute | fixed numeric bound |
| Relative | percentage over baseline |
| Capacity | queue depth or age |
| Rate | events per unit time |

### 3. Hysteresis

Each breaker **must** define:

- a trip condition
- a clear condition that is stricter than the trip

This prevents oscillation and flapping.

---

## 11. Paused State (Normative)

A paused state is distinct from inter-auction delay.

Rules:

- pause **may** occur only between auctions
- no new auctions begin while paused
- no bids are accepted while paused
- existing auctions resolve normally
- settlement and inscription continue or drain safely

Pause:
- does not alter outcomes
- does not grant authority
- does not infer certainty
- does not permit retries

---

## 12. Manual Controls

- operator **may** pause the system only at auction boundaries
- resume **requires** confirmation that blocking conditions are cleared
- all manual actions **must** be durably logged

Manual controls:
- must not reopen auctions
- must not alter resolution
- must not alter finalization
- must not bypass circuit breakers

---

## 13. Logging Requirements

All breaker evaluations **must** record:

- breaker identifier
- observed signal values
- active threshold values
- enforcement action
- timestamp

Pause and resume events are immutable records.

Logs are append-only and non-authoritative.

---

## 14. Design Principle

Limits are features.

They protect:
- users
- operators
- the integrity of the sequence

Safety mechanisms must never rewrite history.

---

## 15. Non-Goals

Limits and circuit breakers do not:

- shape bidding behavior
- enforce fairness
- optimize market outcomes
- recover lost authority
- resolve ambiguity
