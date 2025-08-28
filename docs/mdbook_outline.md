# Numbers mdBook — Proposed Outline

This outline suggests how to organize existing and future `.md` docs into an mdBook for the Numbers project.

---

## 1. Introduction
- What is Numbers?
- Why Numbers? (import from `WHY.md`)
- Project Philosophy (minimalism, permanence, universality)

## 2. Getting Started
- System Requirements
- Setup & Run (import from `README.md`)
- Directory Structure

## 3. Architecture
- System Map (birds-eye)
- Modules Overview: Wallet, Auction, Inscribe, Payment, Settle, Website
- Data Flow (auction to inscription)
- Rendering Models (from `Rendering-Map.md`)

## 4. Auction Engine
- MVP: CLI simulation
- Countdown Timers
- Bidding Loop
- Settlement Flow

## 5. Inscriptions
- Bitcoin + Ordinals background
- Testnet inscription index (JSON → SQLite)
- Mainnet inscription strategy
- Design choice: rarity not a factor

## 6. UI / UX
- Typing Effect (v6, v7 evolution)
- Crossfade digits (auction countdown)
- Voice-like Typist (slogans)
- Motion notes and accessibility (prefers-reduced-motion)

## 7. Infrastructure
- Bitcoin Core Testnet setup
- RPC integration
- Database scaffolding
- CI/CD (Rust tooling, GitHub Actions)

## 8. Roadmap
- MVP → Alpha → Beta → Launch (import from `README.md`)
- Future features
- Community contributions

## 9. Appendices
- Glossary (DOM, RPC, Satpoint, Inscription, etc.)
- Resources (Rust docs, Bitcoin Testnet, Ordinals references)
- Design sketches and diagrams (optional SVG/PNG embeds)

---

**Note:** Each section will grow as you refine existing `.md` docs in `Docs/`. mdBook will provide navigation and search across all of them.

