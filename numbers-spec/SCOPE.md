# Prototype 2 scope

## SC01 — Learning target

Prototype 2 must demonstrate two invited people participating from separate devices in one shared auction history, with one facilitator controlling simulated settlement, on one DigitalOcean Droplet. Each bidder acts through a private credential bound to one simulated identity. The learning target is whether participants can understand bidding, reservations, resolution, and recorded ownership while observing the same backend facts. Rana and settlement remain simulated. This is a facilitated experiment, not a real-value or publicly joinable service.

## SC02 — Included capability

Retain authoritative backend bid evaluation, initial Rana allocation, leader-only reservation, explicit release/replacement, successful capture into the protocol-held balance, explicit simulated settlement, ownership finalization, sequence advancement, durable persistence, strict reconstruction, and Auction and Record views. Add only the credential and role boundary, bounded remote transport, and interface behavior defined in P01, SE07, UI01–UI03, and SN01. Two bidders may submit competing requests; one authoritative evaluator orders them under SE04. Both bidders and the facilitator may inspect shared state, balances, and canonical history.

## SC03 — Excluded capability

The auction engine contains no Bitcoin, wallet, inscription, transaction-broadcast, or chain-confirmation capability. Prototype 2 has no real payment, public registration, password recovery, production identity verification, additional bidder or spectator role, Rana transfer or top-up, discretionary refund or compensation, protocol-balance spending, ownership transfer, appeals, winner substitution, automatic command retry, bid-cap closure, operator pause/resume command, or history repair/reset operation.

Only the invited credential boundary in SE07 is permitted. No publicly joinable auction, additional application writer, distributed service architecture, queue/worker service, PostgreSQL migration, generalized adapter framework, signing service, external checkpoint, independent replica, or Bitcoin anchoring is introduced. Stopping a process for maintenance does not pause auction time or create a protocol transition.

## SC04 — Failure and irreversibility boundary

Finalized auction facts, including the original ownership assignment, must remain irreversible. A future separately specified transfer may append later facts without rewriting that assignment (RT06); no transfer exists in Prototype 2. Unexpected failures must not acquire an economic meaning or a Unowned trigger by inference. Persistence or reconstruction failure follows PR06; it must not cause forfeiture, release, replacement ownership, a skipped number, or another settlement attempt by itself.

Deferred compensation is a future design issue described in `work/TECHNICAL-DEBT.md` F01. Deferral does not leave ordinary settlement accounting undefined. The explicit release of an uncaptured reservation is part of that accounting, not discretionary compensation.
