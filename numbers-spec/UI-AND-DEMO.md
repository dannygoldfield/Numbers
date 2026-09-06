# Interface and demonstration

## UI01 — Two non-authoritative views

Auction View and Protocol View must be served by the same local application and support side-by-side display. The backend performs SE evaluation and returns canonical or mechanically derived facts. The browser must not determine validity, authoritative balances, winner, settlement, or title.

Auction View must show current number/state, a visibly simulated identity selector for demo-1/demo-2, both available/reserved balances for the selected identity, current leading bid, bid input, plain-language acceptance/rejection, the derived 3:45 base countdown, extension changes, fixed winner, settlement status, title assignment, 12-second rhythm pause, and next number.

Auction View must offer the explicit local settlement commands when AwaitingSettlement. Label them “Simulate successful settlement” and “Simulate failed settlement”. They submit settled and expired respectively; expired is only the retained machine label. They do not prove outside payment failure, identity, or human fault. There is no settlement timer or deadline display. The backend alone validates them and performs RT03.

Use typography, rhythm, visible state changes, and a clear title-assignment moment. Animation cannot create an outcome or imply title before its record. No production identity or external ownership claim is permitted.

Protocol View must be read-only: no bid, settlement, title, editing, reset, or other mutating control. It must display the submitted command representation, backend validation/result, resulting records grouped by global order, derived auction state, ledger movements, available/reserved and protocol-held balances, settlement, title, payload hashes, and latest reconstruction receipt. Canonical records must be shown without reinterpretation. Display the submitted command by decoding command_utf8_hex to UTF-8, alongside the unchanged canonical payload; this display decoding creates no new fact. Transient command errors with no canonical record must be labeled as unrecorded command results, never fabricated as events.

## UI02 — Fixed response contract

All endpoints use exactly one envelope: success `{status: success, data: ...}` or failure `{status: error, error: {code, message}, evaluation_records: [...]}`. Here strings such as success/error are literal JSON strings. Error codes are SE02/SE03 codes or storage_unavailable, history_invalid, clock_invalid, arithmetic_unrepresentable. Failed requests must not imply a commit failed if durable outcome is uncertain; their message must state that uncertainty and require inspection. No canonical error record is implied.

- ReadState data is the projection below.
- ReadHistory data is `{records, pagination: {limit, offset, next_offset}}`, containing PR01 canonical records ordered by global sequence_index, including global issuance and invalid bid records. next_offset is null when the selected page reaches the journal end. Pagination uses SE02. ReadHistory runs SE04 first; it is a global journal inspection despite its retained endpoint name.
- SubmitBid data is `{accepted, bid_record, evaluation_records, command_records, state}`. Admission invalidity is success with accepted=false and its invalid BidRecord. Valid acceptance is success with accepted=true. bid_record must be the exact persisted BidRecord; command_records is exactly its complete group. state is the post-command projection. A pre-admission rejection uses the error envelope instead and has no bid_record.
- SubmitSettlement data is `{accepted, evaluation_records, command_records, state}` with accepted=true only after durable G08/G09. command_records is exactly that terminal group. Failed commands use the error envelope.

`evaluation_records` lists only records appended by SE04 before processing this request's command; it is empty if none. They are not attributed to the command. `command_records` lists the requested command's own committed group. Each records array contains the full PR01 envelope and payload. UI must distinguish an audit-only invalid BidRecord from an endpoint rejection with no command record.

A state projection contains exactly:

| Field | Value and derivation |
|---|---|
| server_time | This request's SE04 authoritative boundary timestamp |
| unit | RANA |
| current_number, auction_id | Latest AuctionRecord |
| auction_state | SE01 derived state |
| parameters | Current AuctionRecord payload excluding created_at |
| opened_at, base_end_time | AuctionOpenRecord or null before opening |
| extension_count | Count of current auction ExtensionEventRecord entries, explicitly 0 if absent |
| current_end_time | Base end plus recorded extension increments; null while Scheduled |
| closed_at | AuctionCloseRecord.closed_at or null |
| leading_bid | `{bid_id, bidder_id, amount_rana}` from SE03 high bid, or null |
| resolution | `{record_id, winning_bid_id, winning_bidder_id, winning_amount_rana, resolution_time}` from the immutable resolution and its winning bid; otherwise null |
| settlement | `{record_id, status, settlement_time}` from SettlementRecord, otherwise null |
| title | `{record_id, title_kind, holder_id, finalization_time}` from FinalizationRecord, otherwise null |
| balances | Two objects in demo-1, demo-2 order, each `{bidder_id, available_rana, reserved_rana}` under RT01 |
| protocol_held_rana | RT01 derived balance |
| sequence | `{phase, next_number, next_available_at}`: phase is auction_available for Scheduled, auction_active for Open/Closed/AwaitingSettlement, rhythm_gap for latest Finalized. Only rhythm_gap has next_number=current_number+1 and next_available_at=finalization_time+captured gap; otherwise both are null |
| last_sequence_index, last_group_id | Last committed record's sequence and group ID |
| reconstruction | `{validated_through_sequence_index}` from last successful PR04 receipt; new runtime appends do not pretend to have undergone a subsequent restart |

All defined fields must be present; not-yet-applicable facts use the explicit nulls above. No projection succeeds before successful validation and required startup initialization. No other fields or speculative facts are part of this contract. Selection of a UI identity is local display state and is not canonical state.

## UI03 — Clock, reads, and commands

Countdown display must derive from backend server_time and current_end_time with locally elapsed display time; it must clamp displayed remaining time at zero. A UI reaching zero does not close the auction or assign a winner. Before the first valid bid, show dormant status and the 3:45 base-duration label without running a countdown. If extensions occur, show the backend-derived new end rather than silently claiming an absolute 225-second maximum.

The rhythm display must derive from recorded finalization and next_available_at. The UI must wait for an actual next AuctionRecord before displaying the next number as available. The next countdown stays dormant until its own opening record.

Reading state/history invokes the backend's explicitly prescribed SE04 evaluation. Protocol View still has no controls choosing canonical state: its read cannot select a bid, settlement outcome, title, or transition predicate. UI polling frequency is an observation choice; no animation or client clock becomes a backend trigger rule.

Each explicit UI submission sends its command once. The UI must show the authoritative acceptance/rejection and refresh the derived state and Protocol View after the response. Disable accidental repeat submission while awaiting that response. A failed/lost response must prompt state inspection; it must not initiate automatic replay. Human-readable messages must identify the rule failure and whether canonical records were written. The backend result, not optimistic UI updates, determines displayed balances and title.

## UI04 — Required deterministic demonstration

The exact fixture below is an explicit test/demo configuration, not a runtime default or production parameter declaration. A prototype implementation must demonstrate these expected outcomes, including the ledger and record order, and the restart cases in PR05. Times are seconds relative to the first opening; actual records use absolute UTC millisecond timestamps. Submit otherwise valid commands at these boundaries.

Fixture: starting_number=1; initial allocations=100 rana each; duration_seconds=225; inter_auction_gap_seconds=12; minimum_bid_rana=10; minimum_increment_rana=10; maximum_bid_rana=null; extension_window_seconds=10; extension_increment_seconds=15; max_extensions=2. Initialize at t=-1.

| Boundary | Action / record sequence indices | Expected derived consequence |
|---|---|---|
| -1 | G01: issues 1–2, AuctionRecord 3 | Number 1 Scheduled; A=demo-1 and B=demo-2 each available 100/reserved 0; protocol 0 |
| 0 | A bids 20: Bid 4, Reserve 5, Open 6 | A available 80/reserved 20; leader A20; base/effective end=225 |
| 30 | B bids 30: Bid 7, Release 8, Reserve 9 | A100/0; B70/30; leader B30 |
| 45 | A bids 30: invalid Bid 10 | amount_below_required_increment; balances/leader/end unchanged |
| 60 | B raises to 40: Bid 11, Release 12, Reserve 13 | B60/40; same-identity replacement uses only 10 more Rana |
| 90 | A bids 50: Bid 14, Release 15, Reserve 16 | A50/50; B100/0; leader A50 |
| 225 | Read boundary: Close 17, Resolution 18 in separate groups | Winner A50 fixed; AwaitingSettlement; title unassigned |
| 226 | Submit settled: Settlement 19, Capture 20, Finalization 21 | A50/0; B100/0; protocol 50; winner title A; next available at 238 |
| 237 | Read only | Same final title; gap still pending; no next auction |
| 238 | Read boundary: AuctionRecord 22 | Number 2 Scheduled; no new issuance or countdown |
| 240 | B bids 10: Bid 23, Reserve 24, Open 25 | B90/10; number 2 effective end=465 |
| 465 | Read boundary: Close 26, Resolution 27 | Winner B10 fixed; AwaitingSettlement |
| 466 | Submit expired: Settlement 28, Release 29, Finalization 30 | B100/0; A50/0; protocol remains 50; number 2 title PublicLand; winner result still B10 |
| 478 | Read boundary: AuctionRecord 31 | Number 3 Scheduled, no hold, no countdown; prior titles unchanged |

The first auction demonstrates every requested competitive and economic action. The second demonstrates the actual permitted PublicLand path with release rather than forfeiture. Losing reservations were released when outbid; settlement must not release them twice.

Additional acceptance cases are mandatory specifications of verification, not new UI controls:

- On a separate fresh fixture history, A bids 10 at t=0; B bids 20 at 214 (no extension); A bids 30 at 215 (end becomes 240); B bids 40 at 230 (end becomes 255); A bids 50 at 245 (maximum two extensions already reached, so end stays 255). A submission at 255 is preempted by close. With an explicit max_extensions=0 fixture, no valid bid extends.
- With the main fixture, a well-formed bid above the bidder's affordable replacement amount is an invalid BidRecord with insufficient_available_rana, assuming no higher-priority rejection applies; it changes no hold. Rejected first bids leave Scheduled indefinitely.
- Restart at every PR05 material boundary, particularly first-bid group, extension group, Closed before resolution, and both terminal groups. Verify identical replayed balances, references, title, and event prefix before any new SE04 evaluation.
- A corrupt hash or illegal/incomplete group must halt. A committed settlement with a lost response must not be captured again. Repeated settlement submission after finalization produces no new terminal command records.

No UI control for time travel, corruption injection, clearing history, or forced transition is authorized by these verification cases.
