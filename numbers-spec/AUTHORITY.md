# Candidate authority

## AU01 — Active authority and revisions

[STATUS.md](STATUS.md) records the user's approval of the settlement simplification and implementation following the explicit combined activation request. `numbers-spec/` revision 0.1 is the sole active specification. `codex-spec/` is superseded historical source material and remains untouched.

Future activation must identify an approved revision and replace the previous active set in one explicit decision. Review, audit, a commit, or file creation alone never implies activation. Two simultaneously active sets are forbidden.

## AU02 — Normative files and conflict order

The active precedence is:

1. `STATUS.md` — activation status only.
2. `AUTHORITY.md` — authority and interpretation.
3. `SCOPE.md` — included and excluded prototype capability.
4. `INVARIANTS.md` — system consistency properties.
5. `RANA-AND-OWNERSHIP.md` — economic and ownership meaning.
6. `STATE-AND-EVENTS.md` — lifecycle, commands, events, and commit groups.
7. `PERSISTENCE-AND-RESTART.md` — durable representation and reconstruction.
8. `PROTOTYPE-01.md` — identities and parameter definitions.
9. `UI-AND-DEMO.md` — interface projections and demonstration contract.
10. `GLOSSARY.md` — vocabulary.
11. `SECURITY-NOW-AND-LATER.md` — only its explicitly normative Prototype 1 section.

All these files are normative documents except sections explicitly labeled non-normative. `work/` is entirely non-normative. Recommendations, examples of rejected alternatives, future investigations, and provenance records cannot authorize behavior.

## AU03 — Interpretation and unresolved conflicts

`must` and `must not` express obligations and prohibitions. `only`, `exactly one`, and `at most one` bound permitted behavior. Conditions, allowed commands, and event types form closed sets. Unlisted transitions, corrections, retries, and recovery paths are forbidden.

Higher precedence controls a direct conflict. Silence does not supply a default or permission. If applying the rules still requires an economic, ownership, fairness, authority, timing, history, or transition choice, implementation must stop and the missing rule must be returned to user review. An implementer must not select between conflicting interpretations.

## AU04 — Rule ownership and refinement

Rule IDs identify owning clauses. Cross-references and invariant summaries do not supply competing definitions. An editorial correction must preserve the rule's observable meaning. A semantic revision must be explicitly recorded in `work/SPEC-CHANGES.md`, traced to its decision source, and audited before use. Existing records must never be reinterpreted by a later specification or configuration revision. This candidate imports no behavior by reference from `codex-spec/`; source citations in `work/` explain provenance only.
