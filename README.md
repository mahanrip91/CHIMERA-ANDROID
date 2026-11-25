# CHIMERA for Android

CHIMERA is an installable, Android-first voice synthesizer. It models a voice from airflow through the lungs, vocal folds and vocal tract; it does not assemble speech from prerecorded voice clips.

**Open the app:** https://mahanrip91.github.io/CHIMERA-ANDROID/

## Install on Android

1. Open the link above in the latest Google Chrome on Android.
2. Open Chrome's menu and choose **Install app** (or **Add to Home screen**, depending on the Chrome version).
3. Launch CHIMERA from its new home-screen icon. The app opens in its own window.
4. Start audio with one of the in-app action buttons. Android requires audio playback to begin from a user gesture.

The first visit needs an internet connection to download the engine and language data. Resources are cached as they load; after a successful first load, the app shell and previously loaded resources can be reused offline. Keep Chrome up to date for WebAssembly, AudioWorklet and Web Audio support.

## What is included

- **Play:** live vocal-tract controls, body presets, keys and optional MIDI where the Android browser exposes Web MIDI.
- **Speak:** text and IPA to speech gestures, with language selection and pronunciation feedback.
- **Sing:** lyrics, syllable controls, tunes, tempo, key, pitch roll and recording/sharing where supported by Android.
- **Beatbox:** articulatory drum pads and a step sequencer.
- **Bestiary:** modeled animal vocalizations.
- 2D/3D anatomy views, spectrum, airflow and sound-pressure visualizations.
- Adaptive audio workload: the engine can reduce its internal sample rate on slower phones.

## Android-first scope

This repository contains only the Android browser application and its Rust/WebAssembly voice engine. It is designed for Android Chrome and compatible Chromium-based Android browsers. It is an installable PWA, not a standalone APK.

## Development

Requirements: Bun, Rust, and the `wasm32-unknown-unknown` Rust target.

```sh
bun install
rustup target add wasm32-unknown-unknown
bun run dev       # local development at http://127.0.0.1:5317
bun run build     # WebAssembly + TypeScript checks + production site
bun run test      # Rust engine and browser-side tests
```

The AudioWorklet requires a secure context. Use `localhost` for local development or HTTPS for hosted builds. The GitHub Actions workflow builds the WebAssembly engine and deploys `dist/` to GitHub Pages on every push to `main`.

## Source and license

The application is adapted from [CHIMERA vocal synth](https://github.com/jamesondh/chimera-vocal-synth), originally created by Jameson DH. The original MIT license is retained in [LICENSE](LICENSE), and bundled third-party assets retain their own notices and licenses.
