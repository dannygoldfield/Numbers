# Inscription Format — Numbers

This document defines the canonical inscription content bytes for Numbers.

It is normative.

This document defines only the inscription content payload.
Transaction construction and chain interaction are defined elsewhere.

---

## 1. Canonical Content

For canonical number `N`, the inscription content payload must be:

- The UTF-8 encoding of the base-10 ASCII digits of `N`
- With no leading zeros (except `N = 0`)
- With no trailing newline
- With no surrounding whitespace
- With no additional metadata

Examples:

- N = 0  → payload bytes encode the string "0"
- N = 7  → payload bytes encode the string "7"
- N = 37 → payload bytes encode the string "37"
- N = 100 → payload bytes encode the string "100"

---

## 2. Content Type

The inscription content type must be:

- `text/plain; charset=utf-8`

No other content type is permitted for canonical Numbers inscriptions.

---

## 3. Payload Hash

`payload_hash` used by the inscription machine must be:

- SHA-256 over the exact payload bytes defined in Section 1.

`payload_hash` must be computed deterministically from `N` only.

---

## 4. Size Limits

The payload must be smaller than or equal to:

- 32 bytes

If `N` would produce a payload larger than 32 bytes, execution must halt.

---

## 5. Display and Rendering (Non-Normative)

The system may render the payload text in any font or style.
Rendering has no effect on canonical on-chain payload bytes.

---

## 6. Final Rule

If the payload bytes differ from the rules above, the inscription is non-canonical.
No non-canonical payload may be used for a Numbers inscription.