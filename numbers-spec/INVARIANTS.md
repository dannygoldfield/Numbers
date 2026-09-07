# Candidate invariants

## IV01 — Consistency obligations

Every complete committed group and every successfully reconstructed journal must satisfy the properties below. These are invariant names with references to the owning operational clauses, not alternative transition definitions. Violation requires PR06 halt; it does not select an outcome.

| ID | Invariant | Owning clauses |
|---|---|---|
| I01 | Numbers form one contiguous sequence from the captured start; no reuse, skipping, overlap, or second non-terminal auction. | P02, SE01, SE04 |
| I02 | A number with no valid opening bid remains Scheduled indefinitely; no countdown, resolution, settlement, ownership, or Unowned follows from silence. | SE01, SE03 |
| I03 | Canonical records alone determine auction state, economic balances, settlement, and ownership. | PR01, PR04, RT01, RT04 |
| I04 | Committed records are never changed, removed, reordered, or replaced. | PR01, PR06 |
| I05 | Every accepted bid has the reservation and any opening/extension effects required by the same atomic group. Rejection never changes a hold. | RT02, SE03, SE05 |
| I06 | Each identity's available and reserved balances are nonnegative exact integers; total issued Rana is conserved across available, reserved, and protocol-held balances. | RT01 |
| I07 | Only the leading bid has a live reservation; it equals the full leading amount. Finalized auctions leave no reservation. | RT02, RT03 |
| I08 | Only valid pre-close bids participate; highest amount wins with earliest canonical sequence as tie-break; the result is recorded once. | SE03, SE06 |
| I09 | Resolution, terminal settlement, and ownership are distinct facts, each fixed by its defined record. Settlement never revises the resolved winner. | RT03–RT05, SE06 |
| I10 | Successful capture equals the winning bid and credits protocol-held Rana; failed settlement releases the hold. No protocol spending, extra issuance, or compensation is inferred. | RT01, RT03, SC03 |
| I11 | Each finalized number has exactly one final ownership outcome: the winning identity after settled, or Unowned after expired. Ownership never changes. | RT04 |
| I12 | Unowned is neither a participant/account nor an error or recovery mechanism. | RT04 |
| I13 | Base duration is 225 seconds; extensions obey captured parameters and cannot alter base end; close takes precedence at the effective end. | P02, SE03–SE04 |
| I14 | Finalization precedes the 12-second gap and next-number creation; creation never opens the next auction. | SE04 |
| I15 | Reconstruction preserves the committed prefix exactly and neither reissues Rana nor repeats outcome effects. | PR04–PR06 |
| I16 | One authoritative order and complete atomic groups connect auction, ledger, settlement, and ownership; unavailable or malformed history is not permission. | PR01–PR06, SE05 |
| I17 | UI commands, clocks, balances, animations, and identity selection cannot determine authoritative truth. | UI01–UI03 |
| I18 | Configuration cannot reinterpret captured history, and no unlisted retry or recovery exists. | P02, PR04–PR06, SE02 |
