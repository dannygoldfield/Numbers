# Configuration Reference — Numbers

This document defines the entire configuration surface area for Numbers.

It is normative.

Any configuration key not documented here is forbidden.
Any documented configuration must be validated strictly.

If any configuration conflicts with:
- STATE-MACHINE.md
- INVARIANTS.md
- TRANSITION-INVARIANTS.md
- PERSISTENCE.md
- RESTART-RULES.md

the configuration is invalid and execution must halt.

---

## 1. Purpose

Configuration tunes parameters only.

Configuration must not:
- alter lifecycle semantics
- alter authority rules
- alter invariants
- reinterpret persisted history

Configuration is not a repair mechanism.

---

## 2. Configuration Principles (Normative)

All configuration handling must obey:

- Configuration must not create, remove, merge, or reinterpret states
- Configuration must not bypass safety rules
- Configuration must not enable forbidden retries
- Configuration must not restore consumed authority
- Invalid configuration must fail fast and halt execution
- Defaults must be explicit
- Unknown keys are fatal
- Missing required keys are fatal
- Validation must occur before startup
- Validation must occur again on restart

Once canonical records exist:

- `chain.network` must be immutable
- auction timing parameters must not change for auctions already opened
- settlement parameters must not change for auctions already resolved

Configuration must be snapshotted at startup
and persisted as part of system control state.

---

## 3. Auction Timing

### auction.duration_seconds

- Type: integer
- Required: yes
- Minimum: 1
- Default: none

Defines base auction duration in seconds.

This value is snapshotted at auction open.
It must not change for that auction.

---

### auction.extension_window_seconds

- Type: integer
- Required: yes
- Minimum: 1
- Default: none

Defines the window before `current_end_time`
during which a valid bid triggers an extension event.

Snapshotted at auction open.

---

### auction.extension_increment_seconds

- Type: integer
- Required: yes
- Minimum: 1
- Default: none

Defines seconds added per ExtensionEventRecord.

Snapshotted at auction open.

---

### auction.max_extensions

- Type: integer
- Required: yes
- Minimum: 0
- Default: none

Defines maximum number of extension events allowed.

Snapshotted at auction open.

If set to 0, no extensions are permitted.

---

### auction.inter_auction_gap_seconds

- Type: integer
- Required: yes
- Minimum: 0
- Default: none

Defines delay between FinalizationRecord for auction N
and creation of AuctionRecord for auction N+1.

This value:

- does not alter resolution
- does not alter settlement
- does not alter inscription
- does not alter lifecycle truth

Sequence advancement remains lifecycle-bound.

---

## 4. Bidding Constraints

### bidding.minimum_bid_sats

- Type: integer
- Required: yes
- Minimum: 1
- Default: none

Defines minimum opening bid.

Snapshotted at auction open.

---

### bidding.minimum_increment_sats

- Type: integer
- Required: yes
- Minimum: 1
- Default: none

Defines minimum increase over current highest valid bid.

Snapshotted at auction open.

---

### bidding.maximum_bid_sats

- Type: integer
- Required: no
- Default: null

Defines optional absolute upper bound.

If null, no upper bound exists.

Snapshotted at auction open.

---

## 5. Settlement Parameters

### settlement.deadline_seconds

- Type: integer
- Required: yes
- Minimum: 1
- Default: none

Defines settlement window duration after resolution.

Deadline must be computed and persisted at resolution time.
It must not be extended or reset.

---

### settlement.null_destination

- Type: string
- Required: yes
- Default: none

Defines destination used when:

- settlement fails
- settlement expires
- no valid bids exist

Must be immutable once canonical records exist.

---

## 6. Inscription Parameters

### inscription.enabled

- Type: boolean
- Required: yes
- Default: true

If false:

- inscription initiation is forbidden
- inscription authority is never exercised

Disabling inscription does not restore authority
for auctions already finalized.

---

## 7. Operational Limits

### system.max_concurrent_actions

- Type: integer
- Required: yes
- Minimum: 1
- Default: 1

Must not allow overlapping authority-bearing execution
for the same auction.

---

### system.pause_on_ambiguous_error

- Type: boolean
- Required: yes
- Default: true

If true, system enters `Paused`
when an AmbiguityRecord is written.

Disabling does not permit authority reuse.

---

## 8. Observability

### logging.level

- Type: string
- Required: yes
- Allowed values: `debug` | `info` | `warn` | `error`
- Default: `info`

Must not suppress fatal events or ambiguity events.

Logging must not influence lifecycle or authority.

---

### metrics.enabled

- Type: boolean
- Required: yes
- Default: true

Observational only.

Must not influence:
- timing
- authority
- persistence
- transitions

---

## 9. Chain and Network

### chain.network

- Type: string
- Required: yes
- Allowed values: `mainnet` | `testnet` | `regtest`
- Default: none

Must be immutable once canonical records exist.

---

### chain.confirmation_depth

- Type: integer
- Required: yes
- Minimum: 1
- Default: none

Defines confirmation requirement for:

- settlement success recognition
- inscription confirmation recognition

Affects knowledge acceptance only.
Does not alter authority consumption.

---

## 10. Validation Rules (Normative)

Validation must enforce:

- unknown keys are fatal
- missing required keys are fatal
- invalid values are fatal
- illegal combinations are fatal
- validation must occur before execution begins
- validation must occur again on restart

Execution must not begin if validation fails.

---

## Final Rule

Configuration may tune parameters.

Configuration must never change:

- lifecycle truth
- authority boundaries
- persisted history

If configuration would be required to fix an outcome,
the system must halt.