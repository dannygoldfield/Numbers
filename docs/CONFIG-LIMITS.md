# Configuration and Limits — Numbers

This document defines all configurable parameters that affect
economic exposure, timing, and authority in the Numbers system.

It is normative.

Configuration exists to set boundaries,
not to optimize outcomes.

If there is a conflict, PRD.md, INVARIANTS.md, CORE-SEQUENCE.md, STATE-MACHINE.md, and RESTART-RULES.md take precedence.

---

## Principle

Configuration controls **limits**, not behavior.

Changing configuration must never:
- reinterpret past outcomes
- restore authority
- resolve ambiguity
- alter completed auctions

Configuration applies prospectively only.

---

## 1. Configuration Scope

Only the following categories are configurable:

1. Auction timing
2. Settlement timing
3. Economic exposure limits
4. Operational safety margins
5. Observability thresholds

No other parameters may be made configurable.

If a parameter affects authority and is not listed here,
it must be hard-coded or forbidden.

---

## 2. Auction Timing Limits

The following parameters are fixed at system start and must not change while the system is running.

### Auction Duration

- Defines the length of each auction
- Applies uniformly to all auctions

Rules:
- Must be constant across the sequence
- Must not vary by auction number
- Must not be modified while auctions are active

---

### Inter-Auction Gap

- Defines the delay between auctions

Rules:
- Gap must be fixed
- Gap must not be skipped
- Gap must not be extended to influence participation

Pausing the system must not modify gap semantics.

---

## 3. Settlement Limits

### Settlement Deadline

- Defines the maximum time allowed for a winning bidder to settle

Rules:
- Deadline is fixed at auction close
- Deadline must not be extended
- Deadline must not be shortened after resolution
- Expiration is final

Settlement deadlines are not negotiable.

---

## 4. Economic Exposure Limits

### Fee Ceiling

Defines the maximum acceptable fee for an inscription transaction.

Rules:
- Ceiling is defined as an absolute value or deterministic function
- If the ceiling is exceeded:
  - inscription must not proceed
  - no retry with higher fees is permitted
- Fee ceilings must not be adjusted reactively

Exceeding the fee ceiling results in non-inscription, not escalation.

---

### Wallet Safety Margin

Defines the minimum balance that must remain uncommitted.

Rules:
- Funds below the safety margin are never spendable
- Pending inscriptions must reserve funds explicitly
- If insufficient funds exist:
  - new inscriptions must not begin
  - auctions may continue

Running out of funds does not halt the sequence.
It limits authority only.

---

### Concurrent Exposure Limit

Defines the maximum number of simultaneous unsettled obligations.

Rules:
- Applies to pending settlements and inscriptions
- Must be enforced strictly
- Exceeding the limit prevents new authority from being exercised

This limit exists to prevent compounding risk.

---

## 5. Operational Limits

### Retry Limits (Pre-Authority Only)

Retries are permitted only when authority has not yet been exercised.

Rules:
- Applies only to:
  - network errors
  - transaction construction failures
- Retries must be deterministic
- Retry count must be bounded

Once authority is exercised or ambiguity exists,
retry limits are irrelevant.

---

### Observation Thresholds

Defines tolerances for:

- chain lag
- confirmation delay
- indexer staleness

Rules:
- Thresholds trigger degraded status
- Thresholds must not trigger retries
- Thresholds must not alter outcomes

Observation thresholds exist to inform humans, not to steer behavior.

---

## 6. Configuration Change Rules

### Change Timing

Configuration may be changed only when:

- the system is paused
- no auction is in `Open` or `Closed`
- no settlement or inscription authority is active

Configuration changes must never affect in-flight authority.

---

### Change Recording

Every configuration change must be:

- timestamped
- recorded durably
- associated with a reason
- applied prospectively only

Silent configuration changes are forbidden.

---

## 7. Forbidden Configuration Changes

The following are explicitly forbidden:

- Per-auction configuration
- Retroactive changes
- Emergency overrides
- “Just this once” exceptions
- Configuration changes to avoid loss

Loss is an acceptable outcome.
Drift is not.

---

## 8. Failure Behavior

If configuration is invalid, missing, or contradictory:

- execution must halt
- authority must not be exercised
- the operator must intervene

The system must not guess defaults.

---

## Final Rule

Configuration defines **how far the system may go**.

It must never define **what the system decides**.
