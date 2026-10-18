# Current prototype parameters and implementation boundary

## P01 — Identities and accounts

Exactly two simulated bidder identifiers exist: `demo-1` and `demo-2`. The credential principal `bidder-a` is bound only to `demo-1`; `bidder-b` is bound only to `demo-2`. Each credential is assigned privately to one invited tester for the history. The `facilitator` principal may choose simulated settlement and inspect the same history, but has no Rana account and may not bid. No principal may switch bidder identity or assign another person's credential to an existing bidder during that history. Rotation of a compromised credential may preserve its existing assignment after ingress is stopped; it cannot change records, balances, or permissions. Credential possession proves access permission, not legal identity or ownership outside Numbers.

All three principals may read both bidders' balances and the full shared journal. Presentation must identify bidder accounts as simulated. Credentials and real names must not be placed in canonical command bodies or event payloads.

Initial allocations `demo_1_initial_rana` and `demo_2_initial_rana` are explicit nonnegative integer inputs. SE05 initialization records each once. Initial reservation balances and the protocol-held balance are explicitly zero. `protocol` identifies the accounting destination for captured Rana; it is not a third identity and cannot submit commands or spend.

The fixed unit code is `RANA`. No exchange rate or historical sats conversion exists.

## P02 — Closed parameter surface

The backend must validate configuration before initialization or reconstruction. The table defines the complete behavioral parameter surface. Missing values are invalid except the two explicit defaults below; unknown behavioral keys, wrong types, and invalid combinations are invalid. Integers use the exact integer model in PR02. Configuration errors reject startup without writing canonical records.

| Parameter | Constraint / explicit default | Persistence boundary |
|---|---|---|
| `starting_number` | Integer >= 1; explicit default 1 | Initial AuctionRecord.number; affects empty-history initialization only |
| `demo_1_initial_rana` | Integer >= 0; no default | First RanaIssueRecord |
| `demo_2_initial_rana` | Integer >= 0; no default | Second RanaIssueRecord |
| `duration_seconds` | Exactly 225; explicitly supplied | Each AuctionRecord |
| `inter_auction_gap_seconds` | Exactly 12; explicitly supplied | Each AuctionRecord; governs the gap after that auction |
| `extension_window_seconds` | Integer >= 1; no default | Each AuctionRecord |
| `extension_increment_seconds` | Integer >= 1; no default | Each AuctionRecord |
| `max_extensions` | Integer >= 0; no default; 0 explicitly disables extensions | Each AuctionRecord |
| `minimum_bid_rana` | Integer >= 1; no default | Each AuctionRecord |
| `minimum_increment_rana` | Integer >= 1; no default | Each AuctionRecord |
| `maximum_bid_rana` | Null or integer >= minimum_bid_rana; explicit default null | Each AuctionRecord |

Current configuration must never replace captured inputs. Later timing/bidding configuration affects only newly created auctions. Settlement has no deadline parameter or stored deadline. Starting-number and allocation settings are initialization inputs only once a valid history exists; reconstruction uses their records and must not issue again or reset the sequence.

Changing the fixed 225-second duration or 12-second gap requires a prototype specification revision, not a runtime override. These values are not production declarations.

Operational settings are the database path, loopback host, port, non-authoritative logging verbosity, an explicit allowed HTTPS origin, and an explicit session label. Host remains loopback; port is an integer from 1 through 65535. The allowed origin is exactly one HTTPS scheme/host/port origin, with no wildcard, path, credentials, query, or fragment. The session label is 1–64 ASCII letters, digits, hyphens, or underscores and identifies the independent demonstration to its users; it is not a canonical record identifier. Neither has an implicit default.

The launch procedure binds one session label, build revision, configuration, credential assignment, and verified-new journal location in a non-authoritative session manifest. The association remains fixed for the life of that history. Restart must use that same association. A different independent history requires a distinct location and label; a changed path must not silently initialize a substitute for an existing session.

Gateway credential files, TLS keys, and service settings remain outside the canonical parameter surface and outside Git. They cannot change auction rules. No operational setting may disable the SE07 authentication/permission gates or enable a second writer. Configuration and logging must not reset, reinterpret, or replace history.

`spec_revision` is fixed by PR01 and is not a configuration parameter. Selecting a database path cannot authorize reinterpretation, migration, reset, or replacement of an existing history.

## P03 — Responsibilities

| Responsibility | Authoritative work |
|---|---|
| Access | SE07 authorization before evaluator entry |
| Auction | SE03 admission, timing, leading-bid derivation, close, and fixed resolution |
| Rana accounting | RT01–RT03 ledger preconditions, holds, release, capture, and conservation |
| Settlement | RT03 terminal command validation against the fixed result |
| Ownership | RT04 binding and unique FinalizationRecord |
| Persistence/reconstruction | PR01–PR07 order, atomic groups, durability, validation, replay |
| UI | UI01–UI04 command submission and projections only |

These responsibilities must remain distinct in meaning. They do not require separate processes, services, packages, adapters, or frameworks.

## Implementation recommendation — non-normative

Use one application process behind a same-host HTTPS gateway, one SQLite database, one serialized authoritative writer, and one ordered event table. Use transactions with durable commit behavior sufficient for PR03. SQL constraints can reinforce unique sequence indices and unique outcome records; application code must enforce the complete semantic preconditions. Derived in-memory state is a cache only.

No worker, scheduler, PostgreSQL service, or generalized adapter layer is needed. The request-driven evaluation in SE04 can implement the required behavior. These are implementation recommendations, not alternative protocol rules. See `work/TECHNICAL-DEBT.md` for limits and replacement triggers.
