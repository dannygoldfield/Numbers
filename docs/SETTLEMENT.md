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
- destination address (default: bidder address)
- unique commitment identifier (nonce)

The signature proves:

- control of the bidding address
- intent to bid
- acceptance of settlement rules

No funds are escrowed at bid time.

---

## Settlement Window Timing (Normative)

The settlement window duration is **equal to the auction duration**
for the same environment.

Settlement timing values are:

- computed at auction resolution
- persisted durably
- immutable for that auction

### Mainnet

- Auction duration: `12:34:56`
- Settlement window: `12:34:56`
- Inter-auction pause: `1:23`

### Testnet

- Auction duration: `12:34`
- Settlement window: `12:34`
- Inter-auction pause: `1:23`

---

## Settlement Mechanics (Normative)

1. At auction resolution:
   - settlement intent **must** be persisted
   - settlement deadline **must** be computed and persisted

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

Finalization **must** produce exactly one destination:

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
- inscription authority remains subject to state machine rules
- no retry or compensation is permitted

No settlement state may be re-entered once finalized.

---

## Griefing Controls (Normative)

Numbers permits non-payment.
It does not permit repeated abuse without consequence.

### Testnet

- A bidding address that wins and fails to settle
  enters a **cooldown**.
- During cooldown, bids from that address **must** be rejected.
- Default cooldown: `123` auctions.

Cooldown state **must** be persisted and enforced mechanically.

### Mainnet

- A bidding address that wins and fails to settle
  is **permanently excluded** from future participation.

This exclusion:

- is irreversible
- requires no operator intervention
- is enforced solely by persisted state

---

## Authority Protection (Normative)

Settlement outcome consumes settlement authority.

Once the settlement deadline passes:

- settlement authority is irreversibly consumed
- no late payment may be accepted
- no settlement-related state may be rewritten

Settlement authority exhaustion
does not restore or guarantee inscription authority.

---

## Final Rule

Settlement records **what occurred**, not what was intended.

If payment status is unclear,
the system **must** assume failure
and proceed without guessing.
