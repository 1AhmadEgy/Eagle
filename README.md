# Eagle

Eagle is a security-first, applications-only, multi-platform project targeting Android, Desktop (Windows/macOS/Linux), and iOS.

## Current canonical status

The canonical implementation is the exact `main` commit:

```text
46aa86b6d71d34396b86b35124392cb3ba49c2e9
```

That snapshot currently represents an Android implementation skeleton plus project documentation, CI/scripts, and security/test scaffolding.

The following are **not established as production implementation on this exact main commit**:

- Rust Security Core production workspace;
- KMP shared production layer;
- iOS application implementation;
- Desktop application implementation;
- production E2EE provider/runtime;
- production P2P runtime;
- production release verification.

## Evidence authority

- `main` exact commit = canonical implementation evidence.
- Pull requests = candidate evidence only.
- Non-main branches = candidate/non-canonical evidence.
- Archives and conversation material = provenance/reference unless independently promoted.
- `PR PASS != main PASS`.

## Platform scope

Supported application targets:

- Android
- Desktop: Windows, macOS, Linux
- iOS

**Web/Wasm is out of scope.**

## Release status

```text
Sprint 0 = OPEN
Sprint 1 = BLOCKED
Release   = NO-GO
```

## Verification rule

A component is not considered implemented, verified, or released merely because a proposal, branch, PR, document, or archive describes it. Each transition requires explicit evidence in the repository verification register.
