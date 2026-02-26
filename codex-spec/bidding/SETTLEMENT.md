## Settlement Timing (Normative)

Settlement deadline must be computed as:
```Text
settlement_deadline =
resolved_at + settlement.deadline_second
```

The computed deadline must be persisted exactly once.

Settlement semantics do not define timing constants.

---

## Settlement Mechanics (Normative)

1. At auction resolution:
   - the winning bid reference **must** be persisted
   - the settlement deadline **must** be computed and persisted
   - the winner destination address **must** be extracted from the signed bid
   - the winner destination address **must** be validated
   - the winner destination address **must** be persisted immutably
   - auction state transitions to `AwaitingSettlement`

2. During the settlement window:
   - the system observes the blockchain via the authoritative node defined in `chain/CHAIN-INTERACTION.md`
   - no retries, prompts, reminders, or extensions are permitted

3. Settlement is **successful** if:
   - a valid payment transaction is Known Confirmed
   - confirmation depth is >= `chain.confirmation_depth`
   - confirmation occurs **before** the settlement deadline

4. Settlement **fails** if:
   - no qualifying Known Confirmed payment exists at the deadline
   - regardless of whether a transaction was broadcast earlier

Broadcast does not count.
Confirmation does.

---

## Winner Destination Address (Normative)

The winner must provide exactly one destination address for inscription delivery.

Rules:

- The destination address must be included in the signed bid payload.
- The destination address does not need to match the bidding address.
- The destination address does not need to match the payment source address.
- The destination address must be valid under the permitted Bitcoin address types defined by this specification.
- The destination address must be persisted in canonical records at auction resolution.
- The destination address must be immutable thereafter.

If the destination address is:

- missing,
- malformed,
- invalid under permitted address types,
- or cannot be deterministically converted to a scriptPubKey,

then:

- the bid must be considered invalid at resolution time.

The inscription machine must not select, modify, or reinterpret the destination address.

---

## Finalization Binding (Normative)

Finalization occurs **only after** settlement outcome is determined.

Finalization **must** record exactly one destination:

- settlement succeeds → winner destination address
- settlement fails → `NullSteward`
- no valid bids → `NullSteward`

Finalization is irreversible.

Settlement outcome **must not** rewrite or reinterpret
the resolution record.

---

## Settlement Failure Outcomes (Normative)

Settlement failure is a **valid outcome**, not an error.

On settlement failure:

- auction proceeds to finalization
- destination is set to `NullSteward`
- no retry or compensation is permitted

No settlement state may be re-entered once finalized.

---

## Authority Protection (Normative)

Once the settlement deadline passes:

- no late payment may be accepted
- no settlement-related state may be rewritten

Settlement does not restore or guarantee inscription authority.

---

## Final Rule

Settlement records **what occurred**, not what was intended.

If payment status is unclear,
the system **must** assume failure
and proceed without guessing.