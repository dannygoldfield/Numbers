# Application Programming Interface Specification — Numbers

This document defines the **entire external API surface** exposed by the Numbers backend.

It is **normative**.

This document assumes familiarity with:
- INVARIANTS.md
- STATE-MACHINE.md
- ERROR-TAXONOMY.md

If there is a conflict,
CORE-SEQUENCE.md, STATE-MACHINE-TABLE.md, STATE-MACHINE.md,
and INVARIANTS.md take precedence.

---

## Modal Language Rule (Normative)

In this document and all normative specifications:

- **must / must not** define obligations
- **only / exactly once / at most once** define bounds
- **may** is permitted **only** to describe uncertainty of knowledge,
  never permission, intent, or authority

Forbidden in normative contexts:

- possibly
- likely
- eventually
- for now
- TBD

Any normative statement using forbidden modal language is invalid.

---

## 1. Purpose (Normative)

The API exists to:

- expose canonical system state
- accept explicitly permitted external actions
- surface recorded knowledge without interpretation

The API **does not**:
- infer outcomes
- summarize meaning
- provide convenience abstractions
- grant or restore authority

---

## 2. Design Goals (Normative)

The API is:

- read-only by default
- minimal and enumerable
- deterministic in shape
- explicit in failure
- versioned from inception
- aligned strictly with persisted canonical state

The API reflects **what is recorded**.
It does not speculate.

---

## 3. Knowledge Representation (Normative)

API responses represent **knowledge only**.

Every response **must** distinguish explicitly between:
- what is known
- what is unknown

The API **must not**:
- infer outcomes
- express probability
- collapse ambiguity
- manufacture certainty
- fill gaps with assumed values

Absence of data **must not** be represented as resolution or failure.

Changes in knowledge exposed by the API:
- **do not** imply new permission
- **do not** restore authority
- **do not** enable retries

---

## 4. Authority Model (Normative)

The API is **non-authoritative**.

Authority boundaries are fixed:

- Bitcoin is authoritative for transaction existence.
- The Numbers backend is authoritative for:
  - auction resolution
  - auction finalization
  - inscription intent and ambiguity tracking
- The API exposes **persisted backend records only**.

API availability, delay, restart, or failure
**must not** alter outcomes, authority, or sequencing.

---

## 5. API Shape Rules (Normative)

All endpoints **must** obey:

- strict input validation
- deterministic output shapes
- missing fields represent unknown knowledge, not null outcomes
- field meaning **must not** change within a version
- side effects are forbidden unless explicitly stated

The API must not expose inferred fields.

Mechanically derived fields that are strictly computed
from persisted canonical records are permitted.

---

## 6. Public Endpoints (Normative)

Endpoints listed here define the **entire permitted surface**.
No undocumented endpoint may exist.

---

### GET /state

Returns the current global system state.

Includes only persisted canonical values and mechanically derived values:

- current auction number
- auction lifecycle state
- system control state (`Running` or `Paused`)
- opened_at
- base_end_time
- number_of_extension_events
- closed_at
- finalized_at

Derived values must be computable strictly from persisted records.
Time-remaining projections are forbidden.

This endpoint **must not**:
- predict future transitions
- compute remaining time
- project deadlines
- infer upcoming state changes
- interpolate missing temporal data

If a value cannot be derived directly from persisted records,
it **must** be omitted.

---

### GET /auction/history

Returns finalized auction outcomes in strict sequence order.

Rules:

- results are paginated
- each entry corresponds to **exactly one** auction number
- only finalized auctions are returned
- ordering is canonical and immutable

Each entry includes **finalized recorded knowledge only**:

- auction number
- final destination address
- settlement outcome
- inscription state
- inscription txid (if known)
- inscription satpoint (if known)
- resolution timestamp
- finalization timestamp

This endpoint **must not**:
- expose non-finalized state
- infer ownership
- collapse null outcomes
- hide `NullSteward` results
- rewrite or reinterpret history

---

### GET /auction/{N}

Returns the full recorded state for auction number `N`.

Includes **only persisted records** applicable to `N`:

- auction lifecycle state
- bid summary **only if permitted by lifecycle state**
- resolution record (if present)
- settlement state (if present)
- inscription state
- ambiguity indicators (if present)

If knowledge is unavailable,
it **must** be omitted rather than represented as null or guessed.

---

### POST /bid

Submits a bid for the current auction when auction state is one of: Scheduled or Open

This is the **only write-capable public endpoint**.

#### Preconditions (Normative)

A bid request is valid **only if all are true**:

- system control state is `Running`
- an auction is in `Open` state
- the bid targets the current auction number
- the request proves control of the bidding wallet
- the bid amount satisfies constraints fixed at auction start

If any precondition fails,
the bid **must** be rejected explicitly.

Bid submission **must not**:
- affect auction timing
- alter authority
- imply acceptance beyond validation

---

## 7. Bid Validation (Normative)

Bid validation is evaluated against auction state
**as fixed at auction start**.

A bid is invalid if:

- submitted outside the active auction window
- malformed or incomplete
- below the minimum valid bid
- exceeds a configured maximum (if applicable)
- not provably authorized

The minimum opening bid:

- is defined by configuration
- must not change during the auction

The minimum increment:

- is fixed at auction open
- must not change during the auction

Minimum constraints must not change
in response to other bids.

Invalid bids:

- are rejected explicitly
- do not alter auction state
- do not affect timing or resolution
- do not consume authority

Bid acceptance:

- does **not** imply winning
- does **not** imply settlement
- does **not** imply inscription

---

## 8. Responses (Normative)

All responses use a strict envelope:

- `status`: `"success"` or `"error"`
- `data`: present **only** on success
- `error`: present **only** on error

Silent failure is forbidden.

Error responses **must** include:

- error class (as defined in ERROR-TAXONOMY.md)
- machine-stable error code
- human-readable message

Error codes:

- **must not** change meaning within a version
- **must not** be reused across error classes

The API **must not** retry actions on behalf of clients.

---

## 9. Versioning Rules (Normative)

- the API is versioned from first release
- breaking changes require a new version
- field meaning **must never** change within a version
- older versions **must not** silently change semantics

Deprecation:

- must be explicit
- must not alter historical responses

---

## 10. Security and Isolation (Normative)

The API **must not**:

- expose private keys
- expose persistence internals
- expose operator controls
- allow clients to influence timing, sequencing, or authority

Clients observe.
They do not steer.

---

## 11. Non-Goals

The API does not:

- provide analytics
- infer ownership or intent
- expose internal logs
- explain outcomes
- optimize for convenience

---

## 12. Final Rule

The API exposes **facts**.

Interpretation happens elsewhere.
Authority lives elsewhere.
