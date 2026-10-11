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


## Bid explanations and focused review — 2026-10-10

A focused source review compared the current bidding, settlement, reconstruction and browser paths with `numbers-spec/` revision 0.1. It found no rule mismatch in the reviewed core paths and identified the raw bid-rejection codes in Auction View as a UI01/UI03 presentation gap. This was one builder's bounded review, not an independent audit or production-security assessment.

The corrected Auction View maps each SE03 reason to a readable explanation and derives amount guidance from the backend response snapshot. It preserves the raw result in Protocol View and does not change admission, amount input, API schemas or canonical records. The increment display uses exact integer addition, including beyond JavaScript Number precision. The rejected-bid message explicitly says that the bid was recorded and the balance is unchanged.

`./check` passed after the source update: Rust formatting, Clippy with warnings denied, 26 Rust integration test entries (including the existing crash helper), frontend formatting and six browser checks. Three new checks exercise the actual app script with controlled transport and a minimal DOM: rejected bids display 91 after a leading 90 and remain exact for a very large amount; duplicate submissions are blocked while awaiting acceptance and balances change only after the response; a lost response prompts inspection, refreshes state and does not resubmit. Existing backend and executable-demo tests use isolated test journals.

No new dependency, normative rule, infrastructure or local preview server was introduced. Saved demonstration journals were not opened or changed by this work. The new presentation checks are automated script checks, not a new visual or cross-browser inspection. These results are local; no new commit, push or GitHub CI result is claimed.


## Default dark appearance — 2026-10-10

Removed the wordmark's decorative period and changed the shared stylesheet to dark mode, including native controls, placeholders, buttons, feedback, ownership banners and protocol snippets. The locked Rust build, frontend formatting check and `git diff --check` passed. Calculated color contrast was 16.01:1 for primary text/background, 9.00:1 for button text/background, and 3.20:1 for input border/surface. No new behavioral test was added for this presentation-only change.

The existing local server was stopped and restarted with the same configuration and journal. Startup reported successful reconstruction through record 28. No bid, settlement, reset or balance-rule change was performed as part of the appearance update. The balance explanation follows RT01–RT03 and the existing rendering code, which selects the chosen identity's backend balance.

A live visual check and automatic page refresh were unavailable because the browser tool could not verify its admin-enforced security policy. The user must refresh the page to see the rebuilt interface; no visual-browser verification is claimed for this update.


## Grayscale at minimum contrast thresholds — 2026-10-10

The user requested the lowest accessible contrast and grayscale except for the retained accent. This implementation targets WCAG 2.2 AA color-contrast thresholds: ordinary text at least 4.5:1; text at least 24 CSS pixels and essential control boundaries at least 3:1. Sources: [W3C text contrast](https://www.w3.org/WAI/WCAG22/Understanding/contrast-minimum.html) and [W3C non-text contrast](https://www.w3.org/WAI/WCAG22/Understanding/non-text-contrast.html). This is a targeted color check, not a declaration of full WCAG conformance.

Calculated directly from the final stylesheet using sRGB relative luminance:

| Use | Foreground | Background | Calculated ratio |
|---|---|---|---|
| Body, labels, secondary text, errors | #818181 | #181818 | 4.55788433:1 |
| Number, wordmark, large heading, clock, balances | #646464 | #181818 | 3.00060040:1 |
| Inputs, placeholders, code, final ownership banner | #878787 | #202020 | 4.53543901:1 |
| Input border against inner surface | #6A6A6A | #202020 | 3.01236643:1 |
| Input border against outer page | #6A6A6A | #181818 | 3.28285383:1 |
| Primary button text, normal and hover | #4D4D4D | #97C9A6 | 4.51687946:1 |
| Accent text and focus against page | #97C9A6 | #181818 | 9.48789459:1 |
| Secondary button text on hover | #97C9A6 | #202020 | 8.70614917:1 |

Each threshold comparison used the unrounded value. For the normal-text, large-text, surface-text, inner-control-border and button-label pairs, moving one whole RGB gray step toward lower contrast fails the applicable threshold. Decorative separators are #333333 and are not relied on to identify controls. Inactive controls retain their existing disabled opacity; inactive components are exempt from these contrast requirements. All non-accent hex colors have equal red, green and blue channels. The sole retained accent is #97C9A6.

Button hover now uses an underline rather than another green shade, and the ownership arrival animation retains movement without fading its text below the threshold. Existing rejection and backend-unavailable wording identifies errors without red. No auction behavior or font choice changed.

The locked build, frontend formatting check, source contrast calculations and `git diff --check` passed. The server was restarted against the existing journal and reported successful reconstruction through record 28. Browser inspection/automatic refresh remain unavailable under the previously reported browser-policy check, so no new rendered-browser validation is claimed. No commit, push or remote CI result is claimed for this local update.


## Operating-system font trial — 2026-10-10

At the user's request, the body now selects `system-ui, sans-serif`, and both headings plus protocol snippets explicitly inherit the family. Existing inputs and buttons already inherit their font. This removes the prior Georgia, Inter and monospace overrides so the large number and all site text use the device's interface font. No external font resource is loaded. The palette, sizes, weights and auction behavior were not changed.

The stylesheet formatting check, `git diff --check`, and locked Rust build passed. The running preview was restarted with its existing journal and reconstructed through record 28. The user must refresh to load the new embedded stylesheet. Browser visual validation remains unavailable under the previously recorded browser-policy limitation; the checks confirm the source selection and build, not a measured rendered font on the device.


Following the operating-system font trial, the Numbers wordmark alone changed from weight 750 to semibold 600. Stylesheet formatting, `git diff --check` and the locked build passed. The preview restarted successfully against the existing journal, reconstructing through record 28. The wordmark remains 34 CSS pixels on desktop and 29 on narrow screens, preserving its large-text classification. The accent color was not changed during the accompanying research inquiry; the same browser-verification limitation applies.

Following the user's request for slightly larger smallest text, all six 10-pixel styles changed to 11 pixels: edition metadata, small labels, journal ordering, journal group labels, protocol snippets and footer text. The requested 105% scale was interpreted as a 5% increase (10.5 pixels), rounded up to a whole pixel. Larger text sizes remain unchanged. Stylesheet formatting, `git diff --check` and the locked build passed. The preview restarted against the existing journal and reconstructed through record 28. The previously recorded browser-verification limitation still applies; refresh is required to load the updated stylesheet.


## Record view and project folder guide — 2026-10-10

C63 / S28 is an editorial change: the browser navigation, heading, accessible label and server startup output now say Record. The existing `/protocol` address and all internal identifiers, response contracts, record bytes and auction rules remain unchanged. Current specification and presenter prose use the new view name; historical verification and provenance retain their original wording. The root README now describes the active folders, code responsibilities, saved local histories, generated files and older material, with a short colleague walkthrough.

The complete `./check` passed: Rust formatting, Clippy with warnings denied, 26 Rust integration entries (including the crash-writer helper), frontend formatting and six browser checks. Existing tests exercised the live server and both guided settlement outcomes; no new tests or dependencies were added for this label change. All local links in the folder guide and current presenter documents were checked for existing targets. The three visible/accessibility labels and retained address were checked in source. `git diff --check` passed.

The rebuilt local preview restarted with Record in its startup output and reconstructed the existing journal through record 28. Browser visual inspection remains unavailable under the previously recorded browser-policy limitation; refresh is required to load the embedded page changes. No new commit, push or remote CI run is claimed.


## Audit remediation and preserved baseline — 2026-10-10

An isolated external-library probe demonstrated the audit finding: the public Engine::commit method accepted a planner-derived group whose payload hash had been replaced, while reconstruction rejected the resulting history. No live journal was used. The browser and terminal used the valid planner path; this was an unnecessary library mutation surface, not evidence of an exploited HTTP path.

Engine's live state, configuration, validation receipt and halt state are now private. Its commit and due-evaluation methods are private; public state observations borrow immutably. Two compile-fail documentation tests check that an external caller cannot commit arbitrary groups or clear the live record collection. Current callers use the read-only accessors. REVIEW.md now describes this boundary accurately. Existing protocol rules, canonical bytes and persistence format are unchanged.

The final `./check` passed: Rust formatting, Clippy with warnings denied, all 26 Rust integration entries (including the crash helper), both compile-fail documentation tests, frontend formatting and all six browser script checks. A locked application build also passed. These are local checks; they do not claim a remote CI result. No new dependency was installed.

Documentation cleanup validated local link targets in 25 current guides. All 25 original codex-spec files remained byte-identical to the previous Git checkpoint. Both relocated book configurations matched their original bytes; their archive explains that they are snapshots, not supported build targets at the new paths. Original open-question and debt-register bodies remain intact beneath current-status notes. The new Prototype 2 brief is explicitly non-normative, with no selected scope or activated behavior. `git diff --check` passed.

Seven local SQLite histories were backed up using consistent read-only source snapshots. Each destination passed integrity checking and exact ordered-row comparison; the backup manifest also records file and ordered-row SHA-256 hashes. Counts at backup were 28 for the current demo, 25 for the earlier demo, and 42, 9, 42, 12 and 3 across the five saved guided/live sessions. Seventeen configuration, session, result and receipt JSON files were copied with hashes. The backup directory and zip are under `~/Documents/Numbers-Backups/2026-10-10-prototype-01`; they remain outside Git.

The rebuilt loopback demo restarted with its existing configuration and reconstructed through sequence 28. Its 28 ordered records matched the backup exactly after restart. No reset, issuance, bid or settlement command was submitted during cleanup. No new rendered-browser inspection is claimed; the previously recorded browser-policy limitation remains. C64 authorizes committing and publishing this checkpoint, with the immutable source tag `prototype-01-baseline-2026-10-10`. Remote verification belongs to the corresponding GitHub Actions run.


### Published checkpoint

Commit `d1e3e71279457ed71e7f2c5b9ec12545d096780f` and annotated tag `prototype-01-baseline-2026-10-10` were pushed together successfully. GitHub's [Prototype 1 checks job](https://github.com/dannygoldfield/Numbers/actions/runs/38107022264/job/114374580072) passed for that exact commit. Later documentation-only publication notes do not move the preserved baseline tag. Refer to the [whole workflow](https://github.com/dannygoldfield/Numbers/actions/runs/38107022264) for the separate historical application jobs.

The push reported three dependency advisories. Read-only inspection confirmed all three refer to the historical frontend lockfile; TD12 records their package names, severity, patched versions and follow-up boundary. No dependency upgrade or dismissal was performed.
