# Numbers Prototype 1 — pause checkpoint

Saved on 2026-09-06. This is a continuation note, not a source of protocol rules.

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

GitHub's workflow now includes a separate Prototype 1 job running this same check script with its pinned Rust toolchain and Node 22. The older application's jobs remain separate.

On the existing Mac, the dated history backup is in `~/Documents/Numbers-Backups/2026-09-06-prototype-01/`, with an accompanying zip archive. It contains both SQLite histories, the current configuration, integrity results, record counts and checksums. Each backup's ordered records were compared with its source. The original histories remain in `prototype-01/data/`; both originals and backups are excluded from the source checkpoint.

The backup's README explains how to use a separate working copy if needed. The zip can be copied to an external drive or chosen cloud storage for a copy outside this Mac.

## Work remaining

No unfinished change blocks this pause. The next product step is to use the prototype and choose the next explicit requirement. Real payments, Bitcoin integration, authentication and public hosting still need their own specification and approval. This checkpoint is a verified local demonstration, not a production deployment.
