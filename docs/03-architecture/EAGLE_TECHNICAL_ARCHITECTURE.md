# Eagle — Technical Architecture

## Repository boundaries

```text
apps/
  android/
  ios/
core/
  src/
    identity/
    trust/
    session/
    protocol/
    policy/
    recovery/
    deletion/
transport/
storage/
api/
shared/
test-vectors/
integration-tests/
fuzz/
security/
docs/
scripts/
```

The existing Android application remains in `app/` during migration; the target structure above is the engineering boundary, not a claim that every module already exists.

## Dependency direction

```text
Platform/UI -> API -> Security Kernel
                      |-> Identity/Trust
                      |-> Protocol/Session -> Transport
                      |-> Storage
```

Rules:
1. No UI-to-crypto dependency.
2. No AI/tool path can bypass policy checks.
3. Protocol profiles are versioned and testable.
4. Persistence paths are enumerated before deletion is declared complete.
5. Security-sensitive changes require traceability to an approved decision and evidence.

## First executable slice

The first executable slice is `core/`: a Rust library containing deterministic security-state transitions and negative tests. It deliberately contains no cryptographic code.
