# Numbers Prototype 1 status

```text
project: Numbers
generation: Rana
version: 0.1
status: approved-for-implementation
authority: active
```

## Activation — 2026-09-05

The user approved the settlement simplification and implementation of Prototype 1 in response to the combined simplify/activate/build request. Revision 0.1 of `numbers-spec/` is now the sole active specification. `codex-spec/` is superseded historical source material; its files remain untouched. No two specification sets are active.

Revision 0.1 is the completed inactive candidate plus S24: removal of the unused settlement deadline from configuration, records, projections and examples. All other auction, Rana, title and reconstruction rules are retained. `expired` remains a machine label for explicit simulated failure; time does not select settlement. S25 records activation and implementation authorization.

Q01–Q04 remain resolved: indivisible rana; leader-only reservation; exact winning-bid capture into protocol-held Rana with no spending; failed settlement releases the winning hold and assigns PublicLand.

[AUTHORITY.md](AUTHORITY.md) owns rule precedence. [work/TRACEABILITY.md](work/TRACEABILITY.md) maps rules. [work/AUDIT.md](work/AUDIT.md) preserves the historical candidate audit; later implementation evidence is separate. Approval and document checks are not runtime test results.

Implementation is authorized locally. On 2026-09-06 the user also authorized the existing-repository checkpoint commit and push described in C59 of `work/SOURCE-DECISIONS.md`. Public hosting, real payments and Bitcoin integration remain outside this authorization. `work/` remains non-normative.
