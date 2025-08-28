// Double-buffer crossfade: layer A <-> layer B
const layers = {
  a: document.querySelector(".layer.a"),
  b: document.querySelector(".layer.b"),
};

const DIGITS = ["1","2","3","4","5","6","7","8","9","0"];

// ---- tweakables ----
const SPEED_MS = 600; // 0.1s per change, like a stopwatch
// --------------------

let i = 0;
let front = "a"; // which layer is currently visible ("a" or "b")

function nextDigit() {
  const d = DIGITS[i];
  i = (i + 1) % DIGITS.length;
  return d;
}

function tick() {
  const back = front === "a" ? "b" : "a";

  // Put the next digit on the back layer
  layers[back].textContent = nextDigit();

  // Crossfade: back -> visible, front -> hidden
  layers[back].classList.remove("hidden");
  layers[back].classList.add("visible");

  layers[front].classList.remove("visible");
  layers[front].classList.add("hidden");

  // Swap roles
  front = back;
}

function start() {
  // Initialize both layers to the first two digits so the first fade is clean
  layers.a.textContent = DIGITS[0];
  layers.b.textContent = DIGITS[1];
  i = 2; // we've already shown first two values across the two layers
  // Ensure A is visible, B is hidden at start (matches HTML classes)
  setInterval(tick, SPEED_MS);
}

start();
