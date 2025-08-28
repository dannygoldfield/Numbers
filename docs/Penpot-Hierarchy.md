
# Numbers

## Project Structure


## UI Hierarchy

# Numbers — Penpot UI Component Hierarchy

> Source of truth for how our Penpot components nest, align, and size.
> Keep this synced with the Local Library (Components, Colors, Typographies).

---

## 1) Top‑Level Boards

- **State‑0.1** (mobile, 358 safe width)
  - Guides: 2 px side gutters
  - Preview entry point for the InputArea

> Future: add State‑0.2 (disabled/empty), State‑0.3 (inscribing), State‑0.4 (error)

---

## 2) Component Tree (text)

### InputArea (Component)
Container for the bidding bar and on‑screen numeric keyboard.

- **Layout (Flex)**
  - Direction: Column (↓)
  - Gap: `8`
  - Padding: `L 2` / `R 2` (safe gutters)
  - Width: 358 (safe), Height: Hug

- **Children (in order)**
  1) **BidEntry (Instance)**
     - Layout: Row (→), Gap `4`, Align center (vertical)
     - Children:
       - **BidAmount (Instance)**
         - Layout: Row (→), Gap `6`, Padding `L 8`
         - Width `248`, Height `40`, Radius `6`, Fill `#d9d9d9`
         - Children:
           - **BitcoinSymbol (Instance)** — fixed `16 × 16`
           - **Placeholder Text** — Hug content (e.g., “Enter bid in sats”)
       - **BidButton (Instance)**
         - Fixed width `102`, Height `40`
         - Label centered (“Place Bid”)
         - Alignment rule: **left edge aligns with “8” key column**
  2) **Keyboard (Instance)**
     - Layout: Column (↓)
     - Children:
       - **Row‑1 (Instance)**
         - Layout: Row (→), Gap `6`
         - **Key (Instance)** ×10 (digits `1–0`)
           - Each Key: Width `30`, Height `40`, Radius `6`
       - **Row‑2 (Instance)**
         - Layout: Row (→), Gap `6`, Align center
         - **Key (Instance)** ×2 (`.` and `<`)

---

## 3) ASCII Map (visual)

```
InputArea (Component)            [Flex: Column, Gap=8, Pad L/R=2]
│
├─ BidEntry (Instance)           [Flex: Row, Gap=4, Align center]
│  │
│  ├─ BidAmount (Instance)       [Flex: Row, Gap=6, Pad L=8, W=248, H=40, R=6, Fill=#d9d9d9]
│  │  ├─ BitcoinSymbol (Instance)  [W=16, H=16]
│  │  └─ Placeholder Text          ["Enter bid in sats"]
│  │
│  └─ BidButton (Instance)       [W=102, H=40]  ← left edge aligns with “8” key column
│
└─ Keyboard (Instance)           [Flex: Column]
   │
   ├─ Row‑1 (Instance)           [Flex: Row, Gap=6]
   │  └─ Key (Instance) ×10      [W=30, H=40] → digits 1 2 3 4 5 6 7 8 9 0
   │
   └─ Row‑2 (Instance)           [Flex: Row, Gap=6, Align center]
      └─ Key (Instance) ×2       [W=30, H=40] → "."  "<"
```

---

## 4) Shared Styles (Tokens)

- **Colors**
  - `#070707` background
  - `#333333` key/button surface
  - `#d9d9d9` input fill
  - `#f7931a` Bitcoin accent
  - `#ffffff` text

- **Typographies**
  - Heading, Subheading, Body, Label (currently Inter as placeholder)
  - Future: switch to platform default stacks (SF/Roboto/Segoe) in code; keep Penpot tokens mapped.

---

## 5) Component Rules (Penpot‑specific)

- Only drag **Instances** of Components into other Components (no Main-in-Main).
- Use **Flex Layout** everywhere possible; prefer **Hug** for text where appropriate.
- Keep **fixed widths** where alignment matters:
  - `BidAmount = 248`, `BidButton = 102`, `Key = 30`, Row‑1 Gap `6`.
- Alignment contract: **BidButton left edge ↔ the “8” key’s left edge**.

---

## 6) Planned Sections (stubs)

These headings are placeholders for upcoming work. Add entries as you design:

- **BidButton States**: default / hover / pressed / disabled
- **Error Microcopy & Validation**: invalid chars, min/max bid, network failure
- **Keyboard Variants**: compact, expanded, locale edge cases
- **Accessibility**: focus order, large text, color contrast checks
- **Auction UI**: lot info, timer, current bid, outbid state
- **Wallet / Settle / Inscribe Flows**: stepper, confirmations, progress
- **Empty / Loading / Success States**: screen boards (State‑0.2+)
- **Responsive**: tablet breakpoints, landscape constraints

---

## 7) Maintenance

- When you **update a Main Component**, verify all Instances on boards.
- If an Instance looks wrong, **Reset overrides** first; then re-apply intended overrides.
- Keep this document in sync with Penpot’s **Local Library → Components / Colors / Typographies**.

