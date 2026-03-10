# Numbers Implementation Readiness Plan

Status: evolving  
Objective: eliminate guess-space before backend implementation.

This plan identifies missing normative surfaces.  
It does not introduce new protocol behavior.

All behavior must remain governed exclusively by the state machine and invariants.

---

# 0. Canonical Vocabulary Freeze

File:  
core/CANONICAL-TERMS.md

Purpose:  
Eliminate synonym drift and alias ambiguity across all normative documents.

Must define:

- One canonical term per concept.
- Explicit definitions for all protocol-level nouns.
- Explicit prohibition of interchangeable phrasing in normative sections.
- Explicit mapping of any deprecated terms to canonical terms.
- Capitalization or styling convention for protocol terms.

Rules:

- One concept → One term → One definition → One state effect.
- No undefined protocol term may appear in any normative document.
- No implicit equivalence permitted.
- Narrative documents may use flexible language.
- Normative documents may not.

All normative documents must pass canonical term validation before semantic freeze.

---

# 1. Definition Authority Order

Authority precedence in case of contradiction:

1. INVARIANTS.md  
2. TRANSITION-INVARIANTS.md  
3. STATE-MACHINE-TABLE.md  
4. CANONICAL-TERMS.md  
5. Subsystem specifications (chain, wallet, inscription, config, errors)

If contradiction exists, lower documents must be revised.  
No document may override a higher authority document.

---

# 2. Chain Interaction Specification

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
- Definitions of:
  - Observed
  - Known
  - Unknown
- Node failure classifications
- Mempool usage rules (if any)

Must map all failure classes to ERROR-TAXONOMY.md.

No Bitcoin interaction may occur without mechanical definition here.

---

# 3. Wallet Specification

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

# 4. Inscription Machine Specification

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

# 5. Restart Matrix for Chain Boundary

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
- Ambiguity status
- Whether operational continuation is allowed

No operator discretion permitted.

---

# 6. Error Taxonomy Completion

File:  
errors/ERROR-TAXONOMY.md

Must include:

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

Each error must mandate deterministic system action.

No implicit retry allowed.  
No best-effort semantics allowed.  
No “eventual” resolution language permitted.

---

# 7. Circuit Breaker Integration

File:  
config/CONFIG-REFERENCE.md

Must define:

- Fee ceiling
- Maximum inscription backlog
- Chain stale threshold
- Confirmation depth parameter
- Maximum mempool wait window (if applicable)

Circuit breakers may halt subsystems.  
They may not alter semantic truth.

---

# 8. Boundary Consistency Check

After completing sections 0–7:

Verify:

- STATE-MACHINE-TABLE.md remains sole transition authority.
- No new lifecycle states introduced.
- Inscription states remain consistent with JSON artifact.
- No implicit retries exist.
- No undefined crash window remains.
- No undefined protocol term exists.
- No contradiction exists across authority layers.

---

# 9. Semantic Audit

Perform full cross-document audit:

1. No document contradicts INVARIANTS.md.
2. No document contradicts TRANSITION-INVARIANTS.md.
3. No document introduces unlisted transitions.
4. No default behavior assumed.
5. All authority consumption one-shot.
6. All undefined behavior explicitly forbidden.
7. All protocol-level nouns appear in CANONICAL-TERMS.md.
8. No synonym drift across normative documents.

Output:  
SEMANTICS-AUDIT.md listing conflicts and resolutions.

Semantic freeze occurs only after all conflicts resolved.

---

# 10. Implementation Entry Gate

Codex implementation may begin only when:

- Canonical vocabulary frozen
- chain/ fully specified
- wallet/ fully specified
- inscription/ fully specified
- restart matrix complete
- error taxonomy covers chain and wallet
- no unresolved ambiguity at authority boundaries

If any subsystem requires guessing, stop and add clause.

---

# 11. Implementation Order

1. Persistence
2. State machine enforcement
3. Settlement observer
4. Wallet module
5. Inscription module
6. Error and circuit breaker integration
7. API layer
8. Restart simulation tests