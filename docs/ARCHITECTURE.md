# ARCHITECTURE.md

This document defines normative constraints for Numbers.

This document describes the system shape of Numbers. It is implementation-oriented. It does not justify the project. See `WHY.md` for rationale.

## System Summary

Numbers runs sequential, fixed-duration auctions. Each auction resolves exactly once. Each auction number ultimately produces exactly one inscription. The system advances to the next number without retry.

The interface exists to expose auction state and timing. It does not interpret outcomes or add meaning.

## Timing Policy

Parameters:

- Auction duration: fixed (target value: 12:34:56).
- Inter-auction gap: configurable (default: 0; optional: 1:23).

Notes:

- A zero gap is valid. The next auction may open immediately after the prior auction closes.
- If configured, the 1:23 gap provides a brief boundary between auctions without affecting settlement.
- Settlement does not block auction progression.


## Lifecycle

For auction number N:

1. Open
   - Auction N opens for the configured duration.
   - Bids are accepted until close.

2. Close (single resolution)
   - At close, the auction resolves once.
   - The system records a resolution record: winner (if any), winning bid (if any), and timestamps.
   - The system advances to auction N+1 (immediately or after configured gap).

3. Settlement (asynchronous)
   - Settlement runs in parallel with later auctions.
   - Settlement has a deadline (generous by design).
   - If settlement succeeds before the deadline, N is finalized to the winner.
   - If there are no bids, or if settlement fails or times out, N is finalized to the Null Steward destination.

4. Inscription (exactly one per auction number)
   - After finalization (winner or Null Steward), an inscription transaction is constructed and broadcast.
   - Inscription content is the number only.
   - Resulting txid and satpoint are recorded.

This design allows:
- zero break between auctions
- generous settlement windows
- one inscription per auction number
- no re-auction and no retries of the same number

## Resolution Outcomes

Each auction resolves exactly once and finalizes to exactly one destination.

- Winner settles before deadline: destination is the winner.
- Winner does not settle by deadline: destination is the Null Steward.
- No bids: destination is the Null Steward.

Nonpayment and no-bid outcomes are treated as normal. They do not pause the sequence and should not be amplified as events.

## Components

### Auction Engine

Responsibilities:

- Maintain the current auction number (monotonic, sequential)
- Track open and close times
- Accept bids during the open window
- Resolve once at close and record the resolution
- Advance to the next number immediately or after configured gap

Non-responsibilities:

- Bitcoin transaction construction
- Fee estimation policy
- UI rendering

### Settlement

Responsibilities:

- Enforce settlement deadline policy
- Verify payment (or settlement condition) for a winner
- Finalize destination: winner or Null Steward
- Emit a finalization record used by Inscription

Non-responsibilities:

- Auction winner selection
- UI behavior beyond exposing settlement state

### Inscription Builder and Broadcaster

Responsibilities:

- Construct the inscription payload (the number only)
- Select inputs and bind to satpoints according to wallet policy
- Construct and broadcast the transaction
- Return txid and satpoint reference
- Persist the inscription result

Non-responsibilities:

- Deciding winner or destination
- Recording bid history beyond what is needed for traceability

### Wallet and UTXO Inventory

Responsibilities:

- Provide addresses and signing
- Track spendable UTXOs used for inscriptions
- Reserve and release UTXOs for pending inscriptions
- Prevent double-spends across concurrent pending inscriptions

Notes:

- Because settlement can lag behind auctions, multiple auctions can be pending inscription simultaneously.
- The wallet must be able to reserve enough UTXOs to support this backlog.

### Storage and Index

Responsibilities:

- Persist auction records and resolutions
- Persist settlement state and finalization outcome
- Persist inscription results (txid, satpoint, destination)
- Provide read access for UI and inspection

Targets:

- Start lightweight (flat file or SQLite)
- Optimize for correctness and traceability over performance

### Interface

Responsibilities:

- Expose auction state: current number, time remaining, current high bid, status
- Expose outcome state: resolved, awaiting settlement, finalized, inscribed
- Submit bids through a controlled path

Non-responsibilities:

- Interpretation, traits, rarity, or styling semantics
- Any additional meaning beyond what the viewer’s environment renders

## State Model

Each auction number N progresses through discrete states.

Ephemeral (in-memory):

- Current auction number
- Open auction timer
- Current high bid and bidder reference
- UI-facing status

Durable (persisted, minimum set per auction number):

- Auction number
- Open timestamp, close timestamp
- Winning bid amount (or none)
- Winner identifier (or none)
- Settlement status: pending, settled, expired, none (no bids)
- Final destination: winner or Null Steward (once finalized)
- Inscription status: pending, broadcast, confirmed (optional), failed
- txid (once broadcast)
- satpoint (once known)

## Fee Policy

Fee policy is destination-aware.

- Winner-destination inscriptions:
  - Use normal fee estimation for timely confirmation.
  - Prefer predictable confirmation over minimal cost.

- Null Steward destination inscriptions:
  - Use the lowest fee rate that is still expected to confirm within a reasonable window.
  - Prefer economy confirmation. Do not pay for speed.

Safety rule:
- If a low-fee Null Steward transaction remains unconfirmed beyond a maximum waiting window, allow a fee bump mechanism (RBF or CPFP) according to wallet capability. This is not a retry of the auction. It is completion of an already-finalized inscription.

The goal is low cost without creating permanent limbo.

## Invariants

- Auctions progress strictly in order
- Auction duration is fixed by configuration
- Inter-auction gap is fixed by configuration (may be zero)
- Each auction resolves exactly once
- Each auction number produces exactly one inscription (eventually)
- Inscription content is the number only
- Settlement does not block the sequence
- No re-auction and no retry of the same number

## Operational Modes

Numbers supports distinct operational modes to allow maintenance and recovery without altering auction semantics.

### Normal

- Auctions run sequentially according to the configured timing policy.
- Bids are accepted.
- Settlement and inscription proceed as usual.

### Degraded

- Auction timing and progression continue.
- Bid intake may be temporarily disabled.
- Settlement and inscription may be delayed.
- UI must clearly indicate degraded status.

Degraded mode is used to absorb transient failures (RPC instability, node issues) without stopping the sequence.

### Maintenance Hold (Optional)

- A maintenance hold may delay the start of the next auction.
- A hold may only occur between auctions, never during an open auction.
- Holds do not alter past outcomes or reopen resolved auctions.
- Hold state must be durably recorded and visible.

The goal of maintenance hold is operational safety, not outcome control.

## Restart Semantics

The system must be restart-safe.

On restart:

- The system reconstructs state from durable storage.
- If an auction was open, it is treated as closed at restart time and resolved once according to policy.
- Pending settlements resume with their remaining time.
- Pending inscriptions resume broadcast/confirmation tracking.

A restart must not create a second resolution for the same number.

## Failure Handling

Principle: absorb failures without changing semantics.

- RPC unavailable:
  - UI may degrade.
  - Auction timing continues.
  - Record boundary failures explicitly.

- Settlement verification failure:
  - Do not pause the sequence.
  - Keep settlement pending until deadline or until verification succeeds.

- Broadcast failure:
  - Record failure state.
  - Continue attempting broadcast for the already-finalized destination.
  - Do not reopen or re-resolve the auction.

- Storage failure:
  - Fail fast if persistence cannot be guaranteed.

Avoid recovery logic that introduces alternate timelines for a given number.

## Rendering

The inscription content is the number itself.

When viewed, the number is rendered using the viewer’s default system font and text settings. No styling, layout, or presentation is specified at inscription time. Rendering is determined entirely by the viewing environment.

## UI States to Design For

Minimum states:

- Open auction
- Resolving (close occurred, resolution being recorded)
- Awaiting settlement (winner has time to pay)
- Finalized (destination determined)
- Inscribing (tx construction/broadcast)
- Inscribed (txid and satpoint visible)
- Degraded (RPC down or unknown chain state)

The UI must not claim final ownership before finalization and inscription are complete.
