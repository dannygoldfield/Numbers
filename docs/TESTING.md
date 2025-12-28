# Testing

This document defines how the Numbers system is tested and what “tested enough” means.

Testing exists to reduce risk, not to prove perfection.

---

## Testing Layers

Numbers is tested across four layers.
Each layer serves a distinct purpose.
No layer substitutes for another.

---

### 1. Unit Testing

**Scope**
- Pure, deterministic logic

**Examples**
- Auction state transitions
- Cap enforcement
- Timer behavior
- State serialization and recovery

**Goal**
- Prevent regressions in core logic
- Ensure repeatability and determinism

Unit tests must not interact with:
- Bitcoin Core
- Wallets
- Networks
- External services

Failure at this layer blocks further testing.

---

### 2. Integration Testing

**Scope**
- Interaction between system components

**Examples**
- Backend ↔ Bitcoin Core RPC
- Backend ↔ database
- Backend ↔ wallet operations
- Error propagation across boundaries

**Goal**
- Validate assumptions at component boundaries
- Detect interface drift and dependency breakage

Integration tests may use:
- Regtest
- Testnet
- Mocked services where isolation is required

Mocks must not conceal real-world failure modes.

---

### 3. End-to-End Testing (Testnet)

**Scope**
- Full system execution on a live network

**Includes**
- Auction start and close
- Bid submission
- Cap reached scenarios
- Settlement success and failure
- Inscription construction and broadcast
- Durable recording of outcomes

**Goal**
- Demonstrate correct behavior on a real chain
- Observe timing, latency, and cost under real conditions

This layer is the **minimum requirement** before any Mainnet deployment.

---

### 4. Failure and Degradation Testing

**Scope**
- Non-ideal and adversarial conditions

**Examples**
- High or volatile fee environments
- Delayed confirmations
- Settlement failures or timeouts
- Inscription failures
- Node restarts during operation
- Partial outages and degraded dependencies

**Goal**
- Verify safe failure
- Verify containment of damage
- Verify pause and recovery behavior

The system must fail visibly and safely.
Silent failure is unacceptable.

---

## Explicit Non-Goals

The following are intentionally not guaranteed:

- Continuous availability
- Instant settlement
- Zero pauses
- Maximum throughput

Correctness, safety, and recoverability take precedence.

---

## Testing Gates for Mainnet

Mainnet deployment requires all of the following:

- Successful end-to-end Testnet runs across multiple auctions
- Manual verification of pause and resume behavior
- Manual verification of cap enforcement
- Manual verification of failure handling paths
- Review of logs for unexplained or ambiguous states

If any test produces ambiguity, the system is not ready.

---

## Design Principle

A system that fails quietly is more dangerous than one that pauses loudly.
