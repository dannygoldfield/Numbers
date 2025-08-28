/* ============================================================================
   Numbers v6 — constant box + centered digit + crossfade (DOM + CSS)
   ========================================================================== */

const stage   = document.getElementById("stage");
const overlay = document.getElementById("overlay");

const cont = {
  a: document.querySelector(".layer.a"),
  b: document.querySelector(".layer.b"),
};
const glyph = {
  a: cont.a.querySelector(".glyph"),
  b: cont.b.querySelector(".glyph"),
};

const DIGITS   = ["1","2","3","4","5","6","7","8","9","0"];
const SPEED_MS = 920;          // keep ≥ CSS --fade-ms for a clean crossfade

// ---------- Tweak knobs ----------
const BOX_PAD_Y = 5;           // px added to TOP and BOTTOM of the rectangle
const BIAS_PX = -5;            // global vertical nudge: + DOWN, - UP
const PER_DIGIT_BIAS = {       // optional per-digit nudges
  // '1': -1,
  // '8':  0.5,
};
const SNAP_TO_PIXEL = true;    // reduce shimmer by rounding translateY
// --------------------------------

let i = 0;
let front = "a";

// Canvas-based metrics so we know each digit's ink ascent/descent/width
function measureDigitMetrics() {
  const cs = getComputedStyle(glyph.a);
  const font = `${cs.fontWeight} ${cs.fontSize} ${cs.fontFamily}`;

  const canvas = document.createElement("canvas");
  const ctx = canvas.getContext("2d");
  ctx.font = font;

  const metrics = {};
  let maxA = 0, maxD = 0, maxW = 0;

  for (const d of DIGITS) {
    const m = ctx.measureText(d);
    const A = m.actualBoundingBoxAscent || 0;
    const D = m.actualBoundingBoxDescent || 0;
    const W = Math.ceil(m.width);
    metrics[d] = { A, D, W, H: A + D };
    if (A > maxA) maxA = A;
    if (D > maxD) maxD = D;
    if (W > maxW) maxW = W;
  }
  return { metrics, maxA, maxD, maxW, maxH: maxA + maxD };
}

let M = null;
let BOX_H = null; // constant stage/overlay height including padding

// Set stage to a constant box (width = max glyph width; height = max glyph height + padding)
function sizeStageConstantBox() {
  M = measureDigitMetrics();
  BOX_H = Math.ceil(M.maxH + 2 * BOX_PAD_Y);
  stage.style.width  = `${Math.ceil(M.maxW)}px`;
  stage.style.height = `${BOX_H}px`;
  // #overlay fills the stage via CSS (inset: 0)
}

// Center the current glyph's ink inside the constant box
function centerGlyphVertically(layerKey, digit) {
  const cur = M.metrics[digit];

  // 1) bring ink TOP to stage TOP (based on ascent)
  const topAlign = M.maxA - cur.A;
  glyph[layerKey].style.transform = `translateY(${topAlign}px)`;

  // 2) measure actual rendered height, then add half of leftover space
  const rect = glyph[layerKey].getBoundingClientRect();
  const leftover = (BOX_H - rect.height) / 2;

  // 3) apply global/per-digit bias and optional pixel snapping
  let dy = topAlign + leftover + BIAS_PX + (PER_DIGIT_BIAS[digit] ?? 0);
  if (SNAP_TO_PIXEL) dy = Math.round(dy);

  glyph[layerKey].style.transform = `translateY(${dy}px)`;
}

function nextDigit() {
  const d = DIGITS[i];
  i = (i + 1) % DIGITS.length;
  return d;
}

function tick() {
  const back = front === "a" ? "b" : "a";
  const d = nextDigit();

  // Put next digit in back layer and center it
  glyph[back].textContent = d;
  centerGlyphVertically(back, d);

  // Crossfade layers
  cont[back].classList.remove("hidden");
  cont[back].classList.add("visible");
  cont[front].classList.remove("visible");
  cont[front].classList.add("hidden");

  front = back;
}

function startV6() {
  sizeStageConstantBox();

  // Initialize first two digits centered so first fade is clean
  const d0 = DIGITS[0], d1 = DIGITS[1];
  glyph.a.textContent = d0; centerGlyphVertically("a", d0);
  glyph.b.textContent = d1; centerGlyphVertically("b", d1);
  i = 2;

  setInterval(tick, SPEED_MS);
}

startV6();

// Recompute on resize (font-size uses vw) and recenter the current digit
let resizeTimer;
window.addEventListener("resize", () => {
  clearTimeout(resizeTimer);
  resizeTimer = setTimeout(() => {
    sizeStageConstantBox();
    const d = glyph[front].textContent;
    centerGlyphVertically(front, d);
  }, 120);
});

/* ============================================================================
   Numbers v7 — “voice-like” Typist (DOM text + scheduler)
   ========================================================================== */

class Typist {
  constructor(node, opts = {}) {
    this.node = node;
    this.opts = {
      baseMs: 70,
      jitterMs: 25,
      burstChars: [2, 6],
      interBurstMs: [60, 140],
      rampInMs: 300,
      rampOutMs: 420,
      punctHoldMs: { ',': 120, ';': 150, ':': 160, '.': 380, '!': 420, '?': 420, '—': 280 },
      thinkStallChance: 0.16,
      thinkStallMs: [500, 1200],
      eraseMs: 40,
      loop: true,
      caret: false,
      respectReducedMotion: true,
      seed: null,
      ...opts
    };

    this.lines = [];
    this.lineIndex = 0;
    this.text = '';
    this.pos = 0;
    this.rate = 1.0;
    this.timer = null;
    this.state = 'idle';
    this.events = {};
    this.rng = this.opts.seed ? this.#mulberry32(this.opts.seed) : Math.random;

    if (this.opts.caret) this.node.classList.add('typing-caret');
    if (this.opts.respectReducedMotion && matchMedia('(prefers-reduced-motion: reduce)').matches) {
      // degrade: show whole line, no animation
      this.rate = Infinity;
    }
  }

  on(evt, fn){ (this.events[evt] ||= []).push(fn); return this; }
  #emit(evt, payload){ (this.events[evt] || []).forEach(fn => fn(payload)); }

  start(lines){
    this.lines = Array.isArray(lines) ? lines.slice() : [String(lines || '')];
    this.lineIndex = 0;
    this.#showLine(0);
    return this;
  }
  pause(){ if (this.timer) cancelAnimationFrame(this.timer), this.timer = null; this.#emit('pause'); }
  resume(){ if (!this.timer) this.#tick(); this.#emit('resume'); }
  setRate(mult){ this.rate = Math.max(0.05, mult || 1); this.#emit('rate', this.rate); }
  gotoLine(i){ this.lineIndex = (i % this.lines.length + this.lines.length) % this.lines.length; this.#showLine(this.lineIndex); }

  #showLine(i){
    this.text = this.lines[i] || '';
    this.pos = 0;
    this.node.textContent = '';
    this.state = 'typing';
    this.startTime = performance.now();
    this._didThink = false;
    this.#emit('line', { index:i, text:this.text });
    this.#tick();
  }

  #tick = () => {
    if (this.rate === Infinity) {
      this.node.textContent = this.text;
      return this.#afterLine();
    }

    const now = performance.now();
    const elapsed = now - this.startTime;

    // one-time “thinking” stall after the first quarter of the line
    if (!this._didThink && this.pos > Math.max(1, Math.floor(this.text.length * 0.25))) {
      this._didThink = true;
      if (this.rng() < this.opts.thinkStallChance) {
        return this.#delay(this.#rand(this.opts.thinkStallMs), this.#tick);
      }
    }

    // dynamic cadence: ramp-in -> steady -> ramp-out
    const rampIn = Math.min(1, elapsed / (this.opts.rampInMs / this.rate));
    const rampOutStart = Math.max(0, this.text.length - Math.ceil(this.text.length * 0.18));
    const nearEnd = this.pos >= rampOutStart;
    const rampOutProgress = nearEnd ? (this.pos - rampOutStart) / Math.max(1, this.text.length - rampOutStart) : 0;

    const base = this.opts.baseMs / this.rate;
    const accel = 0.70 + 0.30 * rampIn;          // faster at start
    const decel = 1.0 + 0.50 * rampOutProgress;  // slower near end
    const perChar = base * accel * decel + (this.rng()*2-1) * this.opts.jitterMs;

    // micro-burst
    const [minB, maxB] = this.opts.burstChars;
    const burst = Math.max(1, Math.floor(minB + this.rng() * (maxB - minB + 1)));
    let wrote = 0;

    while (wrote < burst && this.pos < this.text.length) {
      const ch = this.text[this.pos++];
      this.node.textContent += ch;
      wrote++;

      // punctuation pause after a char
      const hold = this.opts.punctHoldMs[ch];
      if (hold) return this.#delay(hold / this.rate, this.#tick);
    }

    if (this.pos >= this.text.length) return this.#afterLine();

    // between bursts
    const [minGap, maxGap] = this.opts.interBurstMs;
    const gap = this.#rand([minGap, maxGap]) + perChar * 0.5;
    this.#delay(Math.max(0, gap), this.#tick);
  }

  #afterLine(){
    this.#emit('lineEnd', { index: this.lineIndex, text: this.text });
    const next = () => {
      const nextIndex = this.lineIndex + 1;
      if (nextIndex < this.lines.length) {
        this.lineIndex = nextIndex; this.#showLine(nextIndex);
      } else if (this.opts.loop) {
        this.lineIndex = 0; this.#showLine(0);
      }
    };
    this.#delay(700 / this.rate, next);
  }

  #delay(ms, fn){
    let start = performance.now();
    const step = () => {
      const now = performance.now();
      if (now - start >= ms) { this.timer = null; fn(); }
      else { this.timer = requestAnimationFrame(step); }
    };
    this.pause();  // clear any prior RAF
    this.timer = requestAnimationFrame(step);
  }

  #rand([a,b]){ return a + this.rng()*(b-a); }

  // deterministic RNG for testing (if seed is given)
  #mulberry32(seed){
    let t = seed >>> 0;
    return function() {
      t += 0x6D2B79F5;
      let r = Math.imul(t ^ t >>> 15, 1 | t);
      r ^= r + Math.imul(r ^ r >>> 7, 61 | r);
      return ((r ^ r >>> 14) >>> 0) / 4294967296;
    };
  }
}

/* ---- v7 init ---- */
const lines = [
  "Universally rendered, locally interpreted.",
  "Numbers name. Ordinals locate.",
  "Every number tells a story.",
  "One bid at a time, the count continues.",
  "Numbers are forever."
];

const typist = new Typist(document.getElementById('typing'), { caret: true, seed: 1337 });
typist.start(lines);

// Example hook for sync (pseudo-code):
// auction.on('tminus', (sec) => {
//   const rate = sec < 10 ? 1.8 : sec < 30 ? 1.4 : 1.0;
//   typist.setRate(rate);
// });
