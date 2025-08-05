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


# GROK: Follow-up Answers to Five Questions

These responses build upon the initial GROKNOTES.md and align with the scoped "MVP+" plan (v0.1.1), which introduces real Bitcoin Testnet functionality while preserving minimal CLI-only structure before Alpha.

---

## 1. MVP Testing – Prioritization

Let’s **prioritize wallet integration first**. Connecting to a real Bitcoin Testnet wallet is essential for validating actual inscriptions and building a trustworthy pipeline. Once that’s working, we can revisit auction edge cases like no-bid or simultaneous-bid resolution logic. These will be addressed as part of v0.2.x (Alpha).

---

## 2. Wallet Integration – RPC Snippet Request

Yes, please provide a **Rust code snippet using the `bitcoincore-rpc` crate** that demonstrates:

* Connecting to Bitcoin Core Testnet via RPC
* Generating a new address (`getnewaddress`)
* Listing unspent outputs (`listunspent`)
* Optional: Funding that address via `sendtoaddress` for manual testing

The snippet will be integrated into `main.rs` as part of **v0.1.1 (MVP+)**, then modularized in Alpha. Secure error handling and clear logging are preferred.

---

## 3. Inscription Simulation – Index Type

We will **start with a lightweight JSON index** for inscriptions in **v0.1.1 (MVP+)**. This keeps the system transparent and easy to inspect.

Example structure:

```json
{
  "number": 17,
  "txid": "abc123...",
  "satpoint": "txid:vout:offset",
  "timestamp": "2025-08-03T15:00:00Z",
  "bidder": "test_btc_address"
}
```

We plan to transition to a SQLite index for querying and performance optimization in **v0.3.x (Beta)**.

---

## 4. Performance Metrics – Stress Testing

Stress testing is planned for **v0.2.x (Alpha)** and **v0.3.x (Beta)**. We’d like to set up a **basic stress test script** (Rust or Python) to simulate multiple bidders. Priorities:

* Simulate 5–10 concurrent bids
* Focus on final 10 seconds of auction
* Log bid timestamp, latency, and result
* CLI-only for now, with optional Tokio or Rayon use

This will guide performance tuning and modular refactoring.

---

## 5. Documentation Roadmap – API & Timeline Draft

Yes, please generate a draft API spec for internal use with endpoints like:

* `POST /start` – Start a new auction
* `POST /bid` – Submit a bid (`number`, `amount`, `bidder`)
* `GET /status` – View current auction state
* `POST /inscribe` – Manually trigger inscription (dev-only)

Update `README.md` with the following roadmap:

* `v0.1.0`: MVP – CLI, simulated logic ✅
* `v0.1.1`: MVP+ – Real Testnet wallet, satpoints, JSON index
* `v0.2.x`: Alpha – Payments, stress testing, minimal web UI
* `v0.3.x`: Beta – Multi-user load testing, finalize SQLite
* `v1.0.0`: Mainnet – Public auctions, secure settlement

Each phase preserves conceptual coherence and prepares for a secure, extensible product launch.

---

## Last Updated
2025-08-03
