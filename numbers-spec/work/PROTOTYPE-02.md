# Prototype 2 planning brief

Status: draft, non-normative, updated 2026-10-18. Prototype 2 has no approved scope or implementation authorization. [STATUS.md](../STATUS.md) identifies the sole active behavioral revision for Prototype 1.

## Starting point

Prototype 1 already demonstrates the complete auction with two simulated identities, exact accounting, append-only records, explicit simulated settlement and restart reconstruction. Browser, live terminal and guided demonstrations use one Rust engine. The merged revision 0.2 baseline is preserved under `prototype-01-revision-0.2-preserved-2026-10-18` at d567a3f55e951114ee4c5c125cdd9366dd70de3b; local journals require the separate backup described in [HANDOFF.md](../../prototype-01/HANDOFF.md).

The user approves preparing the next stage, preserving earlier work, and tracking proportionate technical debt (C64, C70). Readiness assessment is bounded to one DigitalOcean Droplet and invited testers only. This is a planning constraint, not approval of a second prototype's behavior, infrastructure provisioning, or deployment.

For readiness work, start with STATUS.md, AUTHORITY.md and the relevant current clauses, then inspect the current prototype, tests and build configuration. Identify which rules carry forward and which need an explicit decision. Consult historical material only to answer a specific provenance question; it cannot fill a missing current rule. Return a bounded specification-change proposal and acceptance checks before implementation.

## Choose one learning goal

The next goal is undecided. A useful candidate is: can two colleagues understand and participate in one auction from separate devices, using simulated funds? Another is to refine timing, fairness and entertainment in the existing single-device demonstration. Colleague feedback may identify a more useful goal.

Choose one primary question before expanding scope. Write the expected observation and the evidence that would change the design. Avoid combining multi-device access, new payment systems and new auction economics into one experiment.

## Collect observations in one place

Follow the [colleague feedback session](../../prototype-01/DEMO.md#collect-colleague-feedback) and record observations here. Use a session label rather than personal details. Keep what happened separate from the proposed change. No colleague results are recorded yet.

| Date and session | Task attempted | Observation or exact words | Help given | Possible follow-up |
|---|---|---|---|---|
| — | — | — | — | — |

A problem with an existing approved rule belongs in a small Prototype 1 fix. A proposed change to the rule belongs in this planning brief until approved.

## Decisions needed if separate devices are selected

These are questions, not new rules or defaults:

- Which participants may connect, how are simulated identities assigned, and who may choose settlement outcomes?
- Within the one-Droplet, invited-tester planning boundary, what access restriction, participant permissions, and operational responsibilities are required? Specify them before changing the current loopback-only service. No deployment follows from this brief.
- Which server-observed ordering and cutoff rules apply to competing requests, late bids and extensions? Decide whether the existing extension cap still serves the intended experience.
- What should a participant see after losing a connection or a command response? Preserve the existing no-automatic-replay rule unless a replacement is explicitly specified.
- Which Prototype 1 behaviors carry forward unchanged, and which clauses change? Define the exact demonstration and acceptance evidence for each change.
- How are a new session's configuration and history identified? Keep old histories bound to their original rules; no silent reinterpretation or migration.

Real money, Bitcoin integration, production identity and public hosting each need separate explicit decisions. They are not prerequisites for choosing the next learning goal.

## Work in small, reviewable stages

1. Preserve the reviewed Prototype 1 source tag and a separate, verified backup of its local histories. Keep its demonstrations usable.
2. Gather a few colleague observations here and choose one Prototype 2 learning goal.
3. Draft only the changed or newly required clauses, their acceptance examples and the deployment boundary. Use the existing [source decisions](SOURCE-DECISIONS.md) and [change log](SPEC-CHANGES.md) when those decisions are approved; do not create another authority ledger.
4. Have the project owner approve the scope and explicitly activate the next specification revision before implementation. Keep one active behavioral authority; preserved source refs retain each prior revision with its original implementation.
5. Evolve the existing Rust engine on a focused branch. Start with a separate demonstration journal. Create a separate checkout of the preserved tag only when both generations need to run side by side; avoid copying the engine into two maintained implementations.
6. Run the relevant rule, reconstruction and browser checks, demonstrate the agreed learning goal, then record evidence and publish the next reviewed checkpoint.

No Prototype 2 application directory, service or agent team is needed while this brief is undecided. One builder and a focused review appropriate to the change are sufficient for now.

## Technical debt

Use the current table in [TECHNICAL-DEBT.md](TECHNICAL-DEBT.md). Each item states its limitation and the condition that makes it worth addressing. Revisit only the items touched by the approved next scope. In particular, multi-device participation makes identity and access decisions relevant; it does not by itself require multiple backend writers or distributed coordination.

The immediate audit finding—callers being able to commit raw records or mutate the live engine cache—is fixed in Prototype 1. Broader conversion of JSON and string states into Rust types is deferred until a bounded extension benefits from it. Preserve canonical record compatibility while doing that work.
