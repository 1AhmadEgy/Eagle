# Eagle

**Eagle (النسر)** is an Android-first project for a private, security-oriented communications platform. The repository is being built around explicit security boundaries, reproducible verification, documented architectural decisions, and traceable project evidence.

> **Current status:** Foundation / implementation bootstrap. The repository contains an Android application skeleton, baseline tests, CI/Test Lab verification, and an extensive documentation/governance layer. The production messaging, cryptographic, mesh, KMP, and Rust security components are **not claimed as implemented in the current main branch** unless a corresponding source file and verification evidence exist.

## Project direction

The intended product direction is a private communications platform with:

- device-oriented cryptographic identity;
- end-to-end encrypted messaging;
- modular PrivateMesh transport architecture;
- explicit trust boundaries and least-privilege design;
- cross-platform reuse through a shared application layer;
- a platform-independent security core;
- repeatable build, test, security, and provenance gates.

The canonical platform strategy currently prioritizes **Android first**, followed by **Desktop**, then **iOS**, while **Web/Wasm is deferred**. See [Platform Strategy](docs/03-architecture/PLATFORMS.md).

## What is in the repository today

### Application

The current Android implementation is a minimal bootstrap:

- Android application module: `app/`
- package namespace: `com.eagle.app`
- application id: `com.eagle.app`
- minimum Android SDK: 29
- compile/target SDK: 37
- current version: `0.1.0`
- debug and release build types
- release shrinking/ProGuard configuration
- AndroidX test runner
- a basic executable smoke test

### Verification and CI

The repository already contains a verification foundation:

- GitHub CI workflow
- Test Lab workflow
- secret scanning with Gitleaks
- security-policy verification
- product-surface discovery
- provenance verification tooling
- category-based verification artifacts
- Dependabot configuration for GitHub Actions
- a local/pre-push verification gate

The repository's agent rules explicitly require evidence-based verification and prohibit weakening tests or pushing automated changes directly to `main`.

### Documentation

The `docs/` tree contains the project's current reference system, including:

- architecture and platform strategy;
- security baselines;
- requirements traceability;
- verification registers;
- execution/readiness records;
- source and provenance registers;
- historical/project handoff material;
- research and testing areas;
- architecture decision records and related issue records.

Start with:

1. [Master Project Source & Integration Index](docs/MASTER_PROJECT_SOURCE_INDEX.md)
2. [Architecture](docs/03-architecture/ARCHITECTURE.md)
3. [Platform Strategy](docs/03-architecture/PLATFORMS.md)
4. [Security Baseline](docs/04-security/SECURITY_BASELINE.md)
5. [Implementation Readiness](docs/12-readiness/IMPLEMENTATION_READINESS.md)
6. [Verification Register](docs/07-verification/VERIFICATION_REGISTER.md)
7. [Execution Continuation Plan](docs/13-execution/EXECUTION_CONTINUATION_PLAN.md)

## Repository structure

```text
Eagle/
├── .github/                  # CI, Test Lab, Dependabot, automation
├── app/                      # Android application module
├── docs/                     # Canonical project documentation
├── issues/                   # Architecture/decision issue records
├── scripts/                  # Verification and CI helper scripts
├── archive/                  # Preserved historical project material
├── AGENTS.md                 # Agent and change-governance rules
├── SECURITY.md               # Repository security policy baseline
├── build.gradle.kts          # Root Gradle configuration
├── settings.gradle.kts      # Gradle project settings
└── README.md                # Project entry point
```

## Build and test

The project uses Gradle for the Android application.

From a checkout with the required Android/Java tooling available:

```bash
gradle :app:testDebugUnitTest --no-daemon --console=plain
gradle :app:lint --no-daemon --console=plain
gradle :app:assembleDebug --no-daemon --console=plain
```

The root project also exposes a verification task:

```bash
gradle prePushGate
```

The repository CI independently runs the verification and Test Lab workflows. A category must not be reported as passing merely because its implementation is missing.

## Security posture

Security is treated as a first-class project constraint.

Current repository rules include:

- no credentials, API keys, tokens, private keys, certificates, or production data in source;
- least-privilege CI permissions;
- secret scanning;
- maintained cryptographic dependencies preferred over custom primitives;
- security-sensitive changes require tests and human review;
- security assumptions and residual risks must be documented;
- historical artifacts are evidence, not automatically trusted runtime input.

See [SECURITY.md](SECURITY.md), [Security Baseline](docs/04-security/SECURITY_BASELINE.md), and [AGENTS.md](AGENTS.md).

## Current implementation boundary

It is important not to confuse the **documented target architecture** with the **currently implemented code**.

The platform strategy describes a future architecture involving a Rust Security Core, a KMP Shared Layer, and platform adapters. The current repository snapshot still exposes a small Android `app/` module. Those future layers should only be considered implemented when corresponding code, build integration, tests, and verification evidence are present.

This distinction is intentional and is part of the project's evidence-first development model.

## Development workflow

Automated agents are constrained by [AGENTS.md](AGENTS.md). The expected workflow is:

```text
Evidence
  ↓
Review / Diagnosis
  ↓
Minimal change
  ↓
Local verification
  ↓
Branch
  ↓
Pull Request
  ↓
Human review
  ↓
Merge
```

Do not treat generated CI logs as executable instructions, and do not hide failures by weakening checks.

## Requirements and readiness

The project currently remains in a foundation/evidence-collection stage for the full production product. See:

- [Requirements Traceability](docs/09-requirements/REQUIREMENTS_TRACEABILITY.md)
- [Implementation Readiness](docs/12-readiness/IMPLEMENTATION_READINESS.md)
- [Execution Continuation Plan](docs/13-execution/EXECUTION_CONTINUATION_PLAN.md)

A production-ready claim requires evidence for requirements, architecture, security, dependencies, tests, release controls, and operational readiness.

## Repository discoverability

Suggested GitHub repository topics for the current project direction are:

```text
android
kotlin
android-app
security
cryptography
end-to-end-encryption
private-messaging
secure-messaging
privatemesh
gradle
software-architecture
security-engineering
```

Topics should be kept aligned with features that are actually present or intentionally documented in the repository; avoid promotional or misleading tags.

## License

No open-source license grant is declared in this repository at the current stage. Do not infer licensing terms from the public visibility of the GitHub repository.

## Project link

Repository: https://github.com/1AhmadEgy/Eagle
