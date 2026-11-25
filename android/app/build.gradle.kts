plugins {
    id("com.android.application")
    id("org.jetbrains.kotlin.android")
}

android {
    namespace = "com.mahanrip.chimera"
    compileSdk = 35
    defaultConfig {
        applicationId = "com.mahanrip.chimera"
        minSdk = 26
        targetSdk = 35
        versionCode = 1
        versionName = "1.0.0"
        ndk { abiFilters += listOf("arm64-v8a") }
    }
}

dependencies {
    implementation("androidx.webkit:webkit:1.12.1")
}
