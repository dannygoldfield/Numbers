# Verification evidence

## Revision 0.3 publication review — 2026-10-18

C75 authorizes review, corrections within revision 0.3, commit, branch push and an unmerged PR. The review covered specification authority and the approved changes, all implementation changes and added files, authentication before evaluation, serialized command effects, initialization/restart association, revision isolation, browser uncertainty, gateway/service examples, CI and operating guidance. No remaining blocking issue was found in this bounded review; it is not an independent security audit or deployment approval.

Two browser defects were reproduced with failing regression tests and corrected:

- After an access/session change followed by a failed new-state read, old phase and clock labels remained visible. The clearing path now removes those timing/phase facts as well as balances, records and ownership presentation before new reads.
- The exact-number transformer could accept malformed JSON such as a leading-zero number. It now validates the original JSON syntax before preserving numeric tokens. The validation result is discarded, so no rounded numeric value enters presentation or a command. Malformed responses retain uncertainty and cannot unlock another command. The test transport now passes malformed response bytes to the shipped parser instead of rejecting them first.

The access regression also explicitly checks conflicting duplicate principal headers on an overdue auction. It rejects them before any evaluation. These fixes implement existing UI03/SE07 rules and change no auction semantics.

After the fixes, the complete publication `./check` passed: Rust formatting, Clippy with warnings denied, 26 protocol tests, four HTTP access tests, five executable/process tests, two compile-fail documentation tests, frontend formatting and 16 browser-helper tests. Both locked optimized release binaries built. Prior release-guided and real HTTPS results below cover the unchanged engine and gateway configuration; the PR's Linux job repeats its complete configured checks. No new visual-browser inspection is claimed for these small parser/clearing fixes.

Publication rechecks confirmed all 52 files in the new pre-Prototype-2 backup, 53 in the pre-cleanup 0.2 backup, 41 in the 0.1 backup, and all 34 original data/session files still match their checksum manifests. The original main checkout and preserved tag targets remain unchanged. Document links, workspace targets and diff whitespace pass; no generated build, local journal, credential or key is included. Review logs are the ignored `review-regressions-before.log`, `publication-check.log` and `publication-release.log` in the implementation worktree; detailed preservation evidence remains outside Git.

Publication is authorized; merge, repository pruning and deployment are not. The live PR records GitHub checks after push. No actual two-device invited session or deployed host verification has occurred.

## Revision 0.3 — 2026-10-18

Approval: C73–C74 / S34 and the [approved decision packet](https://chatgpt.com/space/page_37906e64d6708191b3c86647190a42a5). Starting source is cleaned main `b5c15ff4d715be6c21881493be0bfd4c795a7823`, after PR #13. At this initial verification checkpoint, implementation was local and uncommitted on `codex/prototype-2-revision-0.3`. These results describe that implementation; they do not add behavioral authority or authorize deployment.

### Preservation before implementation

Preserved all 59 tracked source files in an exact archive and verified Git bundle at `/Users/dannygoldfield/Documents/Numbers-Backups/2026-10-18-revision-0.2-before-prototype-2/`. Copied the checksum-verified matching 0.2 debug/release executables, configuration, locks, all eight journals and session evidence. The new backup contains 52 checksummed files plus its manifest. The existing 0.2 preservation tag remains at the pre-cleanup commit; it was not moved.

All eight journal copies passed SQLite integrity and matching-reader reconstruction unchanged: seven 0.1 histories with 51, 42, 9, 42, 12, 3 and 25 records, and the 0.2 history with three records. The preserved 0.2 release guided fixture also passed with 42 records and separate-process reconstruction. Before source edits, all 53 files in the earlier 0.2 backup and all 41 files in the 0.1 backup matched their manifests. Final rechecking confirmed all three backups, all 34 original local data/session files and preservation tags remain unchanged. The original main checkout remains clean.

### Final local checks

The complete `./check` passed on the implementation worktree: Rust formatting, Clippy with warnings denied, 26 protocol tests, four HTTP access tests, five executable/process tests, two compile-fail documentation tests, frontend formatting and 14 browser-helper tests. The local test servers used explicit permission to bind loopback. Both locked optimized release binaries built. No Rust or npm dependency version changed; Cargo.lock changes only the application version to 0.3.0.

The release guided fixture passed with 42 records: Number 3 Scheduled, A600/B538 available, zero reservations, protocol62, and both separate-process reconstruction checkpoints. The independent UI04 fixture retains 31 records and A50/B100 available, zero reservations and protocol50. Both fixture inputs and economic outcomes remain unchanged; new records use marker 0.3. The auction planner changes only its generated/accepted revision-marker strings.

The added or extended checks establish:

- Missing/unrecognized/duplicate principal, forbidden role or bidder binding, origin/fetch-site, representation and body-size failures cannot evaluate an overdue auction or append canonical records. `/access`, static resources and rejected routes also do not evaluate. Missing/non-string bidder fields still follow the original SE03 validation order after the access gate.
- Exact 4,096-byte commands retain their original bytes; oversized fixed/chunked bodies are rejected. Competing same-amount bids produce one accepted leader in canonical order and shared state. The facilitator alone can select either settlement outcome, without duplicate terminal effects.
- Frozen 0.1 and 0.2 fixtures, mixed/unsupported/mistyped markers and malformed histories are rejected unchanged. Process tests prove explicit first initialization, matching restart, changed-association refusal and refusal to substitute an empty history for a missing intended journal.
- Browser helpers cover fixed roles, exact integers, no retry after a lost/malformed/timed-out response, uncertainty across reload, explicit acknowledgment after fresh inspection, session-cache clearing and refusal of stale projections or late command responses from a previous session. The latter regression required correcting the test's simulated server so a previous session response could not overwrite its independent new history.

### Actual local HTTPS rehearsal

All ten scenario groups passed using the repository NGINX template, the actual debug application and locally compiled NGINX 1.30.6. The rehearsal used disposable generated secrets and a certificate trusted only within its Python process; it changed no system trust or installed service.

It verified TLS checking; authentication on all resources; credential revocation; overwritten caller principal; authorization/origin/media/size rejection without evaluation; full-body buffering; ten-second header and body inactivity limits; the exact body limit; simultaneous authenticated clients with shared facts; response loss after commit with one bid group; and at most one upstream attempt for both POST and GET when the upstream fails. It also confirmed external Authorization values are stripped and test credentials absent from operational logs. The stopped writer's 13-record journal copy passed integrity and matching read-only reconstruction with identical bytes. Disposable passwords/private keys were removed and test servers stopped.

The cold-copy check verifies journal preservation and reconstruction. The full operator bundle procedure additionally requires the matching executable/source, exact configuration, session receipt and relevant nonsecret diagnostics; off-host preservation and deployed restoration remain part of the separately approved operating exercise.

### Browser inspection and limits

The actual static assets were rendered with recorded valid state/history fixtures in separate browser environments for Bidder A, Bidder B and facilitator. Fixed identities, shared balances, facilitator-only settlement, Record navigation with Auction controls hidden and `system-ui, sans-serif` were inspected. These were read-only rendering fixtures, not a live browser authentication test or an actual two-device invited session.

Linux CI is configured to run the checks, release build and local HTTPS rehearsal, but has not run on this unpublished revision. The local rehearsal does not verify any future Droplet's firewall, DNS/TLS renewal, clock, permissions, storage or off-host backup. Those checks and the two-person learning exercise remain gated by deployment approval. No real credentials, invitations, infrastructure changes, repository pruning, commits, pushes or deployment occurred.

Evidence is retained in the creating task's external `outputs/numbers-revision-0.3/`: `preservation-final.json`, `release-guided-final/`, `gateway-rehearsal-final/result.json`, and the bidder/facilitator/Record screenshots. Final check/build/guided/gateway logs are ignored files in its `Numbers/` worktree. Documentation links and whitespace were checked separately. Preserved backups and the original checkout were not used as live test histories.

The following sections are dated historical evidence for 0.2 and cleanup, not verification claims about 0.3 or current authorization gates.

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
