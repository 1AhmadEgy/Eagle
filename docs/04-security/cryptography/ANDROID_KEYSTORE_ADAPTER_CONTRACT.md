# Android KeyStore Security Adapter Contract

**Status:** Implemented foundation; production authorization remains gated.

## Scope

This adapter is for local encrypted application state only. It is not the Eagle messaging protocol, not the Signal/PQXDH implementation, and not a transport credential.

## Security boundary

- AES-256 keys are generated into the Android Keystore provider.
- Application code receives only ciphertext/plaintext results from the adapter; key material is not returned by the adapter API.
- Storage-key aliases are scoped to account, device, and trust epoch.
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
