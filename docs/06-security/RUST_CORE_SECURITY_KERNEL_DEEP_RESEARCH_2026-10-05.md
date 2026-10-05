# Rust Core / Security Kernel — Deep Security Research

**Date:** 2026-10-05  
**Scope:** Rust Core / Security Kernel only  
**Branch:** `execution/rust-core-security-kernel-complete-2026-10-05`  
**Research head:** `c987bb283d8ca9ffac6cbf654c8fc0100711ab74`  
**PR:** #81 (draft)

## 1. Executive finding

The current Rust Core is a deterministic security-policy/state-machine scaffold, not yet a production cryptographic security authority.

The strongest property of the current design is the reduced public mutation surface:

- security-sensitive state is private;
- trust promotion is not publicly callable;
- authorization requires trusted + established state;
- revocation/replacement fail closed;
- protocol acceptance is bounded;
- unsafe Rust is forbidden.

The most important newly identified design risk was authority-state duplication through `Copy`/`Clone`. A caller holding a trusted value could retain an independently usable snapshot after the original object was revoked. This was corrected by removing `Copy`/`Clone` from `SecurityContext`, `Device`, and `Session`.

## 2. Evidence-driven findings

### RUST-K-001 — Authority duplication through Copy/Clone — FIXED

`SecurityContext`, `Device`, and `Session` previously derived `Clone, Copy`. Rust's `Copy` semantics permit implicit bitwise duplication, while `Clone` explicitly creates another value; subsequent mutation of one independently-owned clone does not mutate the other. For a security authority whose revocation must invalidate the authority represented by the object, that semantics is unsafe for the intended model.

Remediation:

- `SecurityContext`: no `Clone`/`Copy`;
- `Device`: no `Clone`/`Copy`;
- `Session`: no `Clone`/`Copy`.

This makes ownership transfer explicit and prevents accidental stale authority snapshots.

Research references:
- https://doc.rust-lang.org/stable/core/clone/trait.Clone.html
- https://doc.rust-lang.org/stable/error_codes/E0382.html

### RUST-K-002 — Device trust and session trust have two state holders — HIGH

The kernel currently models trust in both `DeviceTrustState` and `SecurityContext::trust`. They are not yet cryptographically linked.

Risk:

- a device may be revoked while an independently-held authenticated session context still appears trusted;
- future FFI/platform bindings could accidentally treat the two state machines as separate authorities.

Required architectural outcome:

- define one canonical trust authority;
- bind session authorization to that authority;
- make device revocation invalidate all sessions derived from that device;
- reject stale authorization after replacement or revocation.

This is an integration dependency, not a reason to weaken the current fail-closed API.

### RUST-K-003 — No production trust-elevation path — INTENTIONAL BLOCKER

The only trust-promotion seam is test-only. External callers cannot turn `Pending` into `Trusted`.

This is the correct fail-closed state while ADR-0008/0009/0010 and the identity/trust verifier are unresolved.

Production implementation must not simply make the current test seam public. It needs an authenticated verifier whose inputs and outputs are specified by the accepted identity/protocol/key contracts.

### RUST-K-004 — FrameHeader construction needed a validated public boundary — FIXED

The previous private-field model was security-positive, but the integration test attempted to construct malformed headers directly and therefore failed to compile after the hardening.

The kernel now exposes a constructor that:

1. validates the protocol version;
2. bounds the declared payload length;
3. constructs the header only after validation.

Negative malformed-version behavior remains tested through the constructor's error result rather than by making the fields public.

### RUST-K-005 — Protocol version logic is currently intentionally conservative — MEDIUM

The current version validator rejects versions below `CURRENT_PROTOCOL_VERSION` and above it. This is safe for the current single-version implementation, but it means the configured min/max negotiation window is not yet a true multi-version negotiation protocol.

Before a second wire version is introduced, the protocol design must specify:

- supported-version sets/ranges;
- downgrade detection;
- common-version selection;
- transcript binding of the negotiated version;
- rejection of ambiguous or duplicate offers;
- compatibility and migration rules.

This belongs to the accepted protocol ADR, not ad-hoc kernel expansion.

### RUST-K-006 — Frame flags are fail-closed for protocol v1 — FIXED / FUTURE GATE

`FrameHeader::flags` remains a bounded `u16`, but protocol v1 now accepts only `0`. Any non-zero value is rejected at construction with `UnsupportedFlags`.

When future protocol versions define flags, the accepted protocol ADR must specify:

- reserved bits;
- allowed flags per protocol version;
- unknown-bit handling;
- whether a flag changes parsing or authorization;
- canonical encoding rules.

Unknown security-relevant bits must never silently acquire semantics.

### RUST-K-007 — Replay protection is absent from the current structural envelope — HIGH dependency

The current envelope contains a message identifier and timestamp but does not establish replay resistance.

A production protocol must define:

- authenticated message uniqueness;
- nonce/sequence handling;
- replay windows or durable deduplication;
- device/session binding;
- clock-skew behavior;
- behavior after restart or recovery;
- interaction with offline P2P delivery.

These are protocol/key-management properties and must not be inferred from the current `created_at_epoch_ms` field.

### RUST-K-008 — Bounds are enforced at the object boundary, not yet at the wire parser — MEDIUM

The kernel rejects payloads and identifiers that exceed configured limits after a caller has already supplied them as Rust values.

A production decoder must enforce size limits before unbounded allocation. In hostile P2P input handling, the parser boundary is the actual resource-exhaustion control point.

Required tests:

- declared length larger than available bytes;
- integer conversion overflow;
- nested length amplification;
- repeated malformed frames;
- maximum-valid and maximum+1 cases;
- allocation-budget behavior.

### RUST-K-009 — FFI is a security boundary, not a transport adapter — HIGH

The accepted platform architecture places Rust Core behind UniFFI. UniFFI translates Rust `Result` errors to foreign exceptions and handles Rust panics at generated call boundaries; callback methods that do not return `Result` can panic when foreign code fails.

Implications for Eagle:

- all security-sensitive exported operations should use explicit `Result`-based errors;
- no exported method may expose trust promotion, mutable security state, raw key material, or policy bypass;
- callback interfaces should return compatible `Result` types;
- error messages must not disclose secrets;
- ABI-compatible enums and errors must be versioned deliberately;
- FFI tests must include Kotlin/JVM, Kotlin/Native/Swift, object lifetime, invalid input, and panic/error translation cases.

Research references:
- https://mozilla.github.io/uniffi-rs/next/internals/rust_calls.html
- https://mozilla.github.io/uniffi-rs/next/internals/foreign_calls.html
- https://mozilla.github.io/uniffi-rs/0.27/internals/api/uniffi/ffi/fn.rust_call.html

### RUST-K-010 — Dependency posture is unusually small — POSITIVE

The current kernel has no external Rust runtime dependencies. This minimizes the initial software supply-chain attack surface.

Once cryptography/serialization/FFI dependencies enter the crate, Eagle should add mechanical controls instead of relying on filenames or reputation:

- RustSec vulnerability scanning;
- dependency policy enforcement;
- `cargo vet` audits for deployed dependencies;
- locked dependency graph;
- provenance and version-change review;
- platform-specific dependency review.

RustSec provides `cargo-audit` for vulnerable dependency detection. Cargo Vet records auditable criteria and supports shared audits and differential review.

Research references:
- https://rustsec.org/
- https://mozilla.github.io/cargo-vet/
- https://mozilla.github.io/cargo-vet/audit-criteria.html
- https://mozilla.github.io/cargo-vet/performing-audits.html

### RUST-K-011 — Platform key storage must remain outside the Rust public object model — HIGH dependency

Android Keystore can keep key material non-exportable and can enforce restrictions on key use; supported Android devices may also provide StrongBox-backed KeyMint for stronger isolation.

Apple's Secure Enclave provides a hardware-based key manager. Apple documents that protected private keys can be used by the enclave without exposing plaintext key material to the application.

Therefore the future Eagle key-management design should preserve:

- private-key non-exportability wherever platform capabilities allow;
- explicit key purpose and lifetime;
- authentication/access-control policy;
- device-only protection where required;
- recovery/replacement semantics separate from ordinary account recovery.

Research references:
- https://developer.android.com/privacy-and-security/keystore
- https://developer.apple.com/documentation/cryptokit
- https://developer.apple.com/documentation/Security/protecting-keys-with-the-secure-enclave

### RUST-K-012 — Mobile security verification must cover more than Rust correctness — HIGH dependency

OWASP MASVS treats storage, cryptography, authentication, network, platform interaction, code security, resilience, and privacy as separate control groups.

The Rust kernel can satisfy only a subset of those controls. In particular, platform key storage, backups, logs, app integrity, native bindings, and local persistence remain outside the current crate.

Research references:
- https://mas.owasp.org/MASVS/
- https://mas.owasp.org/MASVS/05-MASVS-STORAGE/
- https://mas.owasp.org/MASVS/controls/MASVS-RESILIENCE-1/

## 3. Security target model

The preferred target is:

```text
Platform Adapter / KMP
        |
        | validated typed API
        v
Rust Security Authority
        |
        +-- cryptographic identity binding
        +-- authenticated peer/session state
        +-- authorization decision
        +-- revocation/replacement authority
        +-- replay/sequence state
        +-- protocol-version state
        |
        v
Platform key custody / secure storage
```

The platform layers must not become parallel security authorities.

## 4. Required verification layers

The current unit/integration suite is necessary but insufficient for production.

Required additions before cryptographic release:

1. deterministic state-machine tests;
2. property tests over legal/illegal transition sequences;
3. fuzzing of all untrusted decoders;
4. serialization round-trip and malformed-input tests;
5. protocol downgrade/replay/concurrency tests;
6. cross-language UniFFI ABI tests;
7. platform key-store integration tests;
8. dependency vulnerability and provenance checks;
9. regression vectors for every security decision;
10. independent security review of the cryptographic boundary.

Missing categories remain PENDING, never PASS.

## 5. Release decision

**Current deterministic kernel implementation:** materially hardened.

**Current verification:** PENDING. PR #81 checks are queued/in progress for head `c987bb283d8ca9ffac6cbf654c8fc0100711ab74`.

**Production security authority:** BLOCKED.

Unresolved blockers:

- approved cryptographic protocol;
- approved key-management design;
- canonical serialization;
- real authenticated trust verifier;
- replay/sequence semantics;
- P2P transport security;
- UniFFI ABI/security implementation;
- platform key-custody integration;
- fuzz/property/interop coverage;
- independent security review.

## 6. Non-negotiable rule

No cryptographic implementation should be added merely to turn the current scaffold green.

The kernel must remain fail-closed until the protocol, key-management, serialization, and identity contracts are accepted and their evidence is available.

## 7. Additional hardening decision — rekey completion

A public `finish_rekey()` operation was identified as unsafe for the pre-cryptographic kernel because the method changed a `Rekeying` session back to `Established` without requiring an authenticated cryptographic proof.

The method is now test/internal-only. Production callers can begin rekey, but an incomplete rekey can only terminate the session through a fail-closed abort path. This prevents a future platform adapter or FFI caller from treating "rekey started" as equivalent to "new keys verified".

This aligns the current scaffold with the intended security property of refusing to continue a session when key-state transition proof is absent.

## 8. Current external reference basis

Rust's ownership model distinguishes implicit `Copy` values from explicit `Clone` values; this supports treating authority-bearing session/device objects as move-only state until a canonical shared authority exists. citeturn987370search7

UniFFI documents explicit handling of Rust `Result` errors and panic boundaries across foreign calls. Security-sensitive Eagle APIs should therefore prefer explicit typed `Result` failures and should not depend on foreign exceptions or callbacks as the trust root. citeturn987370search4turn987370search9

Android Keystore is explicitly designed so key material remains non-exportable while allowing policy restrictions on key use; StrongBox can provide stronger isolation when supported. This supports keeping platform key custody outside the Rust value model and treating the platform store as a constrained key-custody dependency, not an application-level raw key container. citeturn987370search3turn987370search8

OWASP MASVS remains a useful cross-check because it separates storage, cryptography, authentication, network, platform, code, resilience, and privacy controls. The Rust kernel should therefore never be represented as sufficient evidence for the complete mobile security posture. citeturn987370search0turn987370search1

## 9. Updated release conclusion

The deterministic Rust kernel is now materially more fail-closed than the earlier baseline:

- authority-bearing state is move-only;
- public trust elevation is unavailable;
- public rekey completion is unavailable before cryptographic proof exists;
- authentication cancellation cannot grant trust;
- incomplete rekey closes the session;
- frame construction is validated;
- protocol/resource checks remain bounded.

The remaining blockers are architectural/security dependencies, not candidates for "quick fixes" inside the Rust scaffold:
cryptographic protocol selection, key management, canonical serialization, authenticated identity verification, replay/sequence protection, P2P transport security, UniFFI security review, platform key custody, hostile-input parser testing, and independent security review.
