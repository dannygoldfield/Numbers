# Launch Checklist — Numbers

This checklist must be completed before Mainnet launch.

Launch is an operational decision, not a calendar event.

---

## 1. Documentation Lock

The following documents are finalized and frozen:

- [ ] READ-THIS-FIRST.md
- [ ] CORE-SEQUENCE.md
- [ ] ARCHITECTURE.md
- [ ] STATE-MACHINE.md
- [ ] STATE-MACHINE-TABLE.md
- [ ] INVARIANTS.md
- [ ] PRD.md
- [ ] UI-SPEC.md
- [ ] THREAT-MODEL.md
- [ ] KEY-MANAGEMENT-POLICY.md
- [ ] LIMITS-AND-CIRCUIT-BREAKERS.md
- [ ] OBSERVABILITY.md
- [ ] OPERATIONAL-RUNBOOK.md
- [ ] CONFIG-REFERENCE.md
- [ ] PERSISTENCE.md

No document listed above may change after launch.

---

## 2. Invariant Verification (Required)

The following invariants have been explicitly verified on Testnet:

- [ ] Auction numbers advance monotonically and exactly once
- [ ] At most one auction is active at any time
- [ ] Each auction resolves exactly once
- [ ] Finalization produces exactly one destination
- [ ] Settlement failure routes to NullSteward
- [ ] No retry or re-auction occurs under any condition
- [ ] Inscription authority is exercised at most once
- [ ] Ambiguity permanently consumes inscription authority
- [ ] Past states are immutable
- [ ] Terminal states cannot be exited
- [ ] Pause does not alter semantics or timing guarantees

Verification must be based on:
- observed behavior
- persisted records
- state reconstruction after restart

Inference or assumption is not sufficient.

---

## 3. Technical Readiness

- [ ] End-to-end Testnet sequence completed across multiple auctions
- [ ] Restart performed at each lifecycle boundary
- [ ] Persistence replay produces identical state
- [ ] Failure and degradation scenarios tested
- [ ] Ambiguous inscription path exercised
- [ ] NullSteward behavior verified
- [ ] Fee spike and high-cost conditions simulated
- [ ] Circuit breakers trip as specified

---

## 4. Operational Readiness

- [ ] Monitoring and alerts active and verified
- [ ] Log retention and inspection confirmed
- [ ] Manual pause and resume controls tested
- [ ] Incident response rehearsed
- [ ] Operator familiar with runbook procedures
- [ ] Operator understands authority loss rules

---

## 5. Launch Decision

- [ ] Auction parameters explicitly reviewed
- [ ] Circuit breakers enabled
- [ ] Keys loaded according to policy
- [ ] Operator available during initial auctions
- [ ] Residual risk explicitly acknowledged and accepted

---

## Final Assertion

Launch occurs only when:

- All invariants hold in practice, not just in theory
- All authority boundaries are respected
- All failure modes are understood
- Operators are prepared to pause immediately

There is no urgency premium.  
Correctness outweighs continuity.
