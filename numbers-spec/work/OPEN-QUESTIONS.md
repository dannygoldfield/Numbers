# Questions and decision record

Non-normative working record. All four Phase 1 economic questions are resolved by explicit user decisions. No blocking semantic question remains after the Phase 2 audit. This does not activate the candidate or authorize implementation.

## Q01 — Denomination — resolved

The original prompt named Rana but did not define its quantum; the source defined integer sats only. The user accepted the recommendation for indivisible rana and exact integer accounting (C43). Fractional Rana are excluded in Prototype 1. This is restated in RT01 and P01; no sats conversion or production denomination is implied.

## Q02 — Reservation policy — resolved

The user explicitly chose “Hold only the leader's bid” (C45), after the comparison of leader-only and per-identity holds. Only the leader's full amount remains reserved. Outbid holds are released immediately; a same-identity raise replaces its hold using available Rana plus the replaced amount. RT02/SE05 specify exact effects and order.

For the discussed example, both identities begin with 100 rana: A bids 60, B bids 70. The selected model leaves A available/reserved 100/0 and B 30/70. A can fund a subsequent 80 bid, subject to other admission rules. The unselected per-identity model would leave A40/60 but would also fund the 80 by replacement; the comparison did not claim a rebidding-capacity difference between those two models. The selected model minimizes outstanding holds.

No cumulative-per-bid hold, losing-bid promotion, or losing-bid charge is authorized.

## Q03 — Successful capture — resolved

The user explicitly chose capture of exactly the winning bid amount into a protocol-held balance, with no spending operation in Prototype 1 (C44). RT01/RT03 specify the matching reservation debit and protocol credit. The protocol balance is not retired Rana, PublicLand, or a third demo identity.

No redistribution, fee, transfer, later spending, or minting permission follows from the choice.

## Q04 — Failed-settlement reservation — resolved

After reviewing release, capture, and permanent lock, the user agreed with the recommendation to release the uncaptured winning reservation when settlement assigns PublicLand (C46). RT03 defines that original settlement effect; RT04 fixes PublicLand title.

For the discussed 60-rana hold out of 100 rana, the selected result is available 100, reserved 0, protocol credit 0, and PublicLand title. The unselected forfeiture result would have been available 40, reserved 0, protocol credit 60; permanent lock would have been available 40, reserved 60, protocol credit 0. Neither unselected outcome is implemented by this candidate.

## Refund and irreversibility clarification

The user's initial agreement with all four recommendation entries resolved only Q01 because Q02–Q04 initially stated no recommendation. The later explicit answers above resolve the remaining questions. No later recommendation was retroactively treated as already approved.

Release appends the end of an uncaptured reservation. It does not erase the bid or hold, refund completed capture, alter the resolved winner, or reverse PublicLand title. Discretionary refund/compensation capability is deferred scope debt under F01. Irreversibility is a protocol requirement, not temporary debt. Ordinary reservation/settlement semantics are now explicit and are not deferred into code.

## Retained boundaries

- 225 seconds is base duration; the captured extension window/increment/maximum remain explicit. Maximum zero disables extensions only when explicitly configured.
- Local settled/expired commands remain the terminal settlement triggers. There is no automatic local deadline expiry, including after restart. The inert source deadline remains recorded.
- PublicLand follows only the accepted expired settlement group. No-bid dormancy, storage failure, missing holds, malformed history, restart, and time alone are not additional triggers.
- No extra mint, top-up, transfer, compensation, title reassignment, or retry command exists.
- All four decisions affect the inactive candidate only. codex-spec remains solely active until a separate activation decision.

## Non-blocking implementation choices

1. Final local port/path and UI typography/poll interval: choose explicitly when implementing. They do not alter the authoritative evaluator or its recorded timestamps and predicates.
2. Runtime/framework and SQLite transaction settings: choose and verify against PR03 durability before a demo claims to pass. No runtime has been selected or implemented by this specification task.

There are no unresolved economic, title, authority, settlement, ordering, or reconstruction alternatives in the normative candidate.


## Approved revision 0.1 — 2026-09-05

C47 / S24: The user approved removal of the unused settlement deadline after reviewing the two-command settlement proposal. Remove settlement_deadline_seconds, settlement_window_seconds, settlement_deadline, their derivation and UI/example references. Settled captures the full winning hold into protocol-held Rana and assigns winner title; expired releases that hold and assigns PublicLand. AwaitingSettlement retains the hold until an explicit accepted command. Auction timing and economic predicates are unchanged. TD10 is retired before implementation; M12 is superseded only for the deadline. Owners: P02, RT03, SE03/SE04/SE06, PR05, UI01/UI02/UI04.

C48 / S25: In response to the combined simplify/activate/build request, the user said: “Yes I approve the the settlement simplification. With that I approve implementation of Prototype 1.” This activates numbers-spec revision 0.1 as the sole active authority. codex-spec is historical and untouched. Owners: STATUS, AU01. Earlier draft-only statements in this working record describe their historical checkpoint. Local implementation is authorized; no commit, push or publication was requested.

C49: Rana remains useful for this prototype, but need not remain a public concept or permanent internal standard for future payment integrations. Prototype 1 accounting is unchanged.
