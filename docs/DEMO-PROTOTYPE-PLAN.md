# Demo Prototype Plan

## Purpose

Prepare the `codex-spec` document set for the first Codex implementation pass.

The implementation target is:

> Demo 1: browser auction demo without live Ordinals broadcast.

Demo 1 must demonstrate:

- auction lifecycle
- first valid bid opens auction
- deterministic bid admission and rejection
- auction close, resolution, settlement, and finalization
- append-only canonical records
- restart reconstruction
- frontend-visible current state
- frontend-visible auction history

Demo 1 must not depend on:

- live Ordinals broadcast
- `ord`
- Bitcoin Core RPC
- wallet state
- mempool recognition
- confirmation observation
- external SSD availability
- public deployment

Live Bitcoin Testnet / Ordinals integration belongs to Demo 2 unless explicitly moved into Demo 1 by a later scope revision.

## Working Standard

Do not keep expanding the specification.

Only fix contradictions or missing clauses that would cause Codex or an implementer to invent behavior.

Current objective:

> Minimize implementation ambiguity within the declared scope of Demo 1.

Specification work should remain proportional to:

- implementation risk
- security impact
- prototype requirements
- production time

## Current Blocking Issue

The main spec cleanup is substantially complete.

Before opening Codex, the clarified rhythm-gap, NullSteward, and ambiguity semantics must be reflected across the high-authority documents.

Required semantic rules:

1. The `auction.inter_auction_gap_seconds` interval is a rhythm gap only.
2. It is not a recovery window, settlement window, inscription window, or automatic auction-start trigger.
3. `AuctionRecord` for N + 1 may be persisted only after `FinalizationRecord` for N exists and `auction.inter_auction_gap_seconds` has elapsed.
4. Auction N + 1 opens only when the first valid bid is accepted.
5. Inscription progress for auction N must not block auction availability for N + 1.
6. Ambiguity must not interrupt the Numbers count.
7. Ambiguity must not authorize a second semantically distinct inscription.
8. NullSteward is a protocol-visible final destination, not a universal recovery mechanism.
9. NullSteward must not be used to repair inscription ambiguity after inscription authority is consumed or frozen.
10. If no valid bid is accepted, the auction remains Scheduled. No countdown starts, no finalization occurs, and no inscription process begins.


## Immediate Work Plan

### Step 1: Reflect Clarified Semantics

Update these files:

```text
codex-spec/core/CORE-SEQUENCE.md
codex-spec/core/INVARIANTS.md
codex-spec/core/STATE-MACHINE.md
codex-spec/core/STATE-MACHINE-TABLE.md
codex-spec/bidding/SETTLEMENT.md
codex-spec/PROTOTYPE-SCOPE.md
```

Goal:

Make the rhythm-gap, ambiguity, and NullSteward rules explicit without adding new product behavior.

### Step 2: Review High-Authority Core Documents

Review:

```text
codex-spec/core/INVARIANTS.md
codex-spec/core/TRANSITION-INVARIANTS.md
```

Check for:

- stale `InscriptionRecord` language
- live inscription required for sequence advancement
- mutable lifecycle state treated as canonical truth
- retry or recovery assumptions
- invalid bids being discarded instead of recorded
- ambiguity freezing more than its scoped authority
- sequence advancement depending on inscription success
- terminal auction state blocking later inscription work

### Step 3: Review Supporting Bitcoin / Wallet Documents

Review:

```text
codex-spec/wallet/WALLET-SPEC.md
codex-spec/inscription/INSCRIPTION-FORMAT.md
```

Check that:

- wallet behavior is not required for Demo 1 auction correctness
- Bitcoin Core is not required for Demo 1 auction correctness
- live inscription is not required for Demo 1
- inscription format does not alter auction lifecycle semantics
- wallet configuration does not create retry, recovery, or authority behavior
- future NullSteward mainnet behavior can support a provably unspendable burn mechanism

### Step 4: Decide Whether to Add NULLSTEWARD.md

Consider creating:

```text
codex-spec/core/NULLSTEWARD.md
```

Add it only if NullSteward semantics remain scattered or easy to misread.

Minimum content:

- NullSteward is valid.
- NullSteward is not an error.
- NullSteward is not a recovery mechanism.
- NullSteward is not required for sequence advancement.
- NullSteward is operator-funded when live inscription is enabled.
- NullSteward must not repair ambiguity after inscription authority is consumed or frozen.
- Demo 1 uses NullSteward as a protocol label only.
- Demo 2 may use a testnet destination.
- Mainnet launch should use a provably unspendable burn mechanism.

### Step 5: Create IMPLEMENTATION-SLICE-01.md

Create:

```text
codex-spec/IMPLEMENTATION-SLICE-01.md
```

Purpose:

Define exactly what Codex should build first.

Include:

- local backend
- local browser frontend
- SQLite append-only canonical record storage
- canonical event record table
- state reconstruction
- auction lifecycle
- 83-second rhythm gap after finalization before N + 1 becomes available
- first valid bid opens auction
- invalid bids recorded as invalid BidRecord
- bid admission
- auction close
- resolution
- simplified Demo 1 settlement outcome
- finalization
- InscriptionIntentRecord with `adapter_mode = deferred_in_this_slice`
- `GET /state`
- `POST /bid`
- `GET /auction/history`
- restart demo

Exclude:

- live Ordinals broadcast
- `ord`
- Bitcoin Core RPC
- wallet integration
- mempool checks
- confirmation observation
- public deployment
- production wallet funding
- production authentication
- production rate limiting
- generalized payments
- automatic retry behavior
- mainnet NullSteward burn implementation

### Step 6: Run Final Stale-Term Checks

Run:

```bash
grep -R "InscriptionRecord" codex-spec
grep -R "not_committed" codex-spec
grep -R "deadline_second" codex-spec
grep -R "Persist Scheduled" codex-spec
grep -R "reject all bids" codex-spec
```

Expected result:

- no stale `InscriptionRecord`
- no `not_committed`
- no `deadline_second` except false positives inside `deadline_seconds`
- no `Persist Scheduled`
- no `reject all bids`

`Broadcasting` may appear only in negative statements such as:

```text
There is no canonical `Broadcasting` lifecycle state.
```

### Step 7: Refresh codex-spec.zip

After the clarified semantics are reflected:

```bash
rm -f codex-spec.zip
zip -r codex-spec.zip codex-spec -x "*/.DS_Store" "__MACOSX/*"
```

Then run:

```bash
git status
git diff --stat
```

### Step 8: Commit the Spec Changes

Commit the source documents before opening Codex:

```bash
git status
git add codex-spec
git commit -m "Refine demo prototype specification scope"
```

If `codex-spec.zip` is only an export artifact, do not commit it.

If accidentally staged:

```bash
git restore --staged codex-spec.zip
git checkout -- codex-spec.zip
```

Then commit only the source documents.

### Step 9: Open Codex for Read-Only Review

Do not ask Codex to code first.

Use this prompt:

```text
Read the codex-spec directory only.

Do not modify files.

Summarize the implementation-relevant constraints for Demo 1.

Identify any remaining contradictions or missing clauses that would block implementation of:

1. canonical append-only record storage
2. restart reconstruction
3. auction opening by first valid bid
4. bid admission
5. auction close
6. resolution
7. settlement outcome
8. finalization
9. GET /state
10. POST /bid

Do not propose new product behavior.

If behavior is undefined, identify the missing specification clause.

Keep analysis within the deterministic single-machine browser-demo scope.
```

## Current Status

Completed work has been moved to `SPEC-CLEANUP-COMPLETED-WORK.md`.

Remaining before Codex:

1. Reflect clarified rhythm-gap, NullSteward, and ambiguity semantics.
2. Review high-authority core documents.
3. Review wallet and inscription format documents.
4. Decide whether to add `NULLSTEWARD.md`.
5. Create `IMPLEMENTATION-SLICE-01.md`.
6. Run stale-term checks.
7. Refresh `codex-spec.zip`.
8. Commit spec changes.
9. Open Codex for read-only review.
