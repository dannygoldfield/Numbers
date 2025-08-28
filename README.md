# Numbers
by 123456789 and 0

### **What if you could own a number?**

🕐 Estimated read time: 5 minutes

## Table of Contents

1. Overview
2. Product Roadmap

   1. MVP: Minimum Viable Product (v0.1.0)
   2. MVP+: Real Testnet Integration (v0.1.1)
   3. Alpha Prototype (v0.2.x)
   4. Beta Prototype (v0.3.x)
   5. Product Launch (v1.0.0)
3. Artwork Description
4. Current Requirements for MVP
5. Setup and Run
6. Project Structure
7. Contributing
8. License
9. Resources

## Overview

**Numbers** is an experimental auction system that inscribes numbers, one at a time (1, 2, 3, and so on), onto individual satoshis on Bitcoin. The project is being developed in stages, starting with a demo-ready MVP, followed by Testnet prototypes, and culminating in a public Mainnet launch.

It explores digital ownership, permanence, and value by creating a new asset: the number itself.

1. Real-time auctions with countdown timers.
2. Bitcoin inscriptions for immutable ownership.
3. Modular architecture for scalability and testing.
4. Open-source under MIT license, inviting community contributions.

## Product Roadmap

### MVP: Minimum Viable Product (v0.1.0)

A demo-ready CLI model with six simulated components.

**Components:**

1. Wallet: Simulated wallet connection via user name.
2. Auction: CLI bidding via stdin with 56-second countdown.
3. Inscribe: Simulated inscription with mock txid.
4. Payment: Simulated payment recording.
5. Settle: Simulated winner settlement.
6. Website: Planned but not included in MVP.

**Demo Flow:**

1. Enter simulated wallet name and place a bid to start the auction.
2. At countdown end, select winner and simulate inscription.
3. Save result to `results/inscription_index.json`.

### MVP+: Real Testnet Integration (v0.1.1)

A CLI-only upgrade that connects to Bitcoin Core on Testnet.

**New Features:**

1. Wallet: Generates and lists real Testnet addresses via RPC.
2. Inscribe: Binds a number to a real satpoint using UTXO data.
3. JSON Index: Records number, satpoint, txid, and bidder in flat file.
4. RPC: Uses `bitcoincore-rpc` for real blockchain interaction.

**Purpose:**

Establish end-to-end realism before expanding features. Sets foundation for modularization and future UI.

### Alpha Prototype (v0.2.x)

Adds web interface and payment simulation for user testing.

**Goals:**

1. Real Testnet wallets and bidding
2. Simulated payments and mock settlement
3. Basic web frontend for interaction
4. Stress testing with logging and RPC handling

### Beta Prototype (v0.3.x)

Focuses on performance and multi-user support.

**Goals:**

1. Concurrent bidding (5–10 simulated users)
2. Sub-second latency between bid and feedback
3. Logging of all auction activity
4. Transition from JSON to SQLite index

### Product Launch (v1.0.0)

Public Bitcoin Mainnet release with full functionality.

**Modules:**

1. Wallet: Connect and fund securely
2. Auction: Live web bidding with real-time updates
3. Inscribe: Write numbers to real Mainnet satoshis
4. Payment: Real BTC transfers with confirmations
5. Settle: Secure handoff with full audit trail
6. Website: Public UI for auctions, results, and history

**Infrastructure:**

1. SQLite or Postgres for persistence
2. Server-based RPC and node hosting
3. Monitoring and error recovery
4. Contributor and bidder onboarding guides

## Artwork Description

The number, inscribed as a Bitcoin Ordinal, displays in the user’s system default font, inheriting the native type environment. With minimal styling, it remains a conceptual form universally rendered, locally interpreted.

## 🛠️ Current Requirements for MVP

1. **OS**: macOS (Linux/Windows support planned)
2. **Language**: Rust (v1.67 or later)
3. **Bitcoin**: Bitcoin Core (Testnet mode)
4. **Storage**: \~500 GB for Testnet blockchain

## Setup and Run

1. **Run Bitcoin Core in Testnet mode:**

```bash
bitcoind -testnet -datadir=/path/to/bitcoin-testnet
```

2. **Verify blockchain sync:**

```bash
bitcoin-cli -testnet -rpcuser=Numbers -rpcpassword=auction123 getblockchaininfo
```

3. **Set environment variables:**

```bash
export RPC_URL=http://127.0.0.1:18332
export RPC_USER=Numbers
export RPC_PASS=auction123
```

4. **Clone and run Numbers:**

```bash
git clone https://github.com/your-username/Numbers.git
cd Numbers
cargo run
```

> **Note**: Requires a local Bitcoin Core node in Testnet mode. Ensure sufficient disk space for the blockchain.

## Project Structure

```text
Numbers/
├── src/
│   └── main.rs                # Auction logic, bidding, and real inscriptions (WIP)
├── results/
│   └── inscription_index.json  # Logs number, address, satpoint, winner
├── WHY.md                    # Project rationale
└── README.md                 # This document
```

## 🤝 Contributing

Help build Numbers:

1. Fork the repository.
2. Create a feature branch (`git checkout -b feature/your-feature`).
3. Commit changes (`git commit -m "Add your feature"`).
4. Push to your fork (`git push origin feature/your-feature`).
5. Open a pull request.

For major changes, please open an issue first to discuss. Check out our CONTRIBUTING.md for guidelines.

## 📄 License

MIT License — see `LICENSE` file for details.

## 🔗 Resources

1. Bitcoin Testnet
2. Rust Documentation
3. Project Rationale [WHY.md](./docs/WHY.md)
4. GitHub Issues
