# Android Reproducible Build Baseline

## Evidence snapshot

The current Android build configuration declares:
- Android Gradle Plugin: 9.4.0
- compileSdk: 37
- targetSdk: 37
- minSdk: 29
- JDK requirement: 17
- Gradle requirement: 9.6.0 or newer compatible release
- SDK Build Tools baseline: 36.0.0

The official Android documentation for AGP 9.4.0 states Gradle 9.6.0 as the minimum/default compatible version, JDK 17, and maximum supported API level 37.

## Current repository finding

The repository currently does NOT contain the standard Gradle Wrapper files under gradle/wrapper/ or gradlew.

Therefore the Android build is NOT YET REPRODUCIBLE from a clean checkout.

This is an evidence-based blocker, not a test failure.

## Corrections now staged

- Kotlin Android plugin pinned to 2.3.21 to match the existing Kotlin source and the documented AGP 9.4 example.
- AndroidX Test dependencies added using current stable releases documented by Android Developers.
- A real instrumentation behavior test was added for MainActivity using ActivityScenario.
- The test verifies the user-visible Eagle Test Lab label.

These changes are implemented but NOT VERIFIED until a real Android build and instrumentation run produces evidence.

## Required build correction

1. Generate the official Gradle Wrapper using the selected Gradle 9.6.0 release.
2. Commit the wrapper scripts, wrapper JAR, and gradle-wrapper.properties.
3. Configure the distribution URL to an exact version, not a dynamic selector.
4. Record and verify the Gradle distribution SHA-256.
5. Verify the wrapper JAR provenance/checksum against the official Gradle release checksums.
6. Pin the JDK/toolchain used by local and CI builds.
7. Define the Android SDK/platform/build-tools provisioning contract.
8. Run the build from a clean environment and retain exact-SHA evidence.

## Security constraint

No custom wrapper implementation should replace the official Gradle Wrapper merely to avoid committing the official wrapper JAR. Gradle's security guidance requires verification of the wrapper JAR and distribution checksum.

## Behavioral test evidence

Current test implementation:
- app/src/androidTest/java/com/eagle/app/MainActivityBehaviorTest.kt
- Test: launcherActivity_displaysTestLabLabel

Current result: NOT VERIFIED.

No Android behavioral category is PASS until the APK/test APK is built and the test executes successfully on an emulator or physical device with evidence bound to the tested commit.

## CI authorization review

The current standard CI workflow declares contents: read. It performs repository hygiene, secret scanning, security-policy verification, product-surface discovery, and Test Lab orchestration.

A separate workflow_run repair workflow has elevated permissions (contents: write, pull-requests: write, issues: write) but is restricted to the trusted implementation/v1-foundation push boundary and explicitly prohibits the repair agent from modifying .github/workflows/*, secrets, deployment configuration, or unrelated files.

No CI workflow has been modified in this correction pass.

Any future change that provisions Android SDK/JDK, invokes Gradle, launches an emulator, or uploads Android behavioral evidence must first pass the repository's authorization/review boundary.

## Evidence status
- Build reproducibility: BLOCKED
- Kotlin source compilation: NOT VERIFIED
- Android unit-test execution: NOT VERIFIED
- Android instrumentation/behavioral execution: NOT VERIFIED
- Test Lab category PASS: NOT ALLOWED
- CI Android execution: NOT IMPLEMENTED
- Release Gate: BLOCKED