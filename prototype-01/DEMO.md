# Presenting Numbers

Show a rule, the Rust that implements it, a command, and the records it commits. The browser, live terminal and guided walkthrough all use the same Prototype 1 engine.

## Open the presentation workspace

Open `Numbers-demo.code-workspace` in VS Code. It shows only Prototype 1 and the active specification, avoiding the independent historical application at the repository root. Rust navigation uses the installed rust-analyzer extension. Workspace font settings apply only to this workspace.

Use **Terminal → Run Task → Numbers: Guided mechanics**. A terminal opens beside your code and waits for Enter before each step. No task runs automatically when the workspace opens.

To start it yourself, open a terminal in `prototype-01` and run:

```sh
./demo guided
```

The first launch builds the two Rust executables. Start once before a presentation to have the build ready. No new package is needed beyond the installed development environment.

## What the guided walkthrough looks like

```text
Next 3/18: B takes the lead with 30 [Enter] >

── Step 3/18 · t=30s (simulated) ──
B takes the lead with 30
A's reservation is released; B's full bid is reserved.
Rule/code: G03 / RT02 · src/model.rs: plan_bid
Command: bid {"amount_rana":30,"auction_number":1,"bidder_id":"demo-2"}
Command accepted and committed.
  #7 Bid recorded · bid_replace
  #8 Rana released · bid_replace
  #9 Rana reserved · bid_replace
Number 1 · Open
  Bidder A: 600 available · 0 reserved
  Bidder B: 570 available · 30 reserved
```

Enter executes the next prescribed step. Type `records` to inspect the last step's exact saved records, `raw` for its full response, or `quit` to stop and retain the partial journal. These inspection commands do not append events. A completed run keeps its SQLite journal, final result and two reconstruction receipts in the session directory printed at startup. Each new run creates a separate directory.

The timestamps are explicitly **simulated**. The ledger, admission rules, commit groups and reconstruction are real Rust and SQLite operations. There is no clock control on the live server. The guided journal is independent of the browser's live history.

## A short presenter script

| Steps | Show and say | Open in the editor |
|---|---|---|
| 1–3 | “A bid reserves funds. When B takes the lead, A gets its reservation back.” Inspect the three replacement records. | `RANA-AND-OWNERSHIP.md` RT02; `src/model.rs` → `plan_bid` |
| 4 | “An equal bid is recorded as invalid and moves no rana.” | `STATE-AND-EVENTS.md` SE03; `src/model.rs` → `rejection` |
| 5–6 | “Raising your own bid replaces the existing hold. Exactly one leading hold remains.” | `src/model.rs` → `State::fold` and `plan_bid` |
| 7–9 | “Late accepted bids extend the end by 15 seconds, twice at most.” Inspect the extension records and then the accepted bid without another extension. | `STATE-AND-EVENTS.md` SE03; `src/model.rs` → `plan_bid` |
| 10–11 | “The winner is fixed before settlement. Success captures 62 and establishes B's ownership.” | `RANA-AND-OWNERSHIP.md` RT03; `src/model.rs` → `plan_settlement` |
| 12 | “A separate process reconstructed the same records and derived facts.” The receipt changes from 0 to 32 validated records. | `PERSISTENCE-AND-RESTART.md` PR04; `src/model.rs` → `reconstruct` |
| 13–16 | “The next number starts dormant. Failure releases its winning reservation and leaves it Unowned.” | RT03–RT04; `src/model.rs` → `plan_settlement` |
| 17–18 | “Unowned survives reconstruction too. Number 3 is ready; the two earlier outcomes remain recorded.” | `src/store.rs` → `evaluate`; saved restart receipts |

The complete fixture ends with 42 records, A600/B538 available, zero reserved and 62 protocol-held rana. Number 1 belongs to B; Number 2 is Unowned; Number 3 is pending. The current state projection follows Number 3; inspect finalization records 32 and 41 for the earlier outcomes.

Use VS Code's symbol navigation to find the named function. The engine files implement the rules; `src/bin/numbers-demo/` contains only the client, narration, explicit fixture and session management. `tests/demonstration.rs` verifies the shipped executable and real HTTP path.

## Live browser and terminal together

Run **Numbers: Start separate live demo** or, in a first terminal:

```sh
./demo serve
```

This creates a fresh session at [Auction](http://127.0.0.1:8766/) with a [Protocol view](http://127.0.0.1:8766/protocol). It starts Number 1 with 600 rana each, minimum 5 and increment 1. Its first valid bid starts the real 3:45 clock. Existing port-8765 histories are separate.

In a second terminal, use **Numbers: Live terminal** or:

```sh
./demo live
```

| Command | What it does |
|---|---|
| `bid A 20` | A bids 20 on the number last displayed in this terminal |
| `bid B 30` | B takes the lead; the browser reflects the committed result |
| `bid A 30` | Demonstrates an insufficient increase |
| `state` or Enter | Refreshes the terminal's snapshot after browser activity |
| `history` | Lists all records in canonical sequence order |
| `record 8` | Displays the exact record at sequence 8 |
| `raw` | Shows the last response without rounding economic integers |
| `settle success` | Submits the successful simulated outcome, when settlement is available |
| `settle failure` | Submits failure, releases the winning reservation and finalizes Unowned |
| `quit` | Leaves the client; the server continues in its other terminal |

The live terminal shows observations, not a running local countdown. Press Enter after a browser bid to refresh. A command uses the auction last displayed; it does not silently switch to the next number. Each command is sent once. If a response is unavailable, inspect state and history before explicitly submitting again.

The server already supports ordinary browser polling. Use the two clients on one journal to demonstrate that neither interface owns the outcome. The backend does. These are still two simulated identities on one device; Prototype 2 identity and auction proposals are not active.

## Stop and demonstrate restart

In the server terminal, press Ctrl-C. Copy the **Restart this history** command printed when that live session started:

```sh
./demo serve --session '/absolute/session/path/from/startup'
```

Supply the actual printed path. This reconstructs that same history; `./demo serve` without `--session` intentionally creates a different new demonstration. Refresh the client to see the reconstruction receipt. Real time continues while the server is stopped; a due close or next-number event can follow the separate reconstruction phase. Settlement is always explicit.

For an inspection that appends no lifecycle events, use the actual session path:

```sh
./demo inspect '/absolute/session/path/from/startup'
```

Inspection opens SQLite read-only, validates every complete group and reports the derived state at the last recorded timestamp. A guided session can be inspected this way but cannot be resumed as a real-clock live session. Interrupted guided runs remain inspectable; a new walkthrough starts separately.

## Vocabulary and evidence

Use **ownership**, **Owned by Bidder A/B**, and **Unowned — no owner** when presenting. Pending ownership is different from a finalized Unowned outcome. Exact revision 0.1 JSON retains `title`, `title_kind`, `holder_id`, `PublicLand` and `expired_to_publicland` for compatibility with saved history; RT06 documents this mapping. There is no collective owner.

The tools illustrate recorded ownership within this prototype. They do not demonstrate Bitcoin integration, real payment, authentication or external ownership enforcement. Local SQLite records cannot protect themselves from the machine owner.

Run `./check` before a presentation after code changes. `./demo guided --auto` runs all prescribed steps without prompts and saves evidence in a fresh session. `VERIFICATION.md` records completed checks and their limits.
