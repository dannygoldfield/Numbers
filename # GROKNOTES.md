# GROK NOTES

This document contains answers to Grok system queries for consistent project context and planning.

---

## 1. MVP Testing

The MVP (`main.rs`) has been tested successfully using stdin to simulate bidding during a 56-second auction. Key modules like `Auction` and `Inscribe` are functional in simulation. Wallet, Payment, and Settle are currently placeholders.

**Known gaps:**
- No-bid and simultaneous-bid edge cases need logic handling.
- Simulated bids only—no RPC wallet integration yet.
- No persistent data store beyond basic JSON.

---

## 2. Wallet Integration

Wallet support is minimal in v0.1.0. In Alpha (v0.2.x), we plan to:
- Connect to Bitcoin Core Testnet via RPC
- Generate new addresses and fund them
- Extract UTXO and satpoint data for inscription
- Monitor mempool and confirm payments

**Assistance needed:** secure RPC calls in Rust, error handling, and key management for testnet.

---

## 3. Inscription Simulation

Current inscriptions are mock writes. We're transitioning to real Testnet inscriptions using:
- UTXO and satpoint extraction via RPC
- Custom lightweight index (JSON or SQLite)
- Direct handling of satpoint targeting

**No use of `ord`, no sat rarity or heuristics.** We want transparent, manual control for now.

---

## 4. Performance Metrics (Beta Goals)

In Beta (v0.3.x), our focus is on:
- 5–10 concurrent bidders per auction
- Sub-second bid-to-update latency
- Logging all bids, transitions, and auction states
- Resource monitoring (CPU, memory, I/O)

Simulated users and basic stress testing will be used during QA.

---

## 5. Documentation Roadmap

We plan to expand `README.md` and `WHY.md` with:
- Internal API endpoint documentation:
  - `POST /start`
  - `POST /bid`
  - `GET /status`
  - `POST /inscribe`
- Timeline for phases:
  - MVP (v0.1.0): CLI auction + mock inscribe ✅
  - Alpha (v0.2.x): Testnet RPC + real inscribe
  - Beta (v0.3.x): Live bidder testing + performance focus
  - Mainnet (v1.0.0): Public launch + web interface
- WHY.md clarification:
  - Starting at 1 (not 0) as core philosophy
  - Numbers as ownable artifacts, not tokens
  - No rarity. No gamification. Just clarity.

---

## Last Updated
2025-08-03
