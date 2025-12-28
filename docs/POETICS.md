# Poetics

These poems are not requirements.
They are recorded thoughts, partial models, and conceptual probes.

They may influence specifications.
They may also be discarded without consequence.

Nothing here is enforced.

---

### Poem 1: Bitcoin as Memory, Not Mind  

Bitcoin behaves like a memory device.
It remembers events exactly.
It does not remember reasons.

If something is recorded, it is remembered forever.
If something is not recorded, it does not exist.

Bitcoin does not distinguish between accident, intention, or meaning.
That distinction happens elsewhere.

Influence:
- WHAT-IF.md (conceptual framing)
- Threat Model (outcome layer language)

---

### Poem 2: Outcome vs Control  

An outcome can be final even if control is temporary.

Final:
- this inscription exists
- it was produced at this moment
- it was routed to this destination

Not final:
- who controls it later
- what it is worth later
- whether anyone cares later

The system fixes outcomes, not futures.

Influence:
- ARCHITECTURE.md (finalization vs transfer)
- THREAT-MODEL.md (immutability clarification)

---

### Poem 3: Null as a Valid Result  

Failure to settle is not an error.
No bid is not an exception.

Routing to an unspendable destination is still a resolution.
The sequence advances either way.

Absence is treated as a legitimate outcome, not a bug to patch.

Influence:
- CORE-SEQUENCE.md
- PRD.md (nonpayment invariants)
- Null Steward definition

---

### Poem 4: Scale Does Not Change Meaning  

One unspendable output and one million unspendable outputs
are identical to Bitcoin.

There is no threshold where absence becomes presence.
There is no point where burned stops meaning burned.

Any sense of accumulation exists only outside the protocol.

Influence:
- WHAT-IF.md (absence and meaning boundary)
- Rejected second-blockchain ideas

---

### Poem 5: Recognized Without Uniqueness  

Many inscriptions may contain the same number.
Only one is recognized by this system as its outcome.

Recognition is procedural, not metaphysical.
It answers “which one do we point to,” not “which one is real.”

Influence:
- ARCHITECTURE.md (authenticity rule)
- API-SPEC.md (recognized outcomes only)
- UI-SPEC.md (display policy)

---

### Poem 6: Sequence as Constraint  

The sequence never pauses to reconsider.
It does not optimize.
It does not wait for good outcomes.

What happens next is not improved by what happened before.
It simply follows.

The constraint is intentional.

Influence:
- CORE-SEQUENCE.md (monotonic advance)
- PRD.md (no retries, no rewinds)

---

### Poem 7: Where Meaning Is Allowed to Live  

Meaning is allowed to exist only where it cannot be enforced.

If meaning requires:
- rules
- metadata
- UI emphasis
- special cases

then it no longer belongs to the system.

The system records.
People interpret.

Influence:
- UI-SPEC.md (restraint rules)
- Decision to create POETICS.md itself
