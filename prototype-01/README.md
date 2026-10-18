# Numbers · Prototype 1

A local Rust application for the approved Numbers 0.2 specification. Two simulated bidders compete for a number. The journal records bids, Rana movements, resolution, settlement and ownership. Restart reconstructs those records before new work begins.

The sole active behavioral authority is [numbers-spec](../numbers-spec/STATUS.md). Earlier applications and specifications are preserved through the [historical index](../archive/README.md); they are not dependencies of this application.

For the current checkpoint and next activity, see [HANDOFF.md](HANDOFF.md). To try it with colleagues, follow the [feedback session](DEMO.md#collect-colleague-feedback).

## Run

From this directory:

```sh
./run
```

Open [Auction](http://127.0.0.1:8765/) and [Record](http://127.0.0.1:8765/record). The Record page can sit beside the Auction page and contains no outcome controls. Stop the server with Ctrl-C. Running the same command again reconstructs the same journal.

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

The revised demo uses `data/revision-0.2/demo.sqlite3`, relative to this directory. Its first launch creates a separate history. The preserved `data/demo-02.sqlite3`, `data/prototype.sqlite3`, and earlier demonstration sessions require the old build; the new build rejects their format without editing them. See [HANDOFF.md](HANDOFF.md) for the verified backup. Its adjacent `.writer.lock` prevents a second application writer; `.diagnostic.log` records non-authoritative failures when possible. These local files are ignored by Git.

The original winner and finalization record cannot be rewritten. A future separately approved specification may define append-only transfers, including between wallets belonging to the same person or different people; no wallet or transfer mechanism exists here.

The application has no reset, history edit, top-up, protocol spending, compensation or automatic command retry operation. An unavailable response may follow a successful commit: inspect the state and journal before making a new explicit submission. A malformed journal stops processing and is preserved for investigation.

Initial allocations and the starting number apply only to a new history. Changing them does not reset an existing journal. Changing a permitted auction parameter affects future AuctionRecords, not parameters already captured in records. The UI04 acceptance fixture uses its own explicit 100-rana allocations and 10-rana minimum/increment. The [historical verification](https://github.com/dannygoldfield/Numbers/blob/d567a3f55e951114ee4c5c125cdd9366dd70de3b/prototype-01/VERIFICATION.md#fresh-demo-configuration--2026-09-05) records the separate 600-rana demo initialization.

This prototype uses simulated identities and local Rana. It does not accept real payments, establish external ownership, or protect a journal against a person controlling the machine.
