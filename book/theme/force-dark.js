(function () {
  // Pick one: "navy", "coal", "ayu"
  const THEME = "navy";

  try {
    // mdBook stores theme here
    localStorage.setItem("mdbook-theme", THEME);
  } catch (e) {}

  const root = document.documentElement;

  ["light", "rust", "coal", "navy", "ayu"].forEach((t) =>
    root.classList.remove(t)
  );

  root.classList.add(THEME);

  // Tell the browser this is a dark site
  root.style.colorScheme = "dark";
})();
