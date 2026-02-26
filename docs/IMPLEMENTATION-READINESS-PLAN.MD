# Numbers Implementation Readiness Plan

Status: evolving  
Objective: eliminate guess-space before backend implementation.

This plan identifies missing normative surfaces.
It does not introduce new protocol behavior.

Current codex-spec structure:

- core/
- data/
- bidding/
- api/
- config/
- errors/
- chain/ (new, incomplete)
- wallet/ (new, incomplete)
- inscription/ (new, incomplete)

The following workstreams must be completed before semantic freeze.

---

## 1. Chain Interaction Specification

File:
chain/CHAIN-INTERACTION.md

Purpose:
Define how Numbers observes and interacts with Bitcoin.

Must specify:

- Authoritative data source(s)
- Confirmation depth policy for:
  - settlement
  - inscription
- Reorg handling
- Definition of:
  - observed
  - known
  - unknown
- Node failure classifications
- Mempool usage rules (if any)

Must map all failure classes to ERROR-TAXONOMY.md.

No Bitcoin interaction may occur without mechanical definition here.

---

## 2. Wallet Specification

File:
wallet/WALLET-SPEC.md

Purpose:
Define custody, signing, and UTXO policy.

Must specify:

- Custody model
- Key storage model
- UTXO reservation timing
- UTXO release timing
- Dust policy
- Insufficient funds behavior
- Fee ceiling enforcement behavior
- Signing surfaces
- Deterministic transaction construction inputs

Must not introduce new auction states.

Must not alter authority semantics.

---

## 3. Inscription Machine Specification

File:
inscription/INSCRIPTION-MACHINE.md

Purpose:
Define inscription lifecycle separate from auction lifecycle.

Must specify:

- Intent persistence boundary
- Transaction construction boundary
- Broadcast boundary
- Txid persistence rule
- Confirmation observation rule
- Ambiguity detection triggers
- Authority consumption moment
- Replacement policy (if any)
- Fee adaptation policy (if any)

Must enumerate crash windows explicitly.

Ambiguity must freeze authority permanently.

---

## 4. Restart Matrix for Chain Boundary

Either:

- Extend RESTART-RULES.md  
or  
- Create chain/RESTART-MATRIX.md

Must enumerate:

- Crash before persist
- Crash after persist before broadcast
- Crash after broadcast before txid persist
- Crash after txid persist before confirmation
- Crash after confirmation before state transition persist

Each row must define:

- Reconstructed state
- Authority status
- Whether ambiguity triggers
- Whether operational continuation is allowed

No operator discretion permitted.

---

## 5. Error Taxonomy Completion

errors/ERROR-TAXONOMY.md must include:

Chain errors:
- chain_unavailable
- chain_stale
- broadcast_rejected
- broadcast_unknown
- confirmation_reorg

Wallet errors:
- insufficient_funds
- signer_unavailable
- fee_ceiling_exceeded

Each error must mandate system action.

No implicit retry allowed.

---

## 6. Circuit Breaker Integration

config/CONFIG-REFERENCE.md must define:

- Fee ceiling
- Maximum inscription backlog
- Chain stale threshold
- Confirmation depth parameter
- Maximum mempool wait window (if applicable)

These must not alter semantic truth.
They may only halt or pause subsystems.

---

## 7. Boundary Consistency Check

After completing sections 1–6:

Verify:

- STATE-MACHINE-TABLE.md remains sole transition authority.
- No new lifecycle states introduced.
- Inscription states remain consistent with JSON artifact (if retained).
- No implicit retries exist.
- No undefined crash window remains.

---

## 8. Semantic Audit

Perform full cross-document audit:

1. No document contradicts INVARIANTS.md.
2. No document contradicts TRANSITION-INVARIANTS.md.
3. No document introduces unlisted transitions.
4. No default behavior assumed.
5. All authority consumption one-shot.
6. All undefined behavior explicitly forbidden.

Output:
SEMANTICS-AUDIT.md listing conflicts and resolutions.

---

## 9. Implementation Entry Gate

Codex implementation may begin only when:

- chain/ is fully specified
- wallet/ is fully specified
- inscription/ is fully specified
- restart matrix complete
- error taxonomy covers chain and wallet
- no unresolved ambiguity at authority boundaries

If any subsystem requires guessing, stop and add clause.

---

## 10. Implementation Order

1. Persistence
2. State machine enforcement
3. Settlement observer
4. Wallet module
5. Inscription module
6. Error and circuit breaker integration
7. API layer
8. Restart simulation tests