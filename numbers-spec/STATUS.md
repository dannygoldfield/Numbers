# Numbers Prototype 1 status

```text
project: Numbers
generation: Rana
version: 0.2
status: approved-for-implementation
authority: active
```

## Activation — 2026-10-18

The user explicitly approved revision 0.2 and its implementation scope, with the clarification that irreversible auction finalization does not prohibit a separately specified future transfer mechanism (C66–C67; S31–S32). Revision 0.2 of `numbers-spec/` is the sole active behavioral specification for the revised Prototype 1. It retains the auction lifecycle, Rana accounting, settlement outcomes, timing, and immutable original ownership assignment while replacing ownership vocabulary and the canonical history format.

Earlier histories remain governed by their original specification and preserved implementation. Revision 0.2 starts a separate demonstration history under PR07. No two specification sets are active. `codex-spec/` remains historical, not execution authority.

C67 authorized preservation, local implementation, and verification. C68 additionally authorizes review, commit, and push of this revision after checks. Repository pruning and deployment remain separate. Neither approval activates Prototype 2 or implements transfers or wallets.

[AUTHORITY.md](AUTHORITY.md) owns precedence. [SOURCE-DECISIONS.md](work/SOURCE-DECISIONS.md) and [SPEC-CHANGES.md](work/SPEC-CHANGES.md) preserve approval provenance. [VERIFICATION.md](../prototype-01/VERIFICATION.md) records implementation evidence; approval is not a test result. Historical candidate audits and traceability notes under `work/` describe earlier revisions and supply no current behavior.

## Historical activation — non-normative

Revision 0.1 was activated on 2026-09-05 under C48/S25. C59, C62, C64, and C65 record earlier checkpoint/publication permissions; those permissions do not authorize publication of this revision. Preserve the prior source and journals under their original meanings. The existing `prototype-01-baseline-2026-10-10` tag remains fixed; `prototype-01-revision-0.1-preserved-2026-10-18` identifies the exact pre-revision source commit `2cc86353185c6b96947f87810105fe04f8b8ce0c`.
