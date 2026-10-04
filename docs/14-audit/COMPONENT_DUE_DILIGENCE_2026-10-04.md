# Eagle — External Component Due Diligence — 2026-10-04

## Scope

This is the first evidence-based survey of mature external components that may reduce reinvention for Eagle.

**Important:** a candidate being mature or widely used does not automatically make it suitable for Eagle. No component in this document is an approved production dependency unless the Decision column explicitly says **Adopted**.

Evaluation order:

`Requirement → Architecture Fit → Security History → Exact Version → License → Dependency Review → Platform Support → Test Evidence → Failure Behavior → Operational Fit → Exit Strategy`

## Findings

| Area | Candidate | Current evidence | Decision | Constraint |
|---|---|---|---|---|
| Device key protection | Android Keystore / KeyMint | Platform-supported key container; key material can remain non-exportable; supports authorization constraints and optional StrongBox | **Baseline candidate** | Threat model must define algorithms, fallback, user-auth rules, and attestation needs |
| E2EE protocol | Signal libsignal | Implements Signal Protocol/Double Ratchet in Rust and exposes Java/Swift/TypeScript APIs; project explicitly says use outside Signal is unsupported | **Rejected for direct adoption at this stage** | Do not depend on an unsupported external-use API without an explicit support/licensing/security review |
| Symmetric primitives | RustCrypto AEADs | Mature collection of pure-Rust AEAD implementations including AES-GCM-SIV/AES-GCM | **Pending** | Primitive-level library only; does not define Eagle's protocol, identity, ratchet, replay, or key lifecycle |
| Rust crypto backend | aws-lc-rs | Uses AWS-LC; current release line is actively maintained; Apache-2.0/ISC; Rust API compatible with ring-style interfaces | **Pending** | C/C++ build surface and exact platform matrix must be tested for Eagle targets |
| Mesh/networking | rust-libp2p | Established Rust libp2p implementation with security policy; multiple high-severity advisories were published in 2026 | **Pending with heightened scrutiny** | Pin patched exact version, enforce resource limits, define protocol boundary; never equate libp2p with Eagle's Mesh security model |
| Encrypted local DB | sqlcipher-android 4.19.1 | Current Android project; supports API 23+ and current release is 4.19.1; 16 KB page-size support is part of current lineage | **Pending** | Only adopt if local encrypted relational persistence is an explicit requirement; not a substitute for E2EE |
| Shared Kotlin logic | Kotlin Multiplatform | Android and iOS core KMP targets are documented as Stable | **Architecture candidate** | Do not migrate until shared-vs-native boundaries and native security APIs are defined |
| Rust ↔ Kotlin/Swift bridge | Mozilla UniFFI | Used by Mozilla in Firefox mobile/desktop to bind Rust components to Kotlin/Swift | **Architecture candidate** | ABI/API lifecycle and generated-binding tests required |
| Rust dependency advisories | cargo-audit / RustSec | RustSec advisory DB powers cargo-audit; intended for Cargo.lock vulnerability auditing | **Tool candidate** | Applicable only after a Rust dependency graph exists |
| License/advisory/source policy | cargo-deny | Checks licenses, bans, advisories, and allowed sources; current project docs expose exact configuration examples | **Tool candidate** | Add after Rust workspace/dependency policy is frozen |
| Source/lockfile vulnerability scan | OSV-Scanner | Scans source, lockfiles, SBOMs and git directories against OSV | **Tool candidate** | Can complement language-specific SCA; avoid duplicate noisy gates |
| Rust release dependency provenance | cargo-auditable | Embeds dependency tree data into Rust binaries for later audit | **Tool candidate** | Valuable for native/Rust release artifacts once Rust exists |
| Protocol fuzzing | cargo-fuzz | Uses LLVM libFuzzer; supports fuzz targets, minimization and coverage | **Tool candidate** | Requires an actual Rust crate/protocol target; CI platform constraints must be accepted |

## Security-specific conclusions

### 1. Android Keystore should be treated as a platform primitive, not as the E2EE protocol

Android documents that Keystore key material can remain non-exportable and can be bound to secure hardware such as a TEE or StrongBox when supported.

Eagle should use platform key protection for local device-held secrets only after the identity/key lifecycle requirements are fixed.

### 2. libsignal is not a “free Signal implementation” for Eagle

The official libsignal repository states that its products are used by Signal clients/servers and that use outside Signal is unsupported.

Therefore Eagle should not silently copy or depend on libsignal merely because it implements Double Ratchet. Any future reconsideration requires explicit compatibility, support, licensing, and security due diligence.

### 3. libp2p is a transport/networking building block, not Eagle's security model

The Rust libp2p project is active and has a documented security policy, but its security advisories demonstrate that the dependency must be continuously pinned and patched.

Even when adopted, Eagle must own:

- peer identity semantics;
- authorization;
- message envelope authentication;
- replay/duplication policy;
- routing trust;
- quotas/backpressure;
- metadata exposure;
- failure and partition semantics.

### 4. SQLCipher is local-storage protection, not end-to-end encryption

Encrypted database storage can protect data at rest on a device. It does not establish sender-to-recipient E2EE, forward secrecy, identity verification, or Mesh trust.

### 5. KMP + UniFFI remain architecture options

Kotlin Multiplatform is currently Stable for Android and iOS. UniFFI is used at scale by Mozilla for Rust functionality exposed to Kotlin/Swift.

This supports the feasibility of a shared-Rust/shared-logic architecture, but not the conclusion that Eagle must use it.

## Exact-version policy

Never add:

- `latest`;
- floating `+` versions;
- unreviewed Git dependencies;
- copied vendored code without provenance.

Every accepted dependency must be recorded with:

- exact version;
- source URL;
- commit/tag when source-controlled;
- license;
- advisory review date;
- transitive dependency review;
- platform test result;
- integration boundary;
- known failure modes;
- removal/replacement plan.

## Recommended reuse order

1. Prefer platform primitives first (for example Android Keystore).
2. Prefer mature, narrow libraries over whole application copies.
3. Keep protocol composition under Eagle ownership.
4. Add Mesh/networking libraries only after trust boundaries and abuse controls are modeled.
5. Add supply-chain scanners before the first security-sensitive dependency lands.
6. Add protocol fuzzing with the first canonical binary/envelope parser.

## Not yet approved

No external cryptography, E2EE, Mesh, database, KMP, or UniFFI dependency is being added to `app/build.gradle.kts` or to a Rust workspace by this document.

The correct next step is requirement freeze → component contract → exact-version approval → integration test → security review.
