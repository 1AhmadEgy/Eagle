# Eagle — Repository Fact Check — 2026-10-04

## Purpose

This document corrects stale or overly broad historical statements by separating what is **Verified in the repository** from what remains **Pending**.

The goal is traceability, not a claim of product readiness.

## Repository baseline

| Item | Verified state | Evidence |
|---|---|---|
| Repository | `1AhmadEgy/Eagle` | GitHub repository |
| Main baseline used for this audit | `6c46bf53fce06ee7f7b5b2bf35c720ab5bcb7dee` | comparison against audit branch |
| Audit branch | `docs/eagle-evidence-audit-2026-10-04` | current workstream |
| Android module | Present | `app/` |
| Android namespace | `com.eagle.app` | `app/build.gradle.kts` |
| Android min SDK | 29 | `app/build.gradle.kts` |
| Android compile/target SDK | 37 / 37 | `app/build.gradle.kts` |
| App version | 0.1.0 (versionCode 1) | `app/build.gradle.kts` |
| Android Gradle Plugin | 9.4.0 | root `build.gradle.kts` |
| CI Gradle | 9.6.0 | `.github/workflows/testlab.yml` |
| JDK in CI | 17 | `.github/workflows/testlab.yml` |
| Test workflow | Present | `.github/workflows/testlab.yml` |
| Main CI workflow | Present | `.github/workflows/ci.yml` |
| Unit test source | Present | `app/src/test/java/com/eagle/app/SmokeTest.kt` |
| MainActivity | Present and executable bootstrap | `app/src/main/java/com/eagle/app/MainActivity.kt` |
| Production messaging stack | Not implemented/evidenced | repository inspection |
| Production E2EE protocol | Not implemented/evidenced | repository inspection |
| Production PrivateMesh implementation | Not implemented/evidenced | repository inspection |

### Build-system correction

The apparent AGP/Gradle version difference is **not** a defect by itself. Android's official AGP 9.4.0 compatibility table requires Gradle 9.6.0 and JDK 17, and AGP 9.4 supports API level 37.

Therefore the repository's current values:

- AGP 9.4.0
- Gradle 9.6.0 in CI
- JDK 17
- compile/target SDK 37

are mutually consistent at the compatibility level.

This is a compatibility observation, not proof that a successful CI run exists for every current commit.

## Application reality

`MainActivity.kt` currently creates a simple `TextView` and displays the Test Lab label.

`SmokeTest.kt` currently asserts a constant `true` value. It proves only that the test method can be discovered/executed; it does **not** prove message delivery, cryptography, identity, protocol correctness, storage integrity, or Mesh behavior.

This distinction is now canonical.

## CI reality

Two separate concepts exist:

1. `testlab.yml` — builds/tests/lints the Android module and assembles a debug APK.
2. `ci.yml` — is the workflow named `CI` referenced by the privileged AI-repair workflow.

The AI-repair workflow is triggered only by a completed `CI` workflow run on the trusted branch `implementation/v1-foundation`. It is not triggered by `Eagle Test Lab`.

This is an intentional boundary worth preserving, but the live behavior must still be verified from actual workflow runs before being marked operational.

## Corrected evidence states

| Area | Previous broad wording | Corrected state |
|---|---|---|
| Android stack | “unverified” | **Partially Verified** |
| CI/CD | “no CI evidence” | **Configured; live-run evidence Pending** |
| QA | “baseline” | **Very limited executable smoke evidence; product QA Pending** |
| Architecture | “foundation” | **Documented intent/foundation; final implementation architecture Pending** |
| Threat model | “pending” | **Pending against implemented trust boundaries** |
| Product requirements | “pending” | **Pending authoritative freeze** |
| External reuse | “pending survey” | **External due-diligence survey started; no production dependency approved yet** |

## Historical archive boundary

The Filebin source supplied for this project was previously inspected only at the listing/metadata level. The archive bytes were not successfully retrieved and extracted through the available path.

Therefore:

- no ZIP contents are claimed as inspected;
- no historical implementation is promoted based only on its filename;
- the archives remain **Pending intake**;
- future intake must hash, manifest, scan, deduplicate, classify, and preserve provenance before promotion.

## Canonical evidence labels

- **Verified** — directly demonstrated by repository/source evidence.
- **Derived** — calculated or concluded directly from verified evidence.
- **Proposed** — an engineering option, not an implementation claim.
- **Pending** — evidence required before acceptance.
- **Rejected** — evaluated and not accepted for the stated reason.

## Immediate consequences

1. Do not add messaging/E2EE/Mesh dependencies merely because the product vision mentions them.
2. Build the first executable security slice only after requirements and trust boundaries are fixed.
3. Replace tautological smoke assertions with behavior-level tests as soon as the first real component contracts exist.
4. Keep live CI status separate from workflow configuration claims.
5. Treat all historical archives as untrusted inputs until intake is completed.

## Source references

- Android Gradle Plugin 9.4.0 compatibility: https://developer.android.com/build/releases/agp-9-4-0-release-notes
- Android Gradle Plugin compatibility matrix: https://developer.android.com/build/releases/about-agp
