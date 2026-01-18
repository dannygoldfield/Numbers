# Threat Model — Numbers

This document defines the security boundaries,
assumptions, and accepted risks of the Numbers system.

It is **normative** where it constrains system behavior.

Numbers prioritizes correctness, containment, and history integrity
over availability, throughput, or convenience.

If a choice must be made,
the system **must fail safe**.

If there is a conflict,
INVARIANTS.md, ERROR-TAXONOMY.md, FAILURE-MODES.md,
LIMITS-AND-CIRCUIT-BREAKERS.md, PERSISTENCE.md,
and RESTART-RULES.md take precedence.

---

## Modal Language Rule (Normative)

In this document and all normative specifications:

- **must / must not** define obligations
- **only / exactly once / at most once** define bounds
- **may** is permitted **only** to describe uncertainty of knowledge,
  never permission, intent, or mitigation choice

The following terms are forbidden in normative contexts:

- possibly
- likely
- eventually
- for now
- TBD

Any normative statement using forbidden modal language is invalid.

---

## 1. Scope and Trust Boundaries (Normative)

Numbers separates responsibility into two non-overlapping layers:

1. **Process Layer (Off-chain)**
2. **Outcome Layer (On-chain, Bitcoin)**

These layers have distinct trust assumptions,
distinct authority boundaries,
and distinct failure semantics.

No mitigation, response, or operational action
**may** blur this boundary.

---

## 2. Protected Assets (Normative)

Numbers protects the following assets:

- User funds submitted as bids
- System-controlled funds used for transaction fees
- Authority-bearing private keys
- Auction order, timing, and resolution integrity
- One-to-one association between auction numbers and recognized inscriptions
- Public verifiability of recorded outcomes

Loss of availability is acceptable.  
Loss of history integrity is not.

---

## 3. Outcome Layer Threat Model (Bitcoin)

Once an inscription transaction is confirmed on Bitcoin:

- Transaction existence is enforced by Bitcoin consensus
- Inscription data is immutable
- Outputs cannot be reassigned or rewritten
- No operator, bidder, or system action can alter the outcome

This layer inherits:

- Bitcoin’s consensus security
- Bitcoin’s fee market behavior
- Bitcoin’s confirmation uncertainty
- Bitcoin’s irreversibility

Subsequent transfers of an inscription
do not affect Numbers’ recorded outcome.

Numbers **must not** attempt to correct,
override, or reinterpret Bitcoin outcomes.

---

## 4. Process Layer Threat Model (Off-chain)

The process layer coordinates:

- auction timing
- bid intake
- resolution determination
- settlement coordination
- inscription initiation

Properties:

- execution is procedural, not trustless
- operator participation is required
- outages and delays are expected
- safety is enforced by refusal, not recovery

Failure in the process layer:

- **may** prevent participation
- **must not** alter confirmed outcomes
- **must not** recreate authority
- **must not** rewrite history

---

## 5. Threat Classes and Bounded Responses (Normative)

For each threat class, only the responses listed as **Allowed**
are permitted.

No response may violate invariants,
retry consumed authority,
or invent certainty.

---

### 5.1 Fee Market Pressure

**Threat**  
Inscription fees exceed configured tolerance.

**Allowed Responses**
- enforce fee ceilings
- defer inscription broadcast
- activate circuit breakers
- pause auctions at auction boundaries

**Forbidden Responses**
- retry broadcast after ambiguity
- lower confirmation requirements
- reinterpret finalized outcomes

---

### 5.2 Settlement Failure

**Threat**  
Winning bidder does not complete settlement.

**Allowed Responses**
- enforce the settlement deadline
- finalize destination to `NullSteward`
- advance the sequence

**Forbidden Responses**
- extend deadlines
- reopen auctions
- retry settlement after failure

---

### 5.3 Wallet Compromise

**Threat**  
Authority-bearing keys are exposed or stolen.

**Allowed Responses**
- pause at the next auction boundary
- sweep remaining funds
- rotate keys
- record the incident durably

**Forbidden Responses**
- retry past actions
- reclaim spent outputs
- reinterpret ambiguous outcomes

---

### 5.4 Double-Spend or RBF Abuse

**Threat**  
Bid payment is invalidated or replaced.

**Allowed Responses**
- require confirmation thresholds
- refuse optimistic settlement
- classify ambiguity when outcome cannot be proven

**Forbidden Responses**
- assume confirmation
- infer bidder intent
- retry settlement based on belief

---

### 5.5 Denial of Service

**Threat**  
Flooding bids, requests, or malformed inputs.

**Allowed Responses**
- rate limiting
- bid rejection
- circuit breaker activation
- boundary-level pause

**Forbidden Responses**
- dropping persisted state
- skipping auctions
- altering timing guarantees

---

### 5.6 Data Corruption or Partial Persistence

**Threat**  
Incomplete, missing, or contradictory persisted records.

**Allowed Responses**
- halt execution
- classify as Fatal or Ambiguous
- require operator inspection

**Forbidden Responses**
- recomputation
- inference from absence
- repair by assumption

---

## 6. Explicitly Accepted Risks (Normative)

Numbers explicitly accepts the following risks:

- temporary unavailability
- operator error causing pauses
- missed participation due to outages
- fee volatility delaying inscription
- permanent loss of funds routed to `NullSteward`
- permanent ambiguity where proof cannot be established

These risks are **intentional, visible, and non-recoverable**.

---

## 7. Explicit Non-Guarantees (Normative)

Numbers does not guarantee:

- trustless auction execution
- uninterrupted availability
- on-chain enforcement of auction rules
- remediation of operator mistakes
- economic value, fairness, or liquidity

Any claim to the contrary is invalid.

---

## 8. Irreversibility Rules (Normative)

The system enforces:

- each auction resolves exactly once
- authority is consumed exactly once
- ambiguity permanently reduces authority
- confirmed on-chain outcomes are final

No threat permits rewind, retry, or reinterpretation.

---

## 9. Participant Risk Acceptance

By participating, bidders accept that:

- the process layer is not trustless
- execution requires an operator
- confirmed inscriptions are permanent
- ambiguity may result in loss
- no appeal or reversal mechanism exists

Participation implies acceptance of these constraints.

---

## 10. Design Intent

Numbers makes its trust boundaries explicit,
records outcomes procedurally,
and refuses to invent certainty where none exists.

Security is achieved through containment,
not through denial of risk.
