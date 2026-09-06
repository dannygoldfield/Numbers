# Security now and later

## SN01 — Prototype 1 protection — normative candidate rule

Prototype 1 must protect canonical economic and title truth against its own application operations and supported interruptions through the backend-only evaluation, explicit identity eligibility, serialized ordering, durable atomic groups, one-shot references, and validation/reconstruction rules in RT, SE, and PR. Local service exposure must follow P02. UI behavior must follow UI01–UI03. These mechanisms must not introduce additional product or economic predicates.

The implementation must not claim protection against a person controlling the machine, database, process, or server clock. Simulated identity selection must not be presented as authentication. Payload hashes must not be described as production tamper resistance: they exclude the envelope, and an administrator can recompute them or replace/remove a valid journal suffix.

No Prototype 1 requirement mandates record hash chaining, signatures, external attestation, independent replicas, or Bitcoin anchoring. Security failure must follow the existing explicit halt/rejection rules, not create a recovery, compensation, or PublicLand capability.

## Prototype 1 learning record — non-normative

Protected facts are valid bid/hold correspondence, fixed winner, terminal settlement, final title, conserved Rana, and reproducible durable history. The assumed environment is one controlled local machine, cooperative operator, suitable clock and storage, two simulated identities, and no external payment.

Deliberately unprotected threats include malicious host control, direct database rewrite/deletion, fraudulent identity selection, storage destruction, hostile clock changes, remote attackers, and external/legal title enforcement. Prototype 1 tests whether specified mechanics work; it does not establish production safety.

Accepted shortcuts and replacement triggers are located in `work/TECHNICAL-DEBT.md` TD01–TD09. Deferring compensation is F01; finality itself is not temporary debt. No economic question is left to a security mechanism.

Evidence requiring stronger protection includes independent people relying on the history, meaningful real-world loss, additional operators/writers, unexplained integrity failures, unacceptable measured startup/persistence behavior, or a demonstrated need for external attestations. A discovered invariant failure first requires explicit halt and investigation; it is not permission to silently expand the architecture.

## Future mechanism ledger — non-normative

Append a dated section for each later prototype when its threat/prerequisite exists. Preserve prior entries as the security assumptions and evidence of their own phase; do not rewrite them as if later protections existed earlier. None of the following is scheduled or enabled by this document.

| Mechanism | Threat addressed | Prerequisite | Added complexity | Simpler mechanism extended/replaced | How protocol meaning remains fixed |
|---|---|---|---|---|---|
| Full record hash chain | Envelope alteration or internal record deletion/reordering beyond current checks | Need for stronger linkage evidence | Encoding/version boundaries, predecessor commitments, migration design | Independent payload hashes | Adds evidence of existing ordered facts; never changes bid/settlement/title rules |
| Signed checkpoints | Unidentified or unauthenticated attestation to a prefix | Independent verifier and explicit key/trust model | Key custody, rotation, checkpoint cadence and validation | Unsigned local inspection | A signature attests; it cannot grant settlement or title authority |
| Independent publication | Rewriting a history after presenting it to others | Others rely on shared history | Publication, retention, privacy and disagreement handling | Locally retained checkpoints | Publication does not rewrite prior records or choose outcomes |
| Independent replicas | Sole-copy loss or total control over evidence | Actual independent custodians/availability requirement | Replication, consistency evidence and access control | One local durable copy | Replicas remain evidence unless a separately approved specification changes writer authority |
| Bitcoin anchoring | Need for externally durable public evidence of a commitment | Separate future scope approval and demonstrated demand | Chain integration, fees, keys, confirmation/reorg semantics | Independent publication | External proof only under a new bounded design; no change to Prototype 1 auction meaning |

Every future proposal must identify those six dimensions before adoption. No mechanism is justified merely because it is useful in isolation.
