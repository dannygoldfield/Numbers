# Operational Runbook

This document defines how Numbers is operated in production.

If something feels wrong, pause first.

---

## Operating Assumptions

- Auctions must never be interrupted once open
- Pauses occur only at auction boundaries
- Settlement and inscription may lag without blocking the sequence
- Uncertainty is treated as risk

The default response to ambiguity is to stop starting new auctions.

---

## Continuous Monitoring

The following signals must be monitored continuously.

### Chain and Network

- Bitcoin node sync status
- Chain tip height and lag
- Mempool fee levels

### Funds

- Wallet balance
- Reserved funds
- Safety margin

### Auctions

- Current auction number
- Auction open or closed state
- Paused or degraded status

### Settlement and Inscription

- Pending settlements count
- Pending inscriptions count
- Age of oldest pending item

### System Health

- Error rates by category
- Repeated or escalating failures
- Unexpected state transitions

---

## Common Incidents and Response

### High Fee Environment

**Condition**
- Fee levels exceed configured cost tolerance

**Response**
1. Pause auctions at the next auction boundary
2. Allow any open auction to resolve
3. Defer inscription broadcasts if applicable
4. Monitor fee normalization
5. Resume auctions manually when acceptable

---

### Settlement Backlog

**Condition**
- Pending settlements exceed safe operating bounds

**Response**
1. Pause auctions at the next auction boundary
2. Allow backlog to clear or finalize naturally
3. Verify wallet balance and inscription capacity
4. Resume auctions manually when stable

---

### Inscription Failure

**Condition**
- Inscription broadcast fails or remains unconfirmed beyond expectation

**Response**
1. Do not retry automatically
2. Record the failure state durably
3. Apply fee bump or alternate broadcast only if policy allows
4. Finalize to Null Steward if required
5. Do not reopen or re-resolve the auction

---

### Node Failure or Desynchronization

**Condition**
- Node unavailable, desynced, or reporting inconsistent state

**Response**
1. Pause auctions at the next auction boundary
2. Restore or restart the node
3. Verify chain tip and sync status
4. Verify wallet state consistency
5. Resume auctions manually after verification

---

## Pause Procedure

1. Initiate pause (manual or automatic)
2. Confirm no auction is currently accepting bids
3. Verify paused status is visible in the UI
4. Record pause reason and timestamp
5. Investigate and resolve the underlying condition

Pausing does not alter past outcomes.

---

## Resume Procedure

1. Verify all monitored signals are within defined limits
2. Confirm no partial or ambiguous state exists
3. Confirm configuration and secrets are correct
4. Resume auctions manually
5. Monitor closely for the first auction cycle

---

## Escalation Rule

If the operator cannot confidently explain system state,
auctions must remain paused.

---

## Design Principle

Operations favors calm, reversibility, and correctness.

There is no urgency premium.
