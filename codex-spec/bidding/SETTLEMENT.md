# Settlement: Numbers

This document defines resolution, settlement timing, settlement outcome, winner destination binding, and finalization behavior.

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

# 2. Winner Resolution Rule

Resolution determines the winning valid `BidRecord`.

For a closed auction, the winning bid is the valid `BidRecord` for that auction with the highest `amount_sats`.

If more than one valid `BidRecord` has the same highest `amount_sats`, the winning bid is the one with the lowest canonical `sequence_index`.

The tie rule is mandatory even though valid equal leading bids should not occur under bid admission increment rules.

Invalid `BidRecord` entries must not participate in resolution.

Resolution must persist:

- `winning_bid_id`
- `winning_amount_sats`
- `resolution_time`
- `settlement_deadline`
- `resolution_inputs_hash`

Under the current first-valid-bid opening rule, a normally closed auction has at least one valid bid.

A no-winner resolution path must not be implemented for Demo 1.

---

# 3. Settlement Deadline

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

# 4. Settlement Entry

At auction resolution:

- `ResolutionRecord` must be persisted
- winning bid reference must be persisted
- winning amount must be persisted
- settlement deadline must be computed and persisted
- winner destination address must be taken from the winning valid `BidRecord`
- winner destination address must already satisfy bid admission rules
- auction state becomes `AwaitingSettlement`

A no-valid-bid condition does not enter settlement.

If no valid bid is accepted, the auction remains `Scheduled`.

---

# 5. Demo 1 Local Settlement

Demo 1 uses local deterministic settlement control.

Demo 1 settlement is not live payment recognition.

Demo 1 settlement must not require:

- Bitcoin payment detection
- wallet-derived settlement address
- Bitcoin Core RPC
- mempool observation
- confirmation observation

For Demo 1, settlement outcome is produced only by the Demo 1 local settlement control defined in `api/API-SPEC.md`.

The allowed Demo 1 local settlement outcomes are:

- `settled`
- `expired`

For Demo 1 local settlement:

- `SettlementRecord.settlement_source` must be `demo_local`
- `SettlementRecord.confirmation_txid` must be `null`
- `settled` finalizes to the winning destination address
- `expired` finalizes to `NullSteward`

Demo 1 local settlement does not prove payment.

Demo 1 local settlement exists only to demonstrate deterministic lifecycle progression.

---

# 6. Chain-Confirmed Settlement

Chain-confirmed settlement is outside Demo 1.

When chain-confirmed settlement is enabled by a later active implementation slice, settlement is successful only if all are true:

- a valid payment transaction is Known Confirmed
- confirmation depth is greater than or equal to `chain.confirmation_depth`
- confirmation occurs before `settlement_deadline`
- payment satisfies the settlement rules defined by this specification

If chain-confirmed settlement is successful:

- persist exactly one `SettlementRecord`
- `SettlementRecord.status` must be `settled`
- `SettlementRecord.settlement_source` must be `chain_confirmed`
- `SettlementRecord.confirmation_txid` must be non-null
- persist exactly one `FinalizationRecord`
- `FinalizationRecord.destination_address` must equal the winning destination address

---

# 7. Failed or Expired Settlement

For Demo 1 local settlement, `expired` is produced only by the Demo 1 local settlement control.

For Demo 1 local settlement control, `expired` does not require `server_time >= settlement_deadline`.

Demo 1 local `expired` is a deterministic operator/demo control outcome, not observed chain expiry.

For chain-confirmed settlement, settlement expires if no qualifying Known Confirmed payment exists at `settlement_deadline`.

Settlement expiration occurs regardless of whether a payment transaction was broadcast earlier.

If settlement expires:

- persist exactly one `SettlementRecord`
- `SettlementRecord.status` must be `expired`
- `SettlementRecord.confirmation_txid` must be `null`
- persist exactly one `FinalizationRecord`
- `FinalizationRecord.destination_address` must be `NullSteward`

Settlement expiration is irreversible.

No retry or compensation is permitted.

---

# 8. No Valid Bid Condition

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

# 9. Winner Destination Address

The winner destination address is the destination address persisted in the winning valid `BidRecord`.

Rules:

- the destination address must satisfy the active bid admission destination rule
- the destination address does not need to match the bidding identity
- the destination address does not need to match the payment source address
- the destination address must be immutable once the `BidRecord` is persisted
- the destination address must not be modified during settlement
- the destination address must not be modified during finalization
- the destination address must not be modified by the inscription machine

Settlement must not invalidate a previously valid winning bid by reinterpreting destination address rules.

---

# 10. Finalization Binding

Finalization occurs only after settlement outcome is determined.

Finalization must record exactly one destination:

- settlement status `settled`: winning destination address
- settlement status `expired`: `NullSteward`

No valid bid does not finalize in the current first-valid-bid opening model.

Finalization is irreversible.

Settlement outcome must not rewrite or reinterpret `ResolutionRecord`.

No settlement state may be re-entered once finalized.

---

# 11. Authority Protection

Settlement does not restore inscription authority.

Settlement does not guarantee inscription authority.

Settlement does not consume inscription authority.

Once settlement deadline passes for chain-confirmed settlement:

- no late payment may be accepted
- no settlement-related state may be rewritten
- no final destination may be changed

---

# Final Rule

Settlement records what occurred.

Settlement does not record what was intended.

If qualifying chain-confirmed payment status is unclear at the settlement deadline in a live chain-confirmed settlement slice:

The system must treat settlement as expired and finalize to `NullSteward`.
