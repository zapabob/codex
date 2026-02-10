const langToggle = document.getElementById("lang-toggle");
let currentLang = "EN";

langToggle.addEventListener("click", () => {
  currentLang = currentLang === "EN" ? "JP" : "EN";
  langToggle.textContent = currentLang === "EN" ? "JP" : "EN";

  document.querySelectorAll("[data-en]").forEach((el) => {
    if (currentLang === "EN") {
      el.textContent = el.getAttribute("data-en");
    } else {
      el.textContent = el.getAttribute("data-ja");
    }
  });
});

// Smooth scroll adjustments or micro-animations could go here
document.querySelectorAll('a[href^="#"]').forEach((anchor) => {
  anchor.addEventListener("click", function (e) {
    e.preventDefault();
    document.querySelector(this.getAttribute("href")).scrollIntoView({
      behavior: "smooth",
    });
  });
});
