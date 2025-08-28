# Rendering Models — Reference Map

This document outlines the main rendering models available in web development and their relevance for **Numbers**.

---

## 1. Comparison Table

| Model | What it is | How you draw | Accessibility (text/selectable) | Performance profile | Best at | Weak at |
|---|---|---|---|---|---|---|
| **DOM (HTML + CSS)** | Live tree of elements | Change element props/styles | **Yes** (real text) | Great for UI; fine for light animation | Layouts, typography, responsive UI | Many tiny updates each frame can be costly |
| **CSS Animations/Transitions** | Declarative animation in CSS | `transition`, `@keyframes` | Yes | GPU-friendly for transform/opacity | Simple, low‑code motion | Complex sequencing/logic |
| **WAAPI (Web Animations API)** | JS control over animations | `element.animate(...)` | Yes | Same perf as CSS anims | Timelines you can play/pause/scrub | Requires JS orchestration |
| **SVG** | Vector scene graph in DOM | Create/modify `<path>`, etc. | **Yes** | Good for icons/diagrams | Crisp vector art, charts, iconography | Very dense scenes/particles |
| **Canvas 2D** | Immediate‑mode pixel surface | `ctx.fillText`, `ctx.draw…` | **No** (text becomes pixels) | High for batched draws | Particles, image filters, generative sketches | Native text semantics/layout |
| **WebGL / WebGPU** | GPU pipelines (3D/2D) | Shaders, buffers | No | Highest for complex visuals | Massive effects, thousands of objects | Setup complexity |
| **p5.js (over Canvas/WebGL)** | Creative‑coding wrapper | `setup()/draw()` loop | No (unless mixing DOM mode) | Good; depends on content | Rapid visual prototyping, generative art | App‑level UI, semantics |

---

## 2. Decision Flow

- **Need selectable, semantic text or UI layout?** → **DOM**  
  - Simple motion → **CSS**  
  - Timeline control/sync → **WAAPI**
- **Need lots of pixels/particles or custom drawing?** → **Canvas 2D** (or **p5.js** for speed of iteration)  
- **Heavy shader‑based visuals/3D?** → **WebGL/WebGPU**  
- **Icons/diagrams that must stay razor‑sharp and accessible?** → **SVG**

---

## 3. Numbers‑Specific Uses

- **v6 digit crossfade** → **DOM + CSS**
- **v7 voice‑like typist** → **DOM text + JS scheduler** (optional WAAPI for modulation)
- **Countdown badges, micro‑motion** → **CSS**
- **Auction event sync (tempo ramps, pause/resume)** → **WAAPI**
- **Settlement interludes (10‑min generative worlds)** → **p5.js / Canvas 2D** first, **WebGL** if scaling up
- **Static diagrams (system map, flow)** → **SVG**

---

## 4. Cheat Codes

- DOM text animates best with **transform/opacity** (avoid animating `top/left/width`).
- Respect **`prefers-reduced-motion`** in both CSS and JS.
- Canvas text alignment: use `measureText()` and snap positions to integers to reduce shimmer.
- Keep one **global clock/event bus** to sync typing, crossfades, and countdown.

---

