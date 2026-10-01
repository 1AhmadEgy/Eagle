plugins {
    id("com.android.application") version "9.4.0" apply false
}

tasks.register("prePushGate") {
    group = "verification"
    description = "Runs the mandatory pre-GitHub verification gate."
    dependsOn(":app:testDebugUnitTest", ":app:lint", ":app:assembleDebug")
}
