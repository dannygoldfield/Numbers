# Configuration Reference — Numbers

This document defines the **entire configuration surface area** for Numbers.

It is **normative**.

Any configuration key not documented here **is forbidden**.  
Any documented configuration **must be validated strictly**.

If any configuration conflicts with:
- STATE-MACHINE.md
- INVARIANTS.md
- TRANSITION-INVARIANTS.md
- PERSISTENCE.md
- RESTART-RULES.md

the configuration is invalid and execution **must halt**.

---

## 1. Purpose

This document exists to:

- Lock the configuration surface area
- Prevent hidden behavior toggles
- Ensure reproducible deployments
- Enable deterministic validation by humans and LLMs

Configuration **tunes parameters only**.  
Configuration **must not alter semantics, authority, or truth**.

Configuration is not a repair mechanism.

---

## 2. Configuration Principles (Normative)

All configuration handling **must** obey the following rules:

- Configuration must not alter invariants
- Configuration must not create, remove, merge, or reinterpret states
- Configuration must not bypass safety rules
- Configuration must not enable retries forbidden by the specification
- Configuration must not restore or recreate consumed authority
- Invalid configuration **must fail fast and halt execution**

Defaults are explicit.  
Silence is forbidden.

---

## 3. Auction Timing

### auction.duration_seconds

- Type: integer
- Required: yes
- Minimum: 1
- Maximum: implementation-defined hard upper bound
- Default: none

Defines the duration of a single auction in seconds.

This value applies uniformly to all auctions.

---

### auction.inter_auction_gap_seconds

- Type: integer
- Required: yes
- Minimum: 0
- Maximum: implementation-defined hard upper bound
- Default: none

Defines the delay between finalization of auction `N`
and opening of auction `N+1`.

This value:
- does not depend on settlement
- does not depend on inscription
- does not alter auction truth

---

## 4. Bidding Constraints

### bidding.minimum_increment

- Type: integer
- Required: yes
- Minimum: 1
- Default: none

Defines the minimum allowed increase over the current highest bid.

This constraint is fixed at auction start.

---

### bidding.maximum_bid

- Type: integer
- Required: no
- Default: null

Defines an absolute upper bound on bid amount.

If unset (`null`), no upper bound is enforced.

This cap exists to bound exposure, not to influence valuation.

---

## 5. Settlement Parameters

### settlement.deadline_seconds

- Type: integer
- Required: yes
- Minimum: 1
- Maximum: implementation-defined hard upper bound
- Default: none

Defines the time window during which settlement must complete
after auction resolution.

Deadlines:
- must not be extended
- must not be reset
- must not be inferred

---

### settlement.null_destination

- Type: string
- Required: yes
- Allowed values: valid address formats for the configured chain
- Default: none

Defines the destination address used when settlement does not complete.

Routing to this address is:
- valid
- final
- non-exceptional

---

## 6. Inscription Parameters

### inscription.enabled

- Type: boolean
- Required: yes
- Default: true

Controls whether inscription attempts are permitted at all.

Disabling inscription:
- does not retroactively change outcomes
- does not restore authority
- does not alter auction truth
- does not alter settlement truth

---

### inscription.max_attempts

- Type: integer
- Required: yes
- Allowed values: exactly `1`
- Default: 1

Defines the maximum number of inscription attempts.

This value is fixed by invariant.  
Any value other than `1` is invalid and fatal.

---

## 7. Operational Limits

### system.max_concurrent_actions

- Type: integer
- Required: yes
- Minimum: 1
- Default: 1

Defines the maximum number of concurrent authority-bearing actions.

This limit **must not** permit overlapping authority execution.

---

### system.pause_on_ambiguous_error

- Type: boolean
- Required: yes
- Default: true

Defines whether the system automatically enters `Paused`
when an Ambiguous Error is detected.

Disabling this setting:
- must not permit authority reuse
- must not permit retries
- must not permit progression past ambiguity

---

## 8. Observability

### logging.level

- Type: string
- Required: yes
- Allowed values: `debug` | `info` | `warn` | `error`
- Default: `info`

Defines the minimum log severity emitted.

Logging configuration:
- must not suppress Ambiguous errors
- must not suppress Fatal errors

---

### metrics.enabled

- Type: boolean
- Required: yes
- Default: true

Controls emission of operational metrics.

Metrics are observational only.
They must not affect behavior, timing, or authority.

---

## 9. Chain and Network

### chain.network

- Type: string
- Required: yes
- Allowed values: `mainnet` | `testnet` | `regtest`
- Default: none

Defines the Bitcoin network the system operates on.

This value **must be immutable** for the lifetime of persisted data.

Changing this value with existing persisted state is forbidden.

---

### chain.confirmation_depth

- Type: integer
- Required: yes
- Minimum: 1
- Default: none

Defines the number of confirmations required
before a Bitcoin transaction is considered final
for settlement and inscription observation.

This value affects **when knowledge is accepted**,  
not **what outcomes are permitted**.

It must be consistent with:
- STATE-MACHINE.md
- settlement semantics
- ERROR-TAXONOMY.md

---

## 10. Validation Rules (Normative)

Configuration validation **must** enforce all of the following:

- Unknown keys are forbidden and fatal
- Missing required keys are fatal
- Invalid values are fatal
- Defaults must be applied explicitly and recorded
- Validation must occur before startup
- Validation must occur again on restart

Execution **must not begin** if validation fails.

---

## 11. Final Rule

Configuration may tune **behavioral parameters**.

Configuration must never change:
- truth
- authority
- history

If configuration would be required to “fix” an outcome,  
the system must halt instead.
