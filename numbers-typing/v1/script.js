const container = document.getElementById("typing-effect");
const text = "What if you could own a number?";

function createCharSpan(char) {
  const span = document.createElement("span");
  span.classList.add("char");
  span.dataset.final = char;

  const ghost = document.createElement("span");
  ghost.className = "ghost";
  ghost.textContent = char === " " ? "\u00A0" : char;

  const active = document.createElement("span");
  active.className = "active";
  active.textContent = "";

  span.appendChild(ghost);
  span.appendChild(active);
  container.appendChild(span);
}

function animateChar(span, duration = 200) {
  const active = span.querySelector(".active");
  let count = 0;
  const interval = setInterval(() => {
    active.textContent = Math.floor(Math.random() * 10);
    count++;
    if (count > duration / 50) {
      clearInterval(interval);
      active.textContent = span.dataset.final;
    }
  }, 50);
}

[...text].forEach((char, i) => {
  createCharSpan(char);

  const delay = i * 100;
  setTimeout(() => {
    animateChar(container.children[i]);
  }, delay);
});
