/* ============================================================================
   Numbers v7 — typing with a block caret that is a mini v6 counting rectangle
   ========================================================================== */

const DIGITS   = ["1","2","3","4","5","6","7","8","9","0"];
const SPEED_MS = 620; // keep ≥ CSS --fade-ms

/* ----------------------- v6 Stage (caret) helpers ----------------------- */
function makeStage(root, { boxPadY = 2, biasPx = -1, perDigitBias = {}, snap = true } = {}) {
  const cont  = { a: root.querySelector(".layer.a"), b: root.querySelector(".layer.b") };
  const glyph = { a: cont.a.querySelector(".glyph"), b: cont.b.querySelector(".glyph") };
  let front = "a", M = null, BOX_H = null, MAX_W = null;

  function measureDigitMetrics() {
    const cs = getComputedStyle(glyph.a);
    const font = `${cs.fontWeight} ${cs.fontSize} ${cs.fontFamily}`;
    const c = document.createElement("canvas");
    const ctx = c.getContext("2d"); ctx.font = font;

    const metrics = {}; let maxA = 0, maxD = 0, maxW = 0;
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

  function size() {
    M = measureDigitMetrics();
    BOX_H = Math.ceil(M.maxH + 2 * boxPadY);
    MAX_W = Math.ceil(M.maxW);
    root.style.width  = `${MAX_W}px`;
    root.style.height = `${BOX_H}px`;
  }

  function center(layerKey, digit) {
    const cur = M.metrics[digit];
    const topAlign = M.maxA - cur.A;
    glyph[layerKey].style.transform = `translateY(${topAlign}px)`;

    const rect = glyph[layerKey].getBoundingClientRect();
    const leftover = (BOX_H - rect.height) / 2;

    let dy = topAlign + leftover + biasPx + (perDigitBias[digit] ?? 0);
    if (snap) dy = Math.round(dy);

    glyph[layerKey].style.transform = `translateY(${dy}px)`;
  }

  function setDigit(d) {
    const back = front === "a" ? "b" : "a";
    glyph[back].textContent = d;
    center(back, d);
    cont[back].classList.remove("hidden"); cont[back].classList.add("visible");
    cont[front].classList.remove("visible"); cont[front].classList.add("hidden");
    front = back;
  }

  function seed(d0, d1) { size(); glyph.a.textContent=d0; center("a", d0); glyph.b.textContent=d1; center("b", d1); }
  function recenterCurrent() { const d = glyph[front].textContent; center(front, d); }
  function boxHeight(){ return BOX_H; }

  return { size, setDigit, seed, recenterCurrent, boxHeight };
}

/* ---------------------------- Build caret stage ---------------------------- */

const caretStage = makeStage(document.getElementById("caret-stage"), { boxPadY: 2, biasPx: -1 });
(function initCaret(){
  const d0 = DIGITS[0], d1 = DIGITS[1];
  caretStage.seed(d0, d1);
})();
let idx = 2;
setInterval(() => {
  const d = DIGITS[idx]; idx = (idx + 1) % DIGITS.length;
  caretStage.setDigit(d);
}, SPEED_MS);

/* ------------------------ Caret positioning (baseline) --------------------- */

const typingBox = document.getElementById("typing");
const marker    = document.getElementById("marker");
const caretEl   = document.getElementById("caret-stage");

/* small rightward nudge so letters appear to emerge from the caret’s left edge */
const CARET_X_NUDGE_PX = 0;   // set to ~1 if you want a tiny gap

function positionCaret() {
  const m = marker.getBoundingClientRect();
  const b = typingBox.getBoundingClientRect();
  const cs = getComputedStyle(typingBox);
  const lineH = parseFloat(cs.lineHeight);      // px
  const caretH = caretStage.boxHeight();        // px (from metrics + padding)

  const x = Math.round(m.left - b.left + CARET_X_NUDGE_PX);
  const y = Math.round(m.top  - b.top + (lineH - caretH) / 2);

  caretEl.style.transform = `translate(${x}px, ${y}px)`;
}

/* initial placement and on resize (fonts, viewport) */
positionCaret();
document.fonts?.ready.then(positionCaret);

let resizeTimer;
window.addEventListener("resize", () => {
  clearTimeout(resizeTimer);
  resizeTimer = setTimeout(() => {
    caretStage.size(); caretStage.recenterCurrent();
    positionCaret();
  }, 120);
});

/* ------------------------------- Typist ----------------------------------- */

class Typist {
  constructor(node, opts = {}) {
    this.node = node;
    this.opts = {
      baseMs: 70, jitterMs: 25,
      burstChars: [2, 6], interBurstMs: [60, 140],
      rampInMs: 300, rampOutMs: 420,
      punctHoldMs: { ',':120,';':150,':':160,'.':380,'!':420,'?':420,'—':280 },
      thinkStallChance: 0.16, thinkStallMs: [500,1200],
      loop: true, respectReducedMotion: true, seed: null,
      afterChar: () => {},
      ...opts
    };
    this.lines = []; this.lineIndex=0; this.text=''; this.pos=0; this.rate=1;
    this.timer=null;
    this.rng = this.opts.seed ? this.#mulberry32(this.opts.seed) : Math.random;
    if (this.opts.respectReducedMotion && matchMedia('(prefers-reduced-motion: reduce)').matches) this.rate = Infinity;
  }

  start(lines){ this.lines = Array.isArray(lines)?lines.slice():[String(lines||'')]; this.lineIndex=0; this.#show(0); return this; }
  setRate(mult){ this.rate = Math.max(0.05, mult||1); }

  #show(i){ this.text=this.lines[i]||''; this.pos=0; this.node.textContent=''; this.startTime=performance.now(); this._didThink=false; this.opts.afterChar(); this.#tick(); }

  #tick = () => {
    if (this.rate === Infinity) { this.node.textContent=this.text; this.opts.afterChar(); return this.#after(); }

    const now = performance.now(), elapsed = now - this.startTime;

    if (!this._didThink && this.pos > Math.max(1, Math.floor(this.text.length*0.25))) {
      this._didThink = true;
      if (this.rng() < this.opts.thinkStallChance) return this.#delay(this.#rand(this.opts.thinkStallMs), this.#tick);
    }

    const rampIn = Math.min(1, elapsed / (this.opts.rampInMs / this.rate));
    const rampOutStart = Math.max(0, this.text.length - Math.ceil(this.text.length * 0.18));
    const nearEnd = this.pos >= rampOutStart;
    const rampOutProgress = nearEnd ? (this.pos - rampOutStart) / Math.max(1, this.text.length - rampOutStart) : 0;

    const base = this.opts.baseMs / this.rate;
    const accel = 0.70 + 0.30 * rampIn;
    const decel = 1.0 + 0.50 * rampOutProgress;
    const perChar = base * accel * decel + (this.rng()*2-1) * this.opts.jitterMs;

    const [minB, maxB] = this.opts.burstChars;
    const burst = Math.max(1, Math.floor(minB + this.rng()*(maxB-minB+1)));
    let wrote = 0;

    while (wrote < burst && this.pos < this.text.length) {
      const ch = this.text[this.pos++];
      this.node.textContent += ch;
      wrote++;
      this.opts.afterChar();                       // move caret each char
      const hold = this.opts.punctHoldMs[ch];
      if (hold) return this.#delay(hold / this.rate, this.#tick);
    }

    if (this.pos >= this.text.length) return this.#after();

    const [g0,g1] = this.opts.interBurstMs;
    const gap = this.#rand([g0,g1]) + perChar * 0.5;
    this.#delay(Math.max(0, gap), this.#tick);
  }

  #after(){
    const next = () => {
      const ni = this.lineIndex + 1;
      if (ni < this.lines.length) { this.lineIndex=ni; this.#show(ni); }
      else if (this.opts.loop) { this.lineIndex=0; this.#show(0); }
    };
    this.#delay(700 / this.rate, next);
  }

  #delay(ms, fn){
    let start = performance.now();
    const step = () => { (performance.now() - start >= ms) ? (this.timer=null, fn()) : (this.timer=requestAnimationFrame(step)); };
    if (this.timer) cancelAnimationFrame(this.timer);
    this.timer = requestAnimationFrame(step);
  }

  #rand([a,b]){ return a + this.rng()*(b-a); }
  #mulberry32(seed){ let t = seed>>>0; return function(){ t+=0x6D2B79F5; let r=Math.imul(t^t>>>15,1|t); r^=r+Math.imul(r^r>>>7,61|r); return ((r^r>>>14)>>>0)/4294967296; }; }
}

/* ---- init ---- */
const lines = [
  "Universally rendered, locally interpreted.",
  "Numbers name. Ordinals locate.",
  "Every number tells a story.",
  "One bid at a time, the count continues.",
  "Numbers are forever."
];

new Typist(document.getElementById("typed"), {
  seed: 1337,
  afterChar: positionCaret
}).start(lines);
