# Prototype 1 verification

Verified locally on 2026-09-05 against approved numbers-spec revision 0.1, using Rust 1.98.1 on Apple silicon macOS. This is implementation evidence, not a new source of behavioral authority.

## Repeatable checks

`./check` passed in full: Rust formatting, Clippy with warnings denied, 23 Rust test entries (22 behavior tests and one child-process helper), frontend formatting, and three exact-value browser helper tests. `./cargo-local build --locked --release` also passed.

The Rust suite covers the 31-record worked auction, both terminal outcomes, invalid bids and rejection priority, replacing one's own hold, extension-window equality and the extension cap, close-before-admission ordering, zero allocations, explicit defaults, malformed transport, exact large integers, pagination, changed configuration, reconstruction receipts, malformed histories, backward clocks, timestamp overflow, exclusive writers and failed storage.

Every prefix of the worked history is checked: complete groups reconstruct; prefixes ending inside a group are rejected. Correctly rehashed records with false decisions or references are rejected too. Exact-value browser tests exercise the actual display parser and formatter, including integers larger than JavaScript's safe numeric range and strings that resemble numbers.

## Interrupted transactions

The test harness launches and kills real child processes at three terminal-transaction boundaries, for each of successful and failed settlement: after the first uncommitted record, after all uncommitted records, and after commit. Across all six cases, restart sees either the complete preceding history or the complete terminal group. It does not expose a partial group or capture twice.

A separate storage test holds an external exclusive SQLite lock. The failed write is not acknowledged as an accepted bid, does not select PublicLand, and halts subsequent processing. These controls exist only in tests; the application has no fault-injection or clock-setting endpoint.

## Browser walkthrough

Used an isolated local test journal on port 8766 and the real 225-second auction duration. The final user demo uses its own journal on port 8765.

| Action or boundary | Observed result |
|---|---|
| New history | Number 1 Scheduled; A and B each 100 available, zero reserved |
| A bids 20; B bids 30 | A's hold released; B has 70 available and 30 reserved |
| A bids 30 | Invalid bid recorded; no Rana movement |
| B raises to 40; A bids 50 | Holds replaced correctly; A has 50 available and 50 reserved |
| First auction closes | Fixed winner A, 50 Rana still reserved, explicit settlement controls appear |
| Successful settlement | A's 50 captured; protocol holds 50; winner title assigned; 21 records |
| Twelve-second gap | Number 2 becomes Scheduled; 22 records |
| B bids 10 and second auction closes | Fixed winner B; 90 available and 10 reserved |
| Failed settlement | B returns to 100 available and zero reserved; PublicLand title; protocol still 50; 30 records |
| Next twelve-second gap | Number 3 Scheduled; 31 records; no first-bid countdown running |

The separate Protocol page exposed no visible bid or settlement controls. It received the settlement response as a transient observation and displayed the committed balances through its normal backend reads. The rebuilt page was inspected visually, including the journal and derived-state panel. No browser warnings or errors were captured during that final inspection.

## Restart evidence

Stopped the walkthrough server and restarted the release binary against the same journal. The reconstruction receipt reported sequence 31. Number 3 remained Scheduled, A had 50 available, B had 100 available, both reservations were zero, and the protocol held 50.

All 31 ordered `(sequence_index, record_json)` rows were byte-identical before and after restart. SHA-256 of their compact UTF-8 JSON array was:

```text
569c103eaf923b2a16a9fff70e7f9f87b839f5dbd38eed26d8c1e20eb761be99
```

No issuance, release, capture or finalization was appended by reconstruction. This restart occurred at a Scheduled boundary; automated tests separately cover reconstruction and subsequent due evaluation at other complete-group boundaries.

## Repository and evidence limits

All 919 original repository files matched the saved preflight SHA-256 inventory. Tracked and staged diffs were empty. The work adds only `numbers-spec/` and `prototype-01/`; no commit or push was made.

The process-kill tests do not constitute a physical power-loss or failing-hardware test. This work does not establish production throughput, distributed consensus, authentication, real payment handling or resistance to a machine owner replacing the journal. The frontend was inspected in the local in-app browser; a cross-browser and device compatibility campaign has not been performed. Those are outside the approved single-machine demonstration.

## Fresh demo configuration — 2026-09-05

After the user authorized returning to Number 1, the active demo changed to a separate `data/demo-02.sqlite3` history with 600 Rana per identity, minimum_bid_rana=5 and minimum_increment_rana=1. Specification 0.1 and the backend rules are unchanged; the earlier acceptance fixture above retains its own explicit parameters.

An isolated live backend check confirmed: opening 4 rejected without starting the clock; opening 5 accepted; equal 5 rejected; competing 6 accepted, with exact release/replacement accounting. All three open browser views were refreshed and showed Number 1, minimum 5, increase at least 1, and 600 available/0 reserved for both identities.

The prior 25-record user history remains in `data/prototype.sqlite3`. Its ordered rows are identical and its whole-file SHA-256 remains `021d1c04ba611cd282db7bb75b1192287ca8332c2830815a58513774f23224d3`. No later-issuance or current-auction amendment feature was implemented.


## Pause checkpoint — 2026-09-06

After incorporating the nine upstream maintenance commits through `a58ec16`, the full `./check` passed again: Rust formatting, Clippy with warnings denied, all 23 Rust test entries, frontend formatting and all three browser-value tests. The restored C53 presentation was checked in the local browser on this date; the restart preserved its three initialization records. The pause does not introduce auction-rule changes.

The local preview was stopped cleanly. SQLite backups of the current three-record history and earlier 25-record history passed integrity checks and exact ordered-row comparisons; the source file hashes were unchanged. A private local manifest records their hashes and counts. The repository workflow adds a Prototype 1 job running `./check`; the local results above do not claim a remote run has already completed. The user authorized the checkpoint commit and push under C59, superseding the earlier local-only publication restriction for this checkpoint.

All three GitHub jobs subsequently passed for source checkpoint `0ecb53c99de5207c6cd73e7e98f3927f93137190`: historical Rust tests, historical frontend build/lint, and Prototype 1 checks. [Run 34051745603](https://github.com/dannygoldfield/Numbers/actions/runs/34051745603) records the remote results. The legacy frontend dependency alerts found during the push are listed in HANDOFF.md; successful checks do not resolve those alerts.


## Demonstration tools and ownership wording — 2026-09-07

The full `./check` passed after the final source edits: Rust formatting, Clippy with warnings denied, 26 Rust integration test entries (23 existing plus three executable demonstration tests), frontend formatting and all three browser-value tests. No dependency was added. Existing server launch selection remains explicit through Cargo's default-run setting.

The new tests execute the shipped guided binary through all 18 UI05 steps. They verify 42 persisted records, both canonical settlement outcomes, two extensions, final balances A600/B538 with zero holds and protocol62, and separate-process receipts at sequences32 and41. Read-only inspection validates all42. Reusing the directory or serving a guided session with the real clock is rejected without changing its journal bytes.

The live-client integration test launches the actual server, accepts bids5 and6, rejects an equal bid, and preserves fractional and very large amount syntax in recorded command bytes. A separate transport test supplies an unreadable response and observes no automatic retry. The guided Enter, records and quit controls were also exercised interactively through the first three steps; the partial journal was retained.

A manual live session on port8766 accepted A20 through the terminal. The browser reflected the shared journal and the eventual next-number state. After real-clock closing, the explicit failed-settlement command released20 and finalized Unowned. The server was stopped and relaunched with the same session path; it reconstructed all12 records, including the next Number2 AuctionRecord. This test history is retained at `data/demonstrations/live-1788794326916-82682/`.

A complete guided evidence run is retained at `data/demonstrations/guided-1788795356890-83986/`, including result.json and both restart receipts. These are ignored local demonstration files, not source artifacts or protocol events. The provided VS Code workspace was opened with the guide and actual Rust source. Its task commands were exercised directly; the native task picker itself was not automated.

Compatibility checks copied both original journals for read-only validation by the new executable. `data/demo-02.sqlite3` reconstructed3 records; `data/prototype.sqlite3` reconstructed25. The source files remained byte-for-byte identical to their pre-change SHA-256 digests:

- demo-02.sqlite3: `290dd2a4035e0fa544cd89fe655bf49381e58a1084d7b86edab82caecc1fad80`
- prototype.sqlite3: `021d1c04ba611cd282db7bb75b1192287ca8332c2830815a58513774f23224d3`

RT06 preserves literal API/schema keys and enums; only current prose, display and internal variable names change. The active-spec event-field schemas were reviewed against the prior version and current local document links were checked. Historical source specifications and past verification narratives remain historical. No Prototype2 auction rule, real payment or identity behavior was introduced, and this local validation does not constitute new GitHub CI evidence.
