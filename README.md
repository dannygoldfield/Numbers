# Numbers 

### **What if you could own a number?**

🕐 Estimated read time: 5 minutes

## Table of Contents

1. Overview
2. Product Roadmap

   1. MVP: Minimum Viable Product (v0.1.0)
   2. Alpha Prototype (v0.2.x)
   3. Beta Prototype (v0.3.x)
   4. Product Launch (v1.0.0)
3. Artwork Description
4. Current Requirements for MVP
5. Setup and Run
6. Project Structure
7. Contributing
8. License
9. Resources

## Overview

**Numbers** is an experimental auction system that inscribes numbers, one at a time (1, 2, 3, and so on), onto individual satoshis on Bitcoin. The project is being developed in stages, starting with a demo-ready MVP, followed by Testnet prototypes and culminating in a public Mainnet launch.

It explores digital ownership, permanence, and value by creating a new asset: the number itself.

1. Real-time auctions with countdown timers.
2. Bitcoin inscriptions for immutable ownership.
3. Modular architecture for scalability and testing.
4. Open-source under MIT license, inviting community contributions.

## Product Roadmap

### MVP: Minimum Viable Product (v0.1.0)

A demo-ready model showcasing six modular components.

**Components:**

1. Wallet: Simulates wallet connection; users enter a name (e.g., "Danny’s Wallet") tied to their bid.
2. Auction: Accepts bids via a web interface with a 56 second countdown starting on the first bid.
3. Inscribe: Simulates inscribing the winning number onto a random satoshi (mock txid).
4. 💰 **Payment**: Simulates Bitcoin payment from the winner to the project.
5. Settle: Simulates transferring the inscription to the winner.
6. Website: Displays auction status, countdown, winner, and results.

**Demo Flow:**

1. Connect a simulated wallet and place a bid, starting the auction timer.
2. Announce the winner and record a mock inscription at auction close.
3. Save results to `results/inscription_index.json`.

### Alpha Prototype (v0.2.x)

A working prototype using Bitcoin Testnet for limited, supervised user testing.

**Components:**

1. Wallet: Connects real Testnet wallets for bidding.
2. Auction: Supports live Testnet bidding with countdown.
3. Inscribe: Uses mock or real Testnet inscriptions.
4. 💰 **Payment**: Simulates Testnet payments.
5. Settle: Transfers inscriptions on Testnet.
6. Website: Supports multi-user interaction and feedback logging.

**Goals:**

1. Conduct small-group tests with trusted users.
2. Debug and refine modules after each session.
3. Iterate multiple times based on expert/user feedback.
4. Monitor transaction and performance behavior.
5. Improve modular stability for Beta phase.

### Beta Prototype (v0.3.x)

An unsupervised testing phase with trusted external users on Bitcoin Testnet.

**Goals:**

1. Improve scalability and performance.
2. Enable broader feedback loop with clearer onboarding.
3. Simulate live auction behavior with transparent logging.
4. Confirm all Testnet operations are reproducible.
5. Prepare for mainnet migration.

### Product Launch (v1.0.0)

Transitions to a public Bitcoin mainnet product.

**Production Modules:**

1. Wallet: Enables secure connection of real Mainnet wallets for bidding and receiving inscriptions.
2. Auction: Hardened for uptime and public access, managing live bidding with dynamic updates and countdown.
3. Inscribe: Creates real inscriptions on Bitcoin mainnet, inscribing the winning number onto a satoshi.
4. 💰 **Payment**: Processes real Bitcoin payments from winners with transaction confirmation.
5. Settle: Ensures secure inscription transfers to winners with full logging.
6. Website: Scalable, public-facing web interface with support for displaying auction status, results, and inscriptions.

**Infrastructure:**

1. Migrate from JSON to a database (e.g., SQLite or Postgres).
2. Implement monitoring for wallets, inscriptions, and auctions.
3. Use a stable off-site server to maintain the project Bitcoin Node and Ord Index.
4. Provide comprehensive documentation and onboarding.
5. Launch publicly and engage collectors.

## Artwork Description

The number, inscribed as a Bitcoin Ordinal, displays in the user’s system default font, inheriting the native type environment. With minimal styling, it remains a conceptual form universally rendered, locally interpreted.

## 🛠️ Current Requirements for MVP

1. **OS**: macOS (Linux/Windows support planned)
2. **Language**: Rust (v1.67 or later)
3. **Bitcoin**: Bitcoin Core (Testnet mode)
4. **Storage**: \~500 GB for Testnet blockchain (local external drive recommended)

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
│   └── main.rs             # Auction logic, bidding, and simulated inscriptions
├── results/
│   └── inscription_index.json  # Logs number, address, mock txid, winner
├── WHY.md                 # Project rationale
└── README.md              # This document
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
3. Project Rationale (`WHY.md`)
4. GitHub Issues
