# Platform

This document defines where each part of the Numbers system runs.

Numbers is designed as a small number of clearly separated components.

---

## Environments

- Local (development)
- Testnet
- Mainnet

Each environment is isolated.
No shared state, keys, addresses, or inscriptions across environments.

---

## Components

### Static Website
- Purpose: User interface
- Type: Static files
- Hosted on: TBD (GitHub Pages / Cloudflare Pages / Vercel)
- No secrets
- No backend logic

### Numbers Backend
- Purpose:
  - Auction state
  - Bid validation
  - Settlement orchestration
  - Inscription coordination
- Runs as a long-lived service
- Talks to Bitcoin Core
- Owns the database
- Owns all signing keys required for canonical inscriptions

### Bitcoin Core Node
- Full node
- Required for:
  - Transaction creation
  - Fee estimation
  - Chain state verification
- Dedicated instance
- Not shared with other applications

---

## Networking

- Website talks to backend via HTTPS API
- Backend talks to Bitcoin Core via RPC
- No direct browser-to-node communication

---

## Design Principle

The website can fail independently.
The backend can pause independently.
The node can restart independently.

Isolation is a feature.
