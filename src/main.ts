import "./sytle.css";

const supported = (() => {
  try {
    if (typeof WebAssembly === "object"
      && typeof WebAssembly.instantiate === "function") {
      const module = new WebAssembly.Module(Uint8Array.of(0x0, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00));
      if (module instanceof WebAssembly.Module)
        return new WebAssembly.Instance(module) instanceof WebAssembly.Instance;
    }
  } catch (e) {
  }
  return false;
})();

import("../rust-backend/pkg").then(RustBackend => {
  try {
    return RustBackend.main();
  } catch {
    return false;
  }
}).then(status => {
  if (supported && status) {
    document.getElementById("mask-child")?.remove();
    document.getElementById("mask")?.classList.add("preload-mask-opacity");
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
