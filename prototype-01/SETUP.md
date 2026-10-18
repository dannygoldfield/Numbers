# Development environment

## Dependencies

Rust 1.98.1 and its formatter/linter are pinned in `rust-toolchain.toml`. SQLite is bundled by the locked Rust dependency. Node 22+ and locked Prettier 3.9.6 provide development checks; the served browser has no third-party runtime assets. Python 3.9+, OpenSSL and an NGINX build with HTTP SSL/Basic authentication are needed only for gateway acceptance. The core tests also use system curl with its ambient configuration disabled.

```sh
npm ci --ignore-scripts
./cargo-local fetch --locked
./check
./cargo-local build --locked --release --bins
```

`./check` runs Rust formatting, Clippy with warnings denied, protocol/HTTP/process tests, compile-fail engine-boundary tests, frontend formatting and browser helper tests. The helper tests exercise shipped code with controlled DOM/transport; they are not a real two-device trial. Release overflow checks remain enabled.

## HTTPS gateway acceptance

Run the actual gateway template against disposable generated credentials and a test certificate trusted only by the test process:

```sh
./cargo-local build --locked --bins
NUMBERS_TEST_NGINX=/absolute/path/to/nginx python3 tests/gateway.py --directory /absolute/path/to/new-rehearsal
```

The directory must not exist. No service is installed, no system certificate trust is changed, and no cloud resource is used. Servers bind loopback, stop after the run, and disposable secrets/keys are removed. The script retains logs, a test journal, a cold copy and `result.json`. Keep the directory outside the repository. Tests verify authentication/revocation, role and origin enforcement, buffering/limits, exact body bytes, simultaneous bids, lost responses, no upstream retry, credential stripping and cold-copy reconstruction.

The Ubuntu CI job prepares a disposable gateway binary and runs the same checks. A local macOS result does not claim a completed Linux CI run or a verified deployment.

## Configuration and launch

The complete behavioral input surface remains P02. In addition to `database_path`, loopback `host`, `port` and `log_verbosity`, supply `allowed_origin` and `session_label`. The origin is an exact HTTPS origin, using a lowercase hostname and no path/query/fragment; use the browser's normalized origin, omitting a default `:443`. The label is 1–64 ASCII letters/digits/hyphens/underscores. There is no implicit value for either.

The executable accepts:

```sh
./target/release/numbers-prototype-01 --config /absolute/session/config.json --initialize
# After a controlled stop, restart the SAME session with its matching binary:
./target/release/numbers-prototype-01 --config /absolute/session/config.json
```

Initialization refuses an existing database or `config.session.json`. The noncanonical receipt binds the exact configuration bytes, executable hash, absolute database path, session label, revision and fixed principal mapping. It contains no password or real name. Restart requires the receipt, matching association and existing nonempty journal, then validates all records before evaluating time. A failure leaves evidence intact; do not delete the receipt or journal to bypass it. Prepare a separately labeled history for a different build/configuration instead.

[ops/README.md](ops/README.md) owns the gated operating procedure. `./demo guided` and `./demo inspect SESSION_DIRECTORY` retain verification/inspection; `live` and `serve` refuse in the current build. The preserved 0.2 executable retains those old workflows. No time-control endpoint exists.

## Editor

The Numbers workspaces keep the existing directory and point rust-analyzer at `prototype-01/Cargo.toml`. Presenter tasks run guided verification, checks or an already prepared configured session. Editor launch does not install credentials, create a gateway or authorize deployment.
