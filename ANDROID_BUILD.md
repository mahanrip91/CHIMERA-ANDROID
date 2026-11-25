# CHIMERA Mobile

An Android-first, installable web edition of CHIMERA. The original Rust/WebAssembly voice engine and React instrument are retained; this project adds a touch-oriented mobile shell, PWA installation, offline asset caching, and GitHub Pages deployment.

## Local development

Requirements: Bun, Rust, and the `wasm32-unknown-unknown` Rust target.

```sh
bun install
bun run dev
```

## GitHub Pages

The included workflow builds the WebAssembly engine and deploys `dist/` to GitHub Pages whenever `main` is updated. The Vite base path is configured for the repository name `CHIMERA-Android`.

In the repository settings, set **Pages → Build and deployment → Source** to **GitHub Actions**.

## Mobile features

- Responsive controls and screen layout for narrow touch displays.
- Installable PWA manifest and Android app icon.
- Service worker caching for repeat visits and offline app-shell access.

The synthesizer needs a browser with Web Audio, AudioWorklet, WebAssembly, and a secure context (HTTPS or localhost). Offline access is available after the app and its assets have been cached once.

## Upstream

Based on [jamesondh/chimera-vocal-synth](https://github.com/jamesondh/chimera-vocal-synth), licensed under MIT. See [LICENSE](LICENSE) and the upstream project for its credits and notices.
