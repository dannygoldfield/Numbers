# Persistence and reconstruction

## PR01 — One ordered journal

Canonical truth is one durable, append-only journal of the SE06 records. No application operation can modify, delete, reorder, reinterpret, or replace a canonical record. Mutable database columns, caches, logs, UI values, or current configuration cannot replace this truth.

Every record must have exactly these common envelope fields:

| Field | Exact meaning |
|---|---|
| sequence_index | Positive integer; contiguous committed sequence starting at 1; each next record adds exactly 1 |
| record_id | `rec_` plus sequence_index as decimal, zero-padded to at least 12 digits, never truncated |
| record_type | Exact SE06 event name |
| number | Positive auction number; null only for RanaIssueRecord |
| auction_id | `auc_` plus number padded as above; null exactly when number is null |
| server_time | SE04 boundary timestamp |
| group_id | record_id of this group's first record |
| group_type | Exact SE05 group_type |
| group_index | Position in group starting at 1 |
| group_size | Total records in this group |
| payload_json | Exact SE06 payload object |
| payload_hash | PR02 payload hash |

A BidRecord's bid_id is `bid_` plus that record's padded sequence_index. Sequence values must be assigned inside the serialized commit boundary before deriving IDs. A rolled-back allocation never became a canonical ID; committed IDs are never reused. Database row IDs, random IDs, process memory, and timestamps cannot determine canonical ordering or identity. Global issuance records precede the first auction and are not falsely assigned to an auction.

## PR02 — Canonical values and hashes

Economic values and sequence/number arithmetic must be exact integers. Payloads contain only schema-defined objects, arrays, strings, integers, booleans, and null; floating values are forbidden. The ASCII hexadecimal command representation can preserve rejected numeric syntax without turning it into a canonical numeric value. command_utf8_hex must decode to valid UTF-8 and the original JSON object accepted by the transport gate. Validate parsed typed fields and decisions against those decoded bytes; do not normalize the command before parsing. This also prevents NFC normalization from changing invalid input or merging distinct submitted keys.

Canonical payload bytes must be UTF-8 JSON with object keys sorted by Unicode code point, no insignificant whitespace, strings normalized to NFC, arrays in specified order, integers with no leading zeros or positive sign, and exact JSON booleans/null. Strings must escape quotation mark and reverse solidus. Use the short JSON escapes for backspace, form feed, newline, carriage return, and tab; other U+0000..U+001F use lowercase four-digit hexadecimal escapes. Emit non-ASCII text as UTF-8 after NFC normalization; do not escape solidus. Duplicate keys are invalid. The closed schema keys are ASCII.

Timestamps must be UTC `YYYY-MM-DDTHH:MM:SS.mmmZ`, with a valid calendar date, millisecond precision, and no leap-second representation. Timestamp arithmetic must preserve exact millisecond values. Do not use floats for deadline comparisons.

`payload_hash = lowercase hexadecimal SHA-256(canonical payload bytes)`. The hash covers payload_json only, not its envelope. The stored payload_json representation must match the canonical serialization; reconstruction must recompute every payload hash and reject mismatches.

The resolution_inputs_hash is SHA-256 of the canonical serialization of an array of all valid BidRecords for that auction, ordered by sequence_index ascending. Each object contains exactly bid_id, sequence_index, amount_rana, bidder_id, and server_time. Invalid bids are excluded. The immutable resolution record is not replaced during hash validation.

These hashes are consistency checks, not tamper-proof evidence against a machine owner.

## PR03 — Atomicity and durable success

SE05 defines indivisible commit groups. All group fields, payloads, IDs, sequence allocations, ledger effects, and required dependent records must commit together or none may commit. No unrelated record interleaves. An observer, command, or reconstruction sees only whole committed groups. An incomplete group is malformed history and halts; it is never treated as a valid prefix to finish.

Success/acceptance must not be returned before durable commit. Durability must survive process crash, ordinary restart, and machine restart under the supported local storage assumptions. Volatile-only or best-effort persistence is insufficient. If a response is lost after commit, the committed records remain authoritative and must not be appended again.

Close and resolution deliberately form separate complete groups, so an interrupted evaluator can leave a valid Closed boundary. Initialization's two allocations and first auction are one group, preventing reissuance or partially initialized identities. Finalization's ledger action and title form one group, preventing title without the required Rana effect.

## PR04 — Validate and reconstruct before evaluation

Startup must validate P02 configuration, then load the complete committed journal. Before any request or new event, it must validate and fold all groups in sequence order:

1. Validate exact envelope/payload schema, canonical serialization, hashes, deterministic IDs, continuous sequence/group indices, whole group sizes/types, and valid timestamps.
2. Validate the permitted SE05 group shape and SE06 references, per-number prerequisites, uniqueness, and group timestamp consistency. Committed server_time must be nondecreasing. A group with a valid hash but illegal semantics is still invalid.
3. Fold explicit RT ledger effects, verify nonnegative balances, hold references, and conservation after each group. Validate exactly two initial issues in the initialization group; no later issuance is permitted.
4. Derive state by SE01 and validate the captured auction timing/bid rules, effective end and extension causes, close condition, and contiguous number creation only after the previous finalization and gap.
5. Validate recorded resolution references/amount/input hash against the valid-bid history and deterministic ordering as a consistency check. Do not create a new resolution or replace its fixed result. Validate settlement/ledger/title binding, correct terminal outcome, and no outstanding hold after finalization.

If the journal is empty, it is a valid uninitialized store; no balances or auction are asserted until G01. Absence of a not-yet-eligible future record is valid. Absence of a required member/prerequisite of a committed group is invalid. In particular, Closed without ResolutionRecord is valid; SettlementRecord without its ledger effect and FinalizationRecord is not.

Reconstruction must perform no writes, bid replay, new capture, release, title assignment, initial allocation, repair, or deadline adjustment. On successful completion, expose a non-canonical reconstruction receipt identifying the highest validated sequence index (0 for empty history) and successful validation. Then invoke SE04 as a distinct startup evaluation using the current server time. New permitted records from that evaluation are new history, not reconstructed or repaired old history.

## PR05 — Material restart boundaries

The following is the complete supported boundary behavior; all outcomes require valid history and PR04 validation first.

| Durable boundary | Reconstructed facts | Next permitted evaluation |
|---|---|---|
| Empty journal | No allocation or auction yet | G01 initialization |
| After G01 | Both allocations and Scheduled auction; no hold | Reads or explicit bids; never reissue |
| After invalid bid | Invalid audit record and unchanged balance/lifecycle | Ordinary current-state commands |
| Before/after first bid group | Entire G02 absent or present; no half-open/half-funded auction | If present Open, otherwise Scheduled; do not replay lost bid |
| After replacement/extension | Exactly latest committed leader/hold/end | Ordinary Open rules, including close if now due |
| Open while deadline passes during downtime | Original opening/base/extension records and same hold | G06 then G07 if due; no backdated close or invented bid |
| Closed before resolution | Closed and unchanged hold | G07 once; captured resolution time is this new evaluation |
| After resolution | Fixed winner and winning hold | Explicit settlement command only; never automatic local expiry |
| During terminal group write | Whole G08/G09 committed or no terminal group | Use existing outcome if committed; otherwise remain AwaitingSettlement with its prior hold; never replay the command automatically |
| Finalized within rhythm gap | Same terminal settlement, balances, title | Remain finalized until next eligible evaluation |
| Finalized after gap with next absent | Same terminal facts | G10 once, no opening or allocation |
| After next AuctionRecord | Next Scheduled; previous title unchanged | First valid bid needed again |
| Complete commit with lost response | Committed effects unchanged | Read them; no implicit resubmission |

Reconstruction of the same committed journal yields identical authoritative state. Current wall time and observational countdowns can differ; explicit post-reconstruction evaluation can then append new permitted events. This is not a promise that time stopped during downtime.

## PR06 — Interrupted writes, malformed history, and explicit halt

Before admission, known unavailable persistence must reject processing as `storage_unavailable`, without admission or command records. No command is accepted on an inability to record it.

If writing a group fails, return explicit failure if possible and stop authoritative processing. Do not acknowledge acceptance, attempt a partial substitute, or automatically retry. If commit status is uncertain, treat the result as unknown to the caller: halt writes and reconstruct the durable store before further processing. This is inspection of local atomic truth, not replay of an uncertain external action.

A process interruption before commit leaves no new complete group. A complete commit is replayed as fact. A visible incomplete group, bad hash, missing prerequisite, illegal ordering, duplicate terminal record, negative balance, contradictory title, unreadable journal, or invalid time must halt with a durable non-authoritative diagnostic when storage permits. If diagnostic storage is also unavailable, surface that failure; do not invent a canonical error entry. The diagnostic must identify the failing record/group when known and state that no repair was performed.

Malformed history must not be edited, skipped, quarantined out of replay, truncated, supplemented with guessed events, or routed to PublicLand. Existing facts and one-shot limits remain. Repeated restart does not make invalid history valid. After a valid reconstruction, only a new explicit command or SE04 permitted evaluation can proceed; no automatic bid/settlement retry exists.

The application must provide no operation for clearing a journal, resetting allocations, or replacing an existing history. An explicitly initialized separate empty local store is a separate demo history, not a continuation or recovery of another store. This specification defines no migration from the superseded sats generation and no import of its records.
