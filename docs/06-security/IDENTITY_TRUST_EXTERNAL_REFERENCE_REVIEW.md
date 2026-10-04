# Eagle — Identity & Trust External Security Reference Review

**Scope:** Identity & Trust only  
**Review date:** 2026-10-05  
**Purpose:** validate platform/key-boundary claims against authoritative external documentation.

## 1. Android Keystore

Android documents that Keystore key material is intended to be difficult to extract and can remain non-exportable. Depending on device support, keys can be hardware-bound and key-use authorizations can be enforced by the Keystore/secure hardware boundary.

**Engineering consequence:** Eagle should treat Android Keystore/StrongBox as a platform key-protection adapter. It must not use hardware state as the account identity root or as a substitute for account/device membership authorization.

Reference:
https://developer.android.com/privacy-and-security/keystore

## 2. Apple Secure Enclave

Apple documents Secure Enclave as a hardware-based key manager where protected private-key material is not handled directly as plaintext by the application. Apple also documents that Secure Enclave support has algorithm/hardware constraints and that keys created there follow lifecycle limitations.

**Engineering consequence:** the Apple adapter must expose capability/assurance evidence rather than assume every identity-key design can be represented directly by Secure Enclave. The final identity-key algorithm remains a Protocol/Key Management decision.

Reference:
https://developer.apple.com/documentation/Security/protecting-keys-with-the-secure-enclave

## 3. Apple App Attest

Apple documents App Attest as a mechanism for an app to obtain a hardware-backed key and have Apple certify it, with the application using the resulting assertions in interactions with its server. Apple also documents that not all devices support the service and that app behavior should account for unsupported devices.

**Engineering consequence:** App Attest is an integrity/assurance signal for server-facing flows. It must not become the root of P2P identity trust because the Eagle trust ceremony must remain meaningful without a central service.

Reference:
https://developer.apple.com/documentation/DeviceCheck/establishing-your-app-s-integrity

## 4. Signal protocol separation

Signal's Double Ratchet specification describes the ratchet as a post-key-agreement messaging mechanism and requires an external key-agreement step to establish its initial shared secret.

**Engineering consequence:** Eagle Identity & Trust must stop at authenticated identity/device authorization. Session/ratchet state belongs to the protocol/crypto workstream and must consume an already-established authenticated identity context.

References:
https://signal.org/docs/specifications/x3dh/
https://signal.org/docs/specifications/doubleratchet/

## 5. Group protocol boundary

RFC 9420 specifies MLS as a key-establishment protocol for asynchronous groups with forward secrecy and post-compromise security.

**Engineering consequence:** group identity/trust behavior must not be improvised inside the Identity & Trust layer. Any future group membership design must bind to the selected group protocol and its authoritative key schedule.

Reference:
https://www.rfc-editor.org/rfc/rfc9420

## 6. Security conclusion

The authoritative references support the current separation:

```text
Platform key protection / assurance
              ↓
Identity & Device cryptographic authority
              ↓
Explicit Trust / Authorization
              ↓
Session / Messaging protocol
```

No external source reviewed here authorizes the project to collapse these layers into a single platform-authentication decision.

## 7. Evidence classification

- External documentation = authoritative reference for platform capability claims.
- Eagle ADRs = authority for final project technology decisions after approval.
- Identity specification = canonical project security contract once approved.
- Rust Security Core = implementation evidence.
- CI/Test Lab = executable verification evidence.

