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

## Required correction

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

## Test status

No Android behavioral category is PASS until an actual Android implementation is built and the corresponding test produces a verifiable result bound to the exact commit.

## CI authorization boundary

The current repository CI has read-only contents permissions and already performs repository/security/Test Lab checks. Modifying CI to provision Android SDK/JDK, invoke Gradle, install an emulator, or upload Android test evidence is a separate privileged change and must remain gated by the repository's authorization/review process.

## Evidence status
- Build reproducibility: BLOCKED
- Android unit-test execution: NOT VERIFIED
- Android instrumentation/behavioral execution: NOT VERIFIED
- Test Lab category PASS: NOT ALLOWED
- Release Gate: BLOCKED