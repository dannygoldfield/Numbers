# Settlement — Numbers

This document defines **settlement semantics** for Numbers.

It is **normative**.

Settlement governs what happens **after auction resolution**
and **before auction finalization**.

If there is a conflict,
CORE-SEQUENCE.md, STATE-MACHINE.md, INVARIANTS.md,
PERSISTENCE.md, and DATA-MODEL.md take precedence.

---

## Purpose

Settlement exists to:

- bind a winning bid to an obligation
- provide a finite opportunity to fulfill that obligation
- allow failure without authority reuse
- prevent the system from guessing intent

Settlement does **not** enforce payment.
It records whether payment occurred in time.

---

## Definitions

- **Winning Bid**  
  The highest valid bid recorded at auction resolution.

- **Settlement Window**  
  A fixed duration following auction resolution
  during which payment may be made.

- **Settlement Deadline**  
  The exact timestamp at which the settlement window closes.

- **Settlement Success**  
  A valid on-chain payment confirmed before the settlement deadline.

- **Settlement Failure**  
  Absence of a confirmed payment by the settlement deadline,
  for any reason.

---

## Bid Commitment Model (Normative)

A bid is a **signed commitment**, not a payment.

A valid bid **must** include a wallet signature binding:

- auction number
- bid amount
- bidder address
- destination address
- unique commitment identifier

The signature proves:

- control of the bidding address
- intent to bid
- acceptance of settlement rules

No funds are escrowed at bid time.

---

## Settlement Timing (Normative)

Settlement timing values:

- are derived from configuration
- are computed at auction resolution
- **must** be persisted durably
- **must not** change for that auction

Settlement deadline **must** be computed as:
```Text
settlement_deadline =
resolved_at + settlement.deadline_seconds
```

The computed deadline must be persisted exactly once.

Settlement semantics do not define timing constants.

---

## Settlement Mechanics (Normative)

1. At auction resolution:
   - the winning bid reference **must** be persisted
   - the settlement deadline **must** be computed and persisted
   - auction state transitions to `AwaitingSettlement`

2. During the settlement window:
   - the system observes the blockchain
   - no retries, prompts, reminders, or extensions are permitted

3. Settlement is **successful** if:
   - a valid payment transaction is **confirmed**
   - confirmation occurs **before** the settlement deadline

4. Settlement **fails** if:
   - no confirmed payment exists at the deadline
   - regardless of whether a transaction was broadcast earlier

Broadcast does not count.
Confirmation does.

---

## Finalization Binding (Normative)

Finalization occurs **only after** settlement outcome is determined.

Finalization **must** record exactly one destination:

- settlement succeeds → winning destination
- settlement fails → `NullSteward`
- no valid bids → `NullSteward`

Finalization is irreversible.

Settlement outcome **must not** rewrite or reinterpret
the resolution record.

---

## Destination Semantics (Normative)

The inscription destination is determined by the **winning bid signature**.

Rules:

- default destination is the bidding address
- an alternate destination may be specified
- destination **must** be included in the signed bid payload

Settlement payment:

- may originate from any wallet
- does not alter destination

Payment source and inscription destination are independent.

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