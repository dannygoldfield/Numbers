# Core Sequence

This document defines the invariants of Numbers.

Numbers are auctioned sequentially.

Each auction:
- has a predefined end condition
- accepts only valid bids
- resolves exactly once
- produces exactly one canonical inscription.

Content alone does not determine canonicity.

If the winning bidder settles, the inscription is sent to them.

If the winning bidder does not settle, the inscription is sent to a provably unspendable address.

Bid validity is determined at auction start and remains fixed for the duration of that auction.

The sequence never pauses, retries, or rewinds.
