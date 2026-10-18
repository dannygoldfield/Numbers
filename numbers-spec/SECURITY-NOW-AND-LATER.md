# Security now and later

## SN01 — Current prototype protection — normative rule

Prototype 2 protects application-maintained economic and ownership truth through the retained RT, SE, and PR rules. SE07 additionally restricts remote access, bidder identity, settlement permission, request representation and size, and retries. The approved network surface consists of one authenticated HTTPS gateway on one Droplet, one loopback application listener, one writer, and one local journal. The application runs without host-administration privileges. Credentials and keys are protected operational files, never canonical configuration or committed source.

The implementation must not claim protection against a person controlling the machine, database, process, or server clock. Credentials authorize the P01 role; they do not prove human identity or external ownership. Payload hashes must not be described as production tamper resistance: they exclude the envelope, and an administrator can recompute them or replace/remove a valid journal suffix.

No current-prototype requirement mandates record hash chaining, signatures, external attestation, independent replicas, or Bitcoin anchoring. Security failure must follow the existing explicit halt/rejection rules, not create a recovery, compensation, or Unowned capability.

Launch and restart are explicit operator actions; the service must not automatically restart after a halt or crash. Before an intentional restart, inspect the diagnostic and identify the matching history/configuration/build. Successful reconstruction precedes normal startup evaluation. No automatic redeployment, fallback history, or request replay follows a failure.

For controlled shutdown, stop ingress, let the current authoritative boundary finish where possible, then stop the writer. Interrupted writes remain governed by PR03–PR06. Stopping does not pause deadlines or create an expired settlement.

Before and after a facilitated session, preserve a consistent stopped-writer copy of the journal and matching build/configuration, verify its checksum and read-only reconstruction, and retain an off-host copy. A backup is evidence of a particular committed prefix, not a canonical rollback permission. If later commits may be missing, do not serve the older copy as continuation; halt the session and preserve the available evidence.

Use logs for diagnosis without credentials or full command bodies. Resource limits and network errors cannot cause a forfeiture, release, replacement winner, Unowned outcome, or skipped number. A disk, clock, or history failure retains PR06 halt behavior.

## Prototype 1 learning record — non-normative

Protected facts are valid bid/hold correspondence, fixed winner, terminal settlement, final ownership, conserved Rana, and reproducible durable history. The assumed environment is one controlled local machine, cooperative operator, suitable clock and storage, two simulated identities, and no external payment.

Deliberately unprotected threats include malicious host control, direct database rewrite/deletion, fraudulent identity selection, storage destruction, hostile clock changes, remote attackers, and external/legal ownership enforcement. Prototype 1 tests whether specified mechanics work; it does not establish production safety.

Accepted shortcuts and replacement triggers are located in `work/TECHNICAL-DEBT.md` TD01–TD09. Deferring compensation is F01; finality itself is not temporary debt. No economic question is left to a security mechanism.

Evidence requiring stronger protection includes independent people relying on the history, meaningful real-world loss, additional operators/writers, unexplained integrity failures, unacceptable measured startup/persistence behavior, or a demonstrated need for external attestations. A discovered invariant failure first requires explicit halt and investigation; it is not permission to silently expand the architecture.

## Future mechanism ledger — non-normative

Append a dated section for each later prototype when its threat/prerequisite exists. Preserve prior entries as the security assumptions and evidence of their own phase; do not rewrite them as if later protections existed earlier. None of the following is scheduled or enabled by this document.

| Mechanism | Threat addressed | Prerequisite | Added complexity | Simpler mechanism extended/replaced | How protocol meaning remains fixed |
|---|---|---|---|---|---|
| Full record hash chain | Envelope alteration or internal record deletion/reordering beyond current checks | Need for stronger linkage evidence | Encoding/version boundaries, predecessor commitments, migration design | Independent payload hashes | Adds evidence of existing ordered facts; never changes bid/settlement/ownership rules |
| Signed checkpoints | Unidentified or unauthenticated attestation to a prefix | Independent verifier and explicit key/trust model | Key custody, rotation, checkpoint cadence and validation | Unsigned local inspection | A signature attests; it cannot grant settlement or ownership authority |
| Independent publication | Rewriting a history after presenting it to others | Others rely on shared history | Publication, retention, privacy and disagreement handling | Locally retained checkpoints | Publication does not rewrite prior records or choose outcomes |
| Independent replicas | Sole-copy loss or total control over evidence | Actual independent custodians/availability requirement | Replication, consistency evidence and access control | One local durable copy | Replicas remain evidence unless a separately approved specification changes writer authority |
| Bitcoin anchoring | Need for externally durable public evidence of a commitment | Separate future scope approval and demonstrated demand | Chain integration, fees, keys, confirmation/reorg semantics | Independent publication | External proof only under a new bounded design; no change to Prototype 1 auction meaning |

Every future proposal must identify those six dimensions before adoption. No mechanism is justified merely because it is useful in isolation.

## Prototype 2 learning record — 2026-10-18 — non-normative

Two invited credential-bound bidders and one facilitator use one HTTPS gateway and one writer. Host administrators and the local machine remain trusted. The bounded trial measures shared understanding and remote participation; it does not establish public-service availability or real-value safety. Detailed access, transport, uncertainty and preservation rules are owned by P01/P02, SE07, UI03 and SN01.
