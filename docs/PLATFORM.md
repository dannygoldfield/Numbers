# Platform — Numbers

This document defines **where each part of the Numbers system runs**.

It describes:
- deployment boundaries
- execution environments
- trust separation
- communication paths

It does **not** define:
- system behavior
- lifecycle sequencing
- authority rules
- state transitions
- auction or inscription semantics

Those concerns are defined exclusively in:
- ARCHITECTURE.md
- STATE-MACHINE.md
- CORE-SEQUENCE.md

This document is **descriptive**, not normative.

If there is a conflict, behavioral documents take precedence.

---

## Design Goal

Numbers is intentionally composed of a small number of clearly separated components.

Each component:

- has a single responsibility
- runs in a defined execution environment
- fails independently
- does not interpret the output of other components

Isolation is a feature, not an optimization.

---

## Execution Environments

Numbers operates in three fully isolated environments:

1. Local (development)
2. Testnet
3. Mainnet

Each environment is isolated by design.

There is **no sharing** across environments of:

- state
- databases
- keys
- addresses
- inscriptions
- transaction history
- derived artifacts

Artifacts from one environment must never appear in another.

Environment boundaries are absolute.

---

## Components

### 1. Static Website

#### Purpose

- Expose live and historical system state
- Allow bid submission when auctions are open

#### Characteristics

- Static files only
- No backend logic
- No secrets
- No private keys
- No signing capability

#### Hosting

Examples include:
- GitHub Pages
- Cloudflare Pages
- Vercel

Hosting choice does not affect system semantics.

#### Authority

The website is a pure interface layer.

It does **not**:
- define rules
- interpret outcomes
- enforce constraints
- modify system state

---

### 2. Numbers Backend

#### Purpose

- Maintain auction state
- Validate bids
- Orchestrate settlement
- Coordinate inscription creation
- Persist system records

#### Characteristics

- Long-lived service
- Owns all persistent storage
- Owns all signing keys required for canonical inscriptions
- Implements system behavior as defined elsewhere

The backend implements behavior defined in:
- CORE-SEQUENCE.md
- STATE-MACHINE.md
- ARCHITECTURE.md

#### Authority

The backend:

- is authoritative for **system procedure**
- is not authoritative for meaning or ownership
- does not override on-chain outcomes
- does not invent or repair history

It executes rules.
It does not interpret results.

---

### 3. Bitcoin Core Node

#### Purpose

- Provide access to Bitcoin consensus data
- Construct and broadcast transactions
- Verify chain state

#### Characteristics

- Full Bitcoin node
- Dedicated instance
- Not shared with other applications

#### Responsibilities

- Transaction creation
- Fee estimation
- Chain state verification

Bitcoin Core is the **only** component that directly interfaces with the Bitcoin network.

It is treated as an external source of record.
It is observed, not trusted absolutely.

---

## Networking and Communication

Communication paths are strictly constrained:

- Website → Backend: HTTPS API
- Backend → Bitcoin Core: RPC
- Website → Bitcoin Core: **never**

There is no direct browser-to-node communication.

All authority-bearing actions occur behind the backend boundary.

---

## Failure Isolation

Each component is allowed to fail independently:

- The website may be unavailable while auctions continue
- The backend may pause while Bitcoin continues
- The Bitcoin node may restart without corrupting system state

Failures do **not** alter system semantics.

State is preserved.
The sequence continues when execution resumes.

---

## Non-Goals

The platform explicitly does **not**:

- share infrastructure across environments
- embed logic in the frontend
- allow direct client access to Bitcoin Core
- collapse components for convenience
- optimize for minimal hosting cost at the expense of clarity

Operational simplicity is prioritized over consolidation.

---

## Summary

The Numbers platform is deliberately boring.

Few components.  
Clear boundaries.  
Explicit authority separation.

The system works because each part knows:
- what it is responsible for
- what it must never do
