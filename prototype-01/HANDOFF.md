# Continuing Numbers Prototype 1

Current handoff, 2026-10-18. [Revision 0.2](../numbers-spec/STATUS.md) is the sole active behavioral authority. PR #12 was merged as `d567a3f55e951114ee4c5c125cdd9366dd70de3b` and all three checks on that merge passed. C69 records the merge; C70 authorizes the separate cleanup PR, which must not be merged without user approval. Deployment remains separate.

## Run and demonstrate

From `prototype-01/`, use `./run` for Auction at `http://127.0.0.1:8765/` and Record at `/record`. The default journal is `data/revision-0.2/demo.sqlite3`. Use `./demo guided` for a fresh scripted history, or `./demo serve` for a separate live session. [DEMO.md](DEMO.md) owns presenter, inspection, and restart instructions; [SETUP.md](SETUP.md) owns environment setup and checks.

At preservation on 2026-10-18, the revised journal contained three records: Number 1 Scheduled, 600 available Rana per bidder, zero reserved/protocol-held Rana. This is a dated observation. No server was running during preservation. Restart the same history without resetting it.

Current fields are ownership, ownership_kind, owner_id, unowned, and expired_to_unowned. Every canonical record has spec_revision=0.2. The original auction outcome is immutable; a future separately specified transfer may append later facts. Prototype 1 has no transfers or wallets.

## Preserved source, builds, and journals

| Revision | Source preservation | Local verified backup |
|---|---|---|
| 0.2 before cleanup | `prototype-01-revision-0.2-preserved-2026-10-18` at `d567a3f55e951114ee4c5c125cdd9366dd70de3b` | `/Users/dannygoldfield/Documents/Numbers-Backups/2026-10-18-revision-0.2-before-cleanup/` |
| 0.1 before revision change | `prototype-01-revision-0.1-preserved-2026-10-18` at `2cc86353185c6b96947f87810105fe04f8b8ce0c` | `/Users/dannygoldfield/Documents/Numbers-Backups/2026-10-18-revision-0.1/` |

The earlier `prototype-01-baseline-2026-10-10` tag remains fixed. Each backup includes source, matching executables, configuration, dependency locks, journals, and session evidence; its manifest and verification report record checksums and reconstruction. The pre-cleanup backup also preserves the binaries present before rebuilding, plus verified debug/release builds of the merged revision. All eight saved journals passed integrity and read-only reconstruction with their matching readers. The old seven histories remain under their original meanings.

Do not continue an old history with the new executable. For historical inspection, use the matching preserved executable and configuration on a separate copy. Never reset, rename fields, rehash, or migrate saved journals during cleanup. GitHub stores source and tags, not these local backups.

## Next activity and boundaries

Review the cleanup PR and [verification evidence](VERIFICATION.md) before approving its merge. The active engine, tests, configuration, dependency versions, and normative documents are unchanged. Project-instruction simplification is a later reviewed step; do not edit the synced ChatGPT project files as part of repository cleanup.

Continue Prototype 2 readiness work in the [planning brief](../numbers-spec/work/PROTOTYPE-02.md), bounded to one DigitalOcean Droplet and invited testers. Choose the learning goal and approve any new behavioral specification before implementation. Keep deployment separately approved. [Technical debt](../numbers-spec/work/TECHNICAL-DEBT.md#current-register) records accepted limits and revisit conditions.

Earlier handoffs, the old applications, and earlier specifications are available through the [historical index](../archive/README.md), including the [complete previous handoff](https://github.com/dannygoldfield/Numbers/blob/d567a3f55e951114ee4c5c125cdd9366dd70de3b/prototype-01/HANDOFF.md). Historical instructions are not current launch or behavioral rules.
