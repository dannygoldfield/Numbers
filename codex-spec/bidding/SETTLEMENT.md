# Settlement: Numbers

This document defines settlement timing, settlement outcome, winner destination binding, and finalization behavior.

It is normative.

Authority precedence is defined exclusively in `AUTHORITY-ORDER.md`.

Settlement determines whether a resolved auction finalizes to the winning destination address or to `NullSteward`.

`NullSteward` is a protocol-visible final destination.

It is not a universal recovery mechanism.

It must not be used to repair inscription ambiguity after inscription authority is consumed or frozen.

Settlement does not create inscription authority.

Settlement does not consume inscription authority.

Settlement does not alter inscription lifecycle state.

---

# 1. Settlement Principle

Settlement records payment outcome after auction resolution.

Settlement must not:

- rewrite resolution
- reinterpret bid validity
- modify winner destination
- restore authority
- create inscription authority
- delay sequence advancement through speculative recovery

Settlement failure is a valid outcome, not an error.

---

# 2. Settlement Deadline

Settlement deadline must be computed at auction resolution as:

```text
settlement_deadline =
resolution_time + settlement.deadline_seconds
```

The computed settlement deadline must be persisted exactly once as part of `ResolutionRecord`.

Settlement deadline must never be recomputed after persistence.

Settlement deadline must never be extended.

Settlement timing constants are defined by `config/CONFIG-REFERENCE.md`.

Settlement semantics do not define timing constants.

---

# 3. Settlement Entry

At auction resolution:

- `ResolutionRecord` must be persisted
- winning bid reference must be persisted
- winning amount must be persisted
- settlement deadline must be computed and persisted
- winner destination address must be taken from the winning valid `BidRecord`
- winner destination address must already satisfy bid admission rules
- auction state becomes `AwaitingSettlement`

Under the current first-valid-bid opening rule, a normally opened auction has at least one valid bid.

A no-valid-bid condition does not enter settlement.

If no valid bid is accepted, the auction remains `Scheduled`.

---

# 4. Settlement Observation

During the settlement window:

- the system observes payment status through the authoritative node defined in `chain/CHAIN-INTERACTION.md`
- no prompts are permitted
- no reminders are permitted
- no deadline extensions are permitted
- no speculative recovery is permitted
- no late settlement acceptance is permitted

Broadcast does not count as settlement.

Mempool presence does not count as settlement.

Only confirmation to the required depth counts as settlement.

---

# 5. Successful Settlement

Settlement is successful only if all are true:

- a valid payment transaction is Known Confirmed
- confirmation depth is greater than or equal to `chain.confirmation_depth`
- confirmation occurs before `settlement_deadline`
- payment satisfies the settlement rules defined by this specification

If settlement is successful:

- persist exactly one `SettlementRecord`
- `SettlementRecord.status` must be `settled`
- `SettlementRecord.confirmation_txid` must be non-null
- persist exactly one `FinalizationRecord`
- `FinalizationRecord.destination_address` must equal the winning destination address

---

# 6. Failed Settlement

Settlement fails if no qualifying Known Confirmed payment exists at `settlement_deadline`.

Settlement failure occurs regardless of whether a payment transaction was broadcast earlier.

If settlement fails:

- persist exactly one `SettlementRecord`
- `SettlementRecord.status` must be `expired`
- `SettlementRecord.confirmation_txid` must be `null`
- persist exactly one `FinalizationRecord`
- `FinalizationRecord.destination_address` must be `NullSteward`

Settlement failure is irreversible.

No retry or compensation is permitted.

---

# 7. No Valid Bid Condition

No valid bid is not a settlement path in the current first-valid-bid opening model.

If no valid bid is accepted:

- auction state remains `Scheduled`
- no countdown starts
- no auction close occurs
- no resolution occurs
- no settlement outcome is recorded
- no finalization occurs
- no inscription process begins
- no `NullSteward` outcome is produced

---

# 8. Winner Destination Address

The winner destination address is the destination address persisted in the winning valid `BidRecord`.

Rules:

- the destination address must be included in the signed bid payload
- the destination address does not need to match the bidding address
- the destination address does not need to match the payment source address
- the destination address must be valid under the permitted Bitcoin address types defined by this specification
- the destination address must be immutable once the `BidRecord` is persisted
- the destination address must not be modified during settlement
- the destination address must not be modified during finalization
- the destination address must not be modified by the inscription machine

If a bid destination address is missing, malformed, invalid under permitted address types, or cannot be deterministically converted to a scriptPubKey:

- the bid must be rejected during bid admission
- the bid must not become a valid `BidRecord`
- the bid must not participate in winner resolution

Settlement must not invalidate a previously valid winning bid by reinterpreting destination address rules.

---

# 9. Finalization Binding

Finalization occurs only after settlement outcome is determined.

Finalization must record exactly one destination:

- settlement succeeds: winning destination address
- settlement fails: `NullSteward`

No valid bid does not finalize in the current first-valid-bid opening model.

Finalization is irreversible.

Settlement outcome must not rewrite or reinterpret `ResolutionRecord`.

No settlement state may be re-entered once finalized.

---

# 10. Authority Protection

Settlement does not restore inscription authority.

Settlement does not guarantee inscription authority.

Settlement does not consume inscription authority.

Once settlement deadline passes:

- no late payment may be accepted
- no settlement-related state may be rewritten
- no final destination may be changed

---

# Final Rule

Settlement records what occurred.

Settlement does not record what was intended.

If qualifying payment status is unclear at the settlement deadline:

The system must treat settlement as failed and finalize to `NullSteward`.