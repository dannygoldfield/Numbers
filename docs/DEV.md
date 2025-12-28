# Development# Development Setup (Testnet)

This document describes how to run Numbers locally for development and testing on Bitcoin Testnet.

It assumes familiarity with Rust and Bitcoin Core.  
It does not describe system behavior or design rationale.  
See `README.md` and `ARCHITECTURE.md` for those.

The earlier prototype that runs without Bitcoin Core is intentionally not documented here.

---

## Requirements

- macOS or Linux
- Rust 1.67 or later
- Bitcoin Core (Testnet enabled)
- Sufficient disk space for the Testnet blockchain

Windows is not currently supported.

---

## Bitcoin Core (Testnet)

Numbers requires a local Bitcoin Core node running in Testnet mode.

### Start Bitcoin Core

```bash
bitcoind -testnet -datadir=/path/to/bitcoin-testnet
