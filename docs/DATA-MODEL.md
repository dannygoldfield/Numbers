# Data Model

This document defines the canonical data stored by Numbers.

Numbers favors append-only records.

---

## Canonical Records

- Auction
- Bid
- Settlement
- Inscription
- Pause Event

These records represent canonical system state, not arbitrary on-chain data.

Each record is immutable once written.

---

## Auction

Fields include:
- Auction ID
- Number
- Start time
- End time
- Cap
- Outcome

---

## Bid

Fields include:
- Bid ID
- Auction ID
- Amount
- Timestamp
- Status

---

## Settlement

Fields include:
- Auction ID
- Winning bid (or zero)
- TXID
- Status

---

## Inscription

Fields include:
- Inscription ID
- Auction ID
- Satpoint
- TXID
- Status
- Canonical flag
- Authenticity method

---

## Source of Truth

- Bitcoin is the final authority
- Database records reflect system intent and observed outcomes
- History is never rewritten

---

## Principle

Derived views may change.
Recorded events do not.
