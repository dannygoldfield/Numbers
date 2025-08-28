// Single stationary digit that cycles 1..9..0 at a fixed rate.
// Default: 0.1s per change, like a stopwatch tick.

const el = document.getElementById("digit");
const DIGITS = ["1","2","3","4","5","6","7","8","9","0"];

// ---- controls you’ll tweak ----
const SPEED_MS  = 100;   // 100ms = 0.1 second per change
const FADE_MODE = false; // set to true for a tiny crossfade
// --------------------------------

let i = 0;
let timer = null;

function tick() {
  if (FADE_MODE) {
    // quick crossfade: fade out, swap text, fade back in
    el.classList.add("fade");
    // after the fade-out transition starts, swap text
    setTimeout(() => {
      el.textContent = DIGITS[i];
      el.classList.remove("fade");
    }, 45); // half of the CSS transition duration (≈90ms)
  } else {
    // instant swap (stopwatch feel)
    el.textContent = DIGITS[i];
  }

  i = (i + 1) % DIGITS.length;
}

function start() {
  // initialize immediately so you see the first tick without delay
  tick();
  timer = setInterval(tick, SPEED_MS);
}

start();

// If you want to stop later: clearInterval(timer);
