# Eagle — Verified Stack Baseline

**Status:** Verified from repository state  
**Date:** 2026-10-04  
**Branch:** `implementation/stack-baseline-2026-10`

## Evidence

| Area | Verified value | Evidence |
|---|---|---|
| Primary application platform | Android | `settings.gradle.kts`, `app/build.gradle.kts`, `AndroidManifest.xml` |
| Application language | Kotlin | `app/src/main/java/com/eagle/app/MainActivity.kt` |
| Build system | Gradle Kotlin DSL | `build.gradle.kts`, `settings.gradle.kts`, `app/build.gradle.kts` |
| Android Gradle Plugin | 9.4.0 | root `build.gradle.kts` |
| compileSdk | 37 | `app/build.gradle.kts` |
| minSdk | 29 | `app/build.gradle.kts` |
| targetSdk | 37 | `app/build.gradle.kts` |
| Application ID | `com.eagle.app` | `app/build.gradle.kts` |
| Version | 0.1.0 / versionCode 1 | `app/build.gradle.kts` |
| Unit test framework | JUnit 4.13.2 | `app/build.gradle.kts` |
| Repository modules | `:app` only | `settings.gradle.kts` |
| Repositories | Google, Maven Central | `settings.gradle.kts` |
| CI | GitHub Actions | `.github/workflows/ci.yml` |
| Security gate | Repository-local policy verification + secret scan | `.github/workflows/ci.yml` |
| Product verification | Test Lab category runner | `scripts/ci/run-testlab-categories.py` |

## Important limitations

1. No Gradle wrapper files are currently present in the repository tree. A reproducible Gradle invocation therefore has not yet been established.
2. The current product source is a minimal Android skeleton; the only application entry point is `MainActivity`.
3. The existing Test Lab explicitly keeps Build, Unit, Integration, Cryptography, Protocol, Regression, and Fuzz/property categories at `PENDING` because authoritative product suites do not yet exist.
4. No database, network transport, mesh implementation, cryptographic protocol implementation, or AI/agent runtime is currently established by the source tree examined in this baseline.
5. This document records observed facts only. It does not approve future technology choices.

## Next execution gate

The next implementation slice should establish a reproducible Android build/test path and then introduce the first architecture-approved core contracts. No production capability should be inferred until its requirement, implementation, and tests are linked.
