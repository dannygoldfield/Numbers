# Verification evidence

## Revision 0.2 — 2026-10-18

Approved source: C66–C67 / S31–S32. Starting commit: `2cc86353185c6b96947f87810105fe04f8b8ce0c`. This is evidence for the revised local Prototype 1, not authority for additional behavior.

Before implementation, preserved the exact source in a verified Git bundle and source archive; saved debug/release executables, configuration, dependency locks, all seven journals, and session evidence. Backup: `/Users/dannygoldfield/Documents/Numbers-Backups/2026-10-18-revision-0.1/`. Its SHA-256 manifest matched the original files. All seven copies passed SQLite integrity checks and read-only reconstruction with the preserved executable: 51, 42, 9, 42, 12, 3, and 25 records. The old executable also completed a fresh 42-record guided fixture with both separate-process reconstruction checks. The existing baseline tag remained fixed; the new local preservation tag identifies the exact starting commit.

The complete `./check` passed: Rust formatting, Clippy with warnings denied, 25 protocol tests, five executable/HTTP demonstration tests, two compile-fail documentation tests, frontend formatting, and eight browser tests. The locked optimized release build passed. The first sandboxed attempt could not bind local test ports; the complete successful run used permission to open loopback test servers. No dependency versions changed.

New coverage proves:

- The frozen initialization fixture from the preserved 0.1 executable is rejected before finalization. Missing, null, numeric, boolean, unsupported, and mixed markers fail on initial issuance and later records, even with valid payload hashes. Repeated rejected opens leave database bytes unchanged.
- Ownership payloads and projections use only the revision 0.2 schema. Legacy aliases, legacy enum/reason spellings, mixed fields, and extra fields fail reconstruction. Pending ownership is null; Unowned is final with a null owner.
- UI04 retains 31 records and final available balances A50/B100, zero reserved, protocol50. UI05 retains 42 records and A600/B538, zero reserved, protocol62. Both settlement outcomes and all existing material restart/crash boundaries pass.
- The actual HTTP handler serves `/record` and rejects `/protocol` with 404/unknown_endpoint and empty evaluation_records, without evaluating an overdue Open auction. The read-only demonstration inspector also rejects old records without modification.
- Separately, the release executable rejected copies of all seven preserved local journals as history_invalid/spec_revision, with identical before/after file hashes.
- A fresh release smoke session served both browser views. The Record page hid Auction controls; the displayed state used ownership:null without a title alias. Computed fonts on the number, headings, controls, and record snippets were system-ui, sans-serif. No browser console errors were observed. The temporary server and tab were closed.

After applying the reviewed files to the local `revision-0.2-ownership` branch, both debug binaries and the locked release build compiled from the actual repository path. The previously absent `data/revision-0.2/demo.sqlite3` was initialized with exactly three marked records: two 600-Rana allocations and Number 1 Scheduled. A second launch reconstructed through sequence 3 and preserved identical journal bytes, balances, and state. All seven original histories and session files still matched the preservation manifest. The server is stopped. At this initial implementation checkpoint, the changes were uncommitted and unpublished. Changed-document links and whitespace checks passed; the only Cargo.lock change is the application version, with dependency versions unchanged. No files were deleted.

Detailed logs and disposable verification histories are retained outside the repository in the creating task's `outputs/numbers-revision-0.2/` directory. Historical evidence below retains its original names, dates, and results. No old record or hash was rewritten. Repository pruning, GitHub publication, deployment, and transfers were not performed by these checks.

## Review before publication — 2026-10-18

C68 authorizes review, commit, and push of this revision. The review checked the specification changes, record generation and reconstruction, storage opening, HTTP routes, browser and terminal presentation, acceptance fixtures, and current documentation against C66–C67 / S31–S32. No remaining blocking issue was found in this bounded review; it is not an independent security audit.

The review found that the new read-only SQLite inspection connection inherited a five-second busy retry, unlike the existing writer connection's immediate-failure policy. A locked-journal regression test failed before the fix after 5.21 seconds. The inspection connection now explicitly uses a zero busy timeout. The same test passes and verifies that the journal remains byte-identical after rejection. This preserves existing failure behavior rather than introducing a retry or recovery rule.

After that correction, the complete `./check` passed: formatting, Clippy with warnings denied, 26 protocol tests, five executable/HTTP demonstration tests, two compile-fail documentation tests, and eight browser tests. The locked optimized release build also passed. Logs are `publication-check.log` and `publication-release.log` in the external task output directory noted above.

Before publication, all 41 preserved backup files matched their SHA-256 manifest, and all 32 original data/session files remained identical to that manifest. The revised default journal still contained its three initialization records and passed SQLite integrity checking. Both preserved source tags retain their original targets. Source changes are confined to the approved specification, active prototype, tests, and associated guidance; no repository pruning, deployment, history migration, dependency update, or transfer implementation is included. Local histories and generated builds remain excluded from Git.

## Merged revision and preservation before cleanup — 2026-10-18

PR #12 merged as `d567a3f55e951114ee4c5c125cdd9366dd70de3b`. All three [GitHub jobs on that merge](https://github.com/dannygoldfield/Numbers/actions/runs/39148024811) passed. C70 subsequently approved repository cleanup and a PR, with merge and deployment explicitly withheld.

Before removals, preserved all 973 tracked files in an exact source archive and verified Git bundle at `/Users/dannygoldfield/Documents/Numbers-Backups/2026-10-18-revision-0.2-before-cleanup/`. Tag `prototype-01-revision-0.2-preserved-2026-10-18` identifies that source. Existing executables were copied first; locked debug and release builds of both binaries then passed and were preserved with their matching configuration and lockfiles. The backup contains 53 checksummed files plus its manifest.

All eight journal copies passed SQLite integrity checks and read-only reconstruction using the matching revision's preserved reader. The seven revision 0.1 histories retained 51, 42, 9, 42, 12, 3, and 25 records; the revision 0.2 history retained three records. All 34 original local data/session files remained byte-identical. All 41 files in the earlier revision 0.1 backup still matched its original manifest. The preserved revision 0.2 release demo also completed its fresh 42-record guided fixture with separate-process reconstruction. No saved journal was continued or rewritten.

## Cleanup verification — 2026-10-18

C70's exact removal manifest contains 914 tracked paths, leaving 59 tracked files. It is retained in the task's external `outputs/numbers-cleanup/removal-manifest.json`; the cleanup commit supplies the same path-by-path deletion record. All removed source remains in the preserved pre-cleanup snapshot. The 730 tracked root dependency files and both duplicate ZIPs are included in that preservation. No Git history was rewritten.

Structural verification confirmed all eleven normative files and all 32 non-Markdown Prototype 1 files were byte-identical to the preserved revision. This includes source, browser assets, tests and the historical rejection fixture, configuration, manifests, locks, toolchain, launchers, and the demonstration workspace. The retained Prototype 1 CI job was also byte-identical; only the two legacy jobs were retired. Workspace folder targets, local documentation links and heading targets, and all frozen archive links resolved. All 53 backup files still matched their manifest, and all 34 original data/session files remained unchanged after cleanup.

An isolated clean Git checkout of cleanup commit `e302817c78b9b566601b7ffe03474cd40ae930ce` contained none of the retired applications, root dependencies, old specifications, or experiments. Installing the prototype's own locked development dependencies with scripts disabled succeeded. The complete `./check` passed: Rust formatting, Clippy with warnings denied, 26 protocol tests, five executable/HTTP tests, two compile-fail documentation tests, frontend formatting, and eight browser tests. Both optimized release binaries built with locked dependencies. The release guided demonstration completed 42 records with final balances A600/B538, zero reservations, protocol62, and separate-process reconstruction. Its journal was disposable and separate from saved histories. The checkout remained clean afterward.

The subsequent changes only record this evidence and clarify a documentation sentence. No executable or test changed after the clean-checkout run. Logs and disposable histories remain outside the repository in `outputs/numbers-cleanup/`. No new visual-browser inspection is claimed for this source-preserving cleanup. Project instructions, deployment settings, and hosted infrastructure were not changed. The cleanup PR remains subject to user review and separate merge approval.

## Earlier evidence

The complete [revision 0.1 verification chronology](https://github.com/dannygoldfield/Numbers/blob/d567a3f55e951114ee4c5c125cdd9366dd70de3b/prototype-01/VERIFICATION.md#historical-evidence--revision-01) remains frozen with its original dates, terminology, and limitations. The [historical index](../archive/README.md) also links earlier candidate audits and traceability. Their results do not claim verification of later code.
