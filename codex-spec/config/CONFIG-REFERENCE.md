# Configuration Reference: Numbers

This document defines the entire configuration surface area for Numbers.

It is normative.

Authority precedence is defined exclusively in `AUTHORITY-ORDER.md`.

Any configuration key not documented here is forbidden.

Any documented configuration must be validated strictly.

Configuration tunes parameters only.

Configuration must never change:

- lifecycle truth
- authority boundaries
- canonical event record meaning
- persisted history
- restart reconstruction
- finalization meaning

If configuration would be required to fix an outcome, execution must halt.

---

# 1. Purpose

Configuration exists to define bounded operational parameters.

Configuration must not:

- create lifecycle states
- remove lifecycle states
- merge lifecycle states
- reinterpret lifecycle states
- create authority
- restore authority
- consume authority
- permit retry behavior
- bypass safety rules
- reinterpret persisted history
- repair missing records

Configuration is not a repair mechanism.

---

# 2. Configuration Principles

All configuration handling must obey:

- unknown keys are fatal
- missing required keys are fatal unless an explicit default is defined for that key
- invalid values are fatal
- illegal combinations are fatal
- defaults must be explicit
- validation must occur before execution begins
- validation must occur again on restart
- configuration must not alter canonical truth
- configuration must not alter authority semantics
- configuration must not create retry behavior

Once canonical event records exist:

- `chain.network` must be immutable
- auction timing parameters must not change for auctions already opened
- settlement parameters must not change for auctions already resolved
- inscription adapter mode must not reinterpret existing inscription records
- `auction.starting_number` must not reinterpret existing `AuctionRecord` entries

Configuration values that affect persisted lifecycle calculations must be captured in the relevant canonical event records at the boundary where they become fixed.

---

# 3. Demo Scope Configuration

## `prototype.demo_stage`

- Type: string
- Required: yes
- Allowed values: `demo_1` | `demo_2`
- Default: none

Defines the active prototype demonstration stage.

### `demo_1`

Demo 1 is the deterministic single-machine browser auction demo without live Ordinals broadcast.

When `prototype.demo_stage = demo_1`:

- live Ordinals broadcast must not be required
- Bitcoin Core RPC must not be required for auction correctness
- wallet interaction must not be required for auction correctness
- mempool recognition must not be required
- confirmation observation must not be required
- inscription adapter mode must be `deferred_in_this_slice`

### `demo_2`

Demo 2 includes Testnet inscription adapter behavior only when explicitly implemented by the active implementation slice.

`demo_2` does not automatically permit live inscription behavior.

Live inscription behavior must also be permitted by the active implementation slice.

---

# 4. Storage Configuration

## `storage.kind`

- Type: string
- Required: yes
- Allowed values: `sqlite`
- Default: none

Defines the persistence backend for Demo 1.

Demo 1 permits only `sqlite`.

Additional storage kinds require specification revision.

---

## `storage.sqlite_path`

- Type: string
- Required: yes
- Default: none

Defines the local SQLite database path.

The path is operational configuration only.

Changing this path must not reinterpret canonical event records.

---

# 5. Server Configuration

## `server.host`

- Type: string
- Required: yes
- Default: `127.0.0.1`

Defines the local backend host.

This value must not alter lifecycle semantics.

---

## `server.port`

- Type: integer
- Required: yes
- Minimum: `1`
- Maximum: `65535`
- Default: none

Defines the local backend port.

This value must not alter lifecycle semantics.

---

# 6. Auction Timing

## `auction.starting_number`

- Type: integer
- Required: yes
- Minimum: `1`
- Default: `1`

Defines the number used for the first `AuctionRecord`.

For Demo 1, the default starting number is `1`.

`auction.starting_number`:

- determines only the first auction number when no canonical auction records exist
- must not be used to skip an existing canonical sequence
- must not reinterpret existing `AuctionRecord` entries
- must not alter sequence advancement after the first auction is persisted

After the first `AuctionRecord` is persisted, subsequent auction numbers must advance by exactly `1` from the previous finalized auction.

---

## `auction.duration_seconds`

- Type: integer
- Required: yes
- Minimum: `1`
- Default: none

Defines base auction duration in seconds.

This value is captured in `AuctionRecord` when the auction is created.

It must not change for that auction after `AuctionRecord` persistence.

---

## `auction.extension_window_seconds`

- Type: integer
- Required: yes
- Minimum: `1`
- Default: none

Defines the window before `current_end_time` during which a valid bid triggers an extension event.

This value is captured in `AuctionRecord` when the auction is created.

It must not change for that auction after `AuctionRecord` persistence.

---

## `auction.extension_increment_seconds`

- Type: integer
- Required: yes
- Minimum: `1`
- Default: none

Defines seconds added per `ExtensionEventRecord`.

This value is captured in `AuctionRecord` when the auction is created.

It must not change for that auction after `AuctionRecord` persistence.

---

## `auction.max_extensions`

- Type: integer
- Required: yes
- Minimum: `0`
- Default: none

Defines the maximum number of extension events allowed.

This value is fixed for an auction when `AuctionOpenRecord` is persisted.

If set to `0`, no extensions are permitted.

---

## `auction.inter_auction_gap_seconds`

- Type: integer
- Required: yes
- Minimum: `0`
- Default: none

Defines delay between `FinalizationRecord` for auction `N` and creation of `AuctionRecord` for auction `N + 1`.

This value:

- does not alter resolution
- does not alter settlement
- does not alter inscription
- does not alter lifecycle truth
- does not automatically open auction `N + 1`

Sequence advancement remains lifecycle-bound.

---

# 7. Bidding Constraints

## `bidding.minimum_bid_sats`

- Type: integer
- Required: yes
- Minimum: `1`
- Default: none

Defines minimum opening bid.

This value is captured in `AuctionRecord` when the auction is created.

It must not change for that auction after `AuctionRecord` persistence.

---

## `bidding.minimum_increment_sats`

- Type: integer
- Required: yes
- Minimum: `1`
- Default: none

Defines minimum increase over the current highest valid bid.

This value is captured in `AuctionRecord` when the auction is created.

It must not change for that auction after `AuctionRecord` persistence.

---

## `bidding.maximum_bid_sats`

- Type: integer or null
- Required: yes
- Default: `null`

Defines optional absolute upper bound.

If `null`, no upper bound exists.

This value is captured in `AuctionRecord` when the auction is created.

---

# 8. Address Policy

## `address.allowed_networks`

- Type: array of strings
- Required: yes
- Allowed values: `mainnet` | `testnet` | `regtest`
- Default: none

Defines allowed Bitcoin address networks for bid destination addresses.

For Demo 1, this value must match the active prototype network.

---

## `address.allowed_types`

- Type: array of strings
- Required: yes
- Allowed values: `p2wpkh` | `p2tr`
- Default: none

Defines permitted Bitcoin address types for destination addresses.

A destination address outside the permitted address policy must make the bid invalid.

For Demo 1 with `validation_profile = demo_local`, address policy can validate local demo destination identifiers according to the active implementation slice.

---

# 9. Settlement Parameters

## `settlement.deadline_seconds`

- Type: integer
- Required: yes
- Minimum: `1`
- Default: none

Defines settlement window duration after resolution.

`settlement_deadline` must be computed and persisted at resolution time in `ResolutionRecord`.

It must not be extended or reset.

---

## `settlement.null_destination`

- Type: string
- Required: yes
- Allowed value: `NullSteward`
- Default: `NullSteward`

Defines the required destination label used when:

- settlement fails
- settlement expires

In Demo 1, no valid bid does not produce a `NullSteward` destination. The auction remains `Scheduled`.

`NullSteward` is a protocol outcome, not an operator-selected address.

---

# 10. Inscription Parameters

## `inscription.adapter_mode`

- Type: string
- Required: yes
- Allowed values: `deferred_in_this_slice` | `testnet_ordinals`
- Default: none

Defines inscription adapter mode for new `InscriptionIntentRecord` entries.

### `deferred_in_this_slice`

The system can persist inscription intent but must not perform live inscription broadcast.

When `inscription.adapter_mode = deferred_in_this_slice`:

- `InscriptionIntentRecord` persistence is valid
- `InscriptionBroadcastRecord` must not be persisted
- `InscriptionConfirmationRecord` must not be persisted
- live inscription success must not be simulated
- inscription authority must not be consumed

### `testnet_ordinals`

The system can use the Testnet inscription adapter only if permitted by the active implementation slice.

This value alone does not permit live inscription behavior.

Live inscription behavior must be defined by the active implementation slice.

---

# 11. Chain and Network

## `chain.network`

- Type: string
- Required: yes
- Allowed values: `testnet` | `regtest`
- Default: none

Defines the Bitcoin network for prototype operation.

Demo 1 does not require live chain interaction.

Mainnet is excluded from the current prototype scope.

---

## `chain.confirmation_depth`

- Type: integer
- Required: yes
- Minimum: `1`
- Default: none

Defines confirmation requirement for:

- settlement success recognition
- inscription confirmation recognition

This affects knowledge acceptance only.

It does not alter authority consumption.

---

## `chain.rpc_url`

- Type: string or null
- Required: yes
- Default: `null`

Defines Bitcoin Core RPC URL when live chain interaction is enabled.

For Demo 1, `null` is valid.

---

## `chain.rpc_wallet`

- Type: string or null
- Required: yes
- Default: `null`

Defines Bitcoin Core wallet name when live wallet interaction is enabled.

For Demo 1, `null` is valid.

---

## `chain.data_path`

- Type: string or null
- Required: yes
- Default: `null`

Defines local Bitcoin data path for operator configuration.

This value can point to an external SSD.

This value is operational configuration only.

It must not alter lifecycle semantics, authority semantics, or canonical truth.

For Demo 1, `null` is valid.

---

# 12. Observability

## `logging.level`

- Type: string
- Required: yes
- Allowed values: `debug` | `info` | `warn` | `error`
- Default: `info`

Defines logging verbosity.

Logging must not influence:

- lifecycle state
- authority
- persistence
- restart reconstruction

Logging must not suppress fatal events or ambiguity events.

Logs are non-authoritative.

---

## `metrics.enabled`

- Type: boolean
- Required: yes
- Default: `false`

Enables or disables metrics.

Metrics are observational only.

Metrics must not influence:

- timing
- authority
- persistence
- transitions
- restart reconstruction

---

# 13. Validation Rules

Configuration validation must enforce:

- unknown keys are fatal
- missing required keys are fatal unless an explicit default is defined for that key
- invalid values are fatal
- illegal combinations are fatal
- validation must occur before execution begins
- validation must occur again on restart

Execution must not begin if validation fails.

---

# 14. Forbidden Configuration Effects

Configuration must not:

- define retry permission
- create automatic retry
- alter authority boundaries
- restore consumed authority
- restore frozen authority
- alter canonical event record meaning
- alter lifecycle state definitions
- alter allowed transitions
- reinterpret persisted records
- repair missing records
- enable mainnet for the current prototype
- silently enable live inscription behavior
- silently enable live settlement behavior

---

# Final Rule

Configuration tunes declared parameters only.

Configuration must never change:

- lifecycle truth
- authority boundaries
- persisted history
- restart reconstruction

If configuration would be required to fix an outcome:

The system must halt.


# 15. Captured Configuration Rule

When an `AuctionRecord` is persisted, the implementation must copy into the `AuctionRecord` payload the fixed configuration values required by `data/DATA-MODEL.md` for that auction.

Captured values govern that auction after persistence.

Later configuration changes must not alter bid admission, timing, extension, close, resolution, restart reconstruction, or API derivation for an auction whose `AuctionRecord` already exists.
