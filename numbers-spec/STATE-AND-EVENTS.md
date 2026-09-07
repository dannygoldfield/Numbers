# State, commands, and events

## SE01 — State derivation and allowed lifecycle

After each complete commit group, derive each auction's state from its records in the order below, subject to all record/group validation. Record presence does not excuse invalid prerequisites. There is no mutable canonical state field.

| State | Entry evidence | Permitted commands / evaluation | Exit / resulting records | Terminal? |
|---|---|---|---|---|
| Scheduled | AuctionRecord; no AuctionOpenRecord | SubmitBid; reads; invalid evaluated bids append BidRecord | First valid bid with RanaReserveRecord and AuctionOpenRecord in G02 -> Open | No |
| Open | AuctionOpenRecord; no AuctionCloseRecord | SubmitBid; reads; prescribed extension/close evaluation | Valid bid G03/G04 or invalid bid G05 keeps Open; due close G06 -> Closed | No |
| Closed | AuctionCloseRecord; no ResolutionRecord | Required deterministic resolution; reads; bids rejected before admission | G07 ResolutionRecord -> AwaitingSettlement | No |
| AwaitingSettlement | ResolutionRecord; no SettlementRecord/FinalizationRecord | SubmitSettlement; reads; bids rejected before admission | G08 settled or G09 expired -> Finalized | No |
| Finalized | Complete terminal group including FinalizationRecord | Reads; sequence eligibility evaluation; no further lifecycle command for this number | No same-number exit; G10 creates next number Scheduled after the gap | Yes |

A SettlementRecord without its same-group finalization and ledger effect is malformed history, not a pending state. Closed without resolution is a supported interrupted-evaluation boundary. An empty journal has no auction; G01 alone initializes it. No-bid Scheduled has no time-driven exit. Rhythm gap is sequence status, not an auction state. No other lifecycle states or transitions exist.

## SE02 — Command boundary and rejection classes

The API contract is `rana-prototype-1`; it does not assert compatibility with the former sats API. The entire endpoint surface is:

| Logical command | Endpoint and input |
|---|---|
| ReadState | GET /state, no parameters |
| ReadHistory | GET /auction/history; integer limit (default 50, range 1..100), integer offset (default 0, >=0); global record pagination |
| SubmitBid | POST /bid; JSON object with exactly auction_number, bidder_id, amount_rana |
| SubmitSettlement | POST /demo/settlement; JSON object with exactly auction_id, outcome |

The backend must decode valid UTF-8 and parse one JSON object for each POST. Duplicate object keys, invalid JSON, invalid UTF-8, or a non-object top level are transport rejection `malformed_request`, before admission; they produce no command record. Unknown endpoints/methods produce `unknown_endpoint`/`method_not_allowed`, and invalid read parameters produce `invalid_read_parameters`, also with no command record. No evaluator runs for those rejected transport/route/read-shape requests.

All other requests enter one serialized authoritative boundary. Required due evaluation SE04 precedes command validation. Rejection attribution must distinguish evaluation records from command records: an overdue auction can close/resolve before a rejected bid without that bid causing a close or consuming Rana.

SubmitBid when the evaluated current state is Closed, AwaitingSettlement, or Finalized is `auction_not_accepting_bids`: no BidRecord, no Rana or authority effect from the command, and the evaluated state remains. During a rhythm gap the latest number is Finalized. An absent usable state caused by PR06 failure is not a bidding opportunity.

A SubmitBid reaching Scheduled/Open admission always produces one valid or invalid BidRecord, unless persistence fails under PR06. Field and balance rejections follow SE03. Its recorded `command_utf8_hex` is lowercase hexadecimal encoding of the exact UTF-8 request-body bytes after any transport content decoding. This ASCII representation preserves submitted text, numeric spelling, and Unicode without normalization changing the command. It is not a second command or a separate economic event.

SubmitSettlement outside AwaitingSettlement is `settlement_not_available`. Within that state, check exactly this priority: missing required field -> `missing_required_field`; unknown key, wrong primitive type, or malformed auction_id -> `malformed_field`; auction_id unequal to the current auction_id -> `wrong_auction`; outcome outside settled/expired -> `invalid_settlement_outcome`. These command rejections write no canonical command record, change no Rana, ownership, or permission, and leave evaluated state unchanged. A valid terminal command follows RT03; an inconsistent winning hold is a PR06 halt, never a command-selected expired result.

The client and backend must not automatically retry a command. A later explicitly submitted request is a new command evaluated against current history. There is no request deduplication or replay permission; unique terminal preconditions prevent a second settlement. A lost response is resolved by reading/reconstruction, not automatic resubmission.

## SE03 — Bid evaluation, extension, and resolution

At admission, derive the current high bid from valid BidRecords only: greatest amount_rana, then least global sequence_index. The fallback tie rule is mandatory even though the positive increment disallows a new equal high bid. Invalid records never participate.

Evaluate the following rejection conditions in exactly this order. Every rejection in this table appends G05, gives validity=invalid and the listed code, changes no hold/ownership/permission, and leaves the pre-admission Scheduled/Open state and timing unchanged.

| Priority | Failed condition | rejection_reason |
|---|---|---|
| 1 | auction_number parses as an exact integer and does not equal the current number | wrong_auction_number |
| 2 | A required key is absent | missing_required_field |
| 3 | Unknown key; auction_number or amount_rana is not an exact JSON integer token; bidder_id is not a string | malformed_field |
| 4 | bidder_id is neither demo-1 nor demo-2 | unknown_identity |
| 5 | amount_rana < captured minimum_bid_rana | amount_below_minimum |
| 6 | Open and amount_rana < high.amount_rana + captured minimum_increment_rana | amount_below_required_increment |
| 7 | Captured maximum is non-null and amount_rana exceeds it | amount_above_maximum |
| 8 | RT02 replacement affordability fails | insufficient_available_rana |

An integer token has decimal integer syntax with no fractional part, exponent, string wrapper, or boolean substitution. Negative integers parse but fail the applicable positive amount/minimum or current-number check. No float rounding, truncation, or automatic bid increase is permitted. Unknown/missing values remain inspectable by decoding command_utf8_hex; typed BidRecord fields are null when their primitive cannot be parsed as specified. If every check passes, validity=valid, rejection_reason=null, and the bid is the new leader.

If state before admission was Scheduled, G02 sets opened_at to t and base_end_time to t + captured duration_seconds. No extension is evaluated for that opening bid.

If state before admission was Open, compute old effective end `E = base_end_time + extension_increment_seconds * extension_count` before this bid's effects. SE04 already requires t < E. An accepted valid bid triggers exactly one extension when `t >= E - extension_window_seconds` and extension_count < max_extensions; use G04. Otherwise use G03. Invalid bids never extend. Every extension adds the captured increment to E; base_end_time never changes. There is no cap-close trigger.

When Closed is evaluated without ResolutionRecord, choose the winning valid bid by the ordering above and append G07. A closed auction with no valid bid is malformed history and halts. Persist the winning reference/amount, resolution time t, and the PR02 resolution-input hash. An existing resolution must not be replaced or rerun as an outcome decision. The recorded winner governs settlement even when ownership becomes Unowned.

## SE04 — Authoritative time and sequence evaluation

Every supported request boundary and startup after successful reconstruction uses the same serialized evaluator. Authoritative server receipt is the point a request enters this evaluator's exclusive execution, not a browser timestamp or earlier transport arrival. Sample one backend UTC millisecond timestamp t at that boundary. All records generated by that evaluation and command use t; use no new client-supplied or intra-group timing source. Canonical order is commit order, never inferred timestamp sorting.

At the boundary, complete eligible steps in this order:

1. Empty history: append G01 using validated initialization inputs.
2. Current Open with `t >= effective_end`: append G06 using closed_at=t, not a backdated deadline.
3. Current Closed without resolution: append G07 using t.
4. Latest Finalized with no next auction and `t >= finalization_time + captured inter_auction_gap_seconds`: append G10 using t.
5. Evaluate the requested command or form the requested read projection.

Each listed group is a separate durable commit; do not combine close and resolution or initialization and the user's bid. Following a successful terminal command, the gap begins at that command's recorded finalization time. The next boundary after the 12 seconds creates the next number by exactly +1. It remains Scheduled until its first valid bid.

Time passage alone writes no record between evaluation boundaries. No background scheduler is required. Local settlement has no deadline and never auto-expires, including after restart. At equality with the effective auction end, close precedes admission; a late bid cannot rescue or extend the auction.

The supported time assumption is one controlled server clock; no protection against clock manipulation is claimed. A backward timestamp relative to the latest committed server_time is contradictory timing and must halt before writing under PR06; the backend must not clamp, backdate, reset, or repair time. A later forward timestamp is evaluated by the explicit conditions above. Arithmetic or timestamp overflow must fail explicitly before commit, never wrap or round.

## SE05 — Complete atomic commit groups

The table is exhaustive. All records within a group use the same number/auction_id and server_time, except the two global issuance entries in G01. PR03 envelope group fields must describe exactly these ordered records. No other record interleaves. RT invariants apply to complete groups; their specified internal order does not expose an intermediate state to another command or observer.

| ID / group_type | Preconditions | Exact record order | Complete consequence |
|---|---|---|---|
| G01 / initialize | Entire journal empty; valid config | RanaIssueRecord(demo-1), RanaIssueRecord(demo-2), AuctionRecord(starting_number) | Initial allocations and zero protocol balance; Scheduled, no hold/ownership |
| G02 / bid_open | Scheduled; bid valid | BidRecord, RanaReserveRecord, AuctionOpenRecord | Open, new leader and full hold; fixed base end |
| G03 / bid_replace | Open; bid valid; no extension due | BidRecord, RanaReleaseRecord(old hold), RanaReserveRecord(new bid) | Open, new leader/full hold, old hold ended |
| G04 / bid_extend | Open; bid valid; extension due | BidRecord, RanaReleaseRecord(old hold), RanaReserveRecord(new bid), ExtensionEventRecord | G03 consequences plus one bounded extension |
| G05 / bid_rejected | Scheduled/Open admission; bid invalid | BidRecord | Audit fact only |
| G06 / close | Open; t >= effective_end; no close | AuctionCloseRecord | Closed; existing hold unchanged |
| G07 / resolve | Closed; no resolution; at least one valid bid | ResolutionRecord | AwaitingSettlement; winning result fixed; hold unchanged |
| G08 / settle | Valid settled command and RT03 hold | SettlementRecord, RanaCaptureRecord, FinalizationRecord | Finalized, winner ownership, protocol credit, no hold |
| G09 / expire | Valid expired command and RT03 hold | SettlementRecord, RanaReleaseRecord, FinalizationRecord | Finalized, Unowned outcome, released winner funds, no hold |
| G10 / advance | Latest finalized, gap elapsed, next absent | AuctionRecord(previous number + 1) | Next Scheduled, no new allocation, old ownership unchanged |

## SE06 — Exact event schemas and effects

PR01 defines the common envelope. The payload fields below are required and closed; only explicitly nullable fields accept null. All references must resolve to earlier records, including earlier members of the same atomic group, with consistent number, identity, and amount. All event types occur only in their listed SE05 groups. State/Rana/ownership consequences not explicitly assigned are none.

| Event | Exact required payload fields | Preconditions and references | Consequences / ordering |
|---|---|---|---|
| RanaIssueRecord | bidder_id: identity; amount_rana: nonnegative integer | G01 only; first demo-1 then demo-2; configured amounts; global envelope number/auction_id null | Increase recipient available by amount; no ownership; two entries exactly once |
| AuctionRecord | created_at: timestamp; duration_seconds, inter_auction_gap_seconds, extension_window_seconds, extension_increment_seconds, max_extensions, minimum_bid_rana, minimum_increment_rana: integers; maximum_bid_rana: integer or null | G01/G10 only; created_at=t; values obey P02 and snapshot current auction inputs; number and auction_id follow PR01 | Create Scheduled only; no Rana movement or ownership |
| BidRecord | bid_id: ID; command_utf8_hex: string; submitted_auction_number: integer or null; bidder_id: string or null; amount_rana: integer or null; validity: valid/invalid; rejection_reason: string or null | G02–G05; bid_id from own sequence; typed fields parsed from its decoded UTF-8 command as in SE03; valid iff admission passes; invalid reason is first failed check | Valid bid participates in leadership/resolution only through its complete group; invalid is audit only; no ledger movement by the BidRecord alone |
| RanaReserveRecord | bid_id: ID; bidder_id: identity; amount_rana: positive integer | G02/G03/G04; references the same-group valid bid; identity/amount equal bid; no other live hold after preceding release; sufficient available funds | Available -= amount; reserved += amount; this record_id identifies the live hold; no ownership |
| RanaReleaseRecord | reservation_record_id: ID; bid_id: ID; bidder_id: identity; amount_rana: positive integer; reason: outbid/replaced/settlement_expired | G03/G04: reference old live hold; reason replaced if new bidder equals old, otherwise outbid. G09: reference resolved winner hold and reason settlement_expired. All fields match the referenced hold | Reserved -= amount; available += amount; hold ended permanently; no ownership effect by release alone |
| RanaCaptureRecord | reservation_record_id: ID; bid_id: ID; bidder_id: identity; amount_rana: positive integer; recipient: protocol | G08 only; same-group SettlementRecord.status=settled; references live winning hold; amount and identity exactly match ResolutionRecord/winning bid | Reserved -= amount; protocol_held += amount; hold consumed; no ownership until following FinalizationRecord in group |
| AuctionOpenRecord | opening_bid_id: ID; opened_at: timestamp; base_end_time: timestamp | G02; bid reference equals same-group first valid bid; opened_at=t; base_end_time=t+duration_seconds | Open; fixed time; no separate Rana/ownership effect |
| ExtensionEventRecord | trigger_bid_id: ID; extension_increment_seconds: positive integer; extension_index: positive integer | G04; reference same-group bid; increment matches snapshot; index equals previous count+1 and <=max_extensions; SE03 window predicate true | Add exactly one increment to derived end; remain Open; no Rana/ownership effect |
| AuctionCloseRecord | closed_at: timestamp; reason: duration_expired | G06 only; closed_at=t; effective-end predicate met; unique per auction | Closed; valid bidding stops; hold preserved; no ownership |
| ResolutionRecord | winning_bid_id: ID; winning_amount_rana: positive integer; resolution_time: timestamp; resolution_inputs_hash: hash | G07; winner is SE03 choice; amount matches bid; resolution_time=t; hash PR02 | Fixed winner, AwaitingSettlement; hold unchanged; no ownership |
| SettlementRecord | command_utf8_hex: string; resolution_record_id: ID; status: settled/expired; settlement_time: timestamp | G08/G09; command_utf8_hex decodes to the exact UTF-8 body of the accepted SubmitSettlement; resolution reference is unique prior result; status matches command; settlement_time=t | Record terminal outcome only; associated ledger/ownership effects are required later in this same group |
| FinalizationRecord | settlement_record_id: ID; title_kind: winner/PublicLand; holder_id: identity or null; finalization_time: timestamp; finalization_reason: settled_to_winner/expired_to_publicland | G08/G09 only; reference same-group SettlementRecord; t matches group; settled -> winner, winning identity, settled_to_winner; expired -> PublicLand, null, expired_to_publicland | Sole final ownership outcome, terminal Finalized, start rhythm gap; no additional economic movement |

Every event name not in this table is forbidden. No separate correction, authority, command-submission, pending-settlement, ownership-transfer, or ambiguity event is implied.
