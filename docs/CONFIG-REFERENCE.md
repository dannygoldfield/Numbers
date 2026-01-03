# Configuration Reference — Numbers

This document defines all supported configuration parameters for Numbers.

It is normative.

Any configuration not documented here must be ignored.
Any documented configuration must be validated strictly.

If configuration conflicts with STATE-MACHINE.md or INVARIANTS.md,
the configuration is invalid.

---

## 1. Purpose

This document exists to:

- Lock configuration surface area
- Prevent hidden behavior toggles
- Ensure reproducible deployments
- Enable safe validation by humans and LLMs

Configuration controls parameters, not semantics.

---

## 2. Configuration Principles

- Configuration must not alter invariants
- Configuration must not create new states
- Configuration must not bypass safety rules
- Invalid configuration must fail fast

Defaults are explicit.
Silence is forbidden.

---

## 3. Auction Timing

### auction.duration_seconds

- Type: integer
- Required: yes
- Minimum: 1
- Maximum: implementation-defined
- Default: none

Defines the duration of a single auction.

---

### auction.inter_auction_gap_seconds

- Type: integer
- Required: yes
- Minimum: 0
- Default: none

Defines the delay between auctions.

This does not depend on settlement or inscription.

---

## 4. Bidding Constraints

### bidding.minimum_increment

- Type: integer
- Required: yes
- Minimum: 1
- Default: none

Minimum allowed increase over the current high bid.

---

### bidding.maximum_bid

- Type: integer
- Required: no
- Default: null

Upper bound on bid amount, if set.

---

## 5. Settlement Parameters

### settlement.deadline_seconds

- Type: integer
- Required: yes
- Minimum: 1
- Default: none

Time allowed for settlement after resolution.

---

### settlement.null_destination

- Type: string
- Required: yes
- Allowed values: implementation-defined address formats
- Default: none

Destination used when settlement is not completed.

---

## 6. Inscription Parameters

### inscription.enabled

- Type: boolean
- Required: yes
- Default: true

Controls whether inscription attempts are permitted.

Disabling does not retroactively change outcomes.

---

### inscription.max_attempts

- Type: integer
- Required: yes
- Allowed values: 1 only
- Default: 1

Multiple attempts are forbidden by invariant.

---

## 7. Operational Limits

### system.max_concurrent_actions

- Type: integer
- Required: yes
- Minimum: 1
- Default: 1

Limits concurrent authority-bearing actions.

---

### system.pause_on_ambiguous_error

- Type: boolean
- Required: yes
- Default: true

Controls whether the system pauses automatically on ambiguity.

---

## 8. Observability

### logging.level

- Type: string
- Required: yes
- Allowed values: debug | info | warn | error
- Default: info

---

### metrics.enabled

- Type: boolean
- Required: yes
- Default: true

---

## 9. Validation Rules

- Unknown keys are forbidden
- Missing required keys are fatal
- Invalid values are fatal
- Defaults must be applied explicitly

Configuration must be validated before startup.

---

## 10. Final Rule

Configuration may tune behavior,
but it may never change truth.
