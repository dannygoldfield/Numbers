# Numbers Prototype 1 — continuation notes

Updated on 2026-10-10. This is a continuation note, not a source of protocol rules.

## Current checkpoint — 2026-10-10

The focused review of `numbers-spec/` revision 0.1 and Prototype 1 found a presentation gap: rejected bids exposed internal reason codes in Auction View. The local implementation now explains all eight SE03 rejection reasons in plain language under existing UI01/UI03 requirements. Minimum, increment and maximum messages use the backend response snapshot; increment arithmetic preserves exact large integers. The displayed rejection confirms that the bid was recorded and the bidder's balance is unchanged. Record View retains the original response and canonical reason code.

This is an implementation correction, not a specification change. The Rust rules, API contract, record format and stored demo histories are unchanged. No dependency or infrastructure was added. The complete `./check` passed: Rust formatting, Clippy, 26 Rust integration entries, frontend formatting and six browser checks. See the dated evidence in [VERIFICATION.md](VERIFICATION.md).

C64 authorizes the reviewed cleanup checkpoint and publication to the existing GitHub repository. Its source baseline is named `prototype-01-baseline-2026-10-10`; use Git and the corresponding GitHub Actions run to check publication and remote verification. The preceding published source checkpoint was `4a45179`. The browser demo was rebuilt and restarted at http://127.0.0.1:8765/ with its existing `data/demo-02.sqlite3` journal, reconstructing through record 28. Its ordered records matched the new backup exactly after restart. This is a dated observation; later user activity can advance the history.

### Current appearance and balance explanation

The user requested removal of the period after the Numbers wordmark and dark mode by default. Both Auction and Record views now use a charcoal background, light text, green accents and dark controls/snippets. The layout remains as before; the later font trial below supersedes the earlier typeface choices. These presentation choices supersede the light palette for the current local demo; they do not revive the earlier gold-branding experiment.

The subsequent palette request is grayscale at the WCAG AA contrast thresholds, with green `#97C9A6` as the sole chromatic accent. The page is `#181818`; ordinary text is `#818181` (4.55788433:1); large text is `#646464` (3.00060040:1). Fields/snippets use `#202020` with `#878787` text (4.53543901:1). Essential input borders use `#6A6A6A`; decorative separators use `#333333`. Green button labels use gray `#4D4D4D`. Error text remains explicitly labeled but is now gray. The larger text styles remain at least 24 CSS pixels, which permits the 3:1 text threshold. See VERIFICATION.md for the checked pairs and scope.

The latest typography trial uses the operating system's default interface font across the entire site (`system-ui, sans-serif`). The large number, headings, controls and protocol snippets inherit that font; Georgia, Inter and the separate monospace stack are no longer selected. This is an experiment, not a final Numbers brand-font decision. The user may later retain the device font for the number while choosing a specific font for other text. The grayscale palette remains as previously recorded. The smallest text styles are now 11 CSS pixels instead of 10: the requested 105% scale yields 10.5 pixels, rounded up to a whole pixel. Larger text sizes remain unchanged. The Numbers wordmark is now semibold (`font-weight: 600`, reduced from 750) at the user's request.

The user also asked for research to inform the accent color. No accent change was requested or implemented in that inquiry. The recommendation is to retain the current green as a baseline and compare alternatives through actual Numbers tasks if useful, rather than infer a universally best hue from color-emotion associations. [Jonauskaite et al. (2020)](https://www.psychologicalscience.org/journals/psychological-science/0956797620948810/) found both shared and culturally varying associations; [Elliot's review (2015)](https://www.frontiersin.org/journals/psychology/articles/10.3389/fpsyg.2015.00368/full) cautions about real-world generalization. This is design advice, not an approved brand decision or protocol rule.

The user reports being very happy with the functionality demonstrated in Prototype 1. This update changes presentation only. The latest rebuilt preview again reconstructed through record 28; refresh the browser to load its new stylesheet.

Switching the simulated identity only changes which backend balance is displayed. Under RT02, the previous leader's hold is released when outbid. Before prior successful purchases, A5 means A595 available/5 reserved; B6 then means A600/0 and B594/6. Under RT03, successful settlement captures the winning hold, so that bidder carries a lower balance into later auctions. Failed settlement releases the hold. No balance rule was changed.

### Record name and project orientation

C63 names the read-only view Record. Navigation, the page heading, its accessible name, startup output and current presenter documentation now use that name; `/protocol` remains its address. The [repository README](../README.md#project-folder-guide) now maps the project folders and gives a short colleague walkthrough. The dedicated `Numbers-demo.code-workspace` shows the two active folders together: Prototype 1 and Specification. This is an editorial change; no auction behavior or saved record changes. Historical sections below retain their original wording.

### Reviewed baseline and transition preparation

The engine now keeps its live state, configuration, halt state and commit/evaluation methods private. External callers submit commands and receive read-only state access. Two compiler checks protect this boundary; the full check suite passes with 26 Rust integration entries, two compile-fail documentation tests and six browser script checks. This enforces existing rules without changing canonical records, auction decisions or saved history.

The root workspace now shows only the current implementation and specification. [archive/README.md](../archive/README.md) identifies historical sources, and old documentation entrances explicitly point to the current guides. The two misleading default book configurations were preserved byte-for-byte under `archive/legacy-book/`; all 25 original `codex-spec/` files remain byte-identical. Historical notes retain their original content below current-status notices.

A separate verified backup contains all seven local histories, their configuration/session metadata and saved demonstration evidence at `~/Documents/Numbers-Backups/2026-10-10-prototype-01/`. The accompanying `.zip` and `.zip.sha256` are ready to copy to an external drive or the user's chosen cloud storage. No external backup upload was performed. The manifest records SQLite integrity checks, exact ordered-record comparisons, counts and hashes. GitHub source publication does not include these histories.

[PROTOTYPE-02.md](../numbers-spec/work/PROTOTYPE-02.md) is the single draft planning brief, not a new specification. No Prototype 2 goal, behavior or implementation is approved yet. Keep Prototype 1 demonstrable; collect colleague observations, choose one learning goal, draft the necessary changed clauses, then obtain explicit scope/activation approval. Evolve the existing engine on a focused branch with a separate journal. The source tag preserves the original rules and implementation; use a separate checkout only if both generations need to run at once.

The [current debt register](../numbers-spec/work/TECHNICAL-DEBT.md#current-register-as-of-2026-10-10) records accepted limits and revisit triggers. Broad typed-state refactoring, multiple backend services and a standing agent team are not prerequisites for this transition.

### Next useful activity

Use the existing browser and guided terminal demonstrations, following [DEMO.md](DEMO.md), and collect concrete observations about clarity and fairness. The next product requirement should follow those observations. No unresolved specification choice blocks the message correction.

Keep work proportional to the current single-machine prototype. One builder can handle a bounded change; seek a focused additional review when its risk justifies it. A standing agent team, new orchestration service and production infrastructure are not prerequisites. This is a working recommendation, not protocol authority. Any future change to auction or economic meaning still needs an explicit specification decision.

Start a future chat by reading this section, [REVIEW.md](REVIEW.md), the active `numbers-spec/` clauses and the actual working-tree status. Repository notes carry the durable context; do not assume another chat's transcript or a historical live-session observation is current.

## Demonstration checkpoint — 2026-09-07

C60 authorizes live terminal and guided demonstrations for Prototype 1. C61 selects ownership and Unowned (no owner); canonical revision 0.1 record names are preserved by RT06. See [DEMO.md](DEMO.md), [REVIEW.md](REVIEW.md), and `Numbers-demo.code-workspace`. These additions use the same engine and separate demo journals. The browser's restored visual design remains in place.

C62 records the user’s 2026-09-07 authorization to commit and push this demonstration checkpoint to the existing GitHub repository. This checkpoint follows the earlier saved version and includes the current specification, implementation, presenter materials and verification notes. The following dated sections preserve the prior pause state and its backup evidence.

## Historical checkpoint — 2026-09-06

## Where we left off

The local Rust prototype implements the approved [Numbers specification 0.1](../numbers-spec/STATUS.md). It supports bidding by two simulated identities, leader-only reservations, append-only records, explicit settlement outcomes, title assignment and restart reconstruction. The older root application, frontend and `codex-spec/` are historical and independent.

The interface is restored to the point immediately after C53: the original cream-and-green layout and Georgia number, with “THE NUMBER” and the unassigned-title badge removed. C58 records this restoration; later font and dark/basic design experiments are superseded.

The nine upstream maintenance commits through `a58ec16` were incorporated before this checkpoint. C59 records the user's authorization to save and push it.

## State when paused

The local preview has been stopped cleanly. Its current journal is `data/demo-02.sqlite3`:

| Item | Saved state |
|---|---|
| Number | 1 |
| Auction | Scheduled; no bids; countdown not started |
| Bidder A | 600 available Rana, 0 reserved |
| Bidder B | 600 available Rana, 0 reserved |
| Protocol balance | 0 Rana |
| Journal | 3 initialization records |
| Minimum opening bid | 5 Rana |
| Minimum increase | 1 Rana |

The 3:45 auction starts only with its first accepted bid. Successful simulated settlement captures exactly the winning hold into the protocol balance. Failed simulated settlement releases it and assigns PublicLand. There is no settlement deadline or protocol spending operation. The next number appears after the recorded 12-second gap.

The earlier `data/prototype.sqlite3` remains separate and contains 25 records. Neither history was reset or edited for this checkpoint.

## Return to the project

On the existing Mac, open a terminal in this `prototype-01` directory and run:

```sh
./run
```

Open [the auction](http://127.0.0.1:8765/) and, if useful, [the Protocol view](http://127.0.0.1:8765/protocol). The application reconstructs the existing journal. Stop it with Ctrl-C when finished. No settings or environment changes are needed to resume on this Mac.

To resume with Codex, ask: “Resume Numbers Prototype 1 from prototype-01/HANDOFF.md in the Numbers repository.”

Before further code changes, read [REVIEW.md](REVIEW.md) and run `./check`. For a new machine, use [SETUP.md](SETUP.md). A fresh clone contains source and configuration but no local journal; it initializes a separate demonstration unless an existing history is supplied explicitly.

## Verification and backup

The full `./check` passed on 2026-09-06: Rust formatting, Clippy with warnings denied, all 23 Rust test entries and all three browser-value tests, plus frontend formatting. The restored UI was inspected in the browser, and its restart preserved all three current journal records. [VERIFICATION.md](VERIFICATION.md) retains the detailed settlement and restart evidence.

GitHub's workflow now includes a separate Prototype 1 job running this same check script with its pinned Rust toolchain and Node 22. All three jobs passed for checkpoint `0ecb53c`: [GitHub verification](https://github.com/dannygoldfield/Numbers/actions/runs/34051745603). The older application's jobs remain separate.

On the existing Mac, the dated history backup is in `~/Documents/Numbers-Backups/2026-09-06-prototype-01/`, with an accompanying zip archive. It contains both SQLite histories, the current configuration, integrity results, record counts and checksums. Each backup's ordered records were compared with its source. The original histories remain in `prototype-01/data/`; both originals and backups are excluded from the source checkpoint.

The backup's README explains how to use a separate working copy if needed. The zip can be copied to an external drive or chosen cloud storage for a copy outside this Mac.

## Work remaining

No unfinished change blocks this pause. The next product step is to use the prototype and choose the next explicit requirement. Real payments, Bitcoin integration, authentication and public hosting still need their own specification and approval. This checkpoint is a verified local demonstration, not a production deployment.

GitHub reported two open dependency alerts in the historical `frontend/package-lock.json` during the checkpoint push: [browserslist, high severity, patched in 4.28.7](https://github.com/dannygoldfield/Numbers/security/dependabot/46) and [@humanfs/node, moderate severity, patched in 0.16.8](https://github.com/dannygoldfield/Numbers/security/dependabot/45). Record these as maintenance work before returning to the older frontend. Both alerts identify the historical frontend manifest, not Prototype 1's manifest. Its dependencies were not changed as part of this pause checkpoint.


## Latest local demonstration state — 2026-09-07

The new tools passed the complete check script (26 Rust entries and three browser-value tests). A complete guided run and a separately restarted live test are retained under `data/demonstrations/`; see VERIFICATION.md for paths and evidence. Both older journals remain unchanged.

A fresh live session was left at Number1, 600 Rana each, minimum5/increment1, on port8766, in `data/demonstrations/live-1788795541902-85987/`. This is a point-in-time handoff observation: later user activity may advance it. To resume that history after stopping it, use `./demo serve --session data/demonstrations/live-1788795541902-85987`. Use `./demo live` in another terminal. The original port8765 preview remains stopped and can be started with `./run`.

VS Code has the presentation workspace open. Choose Terminal → Run Task → Numbers: Guided mechanics to start a new step-by-step walkthrough. C62 authorizes saving this work as the demonstration checkpoint on main and pushing it to GitHub. Demo journals and generated receipts remain local; the earlier history backup still describes the prior saved journals. Read the Git log for the source checkpoint and GitHub Actions for its remote check results.
