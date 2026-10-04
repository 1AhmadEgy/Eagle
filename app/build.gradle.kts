plugins {
    id("com.android.application")
}

android {
    namespace = "com.eagle.app"
    compileSdk = 36

    defaultConfig {
        applicationId = "com.eagle.app"
        minSdk = 29
        targetSdk = 36
        versionCode = 1
        versionName = "0.1.0"

        testInstrumentationRunner = "androidx.test.runner.AndroidJUnitRunner"
    }

    buildTypes {
        debug {
            isMinifyEnabled = false
        }
        release {
            isMinifyEnabled = true
            isShrinkResources = true
            proguardFiles(
                getDefaultProguardFile("proguard-android-optimize.txt"),
                "proguard-rules.pro"
            )
        }
    }

    testOptions {
        unitTests.isReturnDefaultValues = false
    }

    lint {
        abortOnError = true
        warningsAsErrors = true
        // Android 17 (API 37) is currently preview-only in the CI SDK repository;
        // keep stable Android 16 targeting while retaining the upgrade signal.
        warning.add("OldTargetApi")
    }
}

dependencies {
    testImplementation("junit:junit:4.13.2")
}
