# CHIMERA for Android

CHIMERA is a physical-model voice instrument. It simulates a vocal tract and generates speech-like, sung and beatbox sounds from an articulatory model.

## Android app

The primary Android build is a native APK. Kotlin owns the Android activity, WebView asset loading and `AudioTrack`; the Rust voice engine is compiled as an ARM64 Android shared library and renders on a dedicated audio-priority thread. The packaged React interface controls that native engine through a narrow bridge. Android playback does not use browser `AudioWorklet` or Web Audio for synthesis.

The APK is built by the **Android APK** GitHub Actions workflow. Run it with the workflow-dispatch button or push to `main`; download the `CHIMERA-Android-debug` artifact from the completed run. Build architecture and limitations are documented in [ANDROID_BUILD.md](ANDROID_BUILD.md).

## Engine

- `crates/chimera`: platform-independent physical voice model and synthesis engine.
- `crates/chimera-wasm`: browser/planning API and stable render ABI shared with native hosts.
- `crates/chimera-android`: JNI adapter for the Android audio host.
- `android/app`: Kotlin activity, packaged WebView UI and low-latency `AudioTrack` output.

## Web preview

The browser version remains available as a development/preview target. It uses the Web Audio worklet backend; the Android APK selects the native backend automatically.

## License

MIT. See [LICENSE](LICENSE).
