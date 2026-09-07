# Development environment

The environment is installed for continuing Numbers development, with reproducible versions and checks. No toolchain installation or package download occurs during an auction.

## Installed baseline

| Tool | Baseline | Purpose |
|---|---|---|
| Rust / Cargo | 1.98.1, pinned in rust-toolchain.toml | Compile the authoritative backend and run tests |
| rustfmt | Toolchain component | Consistent Rust formatting |
| Clippy | Toolchain component | Rust correctness and maintainability checks |
| rust-analyzer | Toolchain component | Editor navigation, diagnostics and refactoring |
| Apple Command Line Tools | Existing local installation | Native linker and compilation of bundled SQLite |
| SQLite | Bundled through the locked rusqlite dependency | Durable journal; no separate database service |
| Node.js | Existing 22.23.2 installation; Node 22+ required for UI development checks | Run the formatter and exact-value browser tests |
| curl | Existing macOS command-line tool | HTTP transport for the optional live terminal client; ambient curl configuration is disabled, with no automatic retries |
| VS Code | Existing local installation with rust-analyzer | Optional presentation workspace and source navigation |
| Prettier | 3.9.6, exact development dependency | Format HTML, CSS and JavaScript |

Rust is installed in its standard user locations, `~/.cargo/` and `~/.rustup/`. `rust-toolchain.toml` chooses the project version and required components. `Cargo.lock` and `package-lock.json` pin dependency resolution. The `cargo-local` wrapper also finds the standard Rust installation when the current shell has not reloaded its PATH.

## Reproduce setup

On macOS, install Apple's Command Line Tools if absent, and install Rust using the [official Rust instructions](https://rust-lang.org/tools/install/). Install Node.js 22 or later through the existing Node version manager. Then, in this directory:

```sh
rustup show
npm ci --ignore-scripts
./cargo-local fetch --locked
./check
```

The pinned toolchain automatically requests the matching formatter, linter and language server. `npm ci` installs only development tooling; the served browser interface has no runtime JavaScript dependencies and uses no third-party network assets.

For an editor, enable its rust-analyzer integration and point it at this directory's `Cargo.toml`. The parent repository contains an older, independent Cargo project. Run commands here or pass this manifest explicitly.

## Daily workflow

```sh
./cargo-local fmt --all
npm run format
./check
./run
```

`./check` stops on the first failure and runs Rust formatting, Clippy with warnings treated as errors, Rust tests, frontend formatting and browser-value tests. Run it before presenting a change as ready. Use `./cargo-local test --locked <test_name>` while diagnosing a specific failure.

For a local optimized build:

```sh
./cargo-local build --locked --release
./target/release/numbers-prototype-01 --config ./demo-config.json
```

Release builds retain overflow checks. Economic amounts use arbitrary-precision integers; timestamp or storage-sequence overflow stops processing explicitly.

## Configuration and operation

All behavioral inputs are in `demo-config.json`. Only the starting number and optional maximum bid have specification defaults. The operational fields are `database_path`, a loopback IP `host`, integer `port`, and `log_verbosity` (`quiet` or `requests`). Relative database paths resolve beside the configuration file.

To run an explicitly separate demonstration history, prepare a separate configuration with its own unused database path and local port. That is a separate history, not a reset or recovery of the existing one. Do not change or delete a recorded history to make a test pass.

Tests use temporary directories, explicit test clocks and child processes. The UI05 guided executable also supplies explicit simulated timestamps in a separate journal. No clock or crash control is available through the live HTTP application or browser.

## Continuing infrastructure work

This baseline supplies compilation, storage, formatting, linting, editor support and repeatable verification for the approved local application. Extend it when an actual development requirement appears. Payment services, identity infrastructure, external attestations and public hosting require their own specification and deployment decisions; they are not silently enabled by environment setup.


## Presentation workspace

Open [Numbers-demo.code-workspace](Numbers-demo.code-workspace) for workspace-scoped editor settings and the four presentation/check tasks. [DEMO.md](DEMO.md) explains their use. `./demo` builds both binaries with locked dependencies; `default-run` keeps `./run` targeting the existing server. The live client uses system curl solely for HTTP transport and disables its user configuration. New demo sessions live under ignored `data/demonstrations/` and are never reset by the launcher.
