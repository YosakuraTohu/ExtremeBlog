import "./sytle.css";

import("../rust-backend/pkg").then(RustBackend => {
  try {
    return RustBackend.main();
  } catch {
    return false;
  }
}).then(status => {
  if (status) {
    document.getElementById("mask-child")?.remove();
    document.getElementById("mask")?.classList.add("opacity-0");
    setTimeout(() => document.getElementById("mask")?.remove(), 500);
  } else {
    document.getElementById("message")!.innerText = "Your browser does not support WebAssembly.";
  }
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
