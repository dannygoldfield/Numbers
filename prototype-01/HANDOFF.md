# Continuing Numbers Prototype 1

Current handoff, 2026-10-10. Read this section first; the dated sections below preserve earlier checkpoints. [numbers-spec revision 0.1](../numbers-spec/STATUS.md) remains the sole behavioral authority.

## Start and demonstrate

Open [Numbers-demo.code-workspace](Numbers-demo.code-workspace) in VS Code. From `prototype-01/`, use `./run` for the saved browser demo, `./demo serve` for a fresh live session, or `./demo guided` for the prescribed terminal walkthrough. [DEMO.md](DEMO.md) owns the commands, restart instructions and [colleague feedback session](DEMO.md#collect-colleague-feedback).

The browser has Auction and Record views. Current presentation uses dark grayscale, a green accent and the operating system's interface font throughout. Record retains `/protocol` as its address. Auction outcomes and exact revision 0.1 record spellings are unchanged. Appearance history, contrast measurements and completed checks are in [VERIFICATION.md](VERIFICATION.md).

## Preserved source and histories

The published tag `prototype-01-baseline-2026-10-10` points to `d1e3e71` and must remain fixed. [All three GitHub jobs passed](https://github.com/dannygoldfield/Numbers/actions/runs/38107022264) for that baseline; the subsequent documentation checkpoint `6f540bd` also [passed](https://github.com/dannygoldfield/Numbers/actions/runs/38107104974). Later changes follow the tag in Git history. Check the current Git status and latest workflow when resuming.

The port-8765 journal is `data/demo-02.sqlite3`; an earlier history is `data/prototype.sqlite3`. Guided and separate live sessions are under `data/demonstrations/`. The last verified port-8765 restart reconstructed 28 records on 2026-10-10. These counts are observations, not promises about a later session. No cleanup operation resets a history.

All seven saved histories and their configuration/evidence files were backed up and verified at `~/Documents/Numbers-Backups/2026-10-10-prototype-01/`. Its manifest records integrity checks, exact ordered-row comparisons and hashes. The adjacent zip and checksum are ready for an external drive or chosen cloud storage; no external copy has been made. GitHub stores source, not these local histories.

## Next activity

Run the [colleague feedback session](DEMO.md#collect-colleague-feedback). Keep observations in the [Prototype 2 brief](../numbers-spec/work/PROTOTYPE-02.md#collect-observations-in-one-place). Its learning goal remains undecided. Choose one goal, approve the necessary specification changes, then evolve the same engine on a focused branch with a separate journal. The baseline tag preserves Prototype 1; a separate checkout is useful only when both generations must run together.

[TECHNICAL-DEBT.md](../numbers-spec/work/TECHNICAL-DEBT.md#current-register-as-of-2026-10-10) owns deferred work and revisit triggers. The live engine mutation boundary is fixed. Typed-state refactoring remains deferred. Three recorded dependency alerts affect the historical frontend and remain open under TD12. No new infrastructure or standing agent team is required for colleague feedback.

## Find the right guide

| Need | Read |
|---|---|
| Explain the repository | [Project folder guide](../README.md#project-folder-guide) and [archive index](../archive/README.md) |
| Follow a rule through the code | [REVIEW.md](REVIEW.md), starting with `src/model.rs` |
| Reproduce the environment or run checks | [SETUP.md](SETUP.md); `./check` is the complete check entry point |
| See what was verified | [VERIFICATION.md](VERIFICATION.md) |

The bounded simplification pass ends with colleague feedback. Reopen implementation for a concrete defect, an unclear existing requirement or an explicitly approved next requirement. Repository notes and actual Git state carry the durable context across chats.

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
