# Historical Numbers material

The current application is [Prototype 1](../prototype-01/README.md), governed only by [numbers-spec revision 0.1](../numbers-spec/STATUS.md). Proposed work belongs in the [Prototype 2 planning brief](../numbers-spec/work/PROTOTYPE-02.md). Historical references to “normative”, “active” or “current” apply to their own period.

Most earlier files remain at their original paths so links, source citations and existing checks keep working. Their location does not grant current authority. The original contents remain available in Git history, including the pre-cleanup checkpoint `4a45179`.

| Historical location | What is preserved |
|---|---|
| [codex-spec/](../codex-spec/) | The superseded specification. Its original rule files are unchanged. |
| [docs/](../docs/) | Earlier product, implementation and planning documents. Entry notices identify their historical status. |
| Root [src/](../src/), [tests/](../tests/), [Cargo.toml](../Cargo.toml), [Cargo.lock](../Cargo.lock) | The earlier Rust application; separate from Prototype 1. |
| [frontend/](../frontend/) | The earlier React application. Prototype 1's browser code is in `prototype-01/static/`. |
| [results/](../results/), [Experiments/](../Experiments/), [numbers-typing/](../numbers-typing/) | Earlier output and visual/typing experiments. |
| [legacy-book/root-book.toml](legacy-book/root-book.toml) | Exact former root `book.toml`. |
| [legacy-book/nested-book.toml](legacy-book/nested-book.toml) | Exact former `book/book.toml`. |
| [book/](../book/) | Historical themes and any locally generated book output. |

The two book configurations are source snapshots, not supported build entry points from their new locations: relative paths still describe their original locations. They were archived byte-for-byte to remove the misleading default book target. No documentation generator is required for the current Markdown guides. Use Git history in a separate checkout if historical build reconstruction becomes necessary.

The historical application's existing GitHub jobs remain separate from Prototype 1 checks. Moving all old code or reviving its build/dependency maintenance is a separate task; neither is needed to prepare Prototype 2.

Local journals under `prototype-01/data/` are operational history, not this source archive. They are excluded from Git and backed up separately. See [HANDOFF.md](../prototype-01/HANDOFF.md).
