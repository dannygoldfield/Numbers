# Numbers Prototype 2

Revision 0.3 extends the same Rust auction engine to two invited bidders and one facilitator. Each bidder has a private credential and fixed simulated account. All three inspect the same journal and balances; only the facilitator can select simulated settlement. [STATUS](../numbers-spec/STATUS.md) and [AUTHORITY](../numbers-spec/AUTHORITY.md) define current behavior.

The `prototype-01/` directory and Rust package name are retained to avoid copying or renaming the engine. Prototype 1 remains usable through its preserved revision 0.2 build and journals; see [HANDOFF](HANDOFF.md). Revision 0.3 never continues those histories.

## Verify locally

```sh
npm ci --ignore-scripts
./check
./cargo-local build --locked --release --bins
./demo guided --auto
```

The guided executable uses the actual engine with explicit simulated time and a fresh journal. Its 42-record fixture proves both settlement outcomes and reconstruction in separate processes. The 31-record fixture, timing, Rana accounting and ownership rules remain unchanged. Tests use disposable journals; never point them at saved sessions.

[SETUP](SETUP.md) describes dependencies and the separate real HTTPS gateway rehearsal. [REVIEW](REVIEW.md) maps rules to implementation and checks. [VERIFICATION](VERIFICATION.md) records results and limits.

## Prepare an invited session

The live interface requires an authenticated HTTPS gateway. The application remains on loopback and rejects requests without a gateway principal assertion. Merely opening its port in a browser does not create authenticated access.

Use the [reviewable operating examples](ops/README.md) to prepare a configuration, dedicated history location, fixed credential assignment and session manifest. First launch is an explicit `--initialize`; subsequent starts omit it. A missing intended journal, changed configuration or changed build stops startup instead of creating a replacement. Keep the matching binary and configuration with each history.

`demo-config.json` contains explicit 600-Rana demo inputs plus an example origin, label and independent `data/revision-0.3/trial-001.sqlite3` path. The example origin is not a deployment. No credentials, keys or real hostname belong in Git. The convenience `./run --initialize` / `./run` use that exact configuration; review it and the gateway before launch.

Auction is `/`; Record is `/record`. Both require a credential. The Record view contains no mutation controls. The bidder sees its assigned account without a selector; the facilitator sees settlement controls and no bid form. All use the device system font.

## Unchanged mechanics

The first valid bid starts a 225-second auction. A valid bid in the final configured 10-second window extends by 15 seconds, at most twice. Only the leader has a reservation. Resolution fixes the winner; successful settlement captures the full hold and assigns original ownership, while failed settlement releases it and finalizes Unowned. After a 12-second gap the next number becomes available, dormant until its first valid bid. Settlement has no deadline.

Network and browser time do not grant priority or change these rules. A lost response may follow a committed command. The browser never resubmits it; inspect the records and explicitly acknowledge before submitting a new command. If the outcome remains unclear, involve the facilitator and use the stop/inspect procedure.

No Bitcoin, payments, wallets, transfers, top-ups, reset, history edits, compensation or automatic recovery exist. Original auction facts remain immutable; a separately approved future transfer could append later facts. Host administrators remain trusted. Deployment and repository pruning require separate authorization.
