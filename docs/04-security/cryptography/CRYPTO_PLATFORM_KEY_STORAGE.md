# Eagle — Platform Key Storage Contract

## Android

Primary boundary: Android Keystore. Prefer hardware-backed key protection when supported; treat StrongBox as an optional higher-isolation capability. Record actual security level and test invalidation/restore behavior. Attestation is assurance evidence, not the messaging identity itself.

## Apple

Primary boundary: Keychain. Use Secure Enclave only for key types and operations supported by the exact target OS/device. Do not claim Secure-Enclave residency for unsupported protocol algorithms. Test migration, restore and invalidation behavior.

## Desktop

Use native protected key/credential facilities on each supported operating system. Application-encrypted state is fallback only and receives a lower assurance label when hardware protection is absent.

## Contract

- Crypto core consumes abstract key-domain handles.
- UI does not access private key material.
- Mesh/transport does not access private key material or plaintext.
- Storage encryption keys remain separate from messaging keys.
- Recovery keys remain separate from messaging and storage keys.
- Missing required platform protection causes the relevant operation to fail closed.

## Assurance labels

HW_BACKED — hardware-backed protection verified.
OS_PROTECTED — OS-protected, hardware backing not established.
APP_ENCRYPTED — application-level at-rest encryption only.
UNTRUSTED — required protection not met; reject security-sensitive operation.
