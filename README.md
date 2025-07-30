# Numbers

Inscribing numbers on Bitcoin, one at a time.

## How it Works

1. **Start Auction**: Auction begins with the first bid.
2. **Take Bids**: Real-time bidding via command line, 60-second duration for testing.
3. **Pick Winner**: Highest bid after timer expires.
4. **Inscribe Number**: Inscribe on a random satoshi using Bitcoin Testnet.
5. **Settle Transaction**: Handle payments by the winner and transfer the inscription.
6. **Save Result**: Log metadata in JSON.
7. **Repeat**: Next number, maintaining sequence.

## Requirements

- macOS
- Rust
- Bitcoin Core (Testnet)

## Run It

```sh
bitcoind -testnet -datadir=/Volumes/DannyGoldfield-WorkDrive/bitcoin-testnet

bitcoin-cli -testnet -rpcuser=Numbers -rpcpassword=auction123 getblockchaininfo

export RPC_URL=http://127.0.0.1:18332
export RPC_USER=Numbers
export RPC_PASS=auction123

cd ~/Github/Numbers
cargo run
```

> Note: Requires local Bitcoin Core node in Testnet mode.

## Files

- **`src/main.rs`**: Auction logic, bidding, timing, simulated inscriptions.
- **`results/inscription_index.json`**: Logs number, address, mock txid, winner.
- **`WHY.md`**: Project rationale.
- **`README.md`**: This document.

## Modules

The MVP consists of five integrated modules:

- **`auction`**: Manages bidding and timing, ensuring a 60-second duration for testing and sequential progression. Currently handles real-time bids via command line.
- **`inscribe`**: Handles writing numbers to random satoshis, currently simulated for prototyping. Plans to transition to real Bitcoin Testnet inscriptions.
- **`settle`**: Handles transactions, including payments by the winner and transfer of the inscription. Currently simulated but will include real transactions in the next version.
- **`view`**: A website to show what’s happening, providing a user-friendly interface for viewing auctions and inscriptions. This module is under development and will be integrated with the other modules for a cohesive user experience.
- **`payment`**: Manages payment processing, handling transfers of Bitcoin from winners to the project and ensuring inscriptions are transferred to winners' addresses. Currently simulated but critical for the MVP's functionality.

## Development Plan

### MVP Goals
- **Functional Auction**: Start with bid, 60-second timer, pick winner.
- **Simulated Inscriptions**: Mock txids, random satoshis for prototyping.
- **Simulated Settlements**: Mock transactions for payments and inscription transfers.
- **Simulated Payments**: Mock payment processing for winners, ensuring transfer logic is in place.
- **JSON Logging**: Traceability without database overhead.
- **Traditional Approach**: Clear, modular code for easy collaboration.
- **Website View**: Basic web interface to display auction status and results.

### Next Version
- **Real Inscriptions**: Transition to Bitcoin Testnet inscriptions.
- **Real Settlements**: Implement actual transactions for payments and inscription transfers.
- **Real Payments**: Implement actual Bitcoin transfers for payment processing.
- **SQLite Database**: Replace JSON for scalability.
- **Enhanced Website**: Fully functional `view` module for user testing.
- **User Testing**: Viable for feedback and collaboration.

## Contributing

We welcome contributions. Fork the repo, make changes, and submit a pull request. For major changes, open an issue first. This project aims to be an MVP for soliciting feedback and potential collaboration with another developer.

## License

MIT