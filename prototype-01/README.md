# Numbers · Prototype 1

A local Rust application for the approved Numbers 0.1 specification. Two simulated bidders compete for a number. The journal records bids, Rana movements, resolution, settlement and ownership. Restart reconstructs those records before new work begins.

The sole active behavioral authority is [numbers-spec](../numbers-spec/STATUS.md). The older root Rust project, frontend and `codex-spec/` remain historical and are not dependencies of this application.

For the saved project state and how to return after the pause, see [HANDOFF.md](HANDOFF.md).

## Run

From this directory:

```sh
./run
```

Open [Auction](http://127.0.0.1:8765/) and [Protocol](http://127.0.0.1:8765/protocol). The Protocol page can sit beside the Auction page and contains no outcome controls. Stop the server with Ctrl-C. Running the same command again reconstructs the same journal.

[SETUP.md](SETUP.md) documents the installed tools, repeatable setup and checks. [REVIEW.md](REVIEW.md) is the backend reading guide. [VERIFICATION.md](VERIFICATION.md) records actual validation evidence and its limits.

## Demonstrate in VS Code, the browser and terminal

Open [Numbers-demo.code-workspace](Numbers-demo.code-workspace) in VS Code, then choose **Terminal → Run Task → Numbers: Guided mechanics**. The walkthrough waits for Enter and shows rules, commands, committed records and balances. It uses simulated time with the actual Rust engine and proves reconstruction in separate processes.

```sh
./demo guided     # Guided mechanics; each run has its own journal
./demo serve      # Separate live browser session on port 8766
./demo live       # Run in a second terminal against that live session
```

[DEMO.md](DEMO.md) contains the presenter script, command reference, saved-session inspection and restart instructions. The live browser and terminal share a real-clock backend; the guided history is independent. The existing `./run` and port-8765 browser demonstration remain available.

## Auction mechanics

The explicit [demo configuration](demo-config.json) starts a separate history at Number 1 with 600 rana per simulated bidder. The opening minimum is 5 rana; each subsequent bid must increase the leading amount by at least 1 rana. For example, a leading bid of 5 can be raised to 6.

1. Bidder A bids 20. The 3:45 auction begins and 20 rana is reserved.
2. Bidder B bids 30. A's reservation is released; B now has 570 available and 30 reserved.
3. A bids 30. The backend rejects the insufficient increase and records an invalid bid.
4. B raises to 40, using its existing reservation. A can then lead with 50.
5. After the auction closes, the resolved winner waits for an explicit settlement choice.
6. **Simulate successful settlement** captures the winning amount into protocol-held Rana and establishes the winner’s ownership. **Simulate failed settlement** releases the winning reservation and leaves the number Unowned (no owner). The failure command retains `expired` as its internal label; there is no settlement deadline.
7. After the 12-second gap, the next number appears and remains dormant until its first valid bid.

A valid bid within the final 10 seconds extends the auction by 15 seconds, up to two times. The backend controls all timing and outcomes. The browser only displays them and submits commands.

## Stored history

The current demo journal is `data/demo-02.sqlite3`, relative to this directory. The earlier `data/prototype.sqlite3` history remains saved separately. Its adjacent `.writer.lock` prevents a second application writer; `.diagnostic.log` records non-authoritative failures when possible. These local files are ignored by Git.

The application has no reset, history edit, top-up, protocol spending, compensation or automatic command retry operation. An unavailable response may follow a successful commit: inspect the state and journal before making a new explicit submission. A malformed journal stops processing and is preserved for investigation.

Initial allocations and the starting number apply only to a new history. Changing them does not reset an existing journal. Changing a permitted auction parameter affects future AuctionRecords, not parameters already captured in records. On 2026-09-05 the user authorized a separate fresh demo at Number 1, with initial allocations of 600 each, minimum bid 5 and minimum increase 1. Its initial allocations include the requested extra 500 each; no credit is applied to the earlier history. The UI04 acceptance fixture and original verification report retain their explicit 100-rana allocations and 10-rana minimum/increment.

This prototype uses simulated identities and local Rana. It does not accept real payments, establish external ownership, or protect a journal against a person controlling the machine.
