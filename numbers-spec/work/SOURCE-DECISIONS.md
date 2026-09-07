# Source decisions and extraction

Non-normative source and decision register. Date: 2026-09-05; updated through Phase 2. This file authorizes no implementation.

## Evidence and classification

Only the task prompt and `codex-spec/` supplied behavioral evidence. No application code, README, external documents, or prior conversations supplied protocol semantics. All 25 source documents were read; the inventory and hashes are in `AUDIT.md`.

- **C**: confirmed user decision, cited to an exact heading in the supplied prompt.
- **K**: retained written behavior, cited to an exact source file and section.
- **D**: consequence required by confirmed decisions, without selecting unresolved economics.
- **P**: provisional recommendation; not a protocol rule.
- **S**: superseded source assumption, catalogued in `SPEC-CHANGES.md`.
- **F**: deferred concern, catalogued in `TECHNICAL-DEBT.md`.
- **Q**: unresolved semantic question, catalogued in `OPEN-QUESTIONS.md`.

Prompt references below refer to the self-contained request supplied for this task. They do not claim authority from prior conversations.

## Confirmed decisions from the prompt

| ID | Decision | Exact prompt heading |
|---|---|---|
| C01 | Work only in `/Users/dannygoldfield/Projects/Numbers-Repos/Numbers/numbers-spec`; preserve existing work; no code changes, commit, push, or publication. | Opening instructions; Preflight |
| C02 | `codex-spec/` stays frozen and solely active; `numbers-spec/` stays an inactive candidate until separate explicit activation; never two active sets; explicitly restate retained rules. | Source and authority rules |
| C03 | Read all source documents; distinguish confirmed, retained, consequential, recommended, superseded, deferred, and unresolved material. | Source and authority rules |
| C04 | Audit before drafting; if an implementation-blocking semantic question remains, record it and stop at Outcome A in this task. | One-prompt, two-phase process; Completion outcomes |
| C05 | Optimize Prototype 1 for learning, iteration speed, the core experience, clarity, and a lean codebase; substantial machinery must serve that learning. | Mission; Prototype-development rule |
| C06 | Single-machine operation and a minimal locally hosted browser interface. | Confirmed Prototype 1 decisions |
| C07 | UI submits commands and displays derived state; it cannot authoritatively validate bids, compute balances, choose the winner, settle, or assign title. | Confirmed Prototype 1 decisions |
| C08 | Auction engine contains no Bitcoin concepts; exclude Bitcoin, wallets, Ordinals, Regtest, Signet, Testnet, and Mainnet. | Confirmed Prototype 1 decisions |
| C09 | Rana is the in-house value standard. | Confirmed Prototype 1 decisions |
| C10 | PublicLand replaces NullSteward and is an irreversible protocol title status. | Confirmed Prototype 1 decisions |
| C11 | PublicLand is not a person, user account, error, government owner, legal land claim, or general recovery mechanism. | Confirmed Prototype 1 decisions |
| C12 | Prototype 1 base-duration input is 225 seconds, displayed as 3:45; rhythm gap is 12 seconds. These are prototype parameters, not production declarations. Interpretation as base duration follows K05, not an invented extension ban. | Confirmed Prototype 1 decisions; Required complete experience, item 9 |
| C13 | First valid bid opens the auction; next number remains dormant until its own first valid bid. | Confirmed Prototype 1 decisions |
| C14 | Auction resolution and settlement are separate. | Confirmed Prototype 1 decisions |
| C15 | Auction state, Rana balances, and protocol title derive from canonical records. | Confirmed Prototype 1 decisions |
| C16 | Restart is deterministic reconstruction, not speculative repair or recovery. | Confirmed Prototype 1 decisions |
| C17 | Finalized auction results and final title are irreversible; a 12-second gap follows finalization, then the next number becomes available. | Confirmed Prototype 1 decisions |
| C18 | Exactly two fixed local simulated identities; sequential bids through one interface suffice; this is not a production multi-user system. | Confirmed Prototype 1 decisions |
| C19 | Each identity has a stable identifier and explicitly configured initial Rana allocation; available and reserved balances derive from ledger records; both are selectable and visibly simulated. | Two simulated bidders |
| C20 | No registration, passwords, sessions, production authentication, identity verification, separate identity services, or claims that selection proves identity. | Two simulated bidders |
| C21 | Demo shows identity 1 opening, identity 2 competing, rejection, deterministic leadership changes, reservation, earlier-reservation release or replacement, winner selection, winner capture, and losing-reservation release. | Two simulated bidders, demonstration items 1–9 |
| C22 | Complete experience covers pre-bid state through resolution, separate settlement, capture or an explicitly permitted terminal result, title exactly once to winner or PublicLand, finalization, gap, next dormant number, and reconstruction at material boundaries. | Required complete experience, items 1–19 |
| C23 | Append-only events preserve prior facts; settlement consumes the fixed auction result without rewriting it; Rana and title records supply their respective truths; exceptions need specified predicates and authority. | First-principles boundaries |
| C24 | Distinct responsibilities for auction, accounting, settlement, title, persistence/reconstruction, and UI do not require distinct services or frameworks. | Lean implementation boundary |
| C25 | Durable ordered records, stable identifiers, deterministic global order, no application edits/deletions, required atomic persistence, explicit preconditions/effects, and defined malformed-history/interrupted-write handling. | Append-only truth and restart |
| C26 | Application append-only enforcement must be distinguished from tamper resistance against machine/database control; no production tamper-resistance claim; retain payload hashes if simple/useful; no external checkpoints, signing, replicas, or anchoring requirement. | Append-only truth and restart |
| C27 | Two perspectives from the same app, suitable side by side: Auction View and read-only Protocol View. | Demonstration interface |
| C28 | Auction View shows number, status, simulated identity, available/reserved Rana, leader, bid control, plain acceptance/rejection, countdown, winner, settlement, title, rhythm pause, and next number. | Demonstration interface / Auction View |
| C29 | Anticipation, rhythm, typography, visible transitions, understandable consequences, and a clear title moment; no dependence on decorative complexity, hidden rules, or excessive animation. | Demonstration interface / Auction View |
| C30 | Protocol View shows submitted command, validation result, canonical records/order, derived auction state, ledger movements/balances, settlement, title, reconstruction result, and retained hashes; it has no controls changing canonical state. | Demonstration interface / Protocol View |
| C31 | A bid submission in Auction View is immediately inspectable with its backend decision and records in Protocol View. | Demonstration interface / Protocol View |
| C32 | Prototype 1 has no discretionary refund or compensation mechanism; ethical and architectural questions remain deferred. | Confirmed Prototype 1 decisions; Compensation as a deferred issue |
| C33 | Deferred compensation issue covers ethical friction, irreversible facts, later additive compensation versus rewriting, executable predicates, exceptional circumstances, evidence/authority, title consequences, and reconsideration milestone. | Compensation as a deferred issue |
| C34 | Security roadmap states protection, mechanisms, exclusions, assumptions, shortcuts, and evidence for strengthening; later entries are appendable and preserve prior security history. | Security progression |
| C35 | Each future security mechanism records threat, prerequisite, complexity, replaced simpler mechanism, and preservation of protocol meaning; listed future cryptographic mechanisms are not Prototype 1 requirements. | Security progression |
| C36 | Every shortcut is named, justified, contained, locatable, and assigned a replacement condition; none changes behavior, transitions, irreversibility, UI authority, or retry/recovery semantics. | Prototype-development rule |
| C37 | Do not add public hosting, distributed services, generalized scalability, queues/workers, PostgreSQL without need, adapter/plugin frameworks, Bitcoin systems, production monitoring/security, or speculative abstractions. | Prototype-development rule |
| C38 | Use the requested small document set; `work/` is non-normative; required status metadata says draft/inactive; future authority order and conflict/activation rules must be explicit. | Document set |
| C39 | Every eventual state, event, rejection, invariant, and transition needs explicit, testable semantics and traceability; no silent defaults, implicit retries, unspecified recovery, competing rules, unresolved alternatives, or placeholders in normative text. | Specification-writing rules; Final audit |
| C40 | Configuration tunes only declared parameters and never changes historical truth. | Specification-writing rules |
| C41 | Audit the candidate as one system; fix editorial/mechanical conflicts, but return to the checkpoint for unresolved product, economic, fairness, title, authority, or history decisions. | Phase 2: specification drafting and final audit; Final audit |
| C42 | Final report identifies outputs, preserved kernel, changes, debt, questions, conflicts, audit/readiness, and confirms no code/commit/push/publication/activation. | Completion outcomes |

## Confirmed follow-up decisions

| ID | Decision | Exact user source |
|---|---|---|
| C43 | Prototype 1 uses indivisible rana with exact integer accounting. No fractional Rana amounts. | User follow-up selecting OPEN-QUESTIONS.md Q01 recommendation and stating “I agree with Q01, Q02, Q03, Q04 recommendations.” Only Q01 supplied an affirmative recommendation. |
| C44 | Successful settlement captures exactly the winning bid amount into a protocol-held balance; no spending operation exists in Prototype 1. | User’s explicit Q03 answer: “Capture exactly the winning bid amount into a protocol-held balance, with no spending operation in Prototype 1.” |
| C45 | Hold only the leader’s full bid; release an outbid hold immediately, with same-identity replacement using the existing hold and only the additional available Rana. | User annotation “Hold only the leader's bid.” on the explanation recommending the leader-only model and describing replacement. |
| C46 | On local expired settlement assigning PublicLand, release the uncaptured winning reservation; do not forfeit it or leave it permanently locked. | User annotation “I agree.” on “For Prototype 1, I recommend release:” in the explanation of all three failed-settlement options. |

The same follow-up asks whether Q04 is about refunds and assumes no reversibility. This is consistent with C17/C32 but does not select reservation release, capture, or permanent lock. Q02–Q04 each initially stated that no rule was recommended. Later explicit answers C44, C45, and C46 resolve Q03, Q02, and Q04 respectively. All four economic questions are resolved; the candidate remains inactive. The distinction between an original reservation release and a later discretionary compensation event is recorded in OPEN-QUESTIONS.md and TECHNICAL-DEBT.md. No activation follows from this agreement.

## Retained behavioral kernel

These source extraction findings are explicitly restated or have a recorded candidate change in the completed documents mapped by TRACEABILITY. They are not a second active specification.

| ID | Retained behavior | Exact source file and section |
|---|---|---|
| K01 | Source precedence follows its ranked authority list. A lower document cannot invent permission from higher-document silence. Undefined behavior is not executable. | `codex-spec/AUTHORITY-ORDER.md`, authority list and paragraphs immediately following it; `codex-spec/SPEC-INDEX.md`, closing paragraphs |
| K02 | Numbers start at an explicitly configured positive starting number (existing default 1), then increase exactly by 1 without reuse, skips, or overlapping non-terminal auctions. | `core/INVARIANTS.md` I-01/I-02; `IMPLEMENTATION-SLICE-01.md` §5; `config/CONFIG-REFERENCE.md` §6, `auction.starting_number` (all under `codex-spec/`) |
| K03 | States are Scheduled, Open, Closed, AwaitingSettlement, Finalized; Finalized is terminal. No valid bid leaves Scheduled indefinitely, without close, settlement, finalization, or PublicLand predecessor outcome. | `codex-spec/core/STATE-MACHINE-TABLE.md` I, Auction States/Notes/Allowed Transitions; `codex-spec/core/INVARIANTS.md` I-18 |
| K04 | First valid bid and opening record commit atomically. Invalid evaluated bids are recorded and never open, extend, change leadership, or enter resolution. | `codex-spec/core/EVENT-TYPES.md` §4, BidRecord/AuctionOpenRecord; `codex-spec/bidding/BIDDING-ADMISSION.md` §§3, 9–10 |
| K05 | End = fixed base end + extension increment × extension count. An accepted valid bid in Open at or after end minus window, while extension count is below maximum, appends one extension. Close takes precedence at server_time >= end. | `codex-spec/core/STATE-MACHINE.md` §§3.2.2–3.2.3; `codex-spec/core/CORE-SEQUENCE.md` §§5–8 |
| K06 | Extension window and increment are explicit positive integer seconds; maximum is an explicit nonnegative integer, with 0 meaning disabled. No defaults are defined for those three keys. Values are captured at AuctionRecord under higher/earlier capture requirements. | `codex-spec/config/CONFIG-REFERENCE.md` §6 and §15; `codex-spec/IMPLEMENTATION-SLICE-01.md` §5 |
| K07 | Opening bid meets positive configured minimum. Later valid bids meet current high plus positive minimum increment and optional maximum. Highest valid amount leads/wins; lowest global sequence index breaks ties. | `codex-spec/bidding/BIDDING-ADMISSION.md` §§6–7.1; `codex-spec/bidding/SETTLEMENT.md` §2; `codex-spec/IMPLEMENTATION-SLICE-01.md` §§9, 11 |
| K08 | Time-based close only, no bid-cap close. Closed resolves exactly once from persisted valid bids; resolution fixes the result permanently and enters AwaitingSettlement. | `codex-spec/core/INVARIANTS.md` I-05/I-06; `codex-spec/core/STATE-MACHINE-TABLE.md` I, Auction Allowed Transitions; `codex-spec/IMPLEMENTATION-SLICE-01.md` §§10–11 |
| K09 | Existing demo settlement accepts an explicit local command selecting settled or expired only in AwaitingSettlement. Expired requires no elapsed deadline; runtime/restart never automatically expire local settlement. No-winner/not_required is excluded. | `codex-spec/core/STATE-MACHINE-TABLE.md` I, local settlement rows; `codex-spec/bidding/SETTLEMENT.md` §§5, 7–8; `codex-spec/IMPLEMENTATION-SLICE-01.md` §19.1 |
| K10 | Settlement outcome is terminal and cannot rewrite resolution. Settled binds final destination to the winning bid; expired binds it to NullSteward. Finalization fixes it irreversibly. | `codex-spec/core/INVARIANTS.md` I-07/I-14; `codex-spec/bidding/SETTLEMENT.md` §§1, 9–10 |
| K11 | NullSteward preserves the number in the sequence outside winner-controlled circulation; it is neither an error nor a recovery/repair/retry destination. | `codex-spec/core/NULLSTEWARD.md` §§1–3, 6 |
| K12 | Finalization precedes rhythm gap; next evaluation after the gap creates N+1 Scheduled; it never auto-opens. Time alone cannot advance an unbid number. | `codex-spec/core/INVARIANTS.md` I-01/I-18; `codex-spec/core/CORE-SEQUENCE.md` §§1, 3 |
| K13 | One ordered canonical event/record model; immutable history; no corrections without an explicitly defined record; serialized commit path; derived state only; durable crash/restart persistence. | `codex-spec/core/INVARIANTS.md` I-03/I-04; `codex-spec/core/CORE-SEQUENCE.md` §1; `codex-spec/data/PERSISTENCE.md` §§4–5, 14 |
| K14 | Stable deterministic record/auction/bid IDs from canonical sequence or number. Canonical UTF-8 JSON payload hashes use SHA-256; envelope is outside the hash. Resolution-input hash has an explicit valid-bid array format. | `codex-spec/data/DATA-MODEL.md` §2, Demo 1 Identifier Generation/Canonical Payload Serialization; §9, resolution_inputs_hash |
| K15 | Reconstruct from all validated canonical records; malformed, contradictory, missing required, illegal, or hash-mismatched history halts. Existing outcomes never rerun. After reconstruction, explicitly eligible close, unresolved resolution, and next-number creation can proceed. | `codex-spec/data/RESTART-RULES.md` §§1–3, 8–10; `codex-spec/data/PERSISTENCE.md` §3 and final identifier/input-hash/configuration validation sections |
| K16 | Local request/read evaluation serializes initial auction creation, due close, pending resolution, and eligible next-number creation before the response/action. No background scheduler required. | `codex-spec/core/CORE-SEQUENCE.md` §1; `codex-spec/IMPLEMENTATION-SLICE-01.md` §5 |
| K17 | Explicit rejection vocabulary and priority; admission rejections get invalid BidRecord. Closed/settling/finalized endpoint failures do not reach admission and have no BidRecord. No authority/Rana effect can be inferred from rejection. | `codex-spec/bidding/BIDDING-ADMISSION.md` §§2–3, 12.1; `codex-spec/api/API-SPEC.md` §§10, 11; absence of Rana semantics is Q02 |
| K18 | No automatic retries or repair. Pause/resume does not change clocks, deadlines, prior records, or finality; absent PauseEventRecord derives Running. | `codex-spec/errors/ERROR-TAXONOMY.md` §3; `codex-spec/core/INVARIANTS.md` I-16; `codex-spec/core/STATE-MACHINE-TABLE.md` System Control Derivation |
| K19 | Backend evaluates; UI/API project facts; unpersisted belief is not truth. Logs are non-authoritative. | `codex-spec/api/API-SPEC.md` §3; `codex-spec/data/PERSISTENCE.md` §5 |
| K20 | Configuration is strict and snapshots fix historical calculations. Missing values without declared defaults fail explicitly. | `codex-spec/config/CONFIG-REFERENCE.md` §§2, 13–15; `codex-spec/IMPLEMENTATION-SLICE-01.md` §5 |

## Required consequences without new economic choices

| ID | Consequence and basis | Limit |
|---|---|---|
| D01 | C19/C25 require durable records of initial allocation; reconstruction cannot reissue allocation merely because the process restarts. | One rana is the indivisible unit under C43; leader-only reservation and replacement are confirmed by C45. No top-up or transfer facility is authorized by the prompt. |
| D02 | C15/C21/C25 require explicit accounting effects and an atomic boundary connecting bid acceptance/opening with its required reservation changes. | C45 selects the leader-only full hold, immediate outbid release, and replacement affordability; RT02 and SE05 restate these explicitly. |
| D03 | C17/C22/C23 require successful title to depend on recorded successful Rana settlement, without rewriting the fixed winning result. | C44 fixes successful capture as the winning amount credited to the protocol-held balance without a spending operation; C46 requires release of the uncaptured winning hold on expired settlement. |
| D04 | C08 supersedes the deferred inscription requirement, not just live broadcast. Bitcoin-specific states, fields, configuration, wallet reservations, and broadcast authority cannot be inherited into the Rana engine. | This is an explicit candidate-generation change, not a modification to the active source. |
| D05 | C10/C22 bind successful protocol title to the winning simulated participant, superseding arbitrary independent destination strings. PublicLand never becomes a participant balance. | The protocol-held accounting balance selected by C44 is not a third demo bidder and has no Prototype 1 spending operation. |
| D06 | C16/C25 and K15 require separate reconstruction and subsequent eligible runtime evaluation. Replaying the same committed prefix yields the same state; a later clock can then permit new records. | Reconstruction does not promise an unchanged countdown display while real time passes. |
| D07 | C08 removes the source's only external authority scope. K08/K10 retain one-shot resolution and finalization at their durable record boundaries; no pre-commit authority consumption or automatic freeze is inferred for a local write. | C44 fixes successful capture; C45/C46 resolve the remaining hold/release choices, restated with at-most-once commit effects in RT02–RT05 and SE05–SE06; no new authority token or retry machinery is presumed. |
| D08 | C02 means no active-generation migration or reinterpretation of old sats/inscription records is authorized. | A later implementation needs an explicitly separated Rana history; storage naming is a recommendation only. |

## Provisional recommendations and deferred work

P01: Use `rana` as singular and plural and `RANA` as the machine code, exactly as provisionally requested. Naming alone does not determine denomination; the user separately resolved Q01 through C43.

P02: One local process, one SQLite database, one serialized authoritative writer, one ordered journal, one locally served UI. The prompt explicitly makes the storage technology and process topology recommendations; the behavioral ordering/durability requirements remain separate.

P03: Stable display identifiers `demo-1` and `demo-2`, labeled as simulated. Identifier spelling is not an economic decision. Allocation quantities remain explicit configuration, not unapproved defaults.

P04: Retain source payload SHA-256 and deterministic IDs; do not add hash chaining, signatures, or external services. The limitations and replacement triggers are in `TECHNICAL-DEBT.md`.

P05: Preserve the existing request-driven evaluation boundary. Read-only Protocol View has no state-changing controls; backend evaluation remains authoritative even if a read causes a previously specified due transition. Pure snapshot reads versus evaluation reads must be described explicitly in the completed interface contract, not hidden in UI logic.

P06: An explicit demo configuration can choose minimum bid/increment, initial allocations, and extension window/increment/maximum within the declared rules. Their exact fixture values are not missing protocol semantics. No value is silently supplied here. Extension removal, automatic capture, a settlement timeout, or changed pause semantics would be specification changes, not convenience choices.

P07: After the user requested Q02 pros and cons, recommend reserving only the leading bid for this single-active-auction prototype, releasing outbid holds immediately and using replacement accounting for a same-identity raise. This minimizes simultaneous holds. Both principal models can fund a new bid from available Rana plus that identity’s replaced hold; neither grants promotion or payment authority over losing bids. The user subsequently accepted this recommendation as C45; it is restated in RT02.

P08: After the user requested Q04 implications, recommend release of the uncaptured winning reservation on the existing expired/PublicLand settlement branch. This avoids an additional forfeiture policy or a permanent terminal hold and does not refund completed capture or reverse title. The user subsequently accepted this recommendation as C46; it is restated in RT03.

F01: Exceptional compensation is deferred with no Prototype 1 operation. F02: Production identity, economic security, external attestation, shared deployment, and Bitcoin integration are excluded; potential future security work requires a demonstrated threat and separate scope decision.

## Phase 2 consequences and representation choices

The confirmed decisions above supply all economic alternatives selected in the candidate. The following are explicit mechanical consequences or representation choices used to make those decisions testable. They are not mislabeled as user economic decisions, and their addition is catalogued in SPEC-CHANGES.

| ID | Classification and choice | Basis / candidate owner |
|---|---|---|
| M01 | Wire vocabulary: `demo-1`, `demo-2`, `protocol`, `RANA`; exact record/type/key names; padded deterministic identifiers extended without truncation | C18/C19/C25; K14; P01/PR01/GL01. Fixed names implement already-decided roles, not new identities. |
| M02 | Four explicit Rana ledger record types: issue, reserve, release, capture; no second mutable balance ledger | C15/C19/C21/C43–C46; RT01 and SE06. |
| M03 | Full-release/full-reserve records implement a same-identity raise atomically with the approved net increment; no intermediate balance is exposed | C45/C25; RT02/SE05. No additional funds are required beyond available plus replaced hold. |
| M04 | SettlementRecord, one explicit ledger action, and FinalizationRecord form an ordered atomic terminal group; FinalizationRecord itself is the title record | C14/C17/C22/C25/C44/C46; K10; RT03–RT05/SE05. Avoids duplicate title authority. |
| M05 | Initialization is exactly two issue records and the first AuctionRecord in one group; zero reserved/protocol balances are explicit initial equations | C19/C25; K02; P01/RT01/G01. No implicit allocation on restart. |
| M06 | Group identifiers, member indices/sizes/types, and contiguous global record indices identify complete replay boundaries | C25; K13/K14; PR01/PR03/SE05. No committed-prefix gaps or partial groups are repaired. |
| M07 | Lossless command audit bytes are stored as lowercase UTF-8 hexadecimal inside canonical JSON; typed invalid fields are nullable; no free-form invalid reason replaces stable codes | C25/C30; K04/K17; SE02/SE03/SE06/PR02. Prevents malformed input or Unicode normalization from changing recorded commands. |
| M08 | Remove old destination/profile/wallet request fields; retain relevant rejection priority, adding unknown identity and insufficient available Rana after required shape checks / amount constraints | C08/C18/C21/C22/C45; K17; SE02/SE03. Exact schema changes are explicit. |
| M09 | One request/evaluation timestamp sampled at entry to serialized authoritative execution; no client or earlier transport timestamp drives a command | C06/C25; K05/K13/K16; SE04. This makes the source's authoritative server receipt boundary explicit. |
| M10 | ReadHistory projects the entire ordered journal, including global issuance; API revision is explicitly Rana prototype 1; views separate evaluation records from command records | C27–C31; K16/K19; SE02/UI02. No fabricated canonical rejection for endpoint/transport failures. |
| M11 | No pause/resume command or event in the new minimal command surface; no old-history import | C05/C18/C37; K18 supplies restrictions, not an old Demo 1 pause endpoint; SC03/PR06. No reachable source demo pause control is silently reimplemented. |
| M12 | Retain inert settlement deadline, capture its window at resolution, preserve request-driven local settled/expired control and extension configuration | K05/K06/K09/K16/K20; P02/SE03/SE04. No unapproved automatic settlement, expiry, or extension removal. |
| M13 | Explicit 100-rana demo fixture and exact illustrative request times; minimum/increment 10, window 10, extension increment 15, maximum 2, settlement window 60 | C19/C21/C22/C39; permitted configuration K06/K07/K20. UI04 is a specified demonstration, not silent runtime defaults. |
| A01 | Supported local operation assumes a suitable nondecreasing server clock and durable atomic storage. Observed backwards time, overflow, unreadable/invalid history, or uncertain local commit stops processing explicitly | C06/C16/C25/C34; K15/K18; SE04/PR03–PR06/SN01. Declared prototype assumptions, no production time or recovery infrastructure. |

Every retained kernel rule is restated in the owning candidate clauses identified in TRACEABILITY. Historical source descriptions above remain provenance; they cannot override the candidate's explicitly recorded changes or activate it.


## Approved revision 0.1 — 2026-09-05

C47 / S24: The user approved removal of the unused settlement deadline after reviewing the two-command settlement proposal. Remove settlement_deadline_seconds, settlement_window_seconds, settlement_deadline, their derivation and UI/example references. Settled captures the full winning hold into protocol-held Rana and assigns winner title; expired releases that hold and assigns PublicLand. AwaitingSettlement retains the hold until an explicit accepted command. Auction timing and economic predicates are unchanged. TD10 is retired before implementation; M12 is superseded only for the deadline. Owners: P02, RT03, SE03/SE04/SE06, PR05, UI01/UI02/UI04.

C48 / S25: In response to the combined simplify/activate/build request, the user said: “Yes I approve the the settlement simplification. With that I approve implementation of Prototype 1.” This activates numbers-spec revision 0.1 as the sole active authority. codex-spec is historical and untouched. Owners: STATUS, AU01. Earlier draft-only statements in this working record describe their historical checkpoint. Local implementation is authorized; no commit, push or publication was requested.

C49: Rana remains useful for this prototype, but need not remain a public concept or permanent internal standard for future payment integrations. Prototype 1 accounting is unchanged.


## Prototype 1 implementation checkpoint — 2026-09-05

The user requested Rust installation, clear backend design and notation for human and AI reviewers, and a complete development environment rather than shortcuts for speed. The local implementation is in `prototype-01/`, with a pinned standard Rust installation, formatter, linter, editor support, exact dependency locks and repeatable checks. This is an implementation and tooling choice; it adds no economic or protocol rule.

See [the backend review guide](../../prototype-01/REVIEW.md) for rule-to-code reading order and [verification evidence](../../prototype-01/VERIFICATION.md) for the automated checks, two browser settlement paths and restart comparison. The original repository files remain untouched.


## Demo configuration change — 2026-09-05

C50: The user requested “Can we make it increments of 5.” Set the running demonstration's `minimum_increment_rana` to 5. Under the existing SE03 predicate, this is a minimum increase of 5, not a new multiple-of-five restriction. The opening minimum remains 10. This is an authorized P02 parameter choice, not a change to the admission predicate or captured-history rule. Number 2 was already recorded with increment 10; the new value applies to subsequent AuctionRecords, beginning with Number 3 in this history. UI04's explicit 10-rana verification fixture is unchanged.


## C51–C52 — Fresh demonstration configuration, 2026-09-05

The user requested minimum_bid_rana=5, minimum_increment_rana=1, and 500 extra Rana for each bidder, then authorized returning to Number 1 if simpler. Use a separate empty demo journal, starting_number=1, initial allocations=600 each (original 100 plus 500), minimum5/increment1. Preserve the previous journal unchanged. This is existing P02 initialization and PR06 separate-history behavior, not a reset or reinterpretation of recorded facts. The proposed SE07/G11 funding amendment was withdrawn before implementation or use; revision 0.1 remains active with no later-issuance feature. C50's future five-rana increment is superseded for the new demo. UI04 retains its explicit historical test fixture.


## C53 — Auction label cleanup, 2026-09-05

The user requested removal of “THE NUMBER” and the “Title has not been assigned” badge. Remove that heading label and hide the empty title banner before resolution. The resolved-winner and final-title displays remain under UI01. This is presentation only; no auction or journal behavior changes. The user asked for device-font pros and cons before changing typography; the existing font choices remain pending that discussion.


## C54 — Device-native typography, 2026-09-05

The user explained that the number should feel universal by taking its visual form from the user's machine, and approved trying the device's system font. Set the site to `system-ui, sans-serif`, with the large number, headings, controls and journal text inheriting that family. Use normal letter spacing for the large number so the device font supplies its natural spacing. Sizing and layout remain presentation choices; journal values and all protocol behavior are unchanged. This resolves the font discussion left open in C53.


## C55 — Dark presentation and sentence case, 2026-09-06

The user requested a dark presentation and clarified the exact colors: background `#14110F`, text `#F7F5EF` at full opacity, and highlight `#E3C469`. All site text, including secondary labels, placeholders, results and disabled controls, uses the light color without opacity fading. Highlights mark controls, focus and status. Snippet boxes use the dark background, light text, a 2-pixel light border and square corners.

Use sentence case for presentation labels, including readable group, record and state names. Canonical snippets and exact submitted commands retain their original values and spelling under UI01; no CSS text transformation applies to them. The system font choice in C54 remains. These are presentation choices only; auction rules, parameters and stored records do not change.


## C56 — Thinner snippet borders, 2026-09-06

After viewing the dark presentation, the user requested a 1-pixel snippet border. This supersedes only C55's 2-pixel width; the light border color, dark fill and square corners remain. Presentation only.


## C57 — Basic presentation, 2026-09-06

The user requested the most basic design possible after trying their branding on Numbers. Replace C55–C56’s branded presentation with a white background, black system text, ordinary underlined links, native browser controls and a simple single-column layout. Remove decorative branding, accent colors, pills and animation. Snippets remain expandable, with a plain 1-pixel gray rectangular border. Preserve C53’s removed labels, C54’s device system font and sentence case for human-readable labels. Canonical snippets, commands, auction behavior, parameters and stored records remain unchanged. This is presentation only.


## C58 — Restore the presentation after C53, 2026-09-06

The user requested restoration to the version immediately after removing “THE NUMBER”. Restore the original light cream-and-green layout, its original typography (including the Georgia number and monospaced snippets), controls, labels and snippet treatment. Retain C53’s removed heading and hidden unassigned-title badge. This supersedes C54–C57’s later presentation experiments. The saved pre-dark stylesheet and browser code, together with the recorded C54 typography edit and C57 markup edit, establish the restoration point. This is presentation only; the current auction parameters, balances, journal, exact command handling and settlement behavior remain unchanged.


## C59 — GitHub checkpoint and project pause, 2026-09-06

After reviewing the proposed pause tasks, the user authorized all three: incorporate the nine existing GitHub maintenance updates, commit and push the specification and prototype with a handoff note, and separately back up the local demonstration histories. The updates were incorporated by fast-forward. Include the prototype in repository checks, preserving the existing historical-application jobs. This extends the earlier local-only authorization to this checkpoint commit and push; public hosting and payment integration remain outside scope. The local preview is stopped for the pause. The current and earlier journals are preserved, backed up separately from GitHub, and checked for matching ordered records. No protocol rule or journal entry changes.


## C60 — Frontend and backend demonstrations, 2026-09-07

The user approved all proposed Prototype 1 demonstration additions: retain the browser demo; add readable live terminal commands through the existing HTTP endpoints; add a guided walkthrough using the same Rust engine with a clearly labeled simulated clock and a separate journal; provide presenter notes, inspectable records and a VS Code workspace; demonstrate exact reconstruction in a separate process. This is demonstration tooling, not approval of Prototype 2, a new closing rule or a live time-control endpoint. Each explicit live command is sent once. Fresh demonstration runs use newly created directories and cannot reset or overwrite an existing journal. UI05 owns these presentation and fixture requirements.

## C61 — Ownership and Unowned terminology, 2026-09-07

The user requested ownership in place of the property-associated term “title”, expressing the Numbers question as “What if you can own a number?” After clarification, the user selected “Unowned — no owner” for the outcome formerly displayed as PublicLand. It does not mean collective ownership. Use ownership/Unowned in current explanatory documentation, code concepts and human-facing displays. Before finalization ownership is pending; a finalized Unowned result is distinct from that pending state.

This is terminology, not a change to winning, settlement, accounting, finality or external rights. Revision 0.1 serialized keys and values (including title, title_kind, holder_id, PublicLand and expired_to_publicland) remain exact compatibility spellings. Never rewrite saved records, hashes, API representations or submitted commands to update prose. Current documentation explains the mapping; historical provenance and frozen codex-spec preserve their original wording. RT04 and the glossary own the meaning.


## C62 — Commit and push the demonstration checkpoint, 2026-09-07

After trying the guided terminal demonstration and reviewing the code presentation, the user requested: “Make commits and push to github”. This authorizes committing the approved C60/C61 specification, implementation, presenter workspace, documentation and verification notes to the existing Numbers repository and pushing to its GitHub remote. Local journals, generated demonstration receipts, build outputs and machine configuration remain outside the source checkpoint. No auction behavior change or public deployment follows from this instruction.
