# DEV.md

This document describes how to run Numbers locally for development and testing on Bitcoin Testnet. The earlier prototype that runs without Bitcoin Core is not documented here.

It assumes familiarity with Rust and Bitcoin Core. It does not describe system behavior or design rationale. See `README.md` and `ARCHITECTURE.md` for that.

## Requirements

- macOS or Linux
- Rust 1.67 or later
- Bitcoin Core (with Testnet enabled)
- Sufficient disk space for the Testnet blockchain (hundreds of GB)

Windows is not currently supported.

## Bitcoin Core (Testnet)

Numbers requires a local Bitcoin Core node running in Testnet mode.

### Start Bitcoin Core

```bash
bitcoind -testnet -datadir=/path/to/bitcoin-testnet
```

Use a dedicated data directory to avoid interfering with Mainnet data.

### Verify Sync Status

```bash
bitcoin-cli -testnet getblockchaininfo
```

Wait until `initialblockdownload` is `false` before running Numbers.

## RPC Configuration

Numbers communicates with Bitcoin Core via RPC.

Set the following environment variables:

```bash
export RPC_URL=http://127.0.0.1:18332
export RPC_USER=<rpc-username>
export RPC_PASS=<rpc-password>
```

These values must match the RPC configuration used by your Bitcoin Core node.

## Running Numbers

Clone the repository:

```bash
git clone https://github.com/<your-username>/Numbers.git
cd Numbers
```

Run the application:

```bash
cargo run
```

The process starts the auction loop and blocks while running.

Auction timing and state are logged to stdout.

## Output and State

Local outputs are written to the `results/` directory.

Current artifacts may include:

- Auction outcomes
- Inscription metadata
- Transaction IDs
- Satpoint references

These files are for development and inspection only. Formats may change.

## Common Issues

### Bitcoin Core Not Synced

**Symptoms**
- RPC calls hang or fail
- No UTXOs available

**Resolution**
- Verify Testnet sync status
- Confirm correct `datadir`

### RPC Authentication Errors

**Symptoms**
- `401 Unauthorized`
- Connection refused

**Resolution**
- Check `RPC_USER` and `RPC_PASS`
- Confirm RPC port and URL
- Ensure Bitcoin Core allows RPC connections

### Insufficient Disk Space

**Symptoms**
- Bitcoin Core crashes or fails to sync

**Resolution**
- Allocate additional disk space
- Move Testnet datadir to an external drive

## Scope Notes

- This setup is for development and demonstration only.
- Payment handling may be simulated or incomplete.
- UI components, if present, are for observability, not production use.
- No backward compatibility is guaranteed between commits.

## Stopping the Process

Terminate the running process with:

```bash
Ctrl+C
```

Bitcoin Core can be stopped separately if desired.
