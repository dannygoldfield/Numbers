# Threat Model

This document defines the security boundaries, assumptions, and accepted risks of Numbers.

Numbers prioritizes correctness and containment over availability.
If a choice must be made, the system fails safe.

This document defines what Numbers guarantees
and what it explicitly does not.

---

## Scope

Numbers separates responsibility into two layers:

1. **Process layer**  
   Auction timing, bid submission, settlement coordination, and inscription initiation.

2. **Outcome layer**  
   The final inscription recorded on the Bitcoin blockchain.

These layers have distinct trust assumptions
and distinct failure modes.

---

## Assets

The system protects:

- User funds submitted as bids
- System-controlled funds used for fees
- Private keys for operational wallets
- Auction ordering and resolution integrity
- Correct association between numbers and inscriptions
- Public confidence in recorded outcomes

---

## Outcome Layer (Bitcoin)

Once an inscription transaction is confirmed on Bitcoin:

- Output control is enforced by Bitcoin consensus
- Records are public, verifiable, and append-only
- Inscription content cannot be altered or reassigned
- No operator or bidder can retroactively change a confirmed outcome

This layer inherits Bitcoin’s security properties
and Bitcoin’s failure modes.

Subsequent transfers of an inscription
do not affect the recorded outcome.

---

## Process Layer (Off-chain)

Auction execution relies on off-chain coordination.

At this layer:

- The operator enforces published auction rules
- Timing and resolution are enforced procedurally
- Bid submission and settlement are subject to standard Web risks
- Temporary outages may affect participation

This layer does **not** provide trustless execution guarantees.

Failure at this layer may prevent participation,
but cannot alter confirmed outcomes.

---

## Threat Considerations

The following risks operate within the trust boundaries defined above.

### High Fee Environment
- **Risk:** Inscription transactions become uneconomical or delayed
- **Mitigation:**
  - Fee ceilings enforced
  - Automatic auction pause when thresholds are exceeded
  - Manual intervention required to resume

### Settlement Failure
- **Risk:** Winning bidder does not settle
- **Mitigation:**
  - Settlement window enforced
  - On failure, destination defaults to NullSteward
  - Sequence advances without rollback

### Wallet Compromise
- **Risk:** Loss of system-controlled funds
- **Mitigation:**
  - Minimal hot wallet balances
  - No long-term custody of user funds
  - NullSteward outputs intentionally unspendable

### Double Spend or RBF Abuse
- **Risk:** Bid payment invalidated
- **Mitigation:**
  - Confirmation threshold required
  - Conservative mempool policy
  - No optimistic settlement

### Denial of Service
- **Risk:** Flooding bids or requests
- **Mitigation:**
  - Rate limiting
  - Auction caps
  - Automatic pause on error thresholds

### Data Corruption
- **Risk:** Incorrect auction state or history
- **Mitigation:**
  - Append-only records
  - Immutable sequence guarantees
  - External verification via txid and satpoint

---

## Explicit Non-Guarantees

Numbers does not guarantee:

- Trustless auction execution
- On-chain enforcement of bidding rules
- Automatic remediation for operator error
- Immunity from all forms of operator failure
- Continuous availability

These limits are intentional and disclosed.

---

## Irreversibility

The system enforces the following:

- Each auction resolves exactly once
- The sequence never rewinds, retries, or re-auctions
- Confirmed on-chain outcomes are final

Irreversibility applies to outcomes,
not to the off-chain auction process.

---

## Risk Acceptance

By participating, bidders accept that:

- The auction process is not fully autonomous
- Operator execution is required
- Confirmed inscriptions are permanent
- There is no appeals process for completed outcomes

---

## Design Intent

Numbers makes its trust boundaries explicit.

It records outcomes.
It does not attempt to automate trust away.
