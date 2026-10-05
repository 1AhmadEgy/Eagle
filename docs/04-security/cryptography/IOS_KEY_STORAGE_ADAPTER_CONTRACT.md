# iOS Key Storage Adapter Contract

## Purpose

Define the minimum security contract for iOS/iPadOS key custody without claiming that Secure Enclave supports protocol keys it cannot represent.

## Required properties

- Keychain is the baseline persistent secret store.
- Identity/security-anchor keys must be device-bound where the selected profile requires it.
- Secure Enclave is preferred for supported hardware-bound keys and operations.
- The adapter must expose the actual algorithm/key type and custody level.
- The adapter must never report Secure Enclave custody for a key that was generated outside the enclave or for an unsupported key type.
- Recovery keys remain a separate domain and are never silently reused as messaging keys.
- Keychain access controls must be explicit and versioned.
- Deletion/revocation must be reflected in the lifecycle registry before a handle can be reused.

## Secure Enclave constraint

Apple documents that Secure Enclave private-key support is limited to supported P-256 operations and that pre-existing keys cannot be imported. Therefore a Signal/PQXDH implementation must not pretend that its protocol identity key is Secure-Enclave-backed unless the exact algorithm and operation have been independently verified on the target OS/device.

A separate hardware-bound attestation/security anchor may be used to establish device security properties while the protocol provider uses an approved non-exportable Keychain representation for algorithms not supported by Secure Enclave.

## Fail-closed rules

1. Unsupported algorithm + requested hardware custody => reject.
2. Missing device-binding evidence for a profile that requires it => reject.
3. Hardware capability absent => use only an explicitly approved lower-assurance profile or reject; never silently downgrade the declared security level.
4. Key material must never be logged, serialized into application telemetry, or exported to the transport layer.
