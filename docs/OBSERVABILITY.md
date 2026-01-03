# Observability — Numbers

This document defines how the health and correctness of the Numbers system is observed.

Observability exists to detect risk early,
before it becomes irreversible.

---

## Observability Goals

At all times, the operator must be able to determine:

- whether the auction sequence is progressing correctly
- whether the system is operating on accurate chain state
- whether financial exposure remains within defined bounds
- whether failures are isolated or compounding

If this cannot be determined, the system must be treated as degraded.

---

## Required Signals

The following signals must be continuously observable.

### Chain and Network

- Bitcoin node sync status
- Chain tip height and lag
- Mempool fee levels

### Auction State

- Current auction number
- Auction open or closed state
- Time remaining in the active auction
- Paused or degraded status

### Settlement and Inscription

- Pending settlements count
- Pending inscriptions count
- Age of oldest pending item
- Finalization outcomes

### Wallet and Funds

- Available wallet balance
- Funds reserved for pending inscriptions
- Safety margin relative to configured thresholds

### System Health

- Error counts by category
- Repeated or escalating failures
- Unexpected or invalid state transitions

---

## Alerts

Alerts indicate conditions requiring human attention.

Alerts trigger when:

- Fee ceilings or cost thresholds are exceeded
- The Bitcoin node falls behind the chain tip beyond tolerance
- Wallet balance drops below the safety threshold
- Error rates exceed defined limits
- State transitions violate the expected sequence

Alerts do not take corrective action automatically.

---

## Operator Response

On alert:

- Investigate the underlying condition
- Determine whether the system remains safe to continue
- Pause auctions at the next auction boundary if uncertainty persists

Automatic recovery must not mask underlying faults.

---

## Visibility and Records

The system must provide durable visibility into:

- Logs with sufficient context to reconstruct events
- Timestamps for all state transitions
- Pause and resume events, including cause
- Configuration and limit context active at the time of events

Observability data must be retained long enough to support post-incident analysis.

---

## Design Principle

Silence is suspicious.  
Noise is worse.

If the system cannot explain itself clearly,
it should not continue unattended.
