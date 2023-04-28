import "./sytle.css";

import("../rust-backend/pkg").then(RustBackend => {
  RustBackend.main();
}).finally(() => {
  document.getElementById("mask-child")?.remove();
  document.getElementById("mask")?.classList.add("opacity-0");
  setTimeout(() => document.getElementById("mask")?.remove(), 500);
});

if (process.env.NODE_ENV === "production") {
  if ("serviceWorker" in navigator) {
    window.addEventListener("load", () => {
      navigator.serviceWorker.register("service-worker.js").then(registration => {
        console.log("SW registered: ", registration);
      }).catch(registrationError => {
        console.log("SW registration failed: ", registrationError);
      });
    });
  }
}
