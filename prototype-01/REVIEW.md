# Reading and reviewing the backend

Start with the [worked auction](../numbers-spec/UI-AND-DEMO.md), then follow the code in request order. A reviewer should be able to identify a rule, its implementation and the evidence for its behavior without reconstructing this conversation.

## Reading order

| File | Responsibility | Specification owner |
|---|---|---|
| `src/main.rs` | Validate operational setup; open the store; evaluate startup; own the serial HTTP loop | P02, SE04, PR04 |
| `src/http.rs` | Reject malformed transport and unsupported routes before evaluation; sample authoritative time | SE02, SE04 |
| `src/store.rs` | Evaluate due groups; commit each atomically; return outcomes only after commit; halt on failures | SE04–SE05, PR03–PR06 |
| `src/model.rs` | Derive state, check ordered admission predicates, plan exact groups, reconstruct and project | P02, RT01–RT05, SE01–SE06, PR04, UI02 |
| `src/values.rs` | Preserve command bytes, exact integers, canonical JSON, hashes and timestamps | PR01–PR02 |
| `tests/protocol.rs` | Worked examples, rejection boundaries, reconstruction, corrupted history and killed writers | UI04, PR05–PR06 |

The browser files are under `static/`. They submit commands once and display backend facts. `tests/browser-values.cjs` checks that presentation does not round large amounts or turn numeric-looking strings into numbers.

## The core path

```text
request
  → transport gate
  → sample server time inside the single-writer loop
  → commit any due initialization / close / resolution / advancement groups
  → plan the command's exact group against committed state
  → check its complete folded result
  → commit all its records in one SQLite transaction
  → expose the committed state and response
```

`plan` is pure. It describes the required records and never writes them. `plan_bid` and `plan_settlement` keep the two command paths separately readable. `rejection` follows the specification's exact priority order. `State::fold` applies ledger facts and verifies nonnegative balances, conservation and the live hold.

SQLite is the durability boundary. The in-memory State is a derived cache that changes only after commit succeeds. The companion writer lock is separate from SQLite's locking, which matters on macOS. The application never retries a failed transaction.

## Reconstruction is a check of existing facts

`reconstruct` reads complete groups in canonical order and compares them with the exact group permitted by their historical inputs. This validates the recorded decision; it does not re-submit a command or append the comparison result. Captured configuration comes from AuctionRecords and issuance records. Only a separate subsequent evaluation may create new due facts.

The reconstruction receipt names the highest sequence actually validated at startup. Later appends do not change that receipt to imply a restart occurred.

## Representation choices

Records use explicit JSON objects to keep field names directly comparable with SE06. A record is accepted only if its entire canonical representation equals the permitted group, including fields, references, IDs, timestamps and payload hashes. Valid hashes alone are insufficient.

Economic integers use `BigInt`. Raw command fields retain their submitted numeric syntax until admission, so `10`, `10.0`, `1e1`, `true` and `"10"` remain distinguishable. Original UTF-8 bytes are preserved in the specified hexadecimal field. Payload strings use NFC only during canonical serialization.

The journal stores one canonical record JSON string per ordered row. SQLite transactions enforce whole groups, and triggers reject updates and deletes through ordinary SQL operations. These controls do not make a machine owner unable to replace the database.

## Notation and change discipline

Use a short rule reference when it helps locate authority: `RT02` for reservation replacement, `G08` for successful settlement, `PR04` for reconstruction. Comments should explain intent, ordering or an otherwise surprising constraint. They should not restate an assignment or invent a new rule.

Keep event fields vertically readable. Prefer meaningful local names and small functions with one responsibility. Avoid hidden defaults, retry helpers, speculative abstractions and catch-all recovery. If a change affects economics, ownership, time, ordering or history, identify its specification clause before changing code.

A review should follow one accepted bid through the reservation group and one settlement through the terminal group, then inspect the corresponding tests. Both success and failure must preserve the fixed winner; only the specified ledger effect and ownership outcome differ.


## Demonstration code — UI05

`Numbers-demo.code-workspace` opens this application and the active specification together. [DEMO.md](DEMO.md) maps presenter moments to rules and functions.

| File | Responsibility |
|---|---|
| `src/bin/numbers-demo/main.rs` | Command selection and help |
| `src/bin/numbers-demo/live.rs` | Existing HTTP commands, exact numeric spelling, one submission per command |
| `src/bin/numbers-demo/guided.rs` | Explicit UI05 fixture and separate-process reconstruction comparison |
| `src/bin/numbers-demo/presentation.rs` | Human-readable state and record summaries |
| `src/bin/numbers-demo/session.rs` | Separate journal directories, live launch and read-only inspection |
| `tests/demonstration.rs` | Execute the walkthrough, inspect persisted evidence, exercise the real server and an unreadable response |

The walkthrough calls `Engine::request`, not duplicate bid or settlement logic. It releases its writer before a child process reconstructs the journal, then compares exact ordered rows and all projected facts. Only the observational reconstruction receipt differs. Restart receipts are presentation files, not protocol events.

The internal State uses `ownership`. Literal revision 0.1 JSON keys remain unchanged under RT06; this preserves existing journal bytes, hashes and API compatibility. Human-facing labels translate the legacy outcome to Unowned with no owner.
