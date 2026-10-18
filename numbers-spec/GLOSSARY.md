# Vocabulary

## GL01 — Fixed terms

| Term | Meaning / owning rule |
|---|---|
| Rana; `rana`; `RANA` | In-house value standard. One rana is indivisible; `rana` is singular and plural, `RANA` is the machine code. Economic rules: RT01. |
| Simulated identity | Either `demo-1` or `demo-2`; selecting one proves no human identity. P01. |
| Number | The abstract mathematical value identified, within this prototype, by its canonical positive integer. Font, typography, rendering, and display location do not determine its identity. |
| Auction | The SE01 lifecycle for a number within one demonstration history. It exists in Scheduled before its first valid bid. |
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
| Protocol ownership | Original auction ownership outcome recorded by FinalizationRecord. Its historical assignment is immutable; Prototype 1 has no transfer. RT04/RT06. |
| Unowned | A final outcome with no owner, defined only by RT04; distinct from ownership pending. |
| Rhythm gap | Twelve seconds after finalization before the next number is eligible for creation. SE04. |
| Reconstruction | Validation and folding of the committed journal into derived state; it performs no new economic action. PR04. |
| Authority / one-shot permission | Permission for a specified irreversible record effect; bounded by its preconditions and unique durable commit. RT05. No external inscription authority exists here. |
| Server time | The backend timestamp sampled at the serialized authoritative evaluation boundary; it is never supplied by the browser. SE04. |
| Halt | Refusal to continue authoritative processing because required certainty or validity is absent. It is an execution condition, not a new auction state or ownership. PR06. |


## GL02 — Number and recorded ownership

Ownership is the protocol's recorded binding of a number to its successful bidder. The original auction assignment is irreversible; this does not prohibit a separately specified future transfer under RT06. Ownership does not alter the mathematical concept or assert ownership of mathematics outside the protocol.

Before finalization, ownership is pending. Successful settlement records the fixed winning bidder as owner. Explicitly selected failed settlement records Unowned: no owner exists.

Current representations are defined by RT04, RT06, SE06, and UI02. Historical records retain their original meaning and representation under PR07.
