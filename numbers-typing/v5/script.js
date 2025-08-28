// Numbers v5 — constant rectangle + centered digit + crossfade
// Choose rectangle height via BOX_PAD_Y (extra px above AND below).

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
const SPEED_MS = 920;          // keep ≥ fade duration for a clean crossfade

// ---------- Tweak knobs ----------
const BOX_PAD_Y = 5;           // px added to TOP and BOTTOM of the rectangle
const BIAS_PX = -5;             // global vertical nudge: + pushes DOWN, - pushes UP
const PER_DIGIT_BIAS = {       // optional per-digit nudges (uncomment to use)
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

function start() {
  sizeStageConstantBox();

  // Initialize first two digits centered so first fade is clean
  const d0 = DIGITS[0], d1 = DIGITS[1];
  glyph.a.textContent = d0; centerGlyphVertically("a", d0);
  glyph.b.textContent = d1; centerGlyphVertically("b", d1);
  i = 2;

  setInterval(tick, SPEED_MS);
}

start();

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
