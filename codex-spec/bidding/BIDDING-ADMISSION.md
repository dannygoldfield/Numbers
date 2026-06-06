# Bidding Admission: Numbers

This document defines bid admission rules for Numbers.

It is normative.

Authority precedence is defined exclusively in `AUTHORITY-ORDER.md`.

Bid admission governs whether a bid submission is evaluated as valid or invalid at authoritative server receipt time.

Bid admission does not define auction resolution, settlement, inscription, finalization, or meaning.

If a behavior is not specified here, it is forbidden.

---

# 1. Purpose

Bid admission exists to:

- evaluate bid submissions deterministically
- record evaluated bid submissions as canonical `BidRecord` entries
- prevent invalid bids from altering auction lifecycle state
- enforce configured economic boundaries
- enforce authorization requirements
- enforce destination address validity
- isolate eligibility logic from downstream phases

Bid admission is a deterministic gate.

It does not negotiate, infer, explain, or repair.

---

# 2. Admission Scope

Bid admission applies only when:

- system control state is `Running`
- auction state is `Scheduled` or `Open`
- a bid submission is received for the current auction number

Bid admission does not apply:

- after auction close
- during settlement
- during inscription
- during restart reconstruction
- when system control state is `Paused`

If system control state is not `Running`, the bid must be rejected explicitly.

If auction state is `Closed`, `AwaitingSettlement`, or `Finalized`, the bid must be rejected explicitly.

---

# 3. Bid Recording Rule

Every bid submission attempt that reaches admission evaluation must produce exactly one `BidRecord`.

A valid bid must produce:

```text
BidRecord.validity = valid
```

An invalid bid must produce:

```text
BidRecord.validity = invalid
```

Invalid `BidRecord` entries:

- are append-only audit truth
- must include a non-null `rejection_reason`
- must not open an auction
- must not trigger an extension
- must not participate in winner resolution
- must not alter lifecycle state

If persistence is unavailable before admission evaluation begins, the request must fail without admission evaluation.

If admission evaluation begins, the admission result must be persisted as exactly one `BidRecord`.

Silent rejection is forbidden.

---

# 4. Required Bid Fields

A bid submission must include:

- `auction_number`
- `bidder_address`
- `amount_sats`
- `destination_address`
- `nonce`
- `signature`

No additional request field may alter bid semantics.

---

# 5. Admission Conditions

A bid is valid if and only if all of the following are true:

- system control state is `Running`
- auction state is `Scheduled` or `Open`
- the bid targets the current auction number
- all required fields are present
- all required fields are well-formed
- the bid signature is valid
- the request proves control of the bidding wallet
- the bid amount satisfies the applicable minimum bid rule
- the bid amount satisfies the applicable increment rule
- the bid amount does not exceed a configured maximum if such maximum is defined
- the destination address satisfies configured address policy
- the destination address is deterministically convertible to a valid scriptPubKey
- nonce and replay rules are satisfied if such rules are defined
- exclusion status is determinable
- the bidding address is not excluded by persisted state

A bid is invalid if any required condition fails.

No bid may be accepted by inference.

---

# 6. Minimum Bid Rule

Each auction has a configured value:

```text
bidding.minimum_bid_sats
```

Rules:

- `bidding.minimum_bid_sats` must be a positive integer
- it must be fixed before the first valid bid
- it must not change after the auction enters `Open`
- the first valid bid must be greater than or equal to `bidding.minimum_bid_sats`

Zero is forbidden.

A bid with amount less than `bidding.minimum_bid_sats` must be recorded as invalid.

---

# 7. Increment Rule

If auction state is `Open` and at least one valid bid already exists:

- the bid amount must satisfy the configured minimum increment rule
- the increment rule must be fixed at auction open
- the increment rule must not change during the auction

A bid that does not satisfy the required increment must be recorded as invalid.

If no increment rule is defined by configuration or specification, no implementation may invent one.

---

# 8. Destination Address Rule

The bid destination address is the address that may receive the inscribed output if the bid wins and settlement succeeds.

Rules:

- the destination address must be included in the signed bid payload
- the destination address does not need to match the bidding address
- the destination address does not need to match the payment source address
- the destination address must be valid under the permitted Bitcoin address types defined by this specification
- the destination address must be deterministically convertible to a valid scriptPubKey
- the destination address must be immutable once the `BidRecord` is persisted

If the destination address is missing, malformed, invalid under permitted address types, or not deterministically convertible to a valid scriptPubKey:

- the bid must be recorded as invalid
- the bid must not participate in winner resolution

Settlement and inscription must not later reinterpret the destination address of a valid `BidRecord`.

---

# 9. First Bid Semantics

If auction state is `Scheduled` and a bid is valid:

The following must occur atomically:

1. Persist `BidRecord` with `validity = valid`.
2. Persist `AuctionOpenRecord` containing:
   - `opening_bid_id`
   - `opened_at = server_time`
   - `base_end_time = opened_at + auction.duration_seconds`

No intermediate lifecycle state is permitted.

Presence of `AuctionOpenRecord` proves transition from `Scheduled` to `Open`.

If auction state is `Scheduled` and the bid is invalid:

- persist `BidRecord` with `validity = invalid`
- auction state remains `Scheduled`

If atomic persistence of a valid first bid and `AuctionOpenRecord` fails:

- no partial lifecycle transition may occur
- auction state must remain `Scheduled`
- execution must halt or return an explicit persistence error according to the governing error specification

---

# 10. Open-State Admission

If auction state is `Open` and a bid is valid:

- persist `BidRecord` with `validity = valid`
- evaluate extension rule as defined by the governing auction timing specification
- persist `ExtensionEventRecord` only if explicitly required

Extension evaluation must use authoritative `server_time`.

Admission must occur before extension evaluation.

If auction state is `Open` and the bid is invalid:

- persist `BidRecord` with `validity = invalid`
- auction state remains `Open`
- no extension evaluation may occur for that bid

---

# 11. Exclusion Semantics

Exclusion status:

- must be derived exclusively from persisted canonical event records
- must be evaluated mechanically
- must not depend on operator interpretation
- must not be inferred from absence of data

If exclusion status cannot be determined with certainty, the bid must be recorded as invalid.

---

# 12. Forbidden Bid Effects

Bid admission must not:

- consume inscription authority
- imply winning
- imply settlement
- imply inscription
- alter finalization
- reopen a closed auction
- alter `base_end_time`
- modify persisted records
- trigger inscription broadcast
- retry on behalf of the client
- accept a bid after `AuctionCloseRecord`

Bid acceptance means only that the bid became a valid `BidRecord`.

Bid acceptance does not imply winning.

Bid acceptance does not imply settlement.

Bid acceptance does not imply inscription.

---

# 13. Non-Rules

Bid admission must not consider:

- bidder intent
- historical interpretation
- fairness
- probability of settlement
- auction value
- UI presentation
- external economic conditions

Bid admission is deterministic.

A bid is recorded as valid or invalid.

---

# Final Rule

If the system cannot prove that a bid meets all admission requirements:

The bid must be recorded as invalid if admission evaluation was reached.

The bid must not alter auction lifecycle state.