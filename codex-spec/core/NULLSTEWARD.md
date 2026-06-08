# `NullSteward`: Numbers

This document defines `NullSteward` within the Numbers protocol.

It is normative.

`NullSteward` is a valid final destination for a number that remains in the Numbers sequence but does not enter ordinary winner-controlled circulation.

---

## 1. Definition

`NullSteward` is a protocol-visible final destination.

A number assigned to `NullSteward`:

- remains accounted for in the canonical Numbers sequence
- is not skipped
- is not removed from history
- is not assigned to an ordinary winner-controlled destination
- does not interrupt sequence advancement

---

## 2. What `NullSteward` Is Not

`NullSteward` is not:

- an error state
- a recovery mechanism
- a retry mechanism
- a repair path
- a configurable destination
- a wallet policy
- an inscription authority reset
- a substitute for ambiguity handling

Routing a number to `NullSteward` does not imply system failure.

---

## 3. Ambiguity Boundary

`NullSteward` must not be used to repair inscription ambiguity after inscription authority is consumed or frozen.

If inscription authority is consumed or frozen, the system must not create a second semantically distinct inscription attempt for the same number by routing that number to `NullSteward`.

Ambiguity is an inscription status, not a third final destination category.

---

## 4. Demo 1 Scope

For Demo 1, `NullSteward` is a protocol label only.

Demo 1 does not require:

- live `NullSteward` inscription
- burn address implementation
- burn script implementation
- wallet funding
- Bitcoin Core RPC
- mempool observation
- confirmation observation

---

## 5. Later Live Inscription Scope

When live inscription is enabled by an active implementation slice, inscription to `NullSteward` is operator-funded.

This document does not define the live `NullSteward` destination mechanism.

For mainnet launch, the `NullSteward` destination mechanism must be specified before use.

The intended mainnet design target is a provably unspendable burn mechanism, but the exact mechanism is not defined by this document.

---

## 6. Final Rule

`NullSteward` preserves sequence accounting without creating recovery authority.

No behavior may treat `NullSteward` as permission to retry, repair, replace, or override an inscription outcome.