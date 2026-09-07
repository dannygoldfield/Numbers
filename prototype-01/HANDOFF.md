# Numbers Prototype 1 — continuation notes

Updated on 2026-09-07. This is a continuation note, not a source of protocol rules.

## Current work — demonstrations and ownership

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
