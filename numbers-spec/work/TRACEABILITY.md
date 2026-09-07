# Candidate traceability

Non-normative audit map. Every candidate normative clause, invariant, lifecycle transition/commit group, and event is mapped below. C/K/M/A identifiers resolve to exact user decisions, source file/sections, explicit representation choices, or declared assumptions in SOURCE-DECISIONS.md. That provenance does not activate this candidate or import unstated behavior.

## Normative clause coverage

| Clause | Candidate owner | Authoritative decision/source basis |
|---|---|---|
| STATUS | STATUS.md | C02/C38: draft/inactive metadata and separate activation |
| AU01 | AUTHORITY.md | C02/C38; K01 |
| AU02 | AUTHORITY.md | C38; K01; explicit candidate organization |
| AU03 | AUTHORITY.md | C04/C39/C41; K01/K18 |
| AU04 | AUTHORITY.md | C02/C03/C39/C40/C41; K01/K13/K20 |
| SC01 | SCOPE.md | C05/C06/C13–C23 |
| SC02 | SCOPE.md | C07/C18–C25/C27–C31/C43–C46 |
| SC03 | SCOPE.md | C08/C18/C20/C32/C37; K18; M11 |
| SC04 | SCOPE.md | C16/C17/C23/C25/C32/C33; K15/K18 |
| GL01 | GLOSSARY.md | C09–C19/C43–C46; K03/K09–K15; M01/M04 |
| IV01 | INVARIANTS.md | C15–C25/C39; K01–K20; invariant-by-invariant map below |
| P01 | PROTOTYPE-01.md | C09/C18/C19/C43/C44; M01/M05 |
| P02 | PROTOTYPE-01.md | C12/C19/C40; K02/K06/K07/K09/K20; M12/A01 |
| P03 | PROTOTYPE-01.md | C07/C24/C25; K19 |
| RT01 | RANA-AND-OWNERSHIP.md | C15/C19/C21/C25/C43/C44; D01/M02/M05 |
| RT02 | RANA-AND-OWNERSHIP.md | C21/C25/C45; D02/M03 |
| RT03 | RANA-AND-OWNERSHIP.md | C14/C17/C21–C23/C44/C46; K09/K10; M04/M12 |
| RT04 | RANA-AND-OWNERSHIP.md | C10/C11/C17/C22/C46; K10/K11; D05/M04 |
| RT05 | RANA-AND-OWNERSHIP.md | C16/C17/C23/C25; K08/K10/K15/K18; D07/M04/A01 |
| SE01 | STATE-AND-EVENTS.md | C13/C14/C17/C22; K02–K04/K08–K12; C45/C46 |
| SE02 | STATE-AND-EVENTS.md | C07/C20/C25/C30; K04/K09/K16–K19; M07/M08/M10/M11 |
| SE03 | STATE-AND-EVENTS.md | C12/C21/C43/C45; K04–K09/K17/K20; M07/M08/M12 |
| SE04 | STATE-AND-EVENTS.md | C06/C12/C13/C16/C17/C25; K02/K05/K09/K12/K16; M09/A01 |
| SE05 | STATE-AND-EVENTS.md | C19/C21–C25/C43–C46; K04/K08–K13; M02–M06 |
| SE06 | STATE-AND-EVENTS.md | C15/C19/C21–C25/C30/C43–C46; K04–K15; M01–M08/M12 |
| PR01 | PERSISTENCE-AND-RESTART.md | C25; K13/K14; M01/M05/M06 |
| PR02 | PERSISTENCE-AND-RESTART.md | C25/C26/C43; K14; M07/M08; S22 |
| PR03 | PERSISTENCE-AND-RESTART.md | C16/C25; K04/K10/K13/K15; M03–M06/A01 |
| PR04 | PERSISTENCE-AND-RESTART.md | C15/C16/C25/C40; K13–K15/K20; M05–M09/A01 |
| PR05 | PERSISTENCE-AND-RESTART.md | C16/C22/C25; K15; C43–C46; M03–M06 |
| PR06 | PERSISTENCE-AND-RESTART.md | C16/C23/C25/C26; K15/K18; D08/A01 |
| UI01 | UI-AND-DEMO.md | C07/C18/C20/C27–C31; K09/K19; M07/M10 |
| UI02 | UI-AND-DEMO.md | C07/C25/C28/C30/C31; K16/K17/K19; M07/M08/M10 |
| UI03 | UI-AND-DEMO.md | C07/C12/C13/C17/C28–C31; K05/K12/K16/K18/K19; M09 |
| UI04 | UI-AND-DEMO.md | C19/C21/C22/C39; C43–C46; M13; K05/K06/K15 |
| SN01 | SECURITY-NOW-AND-LATER.md | C25/C26/C34–C37; K13–K15; A01 |

Implementation recommendations in PROTOTYPE-01, future security entries, and all working files are non-normative. No economic choice is delegated to them.

## Invariants

| Invariant | Owning candidate clauses | Source/decision provenance |
|---|---|---|
| I01 | P02, SE01, SE04 | K02/K12; C17 |
| I02 | SE01, SE03 | C13; K03/K04 |
| I03 | PR01, PR04, RT01, RT04 | C15/C23; K13 |
| I04 | PR01, PR06 | C23/C25; K13/K15 |
| I05 | RT02, SE03, SE05 | C21/C25/C45; K04/K05 |
| I06 | RT01 | C15/C19/C21/C43/C44; M02 |
| I07 | RT02, RT03 | C45/C46 |
| I08 | SE03, SE06 | K07/K08 |
| I09 | RT03–RT05, SE06 | C14/C17/C23; K08–K10 |
| I10 | RT01, RT03, SC03 | C21/C32/C44/C46 |
| I11 | RT04 | C10/C17/C22; K10 |
| I12 | RT04 | C11; K11 |
| I13 | P02, SE03–SE04 | C12; K05/K06 |
| I14 | SE04 | C13/C17; K12 |
| I15 | PR04–PR06 | C16/C25; K15 |
| I16 | PR01–PR06, SE05 | C25; K13/K14 |
| I17 | UI01–UI03 | C07; K19 |
| I18 | P02, PR04–PR06, SE02 | C40; K18/K20 |

## Every permitted transition and commit group

| Group | State transition | Decision/source basis | Records and restart owner |
|---|---|---|---|
| G01 | Empty store -> first Scheduled plus allocations | K02/C19/C25; M05 | SE05/SE06, PR03/PR05 |
| G02 | Scheduled -> Open | K04/C45 | Bid, Reserve, Open; SE05/SE06, PR05 |
| G03 | Open -> Open, valid replacement | K04/K07/C45; M03 | Bid, Release, Reserve; SE05/SE06, PR05 |
| G04 | Open -> Open, valid extension | K05/K06/C45 | Bid, Release, Reserve, Extension; SE05/SE06, PR05 |
| G05 | Scheduled/Open -> same state, rejected admission | K04/K17; M07/M08 | Invalid Bid; SE02/SE03/SE05/SE06, PR05 |
| G06 | Open -> Closed | K05/K08 | Close; SE04/SE05/SE06, PR05 |
| G07 | Closed -> AwaitingSettlement | K08/K09 | Resolution; SE03/SE05/SE06, PR05 |
| G08 | AwaitingSettlement -> Finalized, winner title | C44/K09/K10 | Settlement, Capture, Finalization; RT03/SE05/SE06, PR05 |
| G09 | AwaitingSettlement -> Finalized, PublicLand title | C46/K09/K10/K11 | Settlement, Release, Finalization; RT03/SE05/SE06, PR05 |
| G10 | Prior remains Finalized; next number becomes Scheduled | C17/K02/K12 | AuctionRecord; SE04/SE05/SE06, PR05 |

All state entries, permitted commands, exits, resulting records, and terminal flags are in SE01. Finalized has no same-number exit. Read/evaluation boundaries are SE04; a read is not a separate canonical event. There is no pause lifecycle or pause/resume command in this new minimal surface.

## Every event

| Exact event name | Candidate contract | Authority/provenance |
|---|---|---|
| RanaIssueRecord | SE06; G01; RT01/PR01 | C19/C25/C43; M02/M05 |
| AuctionRecord | SE06; G01/G10; P02/SE04 | K02/K03/K12; C12; M05 |
| BidRecord | SE06; G02–G05; SE02/SE03 | K04/K07/K17; C43/C45; M07/M08 |
| RanaReserveRecord | SE06; G02–G04; RT02 | C21/C25/C45; M02/M03 |
| RanaReleaseRecord | SE06; G03/G04/G09; RT02/RT03 | C21/C45/C46; M02/M03 |
| RanaCaptureRecord | SE06; G08; RT03 | C21/C44; M02/M04 |
| AuctionOpenRecord | SE06; G02; SE03 | K04/K05; C12 |
| ExtensionEventRecord | SE06; G04; SE03 | K05/K06 |
| AuctionCloseRecord | SE06; G06; SE04 | K05/K08 |
| ResolutionRecord | SE06; G07; SE03/PR02 | K07/K08/K09/K14; M12 |
| SettlementRecord | SE06; G08/G09; RT03 | K09/K10; C44/C46; M04 |
| FinalizationRecord | SE06; G08/G09; RT04 | K10/K11; C10/C17/C22; M04 |

Every event's fields, references, preconditions, state consequences, ledger effects, title consequences, and group placement are explicit in SE06. PR01/PR02 supply common envelope, identifiers, serialization, and hashes. FinalizationRecord alone is the title record; there is no duplicate title-event authority.

## Rejection and failure coverage

| Rejection/failure | Record/effect owner | Source/decision |
|---|---|---|
| Invalid UTF-8/JSON/top-level/duplicate keys; unknown route/method; invalid read parameters | SE02: no canonical command record or evaluation | C25/C39; K17; M07/M10 |
| Bid outside Scheduled/Open | SE02: no BidRecord; evaluated state unchanged by command | K17 |
| Comparable wrong auction; missing/malformed bid field; unknown identity | SE03 first-priority codes; G05 audit record; no ledger effect | K17/C18; M07/M08 |
| Minimum, increment, maximum, balance failure | SE03 ordered reasons; G05; no ledger effect | K07/K17/C43/C45 |
| Settlement state, fields, target, or outcome invalid | SE02: no command record or economic/title effect | K09/K10; C25 |
| Storage unavailable before admission | PR06: explicit failure, no admission | K17/K18; C25 |
| Interrupted/uncertain commit | PR03/PR05/PR06: atomic durable truth, halt/inspect, no automatic replay | C16/C25; K15/K18 |
| Bad schema/hash/order/reference/balance/title/group; backwards clock/overflow | PR04/PR06 and SE04: explicit halt, no repair or PublicLand | C23/C25/C34; K15; A01 |

The latter halt assumptions do not supply an alternate economic outcome. All command responses distinguish SE04 evaluation records from command-attributable records under UI02.

## Material reconstruction and demonstration coverage

PR05 enumerates empty/init, invalid bid, first accepted bid, replacement/extension, overdue Open, Closed-before-resolution, fixed resolution, terminal groups, both sides of the gap, next Scheduled, and committed-lost-response boundaries. Every complete group is a supported boundary; partial groups are rejected by PR03/PR06.

UI04's main fixture covers G01/G02/G03/G05/G06/G07/G08/G09/G10 and explicit balances/title; its extension fixture covers G04 and exact window/end/max-extension boundaries. Both outcomes preserve the previous resolved winner and title finality. The audit checks arithmetic and record order as specification examples, not as an implemented engine test.


## Approved revision 0.1 — 2026-09-05

C47 / S24: The user approved removal of the unused settlement deadline after reviewing the two-command settlement proposal. Remove settlement_deadline_seconds, settlement_window_seconds, settlement_deadline, their derivation and UI/example references. Settled captures the full winning hold into protocol-held Rana and assigns winner title; expired releases that hold and assigns PublicLand. AwaitingSettlement retains the hold until an explicit accepted command. Auction timing and economic predicates are unchanged. TD10 is retired before implementation; M12 is superseded only for the deadline. Owners: P02, RT03, SE03/SE04/SE06, PR05, UI01/UI02/UI04.

C48 / S25: In response to the combined simplify/activate/build request, the user said: “Yes I approve the the settlement simplification. With that I approve implementation of Prototype 1.” This activates numbers-spec revision 0.1 as the sole active authority. codex-spec is historical and untouched. Owners: STATUS, AU01. Earlier draft-only statements in this working record describe their historical checkpoint. Local implementation is authorized; no commit, push or publication was requested.

C49: Rana remains useful for this prototype, but need not remain a public concept or permanent internal standard for future payment integrations. Prototype 1 accounting is unchanged.


## C60–C61 follow-up — 2026-09-07

| User decision | Current clauses | Implementation and evidence |
|---|---|---|
| C60: demonstrate frontend and backend with a guided walkthrough | UI05, existing SE02–SE06 and PR04 | `prototype-01/src/bin/numbers-demo/`, `DEMO.md`, `Numbers-demo.code-workspace`, `tests/demonstration.rs` |
| C61: ownership; Unowned means no owner | RT04/RT06, GLOSSARY, UI01 | Internal State.ownership, browser and terminal labels; unchanged literal JSON keys; saved-history reconstruction and both-outcome tests |

S26/S27 distinguish added demonstration tools and terminology from auction semantics. Historical mappings above retain the vocabulary at their decision time.
