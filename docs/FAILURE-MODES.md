# Failure Modes — Numbers

This document enumerates **known and permitted failure modes** of the Numbers system.

It is **normative**.

Failure modes are **not bugs**.
They are expected consequences of the design.

They are outcomes the system **explicitly allows**
to preserve correctness, finality, and authority boundaries.

---

## Modal Language Rule (Normative)

In this document and all normative specifications:

- **must / must not** define obligations
- **only / exactly once / at most once** define bounds
- **may** is permitted **only** to describe uncertainty of knowledge,
  never permission, intent, or action

The following terms are forbidden in normative contexts:

- possibly
- likely
- eventually
- for now
- TBD

Any normative statement using forbidden modal language is invalid.

---

## Interpretation Boundary

Failure modes describe outcomes that are **valid under system rules**.

They do **not** imply:

- refunds
- retries
- bugs
- operator fault
- future correction

Failure modes describe what occurs when the system
**refuses to invent certainty or re-exercise authority**.

---

## Failure Modes

### 1. No Bids

**Condition**  
An auction closes with zero valid bids.

**Why this is allowed**  
Participation is optional. Absence of bids is valid information.

**System behavior**
- Finalizes the auction
- Assigns destination to `NullSteward`
- Advances the sequence

**System refusal**
- Must not reopen bidding
- Must not extend auction duration
- Must not re-auction the number

---

### 2. Winning Bidder Fails to Settle

**Condition**  
A winning bidder does not complete settlement before the deadline.

**Why this is allowed**  
Settlement is an obligation of the bidder, not a guarantee of the system.

**System behavior**
- Finalizes destination as `NullSteward`
- Advances the sequence

**System refusal**
- Must not accept late payment
- Must not retry settlement
- Must not transfer or restore authority

---

### 3. Settlement Completes After Sequence Advances

**Condition**  
Settlement confirmation is observed after the next auction has begun.

**Why this is allowed**  
Settlement does not gate sequence progression.

**System behavior**
- Accepts settlement **only if** it occurred within the recorded deadline
- Finalizes the auction independently of subsequent auctions

**System refusal**
- Must not pause or rewind the sequence
- Must not reorder auctions
- Must not reinterpret earlier transitions

---

### 4. Inscription Broadcast Uncertainty

**Condition**  
The outcome of an inscription broadcast cannot be determined with certainty.

**Why this is allowed**  
Distributed systems cannot guarantee broadcast observability.

**System behavior**
- Persists an ambiguity record
- Permanently freezes inscription authority
- Restricts activity to observation only

**System refusal**
- Must not retry inscription
- Must not broadcast a replacement transaction
- Must not infer or guess outcome

---

### 5. Competing Lookalike Inscriptions Exist

**Condition**  
Other inscriptions containing the same number appear on-chain.

**Why this is allowed**  
Bitcoin permits arbitrary inscriptions by any actor.

**System behavior**
- Recognizes only its own procedurally-derived inscription
- Ignores all external inscriptions

**System refusal**
- Must not assert exclusivity beyond its own records
- Must not police the chain
- Must not interpret ownership of external inscriptions

---

### 6. Indexer or Observer Disagreement

**Condition**  
Different observers report conflicting views of chain state.

**Why this is allowed**  
Observation is external and non-authoritative.

**System behavior**
- Treats the condition as degraded or ambiguous
- Freezes authority where required

**System refusal**
- Must not select a preferred narrative
- Must not advance based on partial certainty

---

### 7. Operator Error Detected Before Authority Exercise

**Condition**  
A misconfiguration or operator mistake is detected before authority is exercised.

**Why this is allowed**  
Human interaction with the system is expected.

**System behavior**
- Halts or pauses safely
- Prevents authority exercise

**System refusal**
- Must not proceed optimistically
- Must not patch behavior mid-auction

---

### 8. Operator Error Detected After Authority Exercise

**Condition**  
An operator mistake is discovered after settlement or inscription authority is exercised.

**Why this is allowed**  
Authority is irreversible by design.

**System behavior**
- Records the outcome
- Continues the sequence

**System refusal**
- Must not undo actions
- Must not compensate through protocol behavior
- Must not reassign outcomes

---

### 9. Extended Network Degradation

**Condition**  
Bitcoin network conditions prevent timely observation or confirmation.

**Why this is allowed**  
The system operates atop an external network.

**System behavior**
- Enters degraded or paused state
- Waits without inference

**System refusal**
- Must not invent certainty
- Must not modify rules to compensate

---

### 10. Permanent Ambiguity

**Condition**  
An inscription outcome never becomes knowable.

**Why this is allowed**  
Not all uncertainty resolves.

**System behavior**
- Preserves ambiguity permanently
- Advances the sequence independently

**System refusal**
- Must not declare success or failure
- Must not retry inscription
- Must not substitute outcomes

---

## Final Rule

Failure modes are **accepted**, not corrected.

If avoiding a failure mode would require:

- retrying authority
- inventing certainty
- revising history

then the failure mode **must be allowed**.

The system prefers **visible loss** over **silent fabrication**.
