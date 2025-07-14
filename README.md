# Numbers

Inscribing numbers to Bitcoin. One at a time.

## How it works

* Start an auction
* Take bids in real time
* Pick a winner
* Inscribe a number to Bitcoin testnet
* Save the result
* Go again

## Requirements

* macOS
* Rust
* Bitcoin Core (testnet)

## Run it

```sh
bitcoind -testnet -datadir=/Volumes/DannyGoldfield-WorkDrive/bitcoin-testnet

bitcoin-cli -testnet -rpcuser=Numbers -rpcpassword=auction123 getblockchaininfo

export RPC_URL=http://127.0.0.1:18332
export RPC_USER=Numbers
export RPC_PASS=auction123

cd ~/Github/Numbers
cargo run
```

> Note: You need a local Bitcoin Core node running in testnet mode. This repo connects to a local RPC port.

## Files

* `src/main.rs`: auction logic
* `results/`: output JSON
* `WHY.md`: why this exists

## Modules in development

* `auction`: bidding and timing
* `inscribe`: write to sats
* `settle`: handle bid logic
* `view`: show what’s happening

Keep it minimal. Document as you go.

## License

MIT
