# Platform — Numbers

This document defines where each part of the Numbers system runs.

It describes deployment boundaries, execution environments, and trust separation.
It does not define system behavior or semantics.

---

## Design Goal

Numbers is intentionally composed of a small number of clearly separated components.

Each component:
- Has a single responsibility
- Runs in a defined environment
- Fails independently

Isolation is a feature, not an optimization.

---

## Environments

Numbers operates in three isolated environments:

- **Local** (development)
- **Testnet**
- **Mainnet**

Each environment is fully isolated.

There is:
- No shared state
- No shared databases
- No shared keys
- No shared addresses
- No shared inscriptions

Artifacts from one environment must never appear in another.

---

## Components

### Static Website

**Purpose**
- Expose live and historical system state
- Allow bid submission when auctions are open

**Characteristics**
- Static files only
- No backend logic
- No secrets
- No private keys
- No signing capability

**Hosting**
- TBD (e.g. GitHub Pages, Cloudflare Pages, Vercel)

The website is a pure interface layer.
It does not define, interpret, or enforce system rules.

---

### Numbers Backend

**Purpose**
- Maintain auction state
- Validate bids
- Orchestrate settlement
- Coordinate inscription creation
- Persist system records

**Characteristics**
- Long-lived service
- Owns all persistent storage
- Owns all signing keys required for canonical inscriptions
- Implements system behavior as defined in `CORE-SEQUENCE.md` and `ARCHITECTURE.md`

The backend:
- Is authoritative for system procedure
- Is not authoritative for meaning or ownership
- Does not override on-chain outcomes

---

### Bitcoin Core Node

**Purpose**
- Provide access to Bitcoin consensus data
- Construct and broadcast transactions
- Verify chain state

**Characteristics**
- Full node
- Dedicated instance
- Not shared with other applications

**Responsibilities**
- Transaction creation
- Fee estimation
- Chain state verification

Bitcoin Core is the only component that directly interfaces with the Bitcoin network.

---

## Networking

Communication paths are strictly constrained:

- Website → Backend: HTTPS API
- Backend → Bitcoin Core: RPC
- Website → Bitcoin Core: **never**

There is no direct browser-to-node communication.

---

## Failure Isolation

Each component is allowed to fail independently:

- The website may be unavailable while auctions continue
- The backend may pause while Bitcoin continues
- The Bitcoin node may restart without corrupting system state

Failures do not alter system semantics.

State is preserved.
The sequence continues.

---

## Non-Goals

The platform explicitly does not:

- Share infrastructure across environments
- Embed logic in the frontend
- Allow direct client access to Bitcoin Core
- Optimize for minimal hosting cost at the expense of clarity

Operational simplicity is prioritized over consolidation.

---

## Summary

The Numbers platform is deliberately boring.

Few components.
Clear boundaries.
Explicit authority.

The system works because each part knows
what it is responsible for
and what it is not.
