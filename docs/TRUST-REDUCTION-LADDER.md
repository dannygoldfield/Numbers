# Numbers Trust Reduction Ladder

## Core Question

Why should participants trust the auction record?

Bitcoin secures ownership of the Inscription after settlement.

The challenge for Numbers is establishing confidence in the auction history that produced that ownership.

The goal is to reduce trust by replacing assumptions with verification wherever practical.


## Rungs of the Ladder

0. Trust the operator.

1. Preserve history.

2. Verify bids.

3. Detect tampering.

4. Replicate history.

5. Anchor history.

6. Bind economic intent.

7. Reduce settlement trust.

Move from trust to verification.



## Level 0: Trust the Operator

The operator controls the auction record.

Participants trust the operator to run the auction honestly.



## Level 1: Preserve History

Record events as append-only facts.

Do not overwrite history.

Current state is derived from recorded events.



## Level 2: Verify Bids

Require bidders to sign bids.

The operator can no longer invent bids on behalf of participants.



## Level 3: Detect Tampering

Hash-chain auction events.

History can still be changed.

Changes become detectable.



## Level 4: Replicate History

Store auction records in multiple places.

Verification no longer depends on a single server.



## Level 5: Anchor History

Publish a cryptographic commitment to Bitcoin.

Auction history gains a durable timestamp and external point of verification.



## Level 6: Bind Economic Intent

Require bidders to prove economic seriousness.

Reduce fake bids and disruptive behavior.



## Level 7: Reduce Settlement Trust

Move settlement authority from operator discretion toward verifiable rules.



## Guiding Principle

Move from trust to verification.
