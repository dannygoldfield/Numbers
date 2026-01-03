# Naming Conventions — Numbers

This document defines naming conventions used throughout the Numbers specification, code, and data.

It is normative.

If a name violates these rules, it must be corrected.

---

## 1. Purpose

The purpose of these conventions is to:

- Eliminate ambiguity between domain concepts and technical primitives
- Prevent semantic drift across documents and implementations
- Improve readability for humans and LLMs
- Ensure consistent naming across text, JSON, code, and logs

---

## 2. Core Principle

Invented, project-specific concepts are ProperNouns.  
Generic system concepts remain descriptive phrases.

A name should signal whether a concept:
- exists only within Numbers, or
- is a general technical or domain concept.

---

## 3. ProperNouns (Single-Token Names)

### Definition

A ProperNoun is:
- Invented for the Numbers project
- Meaningless or ambiguous outside this system
- Referencing a specific role, construct, or artifact

### Rules

- Use PascalCase
- Use a single token
- Do not alias
- Do not pluralize
- Do not rephrase

### Examples

- Numbers
- NullSteward
- NumbersCatalog

### Usage

- In prose: Capitalized, exact spelling
- In JSON: As string literals
- In code: As identifiers or enum values
- In logs: Verbatim

---

## 4. Descriptive Phrases (Multi-Word Names)

### Definition

Descriptive phrases are:
- Composed of standard domain or technical terms
- Meaningful outside the Numbers system
- Used to describe behavior, process, or structure

### Rules

- Use standard English
- Use lowercase unless sentence-initial
- Do not collapse into tokens

### Examples

- auction
- bid
- settlement
- inscription
- canonical inscription
- inter-auction gap
- state machine
- pause state

---

## 5. States and Enums

### Auction States

- Use PascalCase
- Single-token
- Defined exclusively in STATE-MACHINE.md

Examples:
- Scheduled
- Open
- Closed
- Finalized
- Inscribed

### Sub-State Domains

Sub-states in distinct domains may reuse words with different casing.

Example:
- Auction state: Inscribed
- Inscription state: inscribed

This is intentional and not an error.

---

## 6. JSON Field Naming

- Field names: snake_case
- Values:
  - ProperNouns use exact casing
  - States use defined enums
  - Absence uses null, not omission

Example:

```json
{
  "destination": "NullSteward"
}
```

## 7. Forbidden Patterns

The following are forbidden:

- Using `null` (the value) to represent a destination or role
- Creating aliases for ProperNouns
- Mixing tokenized and descriptive forms
- Renaming concepts post-freeze

Examples of forbidden names:

- `null_steward`
- `burn_address`
- `default_catalog`
- `NumbersCatalog` (when referring to the ProperNoun)

---

## 8. Change Control

- Naming decisions are frozen before mainnet
- Any rename requires:
  - Mechanical replacement
  - No semantic change
  - Single canonical form

---

## 9. Final Rule

If there is uncertainty about how to name something:

- Prefer clarity over cleverness
- Prefer explicit naming over shorthand
- Prefer stability over aesthetics
