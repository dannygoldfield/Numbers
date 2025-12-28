# Numbers

## Overview

Numbers is a system that auctions numbers sequentially.

Each auction result produces a canonical inscription on Bitcoin.

The inscription content is the number itself.

---

## Semantics

Numbers is defined by fixed, non-negotiable semantics.

These semantics are enforced by the system and documented in the project specifications. They do not change across environments or stages of development.

---

## Project Structure

Numbers is designed to run in multiple environments:

- Prototype
- Testnet
- Mainnet

Environments differ by infrastructure only. 
Auction behavior and invariants remain constant.

---

## Documentation

Canonical system behavior and constraints are defined in the `docs/` directory:

- `docs/PRD.md`  
  Non-negotiable system invariants.

- `docs/CORE-SEQUENCE.md`  
  The auction and inscription sequence.

- `docs/ARCHITECTURE.md`  
  System structure and component responsibilities.

- `docs/WEBSITE-PRD.md`  
  UI/UX requirements for the Numbers website.

- `docs/WHY.md`  
  Rationale and motivation.

See `docs/README.md` for a full documentation index.

---

## Development

Numbers currently operates on Bitcoin Testnet.

Setup and execution instructions are provided in:

- `docs/DEV.md`

---

## License

MIT
