# Numbers Alpha Prototype Plan
**Aug 24 – Aug 31, 2025 (8 days × 8 hrs = ~64 hrs)**  
**Target:** Alpha v0.2.x checkpoint (roadmap: Testnet wallets + basic web UI)  
**Approach:** Hell-bent focus, one major milestone per day.

---

## Sun Aug 24 (8 hrs)
- **Penpot wrap-up (5 hrs)**  
  - Complete **State-0.1 → State-3** (add State-4 only if trivial).  
  - Export boards/screens.  
  - Update `docs/DESIGN_NOTES.md`.  
- **Repo organization (3 hrs)**  
  - Open “Alpha” milestone in GitHub Issues.  
  - Commit updated Penpot docs.  

---

## Mon Aug 25 (8 hrs)
- **Safety Net I**  
  - Install/configure `rustfmt`, `clippy`.  
  - Run `cargo fmt`, `cargo clippy --fix`.  
  - Add `ci.yml` (fmt, clippy, test) → debug until CI green.  
  - Document in `docs/SAFETY_NET.md`.  

---

## Tue Aug 26 (8 hrs)
- **Safety Net II**  
  - Run `cargo doc`.  
  - Initialize `mdBook` in `docs/`.  
  - Write starter `PROJECT_BRIEF.md`.  
  - Hook `mdBook build` into CI.  
  - Push “Safety Net baseline” commit.  

---

## Wed Aug 27 (8 hrs)
- **Frontend shell**  
  - Scaffold Vite + Tailwind + TypeScript.  
  - Add safe-area container + global tokens (colors, typography).  
  - Render placeholder Header + AuctionCard + InputArea.  

---

## Thu Aug 28 (8 hrs)
- **Auction mock state**  
  - JSON object: number, countdown, bids.  
  - Wire InputArea (BidEntry + Keyboard).  
  - Show static 60-second countdown timer.  

---

## Fri Aug 29 (8 hrs)
- **Interactive bid flow**  
  - “Place Bid” updates mock state + UI.  
  - Show highest bid.  
  - Add micro-animations (orange button flash, fade transitions).  

---

## Sat Aug 30 (8 hrs)
- **Bitcoin Testnet integration I**  
  - Hook into Bitcoin Core RPC (`bitcoincore-rpc`).  
  - List wallets + fetch Testnet address.  
  - Replace mock wallet with real Testnet wallet.  
  - Log bids to `results/inscription_index.json`.  

---

## Sun Aug 31 (8 hrs)
- **Bitcoin Testnet integration II + Demo**  
  - Drive countdown from state.  
  - Auto-close auction after 60s.  
  - Pick winner + log inscription result (mock txid).  
  - Cross-device test (Safari iOS, Chrome Android).  
  - Optimize build (Tailwind purge, bundle).  
  - Internal Alpha demo: start auction → bid → close → log.  

---

## ✅ Deliverables by Aug 31
- **Design:** Penpot States 0–3 documented.  
- **Safety Net:** fmt, clippy, CI, mdBook, project brief.  
- **Frontend:** Shell running with InputArea, countdown, bid flow.  
- **Backend:** Testnet wallet integration, JSON logging.  
- **Alpha Demo:** End-to-end flow, demo-ready.  

---

# Compact Calendar (Aug 24–31)

```
Sun 24 | Penpot finish → Repo org
Mon 25 | Safety Net I (fmt, clippy, CI)
Tue 26 | Safety Net II (docs, mdBook)
Wed 27 | Frontend shell (Vite, Tailwind)
Thu 28 | Auction mock state (JSON, countdown)
Fri 29 | Interactive bid flow (Place Bid, animations)
Sat 30 | Testnet integration I (RPC, wallet, log)
Sun 31 | Testnet integration II + Alpha Demo
```
