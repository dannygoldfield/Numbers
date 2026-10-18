# Numbers

What if you can own a number?

Numbers explores sequential auctions and an explicit record of ownership. The current implementation is **Prototype 2, specification revision 0.3**: one Rust application behind an authenticated HTTPS gateway, two invited simulated bidders and one facilitator, exact Rana accounting, an append-only journal, and restart reconstruction. A number is the abstract mathematical value; its device-native appearance does not define its identity.

## Start here

- [Active status](numbers-spec/STATUS.md) and [authority order](numbers-spec/AUTHORITY.md) — the behavioral specification.
- [Run and verify the current prototype](prototype-01/README.md) — Auction at `/` and Record at `/record`.
- [Demonstrate and collect feedback](prototype-01/DEMO.md).
- [Read the implementation](prototype-01/REVIEW.md) or [reproduce the environment](prototype-01/SETUP.md).
- [Current handoff and preserved histories](prototype-01/HANDOFF.md), with [verification evidence](prototype-01/VERIFICATION.md).
- [Prototype 2 planning](numbers-spec/work/PROTOTYPE-02.md) — non-normative experiment brief; implementation is approved, deployment remains separate.

Open [Numbers-demo.code-workspace](prototype-01/Numbers-demo.code-workspace) for the active specification, implementation, and demonstration tasks. [Numbers.code-workspace](Numbers.code-workspace) opens the same two folders without presenter tasks.

## Project folder guide

| Path | Purpose |
|---|---|
| [numbers-spec/](numbers-spec/) | The eleven documents identified by AUTHORITY.md; only their declared normative sections control behavior. |
| [numbers-spec/work/](numbers-spec/work/) | Decision provenance, explicit changes, current debt, and the next-prototype brief. None supplies executable authority. |
| [prototype-01/src/](prototype-01/src/) | One Rust engine, protected HTTP service, guided verification and read-only inspection. Start with [model.rs](prototype-01/src/model.rs). |
| [prototype-01/static/](prototype-01/static/) | Auction and read-only Record browser views served by the same application. |
| [prototype-01/tests/](prototype-01/tests/) | Protocol, executable-demo, and browser checks. Revision 0.1 and 0.2 fixtures test rejection of old histories. |
| [prototype-01/run](prototype-01/run), [demo](prototype-01/demo), [check](prototype-01/check) | Launch, demonstrate, and verify from the prototype directory. Its manifests and lockfiles own all current dependencies. |
| [prototype-01/demo-config.json](prototype-01/demo-config.json) | Explicit demo inputs. Changing configuration cannot reset or reinterpret a recorded history. |
| `prototype-01/data/` | Local journals and session evidence, excluded from Git and backed up separately. The independent revised example is `data/revision-0.3/trial-001.sqlite3`. |
| `prototype-01/target/`, `prototype-01/node_modules/` | Ignored generated builds and installed development tools. |
| [.github/workflows/ci.yml](.github/workflows/ci.yml) | Current prototype and local HTTPS checks; no deployment job. |
| [archive/README.md](archive/README.md) | Retrieval index for preserved earlier applications, specifications, experiments, and evidence. |

## Scope and future prototypes

Successful settlement records the winner as owner; explicitly failed settlement leaves the number Unowned and releases the winning reservation. The original auction outcome is immutable. A future separately approved transfer mechanism may append later facts without rewriting that outcome; Prototype 2 has no wallets or transfers.

Prototype 2 uses simulated identities and Rana, with fixed credentials and facilitator-only settlement. Auction timing, accounting and both settlement outcomes carry forward unchanged. Deployment is restricted to a separately approved plan for one DigitalOcean Droplet and invited testers. The existing `prototype-01/` directory is retained; there is one engine and no duplicate implementation. See [operating examples](prototype-01/ops/README.md) and [verification](prototype-01/VERIFICATION.md).

The [preserved revision 0.2 source](https://github.com/dannygoldfield/Numbers/tree/prototype-01-revision-0.2-preserved-2026-10-18) includes everything present before cleanup. Historical documents retain their original meaning and cannot supply missing current rules. See the archive index when a specific historical question requires them.

## Authorship and license

Numbers is authored and operated by Danny Goldfield, under the identity **123456789 and 0**. [MIT license](LICENSE).
