# Completed candidate audit

Non-normative final specification audit. Date: 2026-09-05. Outcome B: completed inactive candidate for user review. This is a document/system-consistency audit, not execution or validation of application code.

## Scope and preflight preservation

Exact repository: `/Users/dannygoldfield/Projects/Numbers-Repos/Numbers`.

Output: `/Users/dannygoldfield/Projects/Numbers-Repos/Numbers/numbers-spec`.

At preflight, codex-spec existed, numbers-spec did not, and the main working tree was clean. Repository HEAD was `bdd26f62dedd017ca4b201289ef43ed0f0a2c3f7`. The supplied project instructions were followed; no repository/ancestor AGENTS.md file was found in the performed checks.

All 25 Markdown documents in codex-spec were read in full, including chain, wallet, inscription, API, configuration, and error material. Truncated reads were completed separately. The non-document .DS_Store metadata was preserved and fingerprinted, not interpreted as specification. No README, existing application behavior, prior conversation, network source, or GitHub operation supplied execution semantics.

The original 919 non-Git repository files were fingerprinted before writing. Final verification found every one byte-identical, including all of codex-spec; no original file was removed. Exactly 17 Markdown files are added, all inside numbers-spec. HEAD is unchanged and tracked staged/unstaged diffs are empty. Temporary audit calculations are not application code or repository deliverables.

## Decision history and scope of authority

Phase 1 correctly stopped with four economic questions. The initial agreement with recommendation entries resolved Q01 only because the other entries offered no selected rule. Later explicit user answers resolved:

- C43 / Q01: indivisible rana, exact integer accounting.
- C44 / Q03: capture exactly the winning bid amount into protocol-held Rana; no spending operation in Prototype 1.
- C45 / Q02: hold only the leader's full bid, immediate outbid release, same-identity replacement using its existing hold.
- C46 / Q04: release the uncaptured winning reservation on expired/PublicLand settlement.

No answer or later recommendation was treated as retroactive approval. Phase 2 proceeded after all four decisions. STATUS remains version 0.1-draft, status draft, authority inactive. codex-spec remains the sole active set. No implementation or activation is authorized by this audit.

## Final audit matrix

| Concern | Finding / owning evidence |
|---|---|
| Consistent terms | GL01/RT01/RT04 distinguish Rana, holds, capture, protocol-held balance, resolved winner, title, and PublicLand. Source-only names are provenance, not candidate engine concepts. |
| Explicit states | SE01 gives entry, commands/evaluation, exit records, and terminality for all five auction states; empty history and rhythm gap are explicitly not extra auction states. |
| Explicit transitions | SE05 enumerates all 10 atomic groups. No unlisted transition, no-bid finalization, reopening, or winner substitution exists. |
| Exact events | SE06 defines all 12 event names, required closed payloads, references, preconditions, ordering, state effects, Rana effects, and title effects; PR01 supplies the exact common envelope. |
| Terminality | RT05/SE01 prevent repeated resolution, settlement, hold consumption, and title assignment. Sequence advancement leaves prior finality intact. |
| Deterministic two-identity demo | UI04 covers opening, competing bids, rejection, leadership changes, same-identity replacement, reservation/release, fixed winner, successful capture, title, gap, and next dormancy. A second auction covers PublicLand and release. |
| Rana issuance and conservation | RT01/G01 define exactly two initial issues, zero initial holds/protocol balance, exact integer equations, and total conservation. No later issuance, top-up, or spending is implied. |
| Reservation/replacement | RT02/G02–G04 encode only the leading hold, immediate outbid release, and replacement affordability. A rejected bid moves no Rana. |
| Capture and failed settlement | RT03/G08/G09 explicitly encode successful protocol credit or failed-settlement release. No losing hold is released twice at settlement. |
| Title and PublicLand | RT04 makes FinalizationRecord the sole title record. Winner and PublicLand are unambiguous alternatives selected by fixed terminal settlement. PublicLand has exactly one meaning and one permitted trigger. |
| Auction versus settlement | Closed resolves into AwaitingSettlement; explicit local command chooses terminal settlement under backend preconditions. Resolution never changes. |
| UI authority | UI01–UI03 only submit commands and display canonical/derived facts. Protocol View has no mutating controls. Request-triggered evaluation is explicitly backend-owned and separately attributed. |
| Timing | P02 fixes 225 seconds base duration and 12 seconds post-finalization gap; SE03/SE04 retain bounded extensions and close-before-admission at equality. No silent extension removal or production duration declaration. |
| Local deadline | Source deadline remains recorded and inert. No automatic local expiry at runtime or restart; if shown, UI labels its inactive role. |
| Ordering and atomicity | PR01/PR03 and SE05 provide one serialized order and indivisible groups. Bid/open/reservation/extension and settlement/ledger/title cannot partially persist. Close and resolution have separate supported boundaries. |
| Restart coverage | PR04/PR05 cover every complete group and every requested material boundary, including lost response and expired downtime. Replay performs no writes; subsequent permitted evaluation is separate. |
| Malformed history and interrupted writes | PR06 rejects/halts explicitly; it neither synthesizes missing records nor interprets corruption/uncertainty as PublicLand or forfeiture. |
| Hashes and tamper claims | PR02 retains payload SHA-256 and a precisely adapted resolution-input hash. SN01 states envelope exclusion and host-owner limitations. No production tamper resistance is claimed. |
| Bitcoin exclusion | SC03 removes Bitcoin/network/wallet/inscription concepts and records entirely from the new engine. Source references and future security discussion do not activate them. |
| Debt visibility | TD01–TD10 identify scope/implementation shortcuts, limits, protected behavior, replacement triggers and boundaries. Economic choices are resolved rules, not debt. |
| Deferred compensation | F01 covers ethics, evidence, authority, title, immutable facts, later additive events, and a reconsideration milestone. No executable compensation capability was added. |
| Infrastructure proportionality | One process/SQLite are non-normative recommendations. No production hosting, workers, generalized framework, signing service, replicas, or anchoring requirement. |
| No competing authority | AU02 defines order and section scope; each operational rule has an owning clause. work/ and recommendations are non-normative. |
| Traceability and changes | TRACEABILITY maps all clause/invariant/group/event IDs. SOURCE-DECISIONS contains 46 confirmed-decision entries, 20 source-kernel entries, 13 explicit representation/consequence entries, and one declared-assumption entry. SPEC-CHANGES records S01–S23. |
| Completeness within scope | No remaining blocking semantic question or unresolved alternative/placeholder in normative documents. Non-blocking runtime/path/presentation choices remain explicitly bounded in OPEN-QUESTIONS. |

## Conflicts and mechanical corrections

Source-authority conflicts were resolved using the source's ranked precedence, not new product choices: record-gated next-number creation over the broad no-time-creation wording; configuration capture in AuctionRecord over later max-extension capture; atomic deferred intent over looser later-append wording in the old generation; local settlement table over chain-style prose; and inscription-only authority over broader low-level ambiguity enums. Their source citations are in SPEC-CHANGES. The user explicitly superseded inscription obligations and settled the new economic questions.

During Phase 2, the audit corrected:

- Exact request storage: encoding original UTF-8 bytes as lowercase hexadecimal prevents canonical NFC normalization from altering invalid input or merging distinct request keys during reconstruction.
- Rejection priority: a comparable wrong-auction target precedes missing/malformed field rejections, retaining the relevant source order; new identity/balance checks have explicit positions.
- Display semantics: the retained settlement deadline is explicitly labeled inactive, and read-triggered automatic evaluation records are distinguished from the submitted command's own records.
- Title-trigger wording and working-record status labels were reconciled to the final chosen rules. No rejected economic alternative remains operative.

The server timestamp sampling point and nondecreasing-clock/storage assumptions are explicit in SE04/PR06 and recorded as M09/A01/S19. They bound the single-machine prototype; no distributed fairness, trusted-time, or recovery mechanism is claimed.

## Verification actually performed

1. Checked the exact 11 root and 6 working Markdown filenames, required five STATUS metadata lines, and local Markdown links.
2. Verified unique IDs and traceability for all 35 normative clause IDs, 18 invariants, 10 commit groups, and 12 event types. No missing mappings or duplicate clause IDs were found.
3. Checked normative files for unresolved placeholders; none were found. Read and reconciled state/event/accounting/restart/interface rules as one system.
4. Independently checked eight example balance snapshots: every one is nonnegative and conserves the 200 initially issued rana. Successful capture moves exactly 50 into protocol-held Rana. The PublicLand example releases exactly 10 and leaves the protocol balance at 50.
5. Checked all 31 record positions against complete-group lengths, 225-second auction timing, the 12-second gaps, and exact extension-window/end/max-extension arithmetic.
6. Compared all 919 original file hashes, checked added-file boundaries and Git status/HEAD, and confirmed no tracked code or source changes.

These are specification, structural, and arithmetic checks. No application was written, no runnable prototype was exercised, and SQLite crash durability has not been tested in an implementation. A later implementation must demonstrate UI04 and PR05 before claiming operational correctness.

## Final disposition and file set

Ready for user review as a complete, internally consistent inactive Prototype 1 candidate under the declared local assumptions. No blocking questions remain. Non-blocking choices are local operational settings, presentation details, and implementation/runtime selection constrained by the normative behavior. Activation remains a separate explicit user decision.

Root files: STATUS.md, AUTHORITY.md, SCOPE.md, GLOSSARY.md, INVARIANTS.md, PROTOTYPE-01.md, STATE-AND-EVENTS.md, RANA-AND-TITLE.md, PERSISTENCE-AND-RESTART.md, UI-AND-DEMO.md, SECURITY-NOW-AND-LATER.md.

Working files: SOURCE-DECISIONS.md, SPEC-CHANGES.md, TECHNICAL-DEBT.md, OPEN-QUESTIONS.md, AUDIT.md, TRACEABILITY.md.

No code change, commit, push, publication, or activation occurred.

## Frozen source fingerprints

The following SHA-256 values were captured before any repository write.

| Source document under codex-spec | SHA-256 |
|---|---|
| `AUTHORITY-ORDER.md` | `140dd5bf0b6b7a0af2228b07b114e48d0d3aba979f86dd0400af735f4b6d4c4a` |
| `IMPLEMENTATION-SLICE-01.md` | `0777682f387a804821a9caf0bf1a50355caad3940e5249feafec9c64c97a7aaf` |
| `PROTOTYPE-SCOPE.md` | `32ee27b62700a9dc4d01663b22f351b35d6ba2fb65a2ee141cd849d21308d86a` |
| `SPEC-INDEX.md` | `9f7c4658252e2fd481ce0f3eb6e2f5d4dc9ea0a1662afec64c9d89d1d76c8438` |
| `api/API-SPEC.md` | `68907150934e845ae1bd3bb11843987762ea5b7f9114052054bb7517263eeceb` |
| `api/API-STATE-SHAPES.md` | `990af80c85dee0d78d5bc069ffc087d9a8a54825ba9446518b2c5d1993ffd7b9` |
| `bidding/BIDDING-ADMISSION.md` | `b45afa9cbbbf647fedd921e5fd4147603b89b1a490f75a57154d7d8406f7a140` |
| `bidding/SETTLEMENT.md` | `5dc219f296bc2360344b2503558921b0fd8396ac124d5de4162243ec14d269e7` |
| `chain/CHAIN-INTERACTION.md` | `3f236bded2bb7bcb214ce5bcfac14255dddfab0e09801009b5eb1961c1aeabb5` |
| `config/CONFIG-REFERENCE.md` | `b59c37995210434eeb49c4d4c1631ff0e3b3f9515e8dc09442f479de910c9273` |
| `core/AUTHORITY-CONSUMPTION.md` | `10e0767243454c478ea4a1925976fee53ede7ce5f682a263d9566b31f5134c6b` |
| `core/CORE-SEQUENCE.md` | `926676cb487c850e7e444b49d20df285b25d3bb497cf34b0d595d286c8b6a84d` |
| `core/EVENT-TYPES.md` | `e7c97b65d810874a73ee857dde619a722a539709922c543e12d7a7d52fe7e85e` |
| `core/INVARIANTS.md` | `f5e916fe3db7240d2bf28129f45890aed252e765172f4288fa8dcef3e9d8524a` |
| `core/NULLSTEWARD.md` | `11e3920ac5c6a029b992c113908269e3ab341f5e5ef959f700baa090ae8ebfc0` |
| `core/STATE-MACHINE-TABLE.md` | `aa3e7ad15125f7ec911f512f53eb5235ff60b5f59d143bd6b54cf56f9fde28a8` |
| `core/STATE-MACHINE.md` | `fdd4300a9a83b60191acdb3ca34b1529b568dbe5f13513bb0f9cd1b323ff9f4b` |
| `core/TRANSITION-INVARIANTS.md` | `1286efaa20d82f16a99081f30a116e19ee16455c324448c316eaed53963524f2` |
| `data/DATA-MODEL.md` | `1717cfb52807b3235434623a3995b3e15ed770da282a9c8d1992d850e70cc37b` |
| `data/PERSISTENCE.md` | `b54ae7b35b61494a29c553432d5063abec6e16dbcfc1c63f43da0d8bff2f5ccf` |
| `data/RESTART-RULES.md` | `334f0c7519a802d0a66a5c0aba1ba2b996f8d165901795dbba947f6609960000` |
| `errors/ERROR-TAXONOMY.md` | `b415194c14b88bf2c3d912e79282b0939200c978a00a6c6797e176a63215175e` |
| `inscription/INSCRIPTION-FORMAT.md` | `4b96c9b313b251b92ec63f372d66f51f44b817a88780f62453f835aa470db978` |
| `inscription/INSCRIPTION-MACHINE.md` | `83a160b9ada00b2e42bacb2da5c46ac73d50ee5d3ad0b5ba2ce8204e7ebf4071` |
| `wallet/WALLET-SPEC.md` | `1469ced6b91a3fd0812bd8a3ff2b07cefcf0a5f5486d218484a2e524d345036f` |


## Approved revision 0.1 — 2026-09-05

C47 / S24: The user approved removal of the unused settlement deadline after reviewing the two-command settlement proposal. Remove settlement_deadline_seconds, settlement_window_seconds, settlement_deadline, their derivation and UI/example references. Settled captures the full winning hold into protocol-held Rana and assigns winner title; expired releases that hold and assigns PublicLand. AwaitingSettlement retains the hold until an explicit accepted command. Auction timing and economic predicates are unchanged. TD10 is retired before implementation; M12 is superseded only for the deadline. Owners: P02, RT03, SE03/SE04/SE06, PR05, UI01/UI02/UI04.

C48 / S25: In response to the combined simplify/activate/build request, the user said: “Yes I approve the the settlement simplification. With that I approve implementation of Prototype 1.” This activates numbers-spec revision 0.1 as the sole active authority. codex-spec is historical and untouched. Owners: STATUS, AU01. Earlier draft-only statements in this working record describe their historical checkpoint. Local implementation is authorized; no commit, push or publication was requested.

C49: Rana remains useful for this prototype, but need not remain a public concept or permanent internal standard for future payment integrations. Prototype 1 accounting is unchanged.


## Prototype 1 implementation checkpoint — 2026-09-05

The user requested Rust installation, clear backend design and notation for human and AI reviewers, and a complete development environment rather than shortcuts for speed. The local implementation is in `prototype-01/`, with a pinned standard Rust installation, formatter, linter, editor support, exact dependency locks and repeatable checks. This is an implementation and tooling choice; it adds no economic or protocol rule.

See [the backend review guide](../../prototype-01/REVIEW.md) for rule-to-code reading order and [verification evidence](../../prototype-01/VERIFICATION.md) for the automated checks, two browser settlement paths and restart comparison. The original repository files remain untouched.


## C51–C52 — Fresh demonstration configuration, 2026-09-05

The user requested minimum_bid_rana=5, minimum_increment_rana=1, and 500 extra Rana for each bidder, then authorized returning to Number 1 if simpler. Use a separate empty demo journal, starting_number=1, initial allocations=600 each (original 100 plus 500), minimum5/increment1. Preserve the previous journal unchanged. This is existing P02 initialization and PR06 separate-history behavior, not a reset or reinterpretation of recorded facts. The proposed SE07/G11 funding amendment was withdrawn before implementation or use; revision 0.1 remains active with no later-issuance feature. C50's future five-rana increment is superseded for the new demo. UI04 retains its explicit historical test fixture.
