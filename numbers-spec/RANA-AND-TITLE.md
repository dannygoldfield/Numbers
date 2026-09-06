# Rana and protocol title

## RT01 — Amounts, issuance, and conservation

One rana is indivisible. All allocations, bids, balances, ledger quantities, and arithmetic must use exact integers. Fractional amounts and floating-point economic arithmetic are forbidden. Zero allocation is permitted by P02; bids and reservation/capture/release amounts must be positive.

Only the two RanaIssueRecord entries in initialization issue Rana. Their recipients and amounts must equal the explicit P01 inputs, in identity order. No later issuance, top-up, transfer, fee, destruction, protocol-balance spending, refund, or compensation event exists.

For identity i, fold the journal's complete groups:

- `available(i) = issued(i) - sum(reservations for i) + sum(releases for i)`.
- `reserved(i) = sum(reservations for i) - sum(releases for i) - sum(captures from i)`.
- `protocol_held = sum(all successful captures)`; its initial value is zero.

Capture reduces reserved Rana; available Rana was already reduced on reservation and must not be debited again. Release ends a named hold and increases available Rana by that hold's exact amount. These equations must agree with the live-hold references.

Every complete group must preserve:

`total_initial_issue = available(demo-1) + reserved(demo-1) + available(demo-2) + reserved(demo-2) + protocol_held`.

All five balances must be nonnegative integers. No record other than the four ledger types defined in SE06 changes a balance.

## RT02 — Leader-only reservation and replacement

The current auction has no reservation in Scheduled. While Open, Closed, or AwaitingSettlement, exactly one live hold must exist: it references the current leading bid, or the fixed winning bid after resolution, and equals that bid's full amount.

For candidate bidder i and amount b, let r be the amount of the current hold if it belongs to i, otherwise zero. The balance admission condition is `b <= available(i) + r`. All other admission conditions in SE03 also apply.

On an accepted bid:

1. Record the valid bid.
2. If a live hold already exists, release that entire hold by its reservation-record reference.
3. Reserve the entire new bid amount for the new bid.
4. Apply the required opening or extension effect in the same commit group.

Validation uses the pre-group state; release/reserve steps are indivisible to observers. For the same leader raising, the net new reservation is only the increase, although explicit release and replacement records preserve the accounting trail. For a competing new leader, the previous leader becomes fully unreserved immediately at that group's commit. A rejected bid has no release or reservation effect.

A release or capture must reference an earlier live reservation exactly once and consume the full hold. A previously released or captured hold cannot be used again. There is no cumulative hold per bid or retained losing hold. Losing-reservation release occurs when outbid; terminal settlement must not issue duplicate releases for already released losers.

## RT03 — Separate local settlement

Resolution fixes the winning bid before settlement is eligible. Only `SubmitSettlement` defined in SE02 can cause settlement. It must name the current resolved auction and request exactly `settled` or `expired` while the auction is AwaitingSettlement. No terminal outcome already exists. The live hold must match the recorded winning bid, bidder, and exact winning amount.

A missing/mismatched hold is invalid history or an invariant failure under PR06, not a permitted failed-settlement result.

- **`settled`:** persist SettlementRecord, RanaCaptureRecord consuming the full winning hold and crediting that amount to `protocol`, and FinalizationRecord assigning winner title, in that order and one atomic group.
- **`expired`:** persist SettlementRecord, RanaReleaseRecord consuming the full winning hold and returning it to that bidder's available Rana, and FinalizationRecord assigning PublicLand, in that order and one atomic group. Protocol-held Rana does not increase.

Both outcomes leave no live reservation for the finalized auction. Both preserve the ResolutionRecord and its winning identity. Settled captures exactly the winning bid amount; there is no price adjustment. No spending operation exists for protocol-held Rana.

`expired` is the retained machine label for simulated failed settlement. Settlement has no deadline or participant-fault precondition. Until a terminal command is accepted, the fixed winner’s hold remains reserved. The backend validates and performs it; the UI only submits the command. No automatic capture, expiry, timeout, failure inference, late-payment recognition, or rerun occurs. AwaitingSettlement can persist indefinitely until an accepted command.

## RT04 — Title and PublicLand

FinalizationRecord is the sole title record; no second title-assignment event exists. Before it exists, protocol title is unassigned. Its `title_kind` and `holder_id` mean:

- `winner`: holder_id equals the identity of the fixed winning BidRecord and settlement is settled.
- `PublicLand`: holder_id is null and settlement is expired.

PublicLand is an irreversible protocol title status. The number remains accounted for in the sequence and outside ordinary participant-held title. PublicLand is not a person, bidder, account, error, government owner, legal land claim, configurable recipient, repair path, or general recovery mechanism. It receives no Rana.

The only PublicLand trigger is the accepted expired settlement group in RT03. Absence of bids, storage errors, malformed history, time alone, restart, or operator belief must not substitute for that trigger. Each number receives exactly one final title; later facts or later specification work cannot reassign it.

## RT05 — One-shot boundaries

Resolution permission is exhausted by the durable ResolutionRecord; terminal-settlement permission by its complete settlement group; title-assignment permission by its FinalizationRecord in that same group. A reservation can end only once by its permitted release or capture. These limits are enforced by canonical references, event preconditions, and record/group uniqueness, not a separate mutable authority flag.

A complete commit makes those facts irreversible. An uncommitted local group has no canonical economic effect. An uncertain commit response is handled by halting and reconstructing durable history under PR06, never by speculative retry, forfeiture, or authority consumption before commit. Restart and operator action cannot restore consumed permission. There is no external-action authority scope or ambiguity-repair lifecycle in Prototype 1.
