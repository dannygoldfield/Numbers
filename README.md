# Numbers

What if you can own a number?

Numbers explores that question through sequential auctions and an explicit record of ownership. The current implementation is [Prototype 1](prototype-01/README.md): a local Rust application with two simulated bidders, exact Rana accounting, an append-only SQLite journal and restart reconstruction.

## Start here

- [Active specification and status](numbers-spec/STATUS.md)
- [Run Prototype 1](prototype-01/README.md)
- [Present the browser and backend](prototype-01/DEMO.md)
- [Collect colleague feedback](prototype-01/DEMO.md#collect-colleague-feedback)
- [Read the Rust implementation](prototype-01/REVIEW.md)
- [Development environment](prototype-01/SETUP.md)
- [Continuation notes and saved-history backup](prototype-01/HANDOFF.md)
- [Prototype 2 planning and workflow](numbers-spec/work/PROTOTYPE-02.md) — draft, not active rules
- [Historical source archive](archive/README.md)

Open `prototype-01/Numbers-demo.code-workspace` in VS Code to see the active specification beside the implementation and run its demonstration tasks.

## Project folder guide

On Danny's Mac, the working repository is `/Users/dannygoldfield/Projects/Numbers-Repos/Numbers`. The active project has two main folders: `numbers-spec/` states the rules; `prototype-01/` implements and demonstrates them. This guide explains locations, not additional protocol requirements.

For a presentation, open [Numbers-demo.code-workspace](prototype-01/Numbers-demo.code-workspace) in VS Code. Its sidebar calls these folders **Specification** and **Prototype 1**, respectively. The root `Numbers.code-workspace` also opens these two active folders; the presentation workspace adds demo tasks. Open the repository folder itself to inspect older material. These workspaces are views of the same files, not separate copies.

### Rules and decisions

| Location | What it contains |
|---|---|
| [numbers-spec/STATUS.md](numbers-spec/STATUS.md), [AUTHORITY.md](numbers-spec/AUTHORITY.md), [SCOPE.md](numbers-spec/SCOPE.md) | Which specification is active, which documents control, and what Prototype 1 includes. |
| [RANA-AND-OWNERSHIP.md](numbers-spec/RANA-AND-OWNERSHIP.md), [STATE-AND-EVENTS.md](numbers-spec/STATE-AND-EVENTS.md), [INVARIANTS.md](numbers-spec/INVARIANTS.md) | Balances, bids, settlement, ownership, event order and properties that must always hold. |
| [PERSISTENCE-AND-RESTART.md](numbers-spec/PERSISTENCE-AND-RESTART.md) | How records are saved and validated when the application restarts. |
| [PROTOTYPE-01.md](numbers-spec/PROTOTYPE-01.md), [UI-AND-DEMO.md](numbers-spec/UI-AND-DEMO.md), [GLOSSARY.md](numbers-spec/GLOSSARY.md) | Demo parameters, Auction and Record view requirements, worked examples and vocabulary. |
| [SECURITY-NOW-AND-LATER.md](numbers-spec/SECURITY-NOW-AND-LATER.md) | Current protection requirements and separately labeled future investigations. |
| [numbers-spec/work/](numbers-spec/work/) | Decision sources, specification changes, audit, traceability, questions and deferred work. These are supporting notes, not executable authority. |

### Working application

All paths in this table are inside `prototype-01/`.

| Location | What it does |
|---|---|
| [src/model.rs](prototype-01/src/model.rs) | The central Rust auction rules: bid admission, reservations, settlement, ownership and reconstruction. Start here to show the mechanics. |
| [src/store.rs](prototype-01/src/store.rs) | Saves complete groups of records atomically in SQLite and exposes committed state. |
| [src/http.rs](prototype-01/src/http.rs), [src/main.rs](prototype-01/src/main.rs) | Connect the browser and terminal to the engine; start and run the local application. |
| [src/values.rs](prototype-01/src/values.rs), [src/lib.rs](prototype-01/src/lib.rs) | Exact values and canonical representation; shared engine modules and error types. |
| [static/index.html](prototype-01/static/index.html), [static/style.css](prototype-01/static/style.css), [static/app.js](prototype-01/static/app.js) | Both browser views: page structure, appearance, command submission and display. The backend decides outcomes. |
| [src/bin/numbers-demo/](prototype-01/src/bin/numbers-demo/) | The guided walkthrough, live terminal client, readable summaries and separate demo-session management. These use the same engine. |
| [tests/](prototype-01/tests/) | Rust protocol and demonstration checks, plus browser exact-value and command-display checks. |
| [demo-config.json](prototype-01/demo-config.json) | The current local demo's explicit settings, including its journal path. Initial balances apply to a new history; editing this file does not reset an existing one. |
| [run](prototype-01/run), [demo](prototype-01/demo), [check](prototype-01/check) | Launch the browser demo, run terminal demonstrations, and run the existing verification suite. |
| [Cargo.toml](prototype-01/Cargo.toml), [Cargo.lock](prototype-01/Cargo.lock), [rust-toolchain.toml](prototype-01/rust-toolchain.toml), [cargo-local](prototype-01/cargo-local) | Rust application/dependency definitions, exact dependency versions, selected toolchain and local launcher. |
| [package.json](prototype-01/package.json), [package-lock.json](prototype-01/package-lock.json), [.prettierrc.json](prototype-01/.prettierrc.json) | Browser development checks, their dependency versions and formatting preferences. |
| `data/demo-02.sqlite3` | The current port-8765 demo's local auction journal. `data/prototype.sqlite3` is an earlier, separate history. |
| `data/demonstrations/` | Separate live/guided demonstration sessions, saved journals and generated reconstruction evidence. |
| `target/`, `node_modules/` | Generated Rust build files and installed development dependencies. They are reproducible working files and are excluded from Git. |

`data/` is also excluded from Git: publishing source to GitHub does not back up local auction histories. The separate dated backup location is documented in [HANDOFF.md](prototype-01/HANDOFF.md).

### Guides for colleagues and future work

| Document | When to open it |
|---|---|
| [prototype-01/README.md](prototype-01/README.md) | Starting the application and understanding the demonstrated auction. |
| [DEMO.md](prototype-01/DEMO.md) | Presenting the browser, terminal and guided walkthrough. |
| [REVIEW.md](prototype-01/REVIEW.md) | Following a specification rule through the Rust implementation. |
| [SETUP.md](prototype-01/SETUP.md) | Understanding or reproducing the development environment. |
| [VERIFICATION.md](prototype-01/VERIFICATION.md) | Reading what was actually checked, when, and with what limits. |
| [HANDOFF.md](prototype-01/HANDOFF.md) | Resuming work from the latest checkpoint; earlier dated sections are historical. |
| [.github/workflows/ci.yml](.github/workflows/ci.yml) | GitHub's automated checks. Prototype 1 has a dedicated job; older application checks are separate. |

### Earlier work and experiments

The root `src/`, `tests/`, `Cargo.toml` and `Cargo.lock` belong to the older Rust application. `frontend/` is its separate earlier browser application; the current demo's browser code is in `prototype-01/static/`. `codex-spec/` is the superseded specification, and `docs/` contains older planning and documentation. The two former book configurations are preserved under `archive/legacy-book/`; `book/` retains historical themes and any generated material. `results/` contains earlier application output. `Experiments/` and `numbers-typing/` contain visual and typing experiments. Retained zip archives are historical snapshots. These locations are outside the active Prototype 1 implementation and do not define its behavior.

### A short folder walkthrough

1. Open `numbers-spec/RANA-AND-OWNERSHIP.md`: “These are the written rules for bids, balances and ownership.”
2. Open `prototype-01/src/model.rs`, then find `plan_bid`: “This Rust code applies those rules.”
3. Show Auction beside Record: “One view submits a bid; the other shows the resulting records and balances.” Record retains the existing `/protocol` page address.
4. Use the presenter steps in `prototype-01/DEMO.md`: “The guided terminal demonstration uses the same engine and shows reconstruction in a separate process.”
5. Open `prototype-01/VERIFICATION.md`: “Here is the evidence for what we have checked.”

## From Prototype 1 to Prototype 2

Preserve the reviewed source as the `prototype-01-baseline-2026-10-10` Git tag and back up local journals separately. Keep Prototype 1 available for colleague demonstrations. Collect observations in the [Prototype 2 planning brief](numbers-spec/work/PROTOTYPE-02.md), choose one learning goal, then approve explicit specification changes before implementation. The [current debt register](numbers-spec/work/TECHNICAL-DEBT.md#current-register-as-of-2026-10-10) distinguishes accepted limits from work with a concrete revisit trigger.

Future implementation should evolve the existing engine on a focused branch, using separate test histories. Preserve the tag; use a separate checkout only when both generations must run side by side. No duplicate engine or Prototype 2 service is created during planning.

## Scope and authority

`numbers-spec/` revision 0.1 is the sole active behavioral authority, with precedence defined in [AUTHORITY.md](numbers-spec/AUTHORITY.md). Specification changes are explicit and traced to user decisions. Successful settlement establishes the winner's ownership; failed settlement leaves the number Unowned, with no owner. Both are final protocol outcomes.

Prototype 1 uses simulated identities and Rana. It does not currently integrate Bitcoin, payments or public hosting. The root Rust application, `frontend/`, `codex-spec/`, and older `docs/` describe earlier work and remain separate historical material. Their claims about Bitcoin execution and authority do not describe this prototype.

## Authorship

Numbers is authored and operated by Danny Goldfield, under the identity **123456789 and 0**.

## License

MIT
