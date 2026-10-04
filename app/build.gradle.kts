plugins {
    id("com.android.application")
}

android {
    namespace = "com.eagle.app"
    compileSdk = 36

    defaultConfig {
        applicationId = "com.eagle.app"
        minSdk = 29
        targetSdk = 37
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
        // The Linux CI runner intentionally installs Android 36 only.
        // targetSdk remains 37; GradleDependency would otherwise fail solely
        // because SDK 37 is newer than the build environment's installed SDK.
        disable += "GradleDependency"
    }
}

dependencies {
    implementation(project(":shared"))
    testImplementation("junit:junit:4.13.2")
}
