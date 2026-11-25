# CHIMERA native Android build

The Android app is a native Kotlin shell with a Rust `cdylib` audio engine. The React UI is bundled into the APK and displayed in a locked-down WebView; it sends control commands over a small JavaScript bridge. Audio is rendered by the Rust engine on a dedicated Android audio-priority thread and written to `AudioTrack` as mono float PCM at 32 kHz. The browser AudioWorklet is not used by the Android APK.

## Build

The reproducible APK build runs in GitHub Actions (`Android APK`). It builds the UI with `ANDROID_NATIVE_BUILD=true`, compiles `crates/chimera-android` for `arm64-v8a` using the Android NDK, then runs Gradle `assembleDebug`.

The output is `android/app/build/outputs/apk/debug/app-debug.apk`, published as the `CHIMERA-Android-debug` workflow artifact. The current native package targets ARM64 Android devices (including the Redmi Note 13 4G).

## Audio architecture

- `crates/chimera` remains the shared physical-model engine.
- `crates/chimera-wasm` supplies the stable engine ABI and the non-real-time planning/phonetics API used by the UI.
- `crates/chimera-android` exposes the real-time engine to Kotlin through JNI.
- `NativeAudio.kt` owns `AudioTrack` and a dedicated `THREAD_PRIORITY_AUDIO` render loop. JNI only copies each 256-frame render block into the reusable output array.
- `MainActivity.kt` serves packaged assets with `WebViewAssetLoader`; file access and cleartext traffic are disabled.

This is a native Android APK, not a PWA wrapper around browser audio. Recording through `MediaStream` and live analyser telemetry are not yet implemented in the native backend.
