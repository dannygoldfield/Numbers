# Demo Prototype Plan

## Purpose

This document tracks the specification cleanup work required before opening Codex for the first implementation pass.

The goal is to prepare the codex-spec document set for a deterministic, single-machine browser demo of Numbers.

The first implementation target is Demo 1:

> Browser auction demo without live Ordinals broadcast.

Demo 1 must prove:

* auction lifecycle
* append-only canonical records
* restart reconstruction
* frontend-visible state
* sequence advancement
* AI-assisted implementation readiness

Demo 1 must not depend on:

* live Ordinals broadcast
* ord
* Bitcoin Core RPC
* wallet state
* mempool recognition
* confirmation observation
* external SSD availability

Live Bitcoin Testnet / Ordinals integration belongs to Demo 2 unless explicitly moved into Demo 1 by a later scope revision.

# Current Working Standard

The goal is not to eliminate all possible ambiguity forever.

The goal is:

> Minimize implementation ambiguity within the declared scope of the specification.

Specification effort should remain proportional to:

* implementation risk
* security impact
* prototype requirements
* production time

Do not over-edit.

Only fix contradictions or missing clauses that would cause Codex or an implementer to invent behavior.

# Clarified Semantics Before Zip Refresh

Status: In Progress

These clarifications must be reflected before replacing the refreshed codex-spec.zip.

## Auction Rhythm

The 83-second inter-auction gap is a user-experience rhythm parameter.

It is not:

* a recovery window
* a settlement window
* an inscription window
* an automatic auction-start trigger

Auction N + 1 may become available only after:

```text
FinalizationRecord.finalization_time + auction.inter_auction_gap_seconds
```

Auction N + 1 does not open automatically.

Auction N + 1 opens only when the first valid bid is accepted.

## Inscription Independence

Inscription progress for auction N must not block auction N + 1.

Sequence advancement depends on auction finalization and the inter-auction rhythm gap, not inscription confirmation.

The inscription process may continue independently after auction finalization.

## NullSteward

NullSteward is a protocol-visible final destination for numbers that should remain in the Numbers inscription sequence but should not enter ordinary winner-controlled circulation.

For Demo 1:

* NullSteward is a protocol label only.
* No live NullSteward inscription is required.

For Demo 2:

* NullSteward may use a testnet destination mechanism.

For mainnet launch:

* NullSteward should use a provably unspendable burn script or burn address.
* The exact burn mechanism must be verified before launch.

NullSteward is:

* a valid final destination
* protocol-visible
* operator-funded when live inscription is enabled
* not an error state
* not a recovery mechanism
* not required for sequence advancement

NullSteward must not be used to repair a case where inscription authority may already have crossed the broadcast boundary.

## Ambiguity

Ambiguity must not interrupt the Numbers count.

An ambiguous inscription state must be recorded and displayed.

Ambiguity must not authorize a second semantically distinct inscription.

Every number remains accounted for in the Numbers canonical record sequence, even if its inscription status is ambiguous.

A useful distinction is:

```text
final_destination:
- winner destination
- NullSteward

inscription_status:
- not_started
- deferred
- inscribing
- inscribed
- ambiguous
```

Ambiguity is an inscription status, not a third ownership category.

## Payment Failure

The public launch behavior for settlement failure is not fully finalized.

The likely launch rule is:

```text
settlement deadline expires → final destination = NullSteward
```

Rationale:

* indefinite waiting can block the sequence
* chasing payment creates operational complexity
* NullSteward preserves the sequence without speculative recovery

For Demo 1, settlement may use simplified behavior defined by IMPLEMENTATION-SLICE-01.md.

## No-Bid Condition

Because auctions now open only on the first valid bid, a no-bid condition does not normally finalize to NullSteward.

Instead:

```text
No valid bid → auction remains Scheduled → no countdown → no finalization → no inscription
```

A later mode that sends unbid numbers to NullSteward would require explicit specification.

## Operating Target

NullSteward safety outcomes should be rare.

Operating target:

```text
Target: fewer than 1 NullSteward safety outcome per 1000 auctions
Alarm threshold: 3 per 1000 auctions
Unacceptable: more than 3 per 1000 auctions
```

This is an operating target, not a state-machine invariant.

# Completed Work So Far

## 1. Project Instructions Recalibrated

Status: Complete

The Numbers project instructions were revised to:

* remove the impossible “guess-space to zero” standard
* add production velocity as a real constraint
* define the current phase as a deterministic, single-machine browser demo
* preserve spec-first development
* allow ambiguity consumption and architectural simplification when explicitly specified

Current primary objective:

> Minimize implementation ambiguity within the declared scope of the specification.

## 2. Prototype Scope Added

Status: Complete

A new root-level document was created:

```text
codex-spec/PROTOTYPE-SCOPE.md
```

It defines the current implementation scope:

* deterministic
* single-machine
* browser-demonstrable
* append-only
* restart-reconstructible
* AI-assisted
* not production-hosted
* not distributed

It also defines the Demo 1 / Demo 2 split.

### Demo 1

Browser auction demo without live Ordinals broadcast.

### Demo 2

Browser auction demo with Testnet inscription adapter.

## 3. Authority Order Updated

Status: Complete

AUTHORITY-ORDER.md was revised to include:

```text
1. AUTHORITY-ORDER.md
2. PROTOTYPE-SCOPE.md
3. core/INVARIANTS.md
```

The stale path was fixed:

```text
settlement/SETTLEMENT.md
```

was changed to:

```text
bidding/SETTLEMENT.md
```

Result:

The authority order now points to the actual settlement document and places prototype scope near the top.

## 4. SPEC-INDEX Updated

Status: Complete

SPEC-INDEX.md was revised to include root-level documents:

* AUTHORITY-ORDER.md
* PROTOTYPE-SCOPE.md
* SPEC-INDEX.md

It was also updated to include active folders:

* core/
* data/
* bidding/
* errors/
* config/
* api/
* chain/
* wallet/
* inscription/

Result:

The index now matches the current codex-spec structure.

## 5. Event and Record Model Unified

Status: Complete

core/EVENT-TYPES.md was revised.

The spec now treats these terms as the same persisted truth when used for system state:

```text
event
event record
canonical record
persisted record
```

Working rule:

```text
canonical event record = canonical record = persisted record
```

There is no separate event model and record model.

Result:

Codex should not infer a separate event architecture and record architecture.

## 6. Data Model Revised

Status: Complete

data/DATA-MODEL.md was revised to align with the unified event-record model.

The vague InscriptionRecord was replaced with:

* InscriptionIntentRecord
* InscriptionBroadcastRecord
* InscriptionConfirmationRecord

This separates:

* inscription intent
* broadcast classification
* confirmation observation

Important authority rule:

```text
InscriptionIntentRecord does not consume inscription authority.
```

Settlement alignment was also completed:

* ResolutionRecord includes settlement_deadline
* SettlementRecord does not include settlement_deadline
* SettlementRecord records terminal settlement outcome only

Result:

The data model now matches the revised event-record model, settlement model, and Demo 1 inscription boundary.

## 7. State Machine Table Revised

Status: Complete

core/STATE-MACHINE-TABLE.md was revised.

The old canonical inscription state:

```text
Broadcasting
```

was removed.

Broadcast attempt is now an operation, not a lifecycle state.

Canonical inscription states are now:

* NotStarted
* Inscribing
* Ambiguous
* Inscribed

Broadcast outcome vocabulary was changed from:

```text
not_committed
```

to:

```text
pre_commit_rejected
```

Result:

Inscription state is now reconstructible from canonical event records.

## 8. State Machine Revised

Status: Complete

core/STATE-MACHINE.md was revised.

Fixes:

* Scheduled no longer rejects all bids.
* Scheduled can evaluate bid submissions.
* First valid bid opens the auction.
* Invalid bids may be persisted without changing lifecycle state.
* There is no canonical Broadcasting lifecycle state.
* InscriptionRecord was removed.
* Inscription authority is consumed only at broadcast_commit.
* Demo 1 inscription behavior is explicit.

Result:

The prose state machine now matches the state machine table.

## 9. Core Sequence Revised

Status: Complete, but needs one semantic update

core/CORE-SEQUENCE.md was revised.

Fixes already completed:

* removed stale InscriptionRecord
* removed “Persist Scheduled → Open transition”
* clarified that no separate transition record exists
* removed hardcoded timing constants
* made sequence advancement independent of live inscription success
* added Demo 1 and Demo 2 inscription sequence distinction

Still needs semantic update:

* clarify that the 83-second gap is a rhythm gap
* clarify that AuctionRecord for N + 1 may be created only after finalization plus the configured gap
* clarify that Auction N + 1 opens only on first valid bid
* clarify that inscription progress for N does not block auction availability for N + 1

Current key rule to refine:

```text
Sequence advancement to N + 1 is permitted only after FinalizationRecord exists for number N.
```

Revised target rule:

```text
AuctionRecord for N + 1 may be created only after FinalizationRecord exists for N and auction.inter_auction_gap_seconds has elapsed.

Auction N + 1 opens only when the first valid bid is accepted.

Inscription progress for N must not block auction availability for N + 1.
```

## 10. Inscription Machine Revised

Status: Complete, but may need NullSteward clarification later

inscription/INSCRIPTION-MACHINE.md was revised.

It now aligns with:

* Demo 1 scope
* Demo 2 scope
* no canonical Broadcasting state
* InscriptionIntentRecord
* InscriptionBroadcastRecord
* InscriptionConfirmationRecord
* broadcast_commit
* pre_commit_rejected
* ambiguous

Demo 1 behavior is explicit:

* InscriptionIntentRecord may be persisted
* adapter_mode = deferred_in_this_slice
* no live inscription broadcast required
* no live inscription success may be simulated

Future clarification needed:

* if final destination is NullSteward and live inscription is enabled, operator wallet funds inscription
* mainnet NullSteward should use a provably unspendable burn script or burn address
* exact burn mechanism is deferred until pre-launch design

Result:

Ordinals behavior is bounded and does not block Demo 1.

## 11. Authority Consumption Revised

Status: Complete

core/AUTHORITY-CONSUMPTION.md was revised.

Inscription authority is consumed only at:

```text
broadcast_commit
```

broadcast_commit occurs only when:

1. a candidate inscription transaction is broadcast through the authoritative node
2. the authoritative node reports the transaction present in its mempool

Important rules:

* intent persistence does not consume authority
* transaction construction does not consume authority
* confirmation observation does not consume authority
* pre_commit_rejected does not consume authority
* ambiguous freezes authority permanently
* retry is not automatic
* bounded retry requires an explicit later implementation slice

Result:

Authority boundary is singular and explicit.

## 12. Restart Rules Revised

Status: Complete

data/RESTART-RULES.md was revised.

Restart now means:

```text
reconstruction, not repair
```

Rules added or clarified:

* runtime memory is reconstructed from canonical event records only
* restart does not grant authority
* restart does not trigger inscription broadcast
* restart does not silently simulate inscription success
* Demo 1 may preserve deferred inscription status without live broadcast
* committed inscription authority remains consumed
* ambiguous inscription authority remains frozen

Result:

Restart semantics now align with append-only truth and Demo 1 scope.

## 13. Persistence Revised

Status: Complete

data/PERSISTENCE.md was revised.

Fixes:

* uses canonical event records
* defines common persistence envelope
* aligns with DATA-MODEL.md
* removes old InscriptionRecord
* fixes the broken current_end_time formula
* supports Demo 1 without live Bitcoin or Ordinals
* permits and prefers SQLite for Demo 1
* forbids mutable lifecycle state as canonical truth

Result:

Persistence now supports direct implementation of append-only local storage.

## 14. API Spec Revised

Status: Complete

api/API-SPEC.md was revised.

Fixes:

* API now uses fixed response shapes
* unknown, unavailable, or not-yet-applicable values use null
* fields defined in state shapes must not be omitted
* POST /bid is allowed when auction state is Scheduled or Open
* first valid bid may atomically open a scheduled auction
* bid submission does not imply winning, settlement, or inscription

Result:

API behavior now matches the auction lifecycle.

## 15. API State Shapes Revised

Status: Complete

api/API-STATE-SHAPES.md was revised.

Fixes:

* broken markdown fence around current_end_time
* null vs omission conflict
* Demo 1 inscription fields
* no pending settlement status in canonical SettlementRecord.status
* fixed API state shape expectations for frontend

Canonical settlement status is:

* settled
* expired
* not_required
* null when no SettlementRecord exists

Result:

Frontend-visible API state is now deterministic and fixed-shape.

## 16. Settlement Revised

Status: Complete, but launch behavior remains open

bidding/SETTLEMENT.md was revised.

Fixes:

```text
settlement.deadline_second
```

was corrected to:

```text
settlement.deadline_seconds
```

Settlement now says:

* settlement deadline is computed at resolution
* settlement deadline is persisted exactly once in ResolutionRecord
* SettlementRecord records terminal settlement outcome only
* settlement failure is a valid outcome, not an error
* failed settlement finalizes to NullSteward
* settlement does not create or consume inscription authority

Clarification needed:

Because auctions open only on first valid bid, “no valid bids finalizes to NullSteward” should not be treated as a normal path.

If no valid bid ever arrives, the auction remains Scheduled.

Public launch settlement failure behavior is still open, but the likely rule is:

```text
settlement deadline expires → final destination = NullSteward
```

Result:

Settlement mostly aligns with finalization and data model semantics, but should be checked against the clarified NullSteward model.

## 17. Bidding Admission Revised

Status: Complete

bidding/BIDDING-ADMISSION.md was revised.

Fixes:

* invalid bids are not silently discarded
* invalid bids that reach admission evaluation become invalid BidRecord entries
* invalid bids do not alter lifecycle state
* first valid bid opens a scheduled auction atomically with AuctionOpenRecord
* destination address validation happens at bid admission
* bid acceptance only means valid BidRecord
* bid acceptance does not imply winning, settlement, or inscription

Result:

Bid behavior now aligns with append-only audit truth and lifecycle rules.

## 18. Chain Interaction Revised

Status: Complete

chain/CHAIN-INTERACTION.md was revised.

Fixes:

* no Bitcoin interaction required for Demo 1
* chain interaction does not create lifecycle semantics
* chain interaction does not alter authority semantics
* mempool presence is not confirmation truth
* mempool presence is only permitted for broadcast_commit classification
* no automatic retry exists unless explicitly permitted by active implementation slice
* configuration cannot create retry behavior

Result:

Bitcoin Core and external SSD dependencies no longer block Demo 1.

## 19. Error Taxonomy Revised

Status: Complete

errors/ERROR-TAXONOMY.md was revised.

Fixes:

* retry is not default behavior
* retry is not granted by the error taxonomy
* no automatic retry exists unless explicitly permitted by active implementation slice
* deterministic bid rejection may still produce an invalid BidRecord
* Recoverable was replaced with more constrained operational error handling
* ambiguity freezes scoped authority
* fatal errors halt execution

Result:

Error handling now aligns with authority rules and Demo 1 scope.

## 20. Configuration Reference Revised

Status: Complete

config/CONFIG-REFERENCE.md was revised.

Fixes:

* added prototype.demo_stage
* added Demo 1 / Demo 2 scope configuration
* added SQLite storage configuration
* replaced general inscription.enabled with inscription.adapter_mode
* made NullSteward a fixed protocol outcome, not a configurable destination
* excluded mainnet from current prototype scope
* allowed Bitcoin Core path as operational configuration only
* configuration cannot create retry behavior
* configuration cannot alter authority semantics

Result:

Configuration now tunes parameters only and does not alter truth.

# Sanity Checks Completed

## Stale Term Check

The following stale terms were checked:

```bash
grep -R "InscriptionRecord" codex-spec
grep -R "not_committed" codex-spec
grep -R "deadline_second" codex-spec
```

Result:

* no stale InscriptionRecord
* no stale not_committed
* deadline_second matches only correct deadline_seconds false positives

## Transition Record Check

The following stale transition language was checked:

```bash
grep -R "Persist Scheduled" codex-spec
grep -R "transition record" codex-spec
grep -R "reject all bids" codex-spec
```

Result:

* no stale Persist Scheduled
* no stale reject all bids
* only acceptable negative statement: No separate transition record exists

## File Title Check

The document title scan looked consistent.

The three highest-risk wrong-paste files were rechecked:

* data/DATA-MODEL.md
* data/PERSISTENCE.md
* core/STATE-MACHINE.md

Result:

All three now appear coherent and aligned.

# Remaining Work Before Opening Codex

## Step 1: Reflect Clarified Semantics

Update the following files to reflect the clarified auction rhythm, NullSteward, and ambiguity semantics:

```text
codex-spec/core/CORE-SEQUENCE.md
codex-spec/core/INVARIANTS.md
codex-spec/core/STATE-MACHINE.md
codex-spec/core/STATE-MACHINE-TABLE.md
codex-spec/bidding/SETTLEMENT.md
codex-spec/PROTOTYPE-SCOPE.md
```

Specific required changes:

* 83 seconds is a rhythm gap, not an automatic auction start.
* AuctionRecord for N + 1 may be created only after finalization plus auction.inter_auction_gap_seconds.
* Auction N + 1 opens only on first valid bid.
* inscription progress for N must not block auction availability for N + 1.
* ambiguity must not interrupt the Numbers count.
* ambiguity must not authorize a second semantically distinct inscription.
* NullSteward is a protocol-visible final destination, not a universal recovery mechanism.
* NullSteward must not be used to repair post-authority inscription ambiguity.
* no valid bid should normally leave auction state as Scheduled, not finalize to NullSteward.

## Step 2: Review High-Authority Core Documents

Review:

```text
codex-spec/core/INVARIANTS.md
codex-spec/core/TRANSITION-INVARIANTS.md
```

Check for:

* old InscriptionRecord language
* live inscription required for sequence advancement
* mutable lifecycle state
* retry or recovery assumptions
* invalid bids being discarded instead of recorded
* ambiguity freezing more than its scoped authority
* sequence advancement depending on inscription success
* terminality blocking inscription after auction finalization

## Step 3: Review Supporting Bitcoin / Wallet Documents

Review:

```text
codex-spec/wallet/WALLET-SPEC.md
codex-spec/inscription/INSCRIPTION-FORMAT.md
```

Check that:

* wallet behavior is not required for Demo 1 auction correctness
* Bitcoin Core is not required for Demo 1 auction correctness
* live inscription is not required for Demo 1
* inscription format does not alter auction lifecycle semantics
* wallet configuration does not create retry, recovery, or authority behavior
* future NullSteward mainnet behavior can support a provably unspendable burn mechanism

## Step 4: Consider Adding NULLSTEWARD.md

Consider creating:

```text
codex-spec/core/NULLSTEWARD.md
```

Purpose:

Define NullSteward as a protocol-visible final destination/safety sink, not a recovery mechanism.

Likely rules:

* NullSteward is valid.
* NullSteward is not an error.
* NullSteward is not required for sequence advancement.
* NullSteward is operator-funded when live inscription is enabled.
* NullSteward must not repair ambiguity after inscription authority is consumed or frozen.
* Demo 1 uses NullSteward as a protocol label only.
* Demo 2 may use a testnet destination.
* Mainnet launch should use a provably unspendable burn mechanism.

## Step 5: Create IMPLEMENTATION-SLICE-01.md

Create:

```text
codex-spec/IMPLEMENTATION-SLICE-01.md
```

Purpose:

Tell Codex exactly what to build first.

This document should define Demo 1 only.

It should explicitly include:

* local backend
* local browser frontend
* SQLite append-only canonical record storage
* canonical event record table
* state reconstruction
* auction lifecycle
* 83-second rhythm gap after finalization before N + 1 becomes available
* first valid bid opens auction
* invalid bids recorded as invalid BidRecord
* bid admission
* auction close
* resolution
* simplified settlement outcome for Demo 1
* finalization
* InscriptionIntentRecord with adapter_mode = deferred_in_this_slice
* GET /state
* POST /bid
* GET /auction/history
* restart demo

It should explicitly exclude:

* live Ordinals broadcast
* ord
* Bitcoin Core RPC
* wallet integration
* mempool checks
* confirmation observation
* public deployment
* production wallet funding
* production authentication
* production rate limiting
* generalized payments
* automatic retry behavior
* mainnet NullSteward burn implementation

## Step 6: Run Final Stale-Term Checks

Run:

```bash
grep -R "InscriptionRecord" codex-spec
grep -R "not_committed" codex-spec
grep -R "deadline_second" codex-spec
grep -R "Persist Scheduled" codex-spec
grep -R "reject all bids" codex-spec
```

Expected result:

* no stale InscriptionRecord
* no not_committed
* no deadline_second except false positives inside deadline_seconds
* no Persist Scheduled
* no reject all bids

Broadcasting may appear only in negative statements such as:

```text
There is no canonical `Broadcasting` lifecycle state.
```

That is acceptable.

## Step 7: Refresh codex-spec.zip

After the clarified semantics are reflected, replace the old zip.

Suggested commands:

```bash
rm -f codex-spec.zip
zip -r codex-spec.zip codex-spec -x "*/.DS_Store" "__MACOSX/*"
```

Then run:

```bash
git status
git diff --stat
```

## Step 8: Commit the Spec Changes

Before opening Codex, commit the cleaned spec.

Suggested commands:

```bash
git status
git add codex-spec
git commit -m "Refine demo prototype specification scope"
```

If codex-spec.zip is only an export artifact, do not commit it unless intentionally tracking zip snapshots.

If excluding the zip:

```bash
git restore --staged codex-spec.zip
git checkout -- codex-spec.zip
```

Then commit only the source documents.

## Step 9: Open Codex for a Read-Only First Task

Do not ask Codex to code first.

First Codex task should be read-only.

Prompt:

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

# Current Status Summary

The main surgical cleanup is substantially complete, but the clarified NullSteward and rhythm-gap semantics must still be reflected before refreshing the zip.

Completed:

1. Prototype scope
2. Authority order
3. Spec index
4. Event-record unification
5. Data model
6. State machine table
7. State machine
8. Core sequence initial cleanup
9. Inscription machine
10. Authority consumption
11. Restart rules
12. Persistence
13. API spec
14. API state shapes
15. Settlement initial cleanup
16. Bidding admission
17. Chain interaction
18. Error taxonomy
19. Configuration reference

Remaining before Codex:

1. Reflect clarified rhythm-gap, NullSteward, and ambiguity semantics
2. Review INVARIANTS.md
3. Review TRANSITION-INVARIANTS.md
4. Review WALLET-SPEC.md
5. Review INSCRIPTION-FORMAT.md
6. Consider adding core/NULLSTEWARD.md
7. Create IMPLEMENTATION-SLICE-01.md
8. Refresh codex-spec.zip
9. Commit spec changes
10. Open Codex for read-only review
