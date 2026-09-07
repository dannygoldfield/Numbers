# Numbers

What if you can own a number?

Numbers explores that question through sequential auctions and an explicit record of ownership. The current implementation is [Prototype 1](prototype-01/README.md): a local Rust application with two simulated bidders, exact Rana accounting, an append-only SQLite journal and restart reconstruction.

## Start here

- [Active specification and status](numbers-spec/STATUS.md)
- [Run Prototype 1](prototype-01/README.md)
- [Present the browser and backend](prototype-01/DEMO.md)
- [Read the Rust implementation](prototype-01/REVIEW.md)
- [Development environment](prototype-01/SETUP.md)
- [Continuation notes and saved-history backup](prototype-01/HANDOFF.md)

Open `prototype-01/Numbers-demo.code-workspace` in VS Code to see the active specification beside the implementation and run its demonstration tasks.

## Scope and authority

`numbers-spec/` revision 0.1 is the sole active behavioral authority, with precedence defined in [AUTHORITY.md](numbers-spec/AUTHORITY.md). Specification changes are explicit and traced to user decisions. Successful settlement establishes the winner's ownership; failed settlement leaves the number Unowned, with no owner. Both are final protocol outcomes.

Prototype 1 uses simulated identities and Rana. It does not currently integrate Bitcoin, payments or public hosting. The root Rust application, `frontend/`, `codex-spec/`, and older `docs/` describe earlier work and remain separate historical material. Their claims about Bitcoin execution and authority do not describe this prototype.

## Authorship

Numbers is authored and operated by Danny Goldfield, under the identity **123456789 and 0**.

## License

MIT
