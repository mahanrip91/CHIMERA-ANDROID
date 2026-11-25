/**
 * Entry: load the engine (wasm) first — UI modules call into it as they load (Sing mode builds
 * its syllables at import time) — then import and render the app. A failed load can be retried.
 */
import "./ui/styles.css";
import { initChimera } from "./wasm/chimera.ts";

const root = document.getElementById("root")!;

// Register the installable/offline shell without delaying audio-engine startup.
if (import.meta.env.PROD && "serviceWorker" in navigator) {
  window.addEventListener(
    "load",
    () => {
      void navigator.serviceWorker
        .register(`${import.meta.env.BASE_URL}sw.js`, { scope: import.meta.env.BASE_URL })
        .catch((error: unknown) => console.warn("CHIMERA offline support could not start:", error));
    },
    { once: true },
  );
}

async function boot(): Promise<void> {
  try {
    await initChimera();
    const { mount } = await import("./ui/mount.tsx");
    await mount(root);
  } catch (err) {
    root.replaceChildren();
    const box = document.createElement("div");
    box.className = "boot-error";
    const msg = document.createElement("p");
    msg.textContent = `The voice engine could not load: ${err instanceof Error ? err.message : String(err)}`;
    const retry = document.createElement("button");
    retry.className = "primary";
    retry.textContent = "Try again";
    retry.onclick = () => void boot();
    box.append(msg, retry);
    root.append(box);
  }
}

void boot();
