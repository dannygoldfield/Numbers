# Threat Model

This document defines the security boundaries, assumptions, and accepted risks of Numbers.

Numbers prioritizes correctness and containment over availability.
If a choice must be made, the system fails safe.

This document defines what Numbers guarantees and what it explicitly does not.

---

## Scope

Numbers separates responsibility into two layers:

1. **Process layer**  
   Auction timing, bid submission, settlement coordination, and inscription initiation.

2. **Outcome layer**  
   The final inscription recorded on the Bitcoin blockchain.

These layers have different trust assumptions and failure modes.

---

## Assets

- User funds submitted as bids
- System-controlled funds for fees
- Private keys for operational wallets
- Auction ordering and resolution integrity
- Correct association between auction numbers and inscriptions
- Public confidence in recorded outcomes

---

## Outcome Layer (Bitcoin)

Once an inscription transaction is created and confirmed on the Bitcoin blockchain:

- Control of outputs is enforced by Bitcoin consensus.
- Records are publicly verifiable and append-only.
- The inscription cannot be altered, revoked, or reassigned without invalidating the chain.
- No operator, bidder, or administrator can retroactively change a confirmed outcome.

This layer inherits Bitcoin’s security properties and failure modes.

Subsequent transfers of the inscription are permitted and do not affect the recorded outcome.

---

## Process Layer (Off-chain)

Auction execution relies on off-chain coordination.

Assumptions at this layer include:

- The auction operator executes the published rules faithfully.
- Auction timing and resolution are enforced by the operator.
- Bid submission and settlement coordination are subject to standard Web-based risks.
- Temporary service interruption or operator failure may affect participation, but not confirmed outcomes.

This layer does **not** provide trustless execution guarantees.

---

## Threat Considerations

The following are common operational risks and mitigations.
They operate within the trust boundaries defined above.

### High Fee Environment
- Risk: Transactions become uneconomical or delayed
- Mitigation:
  - Fee ceilings enforced
  - Automatic auction pause if fee thresholds are exceeded
  - Manual intervention required to resume

### Settlement Failure
- Risk: Winning bidder does not pay
- Mitigation:
  - Settlement window enforced
  - On failure, inscription routed to the null steward
  - Auction sequence advances without rollback

### Wallet Compromise
- Risk: Loss of system-controlled funds
- Mitigation:
  - Minimal hot wallet balances
  - No long-term custody of user funds
  - Null steward outputs intentionally unspendable

### Double Spend or RBF Abuse
- Risk: Bid payment invalidated
- Mitigation:
  - Confirmation threshold required before settlement
  - Conservative mempool policy
  - No optimistic settlement

### Denial of Service
- Risk: Flooding bids or requests
- Mitigation:
  - Rate limiting
  - Auction caps
  - Automatic pause on error thresholds

### Data Corruption
- Risk: Incorrect auction state or history
- Mitigation:
  - Append-only records
  - Immutable sequence guarantees
  - External verification via txid and satpoint

---

## Explicit Non-Guarantees

Numbers does not guarantee:

- Trustless auction execution
- On-chain enforcement of bidding rules
- Automatic remediation for settlement failure
- Protection against all forms of operator error or misconduct
- Continuous availability

These constraints are intentional.

---

## Irreversibility

The system is designed so that:

- Each auction resolves exactly once.
- The sequence does not rewind, retry, or re-auction.
- Outcomes, once recorded on-chain, are final.

Irreversibility applies to outcomes, not to the auction process itself.

---

## Risk Acceptance

By participating, bidders accept that:

- The auction process is not fully autonomous.
- The final inscription, once created, is permanent.
- There is no appeals process for completed outcomes.

---

## Design Intent

Numbers makes its trust boundaries explicit rather than implicit.

The system records outcomes.
It does not attempt to automate trust away.
