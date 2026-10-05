# Eagle — Identity & Trust Deep Research / 2026-10-05

**Scope:** Identity & Trust Engineering only  
**Baseline:** `main @ abfc263e6ac28ff7b19a40a4d8e1da93c565a6e9`  
**Purpose:** update security requirements from current authoritative/public security guidance without selecting Eagle's cryptographic protocol or key-management implementation.

## Executive findings

### 1. Pairing authentication must be cryptographically session-bound

NIST SP 800-63B Revision 4 distinguishes phishing resistance from merely displaying or manually entering an authentication code. Manual entry of an authenticator output does not by itself bind that output to the specific session and is therefore not sufficient as the sole phishing-resistant mechanism. Channel binding is a stronger pattern: the authenticated channel identifier is irreversibly bound to the authenticator output and the verifier checks that binding.

**Eagle consequence:** QR/short-code comparison is a human-verification aid, not the root of authentication. The final P2P pairing protocol must cryptographically bind approval to the exact session/transcript/endpoints. This remains delegated to Protocol/Key Management.

Source:
- NIST SP 800-63B Rev. 4: https://pages.nist.gov/800-63-4/sp800-63b.html

### 2. Identity continuity must survive key-change events conservatively

Signal documents a useful operational pattern: when a previously verified contact key changes, the application warns the user and can require manual re-verification before treating the new key as verified. Signal's current key-transparency work further shows the value of detecting unauthorized key association changes without exposing plaintext user data.

**Eagle consequence:** the existing `pending_identity` quarantine design is correct and should remain fail-closed. The verified identity must remain authoritative until the exact candidate is independently reverified. Optional future transparency mechanisms may improve detection, but they must not become a hidden central trust root for Eagle's P2P model.

Sources:
- Signal Safety Number / key-change guidance: https://support.signal.org/hc/en-us/articles/360007060632-What-is-a-safety-number-and-why-do-I-see-that-it-changed
- Signal Automatic Key Verification / key transparency: https://signal.org/blog/automatic-key-verification/

### 3. Bound device authenticators must be unique within an account context

NIST SP 800-63C describes bound authenticators as being associated with an account and requires uniqueness so that two subscribers cannot present the same authenticator for separate accounts.

**Eagle consequence:** Device identity must remain unique to a membership context, and cross-account device reuse must be rejected even when other metadata appears valid. The current registry guard and negative tests directly support this requirement.

Source:
- NIST SP 800-63C Rev. 4: https://pages.nist.gov/800-63-4/sp800-63c.html

### 4. Hardware attestation is evidence, not identity

Android documentation describes hardware-backed key attestation and emphasizes checking the attestation challenge/nonce to prevent replay and checking certificate revocation. Apple App Attest similarly creates hardware-protected keys and uses Apple's service to attest application instances.

**Eagle consequence:** platform evidence should remain an assurance signal. It must never silently create account trust, replace P2P membership proof, or become a dependency that breaks the core offline/P2P trust ceremony.

Sources:
- Android hardware-backed attestation: https://developer.android.com/privacy-and-security/security-key-attestation
- Android Keystore attestation guidance: https://developer.android.com/identity/digital-credentials/credential-issuer/keystore-attestation
- Apple App Attest: https://developer.apple.com/documentation/devicecheck/establishing-your-app-s-integrity

### 5. Epoch-based membership lifecycle is a proven security pattern, but authorization must be authoritative

RFC 9420 models authenticated group membership as a sequence of epochs, with signed member descriptions and explicit add/remove state transitions. KeyPackage objects are intended to be single-use and signed.

**Eagle consequence:** epoch monotonicity, explicit membership transitions, and non-reuse semantics are useful reference patterns. Eagle must not adopt MLS mechanics implicitly; ADR-0008 remains the protocol authority.

Source:
- RFC 9420 — Messaging Layer Security: https://www.rfc-editor.org/rfc/rfc9420.html

## Security changes derived from research

1. Human-readable pairing codes must never be treated as sufficient cryptographic authentication.
2. Pairing approval must bind to exact authenticated session context.
3. Identity replacement remains quarantined until exact-candidate reverification succeeds.
4. Cross-account and duplicate device identities remain hard-deny cases.
5. Platform attestation remains advisory and replay-sensitive.
6. Future or externally asserted epochs cannot advance local authorization state.
7. Epoch observation is kept inside the Security Core API boundary; external callers cannot directly mutate the registry's account epoch state.
8. Missing protocol/key-management evidence remains PENDING rather than PASS.

## New tests required at protocol integration

- pairing approval bound to transcript/session identifier;
- same code presented in a different session → DENY;
- transcript modification after human comparison → DENY;
- attestation challenge replay → DENY;
- attestation for one device presented for another identity → DENY;
- identity-key change without exact candidate proof → QUARANTINE;
- duplicate authenticator across account contexts → DENY;
- remote/future epoch cannot advance local authority;
- rollback to a previously accepted epoch → DENY.

## Non-decisions

This research does **not** select:
- a signature algorithm;
- a key encapsulation algorithm;
- a ratchet;
- a concrete serialization format;
- a recovery threshold;
- a transport library;
- an MLS/Signal/Noise adoption.

Those remain under the corresponding canonical ADRs and specialist scopes.

## Research status

**Result:** security requirements strengthened; no crypto/protocol invention introduced.  
**Release effect:** no release gate is weakened. Identity & Trust remains **BLOCKED / NO-GO** until all external security dependencies and exact-head verification evidence are satisfied.
