# Eagle — Device Identity / Android Key Boundary v0.1

**Audit date:** 2026-10-04  
**Status:** Security design evidence — implementation not yet approved  
**Scope:** Android long-lived device identity key boundary only.

## 1. Current conclusion

Eagle should use Android Keystore as the Android platform key boundary for long-lived device identity material, with StrongBox used opportunistically when the device supports it and the selected key operation is compatible.

This document does not select the cryptographic messaging protocol, identity schema, or attestation acceptance policy. Those remain separate decisions.

## 2. Evidence

Android provides Keystore-backed key generation and attestation facilities. Current Android documentation exposes KeyGenParameterSpec attestation challenges and StrongBox-specific behavior, and documents that StrongBox may be unavailable on a device rather than being universally assumed.

Android documentation also makes an important distinction between different attestation concepts: individual device-ID attestation is restricted to device-owner / managed-device contexts, so Eagle must not design its normal consumer identity flow around those restricted APIs.

References:
- Android DevicePolicyManager / attested key-pair documentation: https://developer.android.com/reference/kotlin/android/app/admin/DevicePolicyManager
- Android Protected Confirmation / Keystore signing: https://developer.android.com/privacy-and-security/security-android-protected-confirmation

## 3. Required Eagle boundary

The intended boundary is:

Protocol / Rust Security Core
→ asks for a permitted key operation
→ Android Key Adapter
→ Android Keystore / KeyMint / StrongBox

Private key material must not be copied into ordinary app storage or passed through the KMP/domain layer.

The adapter may expose:
- key alias / stable local identifier;
- public key / certificate chain when the approved protocol requires it;
- sign/decrypt operations permitted by the key policy;
- hardware/security-level metadata necessary for policy decisions.

The adapter must not expose raw private-key bytes.

## 4. StrongBox policy

StrongBox is an optional stronger hardware security boundary, not a universal availability assumption.

The product policy should therefore be:

1. Prefer StrongBox when supported and when the approved key profile explicitly permits it.
2. Fail closed only when the approved protocol/security policy requires hardware-backed StrongBox specifically.
3. Otherwise permit a Keystore-backed implementation consistent with the approved threat model, while recording the actual security level for diagnostics/policy.
4. Never silently treat software-backed fallback as equivalent to StrongBox.

## 5. Attestation policy

Attestation must be tied to the exact Eagle identity enrollment protocol.

Before implementation, Eagle must define:
- challenge construction and freshness;
- accepted certificate roots;
- verified boot / security level requirements;
- package/application identity checks;
- revocation and re-enrollment behavior;
- server-side verification rules;
- privacy impact of any device-identifying attestation fields;
- behavior on devices without supported attestation.

A clean local key-generation result is not sufficient evidence that the device is trusted.

## 6. What remains blocked

The following must be selected before production identity-key generation is enabled:

- 1:1 E2E protocol;
- identity-key algorithm/profile;
- prekey/session model;
- public identity encoding;
- enrollment protocol;
- attestation acceptance rules;
- recovery/revocation model;
- multi-device semantics;
- conformance vectors and negative tests.

## 7. Current Eagle status

**Android Keystore boundary:** architecture candidate / approved direction for evaluation.  
**Actual device identity key generation:** NOT IMPLEMENTED.  
**Attestation verification:** NOT IMPLEMENTED.  
**Production identity enrollment:** NOT IMPLEMENTED.

This is intentionally stricter than the architecture prose: platform capability is evidence, not product implementation.