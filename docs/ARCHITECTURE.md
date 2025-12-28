# Architecture

This document describes the structure and constraints of Numbers. It is implementation-oriented and procedural. It does not justify the system. See `WHAT-IF.md` for the premise.

## System Summary

Numbers runs sequential auctions of numbers and advances monotonically from one number to the next. The interface exposes auction state and timing only. It does not interpret outcomes or add meaning.

## Timing Policy

- **Auction duration:** fixed by configuration (default: 12:34:56)
- **Inter-auction gap:** fixed at 1:23
- The inter-auction gap provides a boundary between auctions
- Auction timing is independent of settlement state
- Settlement does not block progression

## Lifecycle

For auction number **N**:

### 1. Open
- Auction N opens for the configured duration
- Bids are accepted until close

### 2. Close
- At close, the auction resolves exactly once
- A resolution record is written: winner (if any), winning bid (if any), timestamps
- The system advances to auction N+1 after the inter-auction gap

### 3. Settlement
- Settlement runs asynchronously, in parallel with later auctions
- Settlement has a deadline
- If settlement succeeds before the deadline, N is finalized to the winner
- If settlement fails, times out, or there are no bids, N is finalized to the null steward

The null steward is a provably unspendable address.

### 4. Inscription
- After finalization, an inscription transaction is constructed and broadcast
- Inscription content is the number only
- Multiple inscriptions of the same number may exist on-chain
- An inscription is canonical only if it satisfies the Numbers authenticity rule
- The txid and satpoint are recorded in the Numbers registry

## Canonical Inscription

Numbers distinguishes its own inscriptions from arbitrary lookalike inscriptions on-chain. Anyone can inscribe the number N. Content alone does not prove it came from Numbers.

Numbers recognizes exactly one canonical inscription per auction number N, which must satisfy all of the following:

1. Auction N is finalized (winner or null steward)
2. The inscription txid and satpoint are recorded in the Numbers registry
3. The inscription satisfies the Numbers authenticity rule

### Numbers authenticity rule

The reveal path must require a valid signature from the Numbers Operator key. This proves provenance without changing what is rendered. The rendered content remains the number only. This rule applies to all canonical inscriptions, including null steward outcomes.

## Resolution Outcomes

Each auction finalizes to exactly one destination:

- Winner settles before deadline → winner
- Winner does not settle → null steward
- No bids → null steward

## Components

### Auction Engine

**Responsibilities**
- Maintain the current auction number (monotonic, sequential)
- Track open and close times
- Accept bids during the open window
- Resolve once at close and record the resolution
- Advance to the next number after the inter-auction gap

**Non-responsibilities**
- Bitcoin transaction construction
- Fee estimation
- UI rendering

### Settlement

**Responsibilities**
- Enforce settlement deadline
- Verify payment for a winning bidder
- Finalize destination: winner or null steward
- Emit a finalization record used by inscription

Finalization determines the inscription destination but does not delay subsequent auctions.

**Non-responsibilities**
- Auction winner selection
- UI behavior beyond exposing settlement state

### Inscription Builder and Broadcaster

**Responsibilities**
- Construct the inscription payload (the number only)
- Select inputs and bind to satpoints according to wallet policy
- Construct and broadcast the transaction
- Persist txid and satpoint

**Non-responsibilities**
- Deciding winner or destination
- Interpreting inscription meaning

### Wallet and UTXO Inventory

**Responsibilities**
- Provide addresses and signing
- Track spendable UTXOs used for inscriptions
- Reserve and release UTXOs for pending inscriptions
- Prevent double-spends across concurrent pending inscriptions

Notes:
- Settlement may lag behind auctions
- Multiple auctions may be pending inscription simultaneously
- The wallet must reserve sufficient UTXOs to support concurrent inscriptions

### Storage and Index

**Responsibilities**
- Persist auction records and resolutions
- Persist settlement and finalization state
- Persist inscription metadata (txid, satpoint)

**Non-responsibilities**
- Interpreting outcomes
- Enforcing auction or settlement rules
