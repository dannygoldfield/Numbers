# Prototype 2 experiment and implementation brief

Non-normative, updated 2026-10-18. C73–C74 approve the experiment and revision 0.3 implementation. [STATUS.md](../STATUS.md) and [AUTHORITY.md](../AUTHORITY.md) control behavior. The [approved decision packet](https://chatgpt.com/space/page_37906e64d6708191b3c86647190a42a5) records the review; this brief cannot add rules.

## Learning goal

Can two invited people independently participate in the same auction and explain the relationship between bids, reserved Rana, the fixed winner and final recorded ownership? Two bidders use separate devices and private fixed credentials; one facilitator chooses simulated settlement. All three see shared history and balances.

Use two auctions in one short facilitated session: successful settlement, then failed settlement. Retain the explicit demo inputs, auction timing, accounting and ownership semantics. Observe each tester placing a bid, understanding an outbid/rejection, locating records and explaining both outcomes without the facilitator operating their device. Record help and confusion, including discrepancies between perceived client timing and the authoritative server order. Feedback proposes later changes; it cannot change the running rules.

## Architecture and preservation

Evolve the existing `prototype-01/` Rust application; its directory is retained to avoid a duplicate engine. One HTTPS gateway, one loopback listener, one writer and SQLite. No new database service, queue or distributed coordination. Preserve and verify Prototype 1 source/build/configuration/journals first. Revision 0.3 uses independent fresh histories with an explicit session association. [HANDOFF.md](../../prototype-01/HANDOFF.md) identifies preservation; [VERIFICATION.md](../../prototype-01/VERIFICATION.md) records actual evidence.

## Acceptance and gates

Local checks cover preserved P1, unchanged deterministic fixtures, fixed roles, denials before evaluation, transport limits, simultaneous bids, both settlement outcomes, uncertainty without replay, revision isolation, restart and backup. [REVIEW.md](../../prototype-01/REVIEW.md) maps checks to rules. The [operating examples](../../prototype-01/ops/README.md) are reviewable source, not deployment.

C75 authorizes review, commit, push and an unmerged PR. Merging requires separate authorization. Deployment additionally requires a concrete approved account/Droplet, hostname/TLS, versions, operator, firewall, budget cap, private credential delivery/revocation, backup destination and shutdown/retention plan. Verify that deployed setup and the two-device trial after authorization. Pruning remains separate. No invited tester trial has occurred yet.

## Collect colleague feedback

Use session labels and simulated bidder labels, not personal details.

| Date and session | Task attempted | Observation or exact words | Help given | Possible follow-up |
|---|---|---|---|---|
| — | — | — | — | — |

## Technical debt

[TECHNICAL-DEBT.md](TECHNICAL-DEBT.md) retains request-driven evaluation, trusted host/clock, payload-only hashes and full reconstruction limits. Typed-state and performance refactors require measured need. Scope is proportionate to this invited, simulated experiment.
