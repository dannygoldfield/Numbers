# Demonstrate Numbers

## Guided mechanics

```sh
./demo guided
./demo guided --auto --directory /absolute/path/to/new-guided-session
./demo inspect /absolute/path/to/existing-guided-session
```

Guided time is explicitly simulated. The same engine produces bids, releases/replacements, extensions, resolution, successful capture/ownership, failed settlement/Unowned and the next dormant number. The UI05 fixture ends with 42 records, A600/B538 available, zero reserved and protocol62. Separate processes verify exact rows and derived state after both terminal outcomes. Inspection opens the copied journal read-only and never evaluates lifecycle time.

The current tools inspect only revision 0.3. For Prototype 1, use the matching preserved build/configuration on copies; [HANDOFF](HANDOFF.md) identifies them. The former live terminal is not the Prototype 2 remote interface.

## Invited browser session — after deployment approval

Prepare one dedicated session under the [operating procedure](ops/README.md). Assign bidder-a to demo-1 and bidder-b to demo-2 privately; the facilitator has its own credential and no bidding account. Each participant uses a separate device. Browser-native authentication may retain credentials for the browser session, so use separate browser profiles/private windows and do not share an authenticated device. No reliable custom sign-out or account recovery flow is implemented.

1. Both bidders open Auction and Record through the same HTTPS origin. Confirm the session label, assigned bidder, simulated funds and shared balances.
2. Ask each bidder to place a bid and locate the recorded outcome. Observe an outbid and an insufficient increase without operating their device for them.
3. After resolution, the facilitator selects successful settlement. Ask both people to explain the capture, fixed winner and recorded ownership.
4. After the gap, repeat on the next number with failed settlement. Ask them to explain the released reservation and Unowned outcome.
5. If a command response is uncertain, inspect the records. Do not repeat it. A read's absence is not proof the command cannot still complete. Use the explicit new-command acknowledgment only after inspection; unresolved outcomes require facilitator stop/inspect.
6. Stop ingress and the writer and verify a matching off-host preservation copy before ending the session.

No client clock, visual countdown or operator belief closes an auction. Reading state/history can cause the explicitly prescribed due evaluation. Settlement has no timer. Stopping the process does not pause deadlines.

## Collect colleague feedback

Record observations in [the experiment brief](../numbers-spec/work/PROTOTYPE-02.md#collect-colleague-feedback), using session and simulated bidder labels. Capture task, exact confusion, help given and proposed follow-up separately. Feedback is evidence for a later proposal, not permission to modify current rules during the experiment. No invited trial has yet been performed.
