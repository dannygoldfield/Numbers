# Language-First Software Development Model

Status: Public  
Purpose: Describe the method used to design and implement software systems like Numbers.

---

# Core Idea

Language defines behavior.  
Code executes behavior.

If behavior changes, language changes first.

---

# The Four Phases

## 1. Exploration

Objective:
Understand the problem deeply before writing code.

Activities:

- Test ideas
- Surface edge cases
- Refine terminology
- Clarify system boundaries

This phase is creative and iterative.

---

## 2. Specification

Objective:
Remove ambiguity before implementation.

Activities:

- Define canonical terms
- Lock system invariants
- Define state transitions
- Specify failure behavior
- Eliminate guess-space

If something is not specified, it is forbidden.

---

## 3. Implementation

Objective:
Translate the specification into code.

Implementation should be mechanical and uneventful.

If implementation feels dramatic, the specification was underspecified.

No behavior originates in code.

---

## 4. Validation

Objective:
Ensure the implementation matches the specification.

If drift is detected, the specification is corrected first.

---

# Principle

The specification is the work.

The code is its execution.