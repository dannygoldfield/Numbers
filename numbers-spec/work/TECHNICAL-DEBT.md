# Prototype limits and technical debt

## Current register as of 2026-10-10

Revision 0.2 clarification (2026-10-18): RT06 permits future separately specified append-only transfers without rewriting original finalization; no transfer is implemented. PR07 deliberately separates histories, so compatibility notes below do not require a legacy decoder in the new build.

This is the current planning register. The historical rationale below is preserved. Limits deliberately chosen for Prototype 1 are not automatically defects or promises to add infrastructure. All changes to behavior still require an explicit specification decision. The project owner prioritizes these items when approving the next scope; the implementer records verification when an item is changed.

| Reference | Current disposition | Revisit when |
|---|---|---|
| TD01 | Two simulated identities; no authentication. Accepted local-demo limit. | Before independent participants can act as themselves. Specify identity, access and permissions. |
| TD02–TD03 | One application writer and SQLite. Keep this architecture unless an observed need contradicts it. | Remote operation needs access controls; more servers or a new database require separate evidence, not merely a second browser. |
| TD04 | Operator-selected simulated settlement, with no deadline. Accepted scope. | A new learning goal requires autonomous settlement or actual obligations. Define triggers before implementation. |
| TD05–TD07 | Request-driven evaluation, local clock and payload consistency hashes. Accepted scope, not independently verifiable history. | Remote timing/fairness or independent reliance on records becomes part of the test. |
| TD08 | Full reconstruction and a full in-memory history; commits clone derived state. Accepted for short histories. | Measured startup or command latency obstructs the intended experiment. Measure before adding caches or snapshots. |
| TD09 | Ownership is an assignment within the prototype. External enforcement is outside scope. | A proposal connects the record to external assets or obligations. |
| TD10 | Retired before implementation: no settlement deadline remains. | Only through an explicit proposal for new settlement semantics. |
| TD11 | Internal state/events still use generic JSON and string labels. Deferred maintainability work. | Extending a lifecycle or record shape makes a small Rust type materially reduce ambiguity. Preserve existing wire bytes and test old histories; avoid a wholesale rewrite. |
| TD12 | Historical `frontend/package-lock.json` has three open GitHub dependency alerts: js-yaml (high), browserslist (high), @humanfs/node (moderate). Prototype 1 uses a separate manifest; the legacy frontend still runs in its own CI job. | Address before reviving or deploying that application, or processing untrusted inputs through its affected build tools. Keep dependency maintenance separate from auction semantics. |
| F01 | Exceptional compensation remains undefined and excluded. Ordinary release and capture are already fully specified. | Before economically meaningful loss or a real-value experiment. Specify eligibility, funding and authority without rewriting prior outcomes. |
| Audit 2026-10-10 | Direct journal commit and mutable live engine state are private; callers observe through read-only accessors. | Compiler-checked examples protect the boundary; future changes must keep the planner as the only source of committed groups. |

Prototype 2 planning and decision gates are in [PROTOTYPE-02.md](PROTOTYPE-02.md). Keep each future debt entry specific: limitation, reason to defer, and the evidence that would require addressing it.

### Historical dependency evidence — 2026-10-10

GitHub reported three open alerts during publication. All identify `frontend/package-lock.json`, not Prototype 1's manifest: [js-yaml #48](https://github.com/dannygoldfield/Numbers/security/dependabot/48), fixed in 4.3.2; [browserslist #46](https://github.com/dannygoldfield/Numbers/security/dependabot/46), fixed in 4.28.7; and [@humanfs/node #45](https://github.com/dannygoldfield/Numbers/security/dependabot/45), fixed in 0.16.8. These versions and severities are a dated advisory observation, not a completed upgrade or a claim that the entire repository is vulnerability-free. They remain open maintenance work under TD12.

## Historical candidate register

Non-normative working record, updated through Phase 2. These are candidate recommendations or explicitly confirmed scope limits, not installed mechanisms or executable rules. Every shortcut is local to the named boundary. Q01 has been resolved by the user as indivisible rana with exact integer accounting. Q03 is also resolved by C44 as exact winning-amount capture to a protocol-held balance without a spending operation. C45/C46 now resolve Q02/Q04 as leader-only holds and failed-settlement release. None of those economic decisions is a shortcut or unresolved implementation choice.

## Shortcut register

| ID / shortcut | Reason | Behavior it must not change | Known limitation | Replacement trigger | Likely replacement boundary |
|---|---|---|---|---|---|
| TD01 / Two selectable simulated identities; no authentication (confirmed) | One operator can demonstrate competition without account infrastructure. | Backend eligibility, balances, bids, fixed winner, and title. | Selection proves no human identity; any local operator can select either identity. | First test with independently acting or untrusted participants. | Command admission and identity mapping, preserving recorded stable identities. |
| TD02 / One local process and serialized writer (recommended) | Minimize operational and inter-component complexity. | Total canonical order, atomic groups, one active auction, and no implicit retries. | No availability guarantee through process outage; no concurrent service deployment. | Evidence that one-machine learning needs simultaneous independent writers or remote operation. | Persistence/command serialization, preserving the same transition predicates. |
| TD03 / One SQLite database (recommended) | Credible durable transactions and simple deployment. | Durable truth, append-only application operations, hashes, restart validation, and indivisible economic groups. | Machine/database owner can rewrite or delete files; storage loss can halt the demo. | Shared trust, independently verified history, or demonstrated storage requirements exceed one file-backed database. | Journal persistence only; no change to historical event meaning. |
| TD04 / Existing local settled/expired command (retained source behavior) | Exercises separate settlement and both title outcomes without outside payments. | Fixed resolution, backend preconditions, explicit Rana effects, finality, and exactly-once title. | Operator selects the demo branch; it is not naturally observed payment failure. AwaitingSettlement can wait indefinitely without a command. | Learning requires autonomous settlement, real participant obligations, or objective failure predicates. | Settlement command/evaluation rules; any changed trigger requires a specification revision. |
| TD05 / Request-driven lifecycle evaluation (retained source behavior) | Avoid a scheduler or worker. | Due-close precedence, first-valid-bid opening, fixed recorded timing, and finalization-plus-gap gating. | No new due record until the next evaluation boundary. UI polling cadence affects observation/commit time, not the persisted base deadline. | Demonstration needs progress while no requests occur. | Invocation of the same serialized evaluator; changing recorded timestamp semantics requires explicit review. |
| TD06 / Payload SHA-256 without complete chain or external attestation (recommended retention) | Reuses a defined consistency check with little machinery. | Canonical serialization, recorded order, immutable application operations, and rejection of hash mismatch. | Hash excludes envelope; an administrator can edit content and recompute hashes or remove a valid suffix. It is not proof against host control. | History must be trusted by someone who does not control the machine. | Integrity/attestation layer, without rewriting existing payloads. |
| TD07 / Local authoritative server clock (retained assumption) | Sufficient for one-machine timed learning. | Canonical server timestamps, fixed deadline arithmetic, and reconstruction from records. | No protection against hostile clock control; no fairness claim under manipulated time. | Untrusted timing, observed clock anomalies that invalidate demo assumptions, or shared deployment. | Time acquisition and explicit time-error semantics; no retroactive deadline edits. |
| TD08 / Full journal reconstruction, no snapshots (recommended) | Short histories do not justify caches as durable truth. | Same derived state from the same valid prefix; no repair or replayed financial effect. | Startup cost grows with history. | Measured reconstruction time obstructs the intended demonstration. | A verifiable derived cache; canonical journal remains controlling. |
| TD09 / Local display title only; no external ownership mechanism (confirmed) | Tests protocol title and its consequences without Bitcoin or legal infrastructure. | Exactly one irreversible title assignment from specified settlement. | No identity proof, external enforceability, legal land claim, or asset-delivery claim. | Any proposal to connect title to external assets, humans, or real obligations. | Explicitly specified external boundary; prior protocol title is never silently reinterpreted. |

Rana denomination is now explicitly decided by C43. Successful capture disposition is explicitly decided by C44. Reservation policy and expired-branch release are explicitly decided by C45/C46 and restated in RT02/RT03; they are not deferred into code. Irreversibility of finalized facts is a protocol requirement, not technical debt to undo later.

## Deferred issue F01 — Exceptional compensation

**Status:** Deferred, not executable in Prototype 1. Deferring the discretionary refund/compensation capability is acceptable documented Prototype 1 scope debt; it does not defer the definition of ordinary reservation and settlement accounting. Reconsider before the first prototype exposing independently acting people to economically meaningful Rana loss, and before any real-value or externally enforceable title experiment, whichever comes first.

**Ethical friction:** Mechanical finality can preserve an internally valid outcome that a participant experiences as unfair: mistaken input, misunderstanding, defective presentation, operator misuse, or an implementation defect. Acknowledging harm is necessary; it does not itself specify a permitted transaction.

**What remains irreversible:** Accepted canonical facts, resolved winner, recorded settlement outcome, captured economic history, final protocol title, and consumed permission cannot be erased or reopened by an appeal. PublicLand cannot become an exception-handling account or a route for replacement title.

**Later event versus rewriting:** A future explicitly defined compensation event could record a new transfer or issuance while preserving the original loss. It would still change economic state and would require a source of funds, eligibility predicate, amount rule, authority, ordering, and at-most-once limits. Calling an entry append-only does not authorize it. Rewriting the original result or deleting capture would alter historical truth and remains distinct from a new event. Prototype 1 has neither discretionary operation.

**Why a good reason is insufficient:** “A sufficiently good reason” does not tell an implementation what evidence is admissible, who decides, what action is allowed, which funds pay, or how duplicate requests are prevented. It invites discretionary branches rather than a reproducible protocol rule.

**Categories for later examination:** Participant input mistakes; inaccessible or misleading interface information; implementation defects or inaccurate projections; identity misuse or coercion; operator misconduct; timing or persistence incidents; conflicting evidence about promised outcomes; and interaction with future real-world obligations. These categories are research topics, not accepted claims or current triggers.

**Unanswered evidence and authority:** What proves harm or fault? Who can submit evidence, evaluate it, and contest a decision? Is authority protocol-defined or explicitly external? Who funds payment? What amount/cap applies? What prevents duplicates and conflicts of interest? How are rejected requests recorded? What happens if evidence is unavailable? None is resolved here.

**Unanswered title consequences:** Can monetary compensation ever coexist with unchanged winner/PublicLand title? Would a future rule authorize a new independent award, and how would it avoid duplicating the same number's title? How are expectations communicated without promising reassignment? No current title change is authorized.

A specified release of an uncaptured reservation is ordinary original accounting, not discretionary compensation. The user explicitly selected expired-winner release in Q04/C46; it was not inferred from the no-compensation rule.

## Security learning record — initial Prototype 1 checkpoint (historical)

**Protected requirements:** Canonical economic truth, fixed resolution, final settlement/title, ordered sequence, and exact reconstruction of a valid committed prefix.

**Credible means for later specification:** Backend-only evaluation; one ordered writer; atomic durable transactions; application paths that only append canonical records; strict schema/precondition/reference/order/hash validation; derived balances/title; explicit failure; no automatic command retry. These are requirements or recommendations already sourced in `SOURCE-DECISIONS.md`, not a new security subsystem.

**Deliberately unprotected:** Malicious machine owner, direct database editing/deletion, forged local identity selection, host-clock manipulation, storage destruction, remote adversaries, distributed consensus, and legal ownership. No production authentication, signing service, monitor, replicas, or anchoring is added.

**Assumptions:** One controlled local machine; a cooperative operator; local durable storage capable of atomic commits; a suitable server clock; no real Bitcoin or external irreversible payment. Tests will need to show restart and interrupted-transaction behavior; those tests have not been run because this task does not implement code.

**Evidence requiring change:** Actual harm outside the simulated setting, competing operators, independently relied-upon history, unexplained integrity failures, inability to reconstruct, or measured persistence/time limitations. Failure of an existing invariant requires stopping and investigating; it does not justify silently adding a recovery path.

## Future security mechanism ledger

These are possible future investigations, not planned Prototype 1 work. Append a dated later-prototype entry when a prerequisite is met; do not rewrite this checkpoint as though the later mechanism existed earlier.

| Possible mechanism | Threat addressed | Prerequisite | Complexity introduced | Simpler mechanism extended/replaced | Preservation of protocol meaning |
|---|---|---|---|---|---|
| Complete record hash chain | Undetected envelope edits, internal deletion/reordering, and broken history linkage | Need to detect more local corruption than payload checks cover | Versioned envelope encoding, predecessor linkage, migration boundaries; still no protection against an owner recomputing the chain | Independent payload-only hashes | Adds attestations; does not change bids, economic predicates, title, or existing events. |
| Signed checkpoints | A verifier needs to identify who attested a journal prefix | An independent verifier and an explicit trust/key model exist | Key custody, rotation, checkpoint scheduling and verification | Unsigned local hash inspection | Attests a defined prefix; signatures cannot finalize auctions or override title. |
| Independent checkpoint publication | Operator rewrites an entire previously presented history | An independent audience relies on published history | Publication availability, audience, privacy, cadence, and disagreement rules | Locally retained checkpoint evidence | Publication is evidence, not a new settlement or title authority. |
| Independent replicas | Sole copy loss or operator control of all evidence | Actual independent custodians and availability needs | Replication protocol, consistency evidence, retention and access control | One local durable copy | No second writer or consensus behavior without an explicit new specification; never reinterpret a finalized prefix. |
| Bitcoin anchoring | Need for publicly durable external evidence of a prefix commitment | Separate future scope approval and a demonstrated need for that evidence | Bitcoin integration, fees, confirmation/reorg and key handling | Independent publication of attestations | External evidence only under a new bounded design; no Bitcoin concept enters Prototype 1 auction logic. |

No future mechanism is necessary to answer the current central prototype question under its local assumptions. At the initial checkpoint the semantic blockers were accounting decisions, not missing cryptography. C43–C46 have since resolved them; the completed current security record is SECURITY-NOW-AND-LATER.md.

## Phase 2 disposition

All four economic decisions are now fixed in the inactive candidate. No undefined accounting has been converted to debt. The no-spending operation for protocol-held Rana is a confirmed Prototype 1 capability boundary (C44), not an implementation omission or permission for later discretionary payouts.

TD07's suitable-clock assumption is explicit in SE04: backwards observed time stops processing without clamping or changing deadlines. PR06 specifies the storage-failure boundary. Current security responsibilities are in SECURITY-NOW-AND-LATER.md; future learning sections remain appendable.

TD10 — Retained inactive settlement deadline: keep the source's recorded deadline even though local settlement never auto-expires. Reason: preserve the precise retained local settlement contract without an unapproved trigger change. It must not change settlement, title, or Rana outcomes. Limitation: a displayed deadline can be misunderstood as an active timer; the UI must explain that it does not auto-expire. Replacement trigger: a separate decision to simplify this field or enable objectively timed settlement. Replacement boundary: resolution payload and settlement evaluator in a new explicit specification/API revision, preserving prior recorded deadlines and outcomes.


## Approved revision 0.1 — 2026-09-05

C47 / S24: The user approved removal of the unused settlement deadline after reviewing the two-command settlement proposal. Remove settlement_deadline_seconds, settlement_window_seconds, settlement_deadline, their derivation and UI/example references. Settled captures the full winning hold into protocol-held Rana and assigns winner title; expired releases that hold and assigns PublicLand. AwaitingSettlement retains the hold until an explicit accepted command. Auction timing and economic predicates are unchanged. TD10 is retired before implementation; M12 is superseded only for the deadline. Owners: P02, RT03, SE03/SE04/SE06, PR05, UI01/UI02/UI04.

C48 / S25: In response to the combined simplify/activate/build request, the user said: “Yes I approve the the settlement simplification. With that I approve implementation of Prototype 1.” This activates numbers-spec revision 0.1 as the sole active authority. codex-spec is historical and untouched. Owners: STATUS, AU01. Earlier draft-only statements in this working record describe their historical checkpoint. Local implementation is authorized; no commit, push or publication was requested.

C49: Rana remains useful for this prototype, but need not remain a public concept or permanent internal standard for future payment integrations. Prototype 1 accounting is unchanged.
