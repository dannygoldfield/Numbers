# Prototype 1 parameters and implementation boundary

## P01 — Identities and accounts

Exactly two simulated bidder identifiers exist: `demo-1` and `demo-2`. Display names must clearly identify them as simulated local identities. The UI must allow selection of either. Selection is not proof of operator identity.

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

Operational settings are limited to local database path, loopback host, local port, and non-authoritative logging verbosity. Host must be loopback; port must be an integer from 1 through 65535. An existing database cannot be reset or overwritten by changing a setting. Logging cannot change semantic evaluation. No operational setting grants a protocol capability.

## P03 — Responsibilities

| Responsibility | Authoritative work |
|---|---|
| Auction | SE03 admission, timing, leading-bid derivation, close, and fixed resolution |
| Rana accounting | RT01–RT03 ledger preconditions, holds, release, capture, and conservation |
| Settlement | RT03 terminal command validation against the fixed result |
| Title | RT04 binding and unique FinalizationRecord |
| Persistence/reconstruction | PR01–PR06 order, atomic groups, durability, validation, replay |
| UI | UI01–UI04 command submission and projections only |

These responsibilities must remain distinct in meaning. They do not require separate processes, services, packages, adapters, or frameworks.

## Implementation recommendation — non-normative

Use one locally served application process, one SQLite database, one serialized authoritative writer, and one ordered event table. Use transactions with durable commit behavior sufficient for PR03. SQL constraints can reinforce unique sequence indices and unique outcome records; application code must enforce the complete semantic preconditions. Derived in-memory state is a cache only.

No worker, scheduler, PostgreSQL service, or generalized adapter layer is needed. The request-driven evaluation in SE04 can implement the required behavior. These are implementation recommendations, not alternative protocol rules. See `work/TECHNICAL-DEBT.md` for limits and replacement triggers.
