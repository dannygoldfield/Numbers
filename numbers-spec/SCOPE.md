# Prototype 1 scope

## SC01 — Learning target

Prototype 1 must demonstrate one complete auction on one controlled local machine: two simulated bidders use Rana; a durable append-only journal records economic actions; deterministic resolution precedes separate settlement; settlement assigns irreversible protocol ownership to the successful participant or Unowned; restart reconstructs committed truth; the next number becomes available after the rhythm gap and stays dormant until its first valid bid.

This is a local protocol demonstration, not a production multi-user or real-value service.

## SC02 — Included capability

The prototype must provide authoritative backend bid evaluation, initial Rana allocation, leader-only reservation, explicit release/replacement, successful capture into the protocol-held balance, locally commanded settlement, ownership finalization, sequence advancement, durable persistence, strict reconstruction, and the two views defined in UI-AND-DEMO. Only the commands and records in STATE-AND-EVENTS are permitted.

A read-only Protocol View and an interactive Auction View must be served by the same local application. Sequential bid submission by one operator is sufficient. Their displays must identify both participants as simulated local identities.

## SC03 — Excluded capability

The auction engine must contain no Bitcoin, wallet, Ordinals, Regtest, Signet, Testnet, Mainnet, inscription, transaction-broadcast, or chain-confirmation concepts. No deferred inscription placeholder is required or permitted in this generation's journal.

Prototype 1 has no registration, passwords, sessions, identity verification, production authentication, separate identity service, additional bidder, transfers, top-ups, discretionary refund, compensation, protocol-balance spending, ownership transfer, appeals, winner substitution, automatic command retry, bid-cap closure, operator pause/resume command, or history repair/reset operation.

It must not add public hosting, distributed services, generalized scalability, queues/workers, PostgreSQL without a present requirement, plugin/adapter frameworks, production monitoring/security infrastructure, signing services, external checkpoints, independent replicas, or Bitcoin anchoring. Future work is non-executable until separately specified and approved.

## SC04 — Failure and irreversibility boundary

Finalized facts and ownership must remain irreversible. Unexpected failures must not acquire an economic meaning or a Unowned trigger by inference. Persistence or reconstruction failure follows PR06; it must not cause forfeiture, release, replacement ownership, a skipped number, or another settlement attempt by itself.

Deferred compensation is a future design issue described in `work/TECHNICAL-DEBT.md` F01. Deferral does not leave ordinary settlement accounting undefined. The explicit release of an uncaptured reservation is part of that accounting, not discretionary compensation.
