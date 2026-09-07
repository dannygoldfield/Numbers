# Vocabulary

## GL01 — Fixed terms

| Term | Meaning / owning rule |
|---|---|
| Rana; `rana`; `RANA` | In-house value standard. One rana is indivisible; `rana` is singular and plural, `RANA` is the machine code. Economic rules: RT01. |
| Simulated identity | Either `demo-1` or `demo-2`; selecting one proves no human identity. P01. |
| Number / auction | A single numbered lifecycle. It exists in Scheduled before its first valid bid. SE01. |
| Canonical record / event / journal record | The same immutable persisted object; record type names are event names. PR01. |
| Commit group | A completely ordered set of records persisted atomically. No partial group is a supported material boundary. PR03/SE05. |
| Available Rana | Rana usable for a new reservation, derived from ledger entries. RT01. |
| Reserved Rana / hold | Rana bound to the leading bid until replacement or terminal settlement. RT02. |
| Release | Recorded return from reserved to available Rana; it never erases an earlier reservation. RT02/RT03. |
| Capture | Recorded movement from the winner's reservation to the protocol-held balance. RT03. |
| Protocol-held balance | Derived Rana balance receiving successful captures; not a bidder, not Unowned, and no spending operation exists. RT01/RT03. |
| Leading bid | Current highest valid bid before or after resolution, derived by SE03; leadership alone is not ownership. |
| Resolved winner | Winning bid/identity irreversibly fixed by ResolutionRecord, even if ownership later becomes Unowned. SE03. |
| Settlement | A separate phase consuming a resolved result; `settled` and `expired` are terminal outcomes of an explicit local command. RT03. |
| `expired` | Retained local demo outcome label. It does not assert that a deadline elapsed or an outside payment failed. RT03. |
| Protocol ownership | Irreversible final status for one number, recorded by FinalizationRecord. RT04. |
| Unowned | A final outcome with no owner, defined only by RT04; distinct from ownership pending. |
| Rhythm gap | Twelve seconds after finalization before the next number is eligible for creation. SE04. |
| Reconstruction | Validation and folding of the committed journal into derived state; it performs no new economic action. PR04. |
| Authority / one-shot permission | Permission for a specified irreversible record effect; bounded by its preconditions and unique durable commit. RT05. No external inscription authority exists here. |
| Server time | The backend timestamp sampled at the serialized authoritative evaluation boundary; it is never supplied by the browser. SE04. |
| Halt | Refusal to continue authoritative processing because required certainty or validity is absent. It is an execution condition, not a new auction state or ownership. PR06. |


## Current vocabulary and revision 0.1 compatibility — C61

Ownership is the protocol's recorded binding of a number to its successful bidder. It expresses the project's “What if you can own a number?” question; it does not assert external enforceability or ownership of mathematics outside this protocol.

Unowned is the final result of the explicitly selected failed-settlement branch: no owner is assigned, the winning reservation is released and finality remains irreversible. It is not collective ownership and is distinct from ownership pending before settlement.

The exact revision 0.1 machine spellings remain `title` (ownership projection), `title_kind` (outcome kind), `holder_id` (owner identifier), `PublicLand` (Unowned outcome) and `expired_to_publicland` (that finalization reason). They are compatibility vocabulary only. Canonical records, hashes and raw API snippets preserve them, while human-facing labels use ownership and Unowned. Historical working records retain the vocabulary used when each decision was made.
