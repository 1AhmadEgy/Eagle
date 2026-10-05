# Android KeyStore Security Adapter Contract

**Status:** Implemented foundation; production authorization remains gated.

## Scope

This adapter is for local encrypted application state only. It is not the Eagle messaging protocol, not the Signal/PQXDH implementation, and not a transport credential.

## Security boundary

- AES-256 keys are generated into the Android Keystore provider.
- Application code receives only ciphertext/plaintext results from the adapter; key material is not returned by the adapter API.
- Storage operations require an opaque scope-bound handle carrying account, device, and trust epoch; raw Keystore aliases are not accepted by the encrypt/decrypt/delete API.
- Key deletion is explicit.
- Every storage encryption operation uses AES-GCM with an authentication tag.
- Associated data can bind encrypted state to an authenticated context.
- requireStrongBox=true requests StrongBox-backed key generation and does not silently fall back to a weaker custody mode.

## Fail-closed rules

- zero/invalid account or device identifiers are rejected;
- negative trust epochs are rejected;
- empty plaintext is rejected;
- invalid GCM IV sizes are rejected;
- ciphertext without an authentication tag is rejected;
- missing Keystore keys fail rather than generating an unexpected key during decrypt;
- StrongBox is never silently downgraded when explicitly required.

## Explicit limitations

This implementation does not yet prove:

- hardware backing on a specific device;
- identity-key custody;
- attestation verification;
- encrypted database/WAL/backup/temp-file coverage;
- secure deletion guarantees on flash storage;
- recovery semantics;
- Rust/UniFFI integration;
- cross-platform key-custody parity;
- message E2E, forward secrecy, post-compromise security, or protocol interoperability.

Those remain separate release-gated evidence items.


## Attestation requirements

The Android adapter must expose evidence for the actual security level of each protected key, not merely whether Keystore exists.

Minimum evidence for an identity/security-anchor key:

1. key is non-exportable;
2. KeyInfo.getSecurityLevel() reports TRUSTED_ENVIRONMENT or STRONGBOX when the selected profile requires hardware backing;
3. when remote verification is required, validate the attestation chain, attestation challenge, authorization data, verified boot state and certificate revocation status;
4. record rollback-resistance status when supported and required by the selected key lifecycle profile;
5. never label an unsupported PQXDH key as StrongBox/Secure-Hardware protected solely because another device key is hardware-backed.

StrongBox is preferred for the highest-assurance profile when the required algorithm/operation is supported. If it is unavailable for that operation, the adapter must either use the explicitly approved fallback profile or fail closed; it must not silently upgrade the assurance claim.
