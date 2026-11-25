# Android build and release

CHIMERA is shipped as an Android-installable Progressive Web App (PWA) through GitHub Pages. It uses Chrome's WebAssembly, AudioWorklet and Web Audio implementations.

## Production build

```sh
bun install
rustup target add wasm32-unknown-unknown
bun run build
```

Vite writes the deployable site to `dist/`. GitHub Actions runs the same build on its hosted runner and publishes it to `https://mahanrip91.github.io/CHIMERA-ANDROID/`.

## Install on a phone

Open the HTTPS site in Android Chrome, use the browser menu's **Install app** / **Add to Home screen** action, then launch the installed shortcut. Audio starts only after the user taps an in-app playback control.

## Offline behavior

The service worker caches the app shell and same-origin resources after they are successfully fetched. The first install and first engine/data load require internet access. A resource that has never been downloaded cannot be available offline.
