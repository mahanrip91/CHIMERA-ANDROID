# Android voice engine architecture

The Android application runs the voice model in a Rust-compiled WebAssembly module, called from a Web Audio `AudioWorklet`. The main thread owns React UI state and plans speech; the worklet owns the real-time voice state, audio clock, beat sequencer and synthesis. Messages crossing the worklet boundary are defined in `src/worklet/protocol.ts`.

## Runtime path

1. Vite loads the React application and the generated `src/wasm/chimera.wasm` module.
2. The user starts audio from a tap. `AudioEngine` creates/resumes `AudioContext`, loads the worklet and connects the voice output.
3. The main thread sends articulator, source, creature, beat and gesture-plan updates through the worklet message protocol.
4. Rust advances the physical vocal-tract model in fixed-size audio blocks and returns telemetry for the face, tube, spectrum and level displays.

## Rust crates

- `crates/chimera`: physical voice model, phonetics, speech planning, singing, creatures and beatbox synthesis.
- `crates/chimera-wasm`: WebAssembly bindings used by the Android browser application.

The same core engine is tested natively with deterministic acoustic and phonetic fixtures. The browser build uses `wasm32-unknown-unknown`; the Android app itself is delivered as a Progressive Web App over HTTPS.
