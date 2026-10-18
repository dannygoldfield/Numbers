# Continuing Numbers Prototype 2

Current handoff, 2026-10-18. Revision 0.3 is approved in C74 / S34 and implemented locally on `codex/prototype-2-revision-0.3`, based on cleaned main `b5c15ff4d715be6c21881493be0bfd4c795a7823`. [STATUS.md](../numbers-spec/STATUS.md) and [AUTHORITY.md](../numbers-spec/AUTHORITY.md) define current behavioral authority. C75 subsequently authorizes review, fixes within this scope, commit, push and an unmerged pull request. Merge, pruning and deployment remain separate. The branch is a Git worktree of `/Users/dannygoldfield/Projects/Numbers-Repos/Numbers`; the main checkout there stays unchanged until separately authorized.

## What is ready for review

The existing engine now serves two fixed bidder principals and one facilitator through an authenticated HTTPS gateway. Both bidders see shared history and balances; only the facilitator selects simulated settlement. The application enforces the access and transport gates before evaluation. The browser shows fixed identity/session, retains an uncertain command target across refresh, waits at most 30 seconds, and never resubmits a command automatically.

The auction planner, timing, Rana movements and both settlement outcomes are unchanged. Current ownership fields remain `ownership`, `ownership_kind`, `owner_id`, `unowned`, and `expired_to_unowned`. Every new canonical record has `spec_revision=0.3`. Original auction finalization is immutable; a future separately specified transfer may append later facts. This prototype has no transfers, wallets or Bitcoin integration.

The implementation remains in `prototype-01/`; the directory and binary names are retained. No files were pruned. The old terminal `live` and `serve` commands explicitly refuse in the new binary. Their historical source remains, and the preserved 0.2 build still supports its original workflow.

## Verification and operation

[VERIFICATION.md](VERIFICATION.md#revision-03--2026-10-18) records the actual preservation, local checks, HTTPS rehearsal and visual-rendering evidence, including their limits. [REVIEW.md](REVIEW.md) maps the implementation to specification clauses. [SETUP.md](SETUP.md) gives reproducible build/test commands; [DEMO.md](DEMO.md) gives the later facilitated observation procedure.

Use the guided demonstration for a disposable deterministic history. A live 0.3 process needs an explicit configuration containing `allowed_origin` and `session_label`, an authenticated gateway and a separate fresh journal. `--initialize` is required for an unused history and receipt; later explicit starts use the exact same executable, configuration and journal without that flag. A changed association or missing intended journal fails closed. The checked-in origin is an example, not a hosted service.

[ops/README.md](ops/README.md) and the NGINX/service templates are reviewable operating examples. Nothing has been installed as a service or deployed. The host, its administrators and its clock remain trusted. The backend must stay loopback-only; no automatic process restart, command replay, inferred settlement, reset or rollback is permitted.

## Preserved Prototype 1

| Revision and checkpoint | Source preservation | Verified local backup |
|---|---|---|
| Cleaned 0.2 before Prototype 2 | Main `b5c15ff4d715be6c21881493be0bfd4c795a7823`; exact 59-file archive and verified Git bundle | `/Users/dannygoldfield/Documents/Numbers-Backups/2026-10-18-revision-0.2-before-prototype-2/` |
| 0.2 before cleanup | `prototype-01-revision-0.2-preserved-2026-10-18` at `d567a3f55e951114ee4c5c125cdd9366dd70de3b` | `/Users/dannygoldfield/Documents/Numbers-Backups/2026-10-18-revision-0.2-before-cleanup/` |
| 0.1 before revision change | `prototype-01-revision-0.1-preserved-2026-10-18` at `2cc86353185c6b96947f87810105fe04f8b8ce0c` | `/Users/dannygoldfield/Documents/Numbers-Backups/2026-10-18-revision-0.1/` |

Preservation was completed before implementation. The latest backup contains matching 0.2 debug/release binaries, source, configuration, dependency locks, all eight saved histories and session evidence. Its manifest records checksums and its verification report records integrity/reconstruction. All eight journal copies passed with their matching readers: seven 0.1 histories with 51, 42, 9, 42, 12, 3 and 25 records; the 0.2 history with three records. The preserved 0.2 release executable also completed a separate fresh 42-record guided fixture and reconstruction. All 34 original data/session files stayed unchanged.

The pre-cleanup preservation tag deliberately retains its original commit; it was not moved to the cleaned commit. The earlier baseline tag also remains fixed. GitHub stores source/tags, not these local binary/journal backups. Inspect historical evidence using a copy and its matching preserved reader. Do not run the new binary against old journals or migrate/relabel their records.

## Remaining gates

Review the pull request and its current checks before separately authorizing a merge. Local publication checks and review corrections are recorded in [VERIFICATION.md](VERIFICATION.md#revision-03-publication-review--2026-10-18). The configured Linux CI job runs when the PR is opened; consult its actual status rather than inferring a pass from local checks. A future deployment approval must name the DigitalOcean Droplet/account, hostname/TLS, versions, operator, firewall, budget, credential delivery/revocation, backup destination and shutdown/retention plan. Only then verify the deployed boundary and run the actual two-device invited session. No real credentials have been issued and no tester has been invited by this work.

Continue with the [Prototype 2 experiment brief](../numbers-spec/work/PROTOTYPE-02.md). Pruning stays separate. Historical material is indexed in [archive/README.md](../archive/README.md); it cannot add current execution rules. Synced ChatGPT project files remain read-only and were not edited.
