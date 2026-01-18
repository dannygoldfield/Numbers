# Application Programming Interface Specification — Numbers

This document defines the **entire external API surface** exposed by the Numbers backend.

It is **normative**.

This document assumes familiarity with:
- INVARIANTS.md
- STATE-MACHINE.md
- ERROR-TAXONOMY.md

If there is a conflict,
PRD.md, CORE-SEQUENCE.md, STATE-MACHINE-TABLE.md, STATE-MACHINE.md,
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

- Expose canonical system state
- Accept explicitly permitted external actions
- Surface knowledge without interpretation

The API does **not**:
- infer outcomes
- summarize meaning
- provide convenience abstractions
- grant authority

---

## 2. Design Goals (Normative)

The API is:

- Read-only by default
- Minimal and enumerable
- Deterministic in shape
- Explicit in failure
- Versioned from inception
- Aligned strictly with persisted canonical state

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

Absence of data **must not** be represented as resolution.

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
- The API exposes recorded backend state only.

API availability, delay, restart, or failure
**must not** alter outcomes or authority.

---

## 5. API Shape Rules (Normative)

All endpoints **must** obey:

- Strict input validation
- Deterministic output shapes
- Missing fields represent unknown knowledge, not null outcomes
- Field meaning **must not** change within a version
- Side effects are forbidden unless explicitly stated

---

## 6. Public Endpoints (Normative)

Endpoints listed here define the **entire permitted surface**.
No undocumented endpoint may exist.

---

### GET /state

Returns the current global system state.

Includes:
- current auction number
- auction lifecycle state
- system control state (`Running` or `Paused`)
- remaining time **if directly observed**
- inter-auction gap status **if directly observed**

This endpoint exposes **present knowledge only**.

It **must not**:
- predict future transitions
- compute or project timing
- infer upcoming state changes
- interpolate missing time data

---

### GET /auction/history

Returns finalized auction outcomes in strict sequence order.

Rules:
- Results are paginated
- Each entry corresponds to **exactly one** auction number
- Only finalized auctions are returned
- Ordering is canonical and immutable

Each entry includes only finalized knowledge:
- auction number
- final destination address
- settlement outcome
- inscription state
- inscription txid (if known)
- inscription satpoint (if known)
- timestamps of resolution and finalization

This endpoint **must not**:
- expose non-finalized state
- infer ownership
- collapse null outcomes
- hide NullSteward results
- rewrite or reinterpret history

---

### GET /auction/{N}

Returns the full recorded state for auction number `N`.

Includes:
- auction lifecycle state
- bid summary (only if permitted by state)
- resolution record (if present)
- settlement state
- inscription state
- ambiguity indicators

If knowledge is unavailable,
it **must** be omitted, not guessed.

---

### POST /bid

Submits a bid for the **currently open auction only**.

This is the **only write-capable public endpoint**.

#### Preconditions (Normative)

A bid request is valid **only if all are true**:

- System state is `Running`
- An auction is in `Open` state
- The bid targets the current auction number
- The request proves control of the bidding wallet
- The bid amount satisfies constraints fixed at auction start

If any precondition fails,
the bid **must** be rejected.

---

## 7. Bid Validation (Normative)

Bid validation is evaluated against auction state
**as fixed at auction start**.

A bid is invalid if:

- submitted outside the active auction window
- malformed or incomplete
- below the minimum valid bid
- exceeds configured maximum (if set)
- not provably authorized

The minimum valid bid:
- is computed once at auction start
- remains fixed for the auction duration
- does not change in response to other bids

Invalid bids:
- are rejected explicitly
- do not alter auction state
- do not affect timing or resolution
- do not consume authority

Bid acceptance:
- does **not** imply winning
- does **not** imply settlement
- does **not** imply future inscription

---

## 8. Responses (Normative)

All responses use a strict envelope:

- `status`: `"success"` or `"error"`
- `data`: present only on success
- `error`: present only on error

Silent failure is forbidden.

Error responses **must** include:
- error class (ERROR-TAXONOMY.md)
- human-readable message
- machine-stable error code

Error codes:
- **must not** change meaning within a version
- **must not** be reused for different failure classes

The API **must not** retry actions on behalf of clients.

---

## 9. Versioning Rules (Normative)

- API is versioned from first release
- Breaking changes require a new version
- Field meaning **must never** change within a version
- Older versions **must not** silently change semantics

Deprecation:
- must be explicit
- must not alter historical responses

---

## 10. Security and Isolation (Normative)

The API **must not**:

- expose private keys
- expose persistence internals
- expose operator controls
- allow clients to influence timing or authority

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
