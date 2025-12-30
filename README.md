# Numbers

## What This Repository Is

This repository defines the specification, constraints, and operational rules of the Numbers system.

It is the reference definition of system behavior.
It is not a product roadmap, marketplace, or general-purpose auction framework.

Implementation details may evolve.
The semantics defined here do not.

---

## Overview

Numbers is a system that auctions numbers sequentially.

Each auction resolves exactly once.
Each resolution produces a canonical inscription on Bitcoin.
The inscription content is the number itself.

The system records outcomes.
It does not interpret meaning.

---

## Authorship and Operation

Numbers is authored and operated by Danny Goldfield,
operating under the system identity **123456789 and 0**.

This repository defines the reference specification.
Running code is expected to conform to it.

---

## Semantics

Numbers is defined by fixed, non-negotiable semantics.

These semantics are enforced by the system and documented in the specifications.
They do not change across environments or stages of development.

If an implementation violates these semantics, it is not Numbers.

---

## Project Structure

Numbers operates across multiple environments:

- Prototype
- Testnet
- Mainnet

Environments differ by infrastructure only.
Auction behavior, sequencing, and invariants remain constant.

---

## Documentation

Canonical system behavior is defined in `docs/`:

- `PRD.md`  
  System invariants and non-goals.

- `CORE-SEQUENCE.md`  
  The auction, settlement, and inscription sequence.

- `ARCHITECTURE.md`  
  System structure and authority boundaries.

- `WEBSITE-PRD.md`  
  Constraints for the public interface.

- `WHY.md`  
  Conceptual framing.

See `docs/README.md` for the full documentation index.

---

## Development Status

Numbers currently operates on Bitcoin Testnet.

Development and execution details are intentionally scoped.
This repository prioritizes correctness and definition over velocity.

---

## License

MIT
