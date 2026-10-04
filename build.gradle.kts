plugins {
    id("com.android.application") version "9.3.1" apply false
    id("org.jetbrains.kotlin.multiplatform") version "2.4.20" apply false
    id("com.android.kotlin.multiplatform.library") version "9.3.1" apply false
}

tasks.register("prePushGate") {
    group = "verification"
    description = "Runs the mandatory pre-GitHub verification gate."
    dependsOn(":app:testDebugUnitTest", ":app:lint", ":app:assembleDebug", ":shared:allTests")
}
