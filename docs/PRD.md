# Product Requirements Document (PRD) --- Numbers

## 1. Product Overview

**Numbers** is an experimental auction system that inscribes sequential
integers (1, 2, 3, ...) onto individual satoshis on Bitcoin using the
Ordinals protocol.\
It explores digital ownership, permanence, and value by creating a new
asset class: the number itself.

Key pillars: 1. **Auction**: Live, timed bidding for each number. 2.
**Inscribe**: Immutable ownership on Bitcoin. 3. **Minimal Art**: The
number is rendered in each user's default font. 4. **System**:
Sequential, one number at a time. No skipping.

------------------------------------------------------------------------

## 2. Problem Statement

Today's digital ownership often feels abstract: tokens, coins, and
assets with complex rules. *Numbers* reduces ownership to its simplest
form---claiming a number.\
By combining: - Bitcoin's permanence and security, - Ordinals'
satoshi-level tracking, - Auctions that build community around symbolic,
personal numbers,

we create an **accessible, cultural, and conceptual system of value**.

------------------------------------------------------------------------

## 3. Goals & Non-Goals

### Goals

-   Enable sequential number auctions starting from 1.
-   Bind each number immutably to a satoshi via inscription.
-   Create a lightweight, modular architecture (Wallet, Auction,
    Inscribe, Payment, Settle, Website).
-   Ensure transparent, public record of ownership (JSON → SQLite →
    persistent DB).
-   Design minimal conceptual artwork (numbers in default system font).

### Non-Goals

-   Not a token project (no ERC-20 style abstraction).
-   Not about rare satoshis or rarity markets.
-   Not a trading platform beyond auctions.
-   Not initially focused on scalability beyond a few concurrent users
    (early stages).

------------------------------------------------------------------------

## 4. Users & Audience

**Primary User**:\
The *Collector of the Immutable*: values symbolism, permanence, and the
cultural weight of numbers (birthdays, jersey numbers, lucky digits).

**Secondary Users**:\
- Crypto enthusiasts experimenting with Ordinals.\
- Artists, philosophers, or collectors drawn to minimal digital art.\
- Developers interested in extending auction and inscription
infrastructure.

------------------------------------------------------------------------

## 5. Product Features

### Core Features

1.  **Auction**
    -   Real-time countdown (e.g., 12,345 seconds).\
    -   Sequential auctions: 1 → 2 → 3...\
    -   Bidding via connected Bitcoin wallet.
2.  **Wallet Integration**
    -   MVP: simulated wallet via username.\
    -   Testnet: RPC to Bitcoin Core.\
    -   Mainnet: secure wallet connection and funding.
3.  **Inscribe**
    -   Bind number → satpoint → transaction ID.\
    -   Track forever via Ordinals.
4.  **Payment & Settlement**
    -   Simulated in MVP.\
    -   Testnet BTC transactions in Alpha.\
    -   Mainnet BTC confirmations in v1.0.
5.  **Index & Record**
    -   MVP: flat JSON file.\
    -   Beta: SQLite database.\
    -   Launch: production-grade DB (SQLite/Postgres).
6.  **Website UI**
    -   Minimal interface for live auctions, history, and results.\
    -   Numbers displayed in user's default system font.

------------------------------------------------------------------------

## 6. Technical Requirements

-   **Language**: Rust (v1.67+)\
-   **Blockchain**: Bitcoin Core (Testnet → Mainnet)\
-   **Storage**:
    -   Testnet blockchain: \~500 GB\
    -   Index: JSON → SQLite → Postgres\
-   **Infrastructure**:
    -   Server hosting for Bitcoin node + RPC\
    -   Monitoring, logging, error recovery\
-   **Dependencies**:
    -   `bitcoincore-rpc` for blockchain interaction\
    -   Open-source libraries for auctions, logging, persistence

------------------------------------------------------------------------

## 7. Roadmap

**MVP (v0.1.0)** --- Simulated CLI demo\
- Bidding via stdin\
- Mock wallet + mock inscription\
- Store results in JSON

**MVP+ (v0.1.1)** --- Testnet integration\
- Real Bitcoin Core RPC\
- Real Testnet wallets + inscriptions

**Alpha (v0.2.x)** --- User testing\
- Web UI + Testnet auctions\
- Simulated payments, mock settlement

**Beta (v0.3.x)** --- Scale & performance\
- Multi-user concurrent bidding (5--10 users)\
- Sub-second latency feedback\
- SQLite database

**Launch (v1.0.0)** --- Mainnet release\
- Secure wallet connection + BTC payments\
- Live web auctions with real inscriptions\
- Full persistence, monitoring, onboarding

------------------------------------------------------------------------

## 8. Success Metrics

-   **Functional**:
    -   Auctions proceed sequentially without error.\
    -   Numbers inscribed immutably on satoshis.
-   **Adoption**:
    -   Testnet: ≥50 participants.\
    -   Mainnet: successful auctions of first 100 numbers.
-   **Community**:
    -   Engagement in live auction events.\
    -   Repeat bidders.

------------------------------------------------------------------------

## 9. Risks & Mitigations

-   **Blockchain sync/storage** --- Requires large storage.\
    *Mitigation*: external SSD + pruning strategy.\
-   **Scalability** --- Auctions may lag under load.\
    *Mitigation*: staged rollout, stress testing in Beta.\
-   **Legal/Regulatory** --- Bitcoin auction system might face
    regulatory attention.\
    *Mitigation*: open-source, minimal handling of user funds beyond
    payments.\
-   **User onboarding friction** --- Non-Ordinals users may struggle.\
    *Mitigation*: clear guides + Testnet practice flow.

------------------------------------------------------------------------

## 10. Future Opportunities

-   Expand beyond integers (fractions, sequences).\
-   Sub-number "points" (community extensions).\
-   AR/VR experiences tied to owned numbers.\
-   Treasury: auction revenue funds new projects.
