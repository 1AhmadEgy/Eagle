# Desktop Key Storage Adapter Contract

## Purpose

Define a platform-native custody contract for desktop builds while preserving the same key-domain and lifecycle invariants.

## Required properties

- Prefer the platform-native protected credential/key store over application-managed plaintext files.
- Linux: use Secret Service-compatible secure storage where available; record whether the backend is user-session protected and whether hardware-backed custody exists.
- Windows: use the OS protected credential/cryptographic facilities appropriate to the key operation; do not treat an application file as an equivalent.
- macOS: use Keychain and Secure Enclave only for supported key types/operations.
- The adapter must expose the real custody level to the provider approval gate.
- Protocol private keys, session keys and recovery material remain separate domains.
- No private key material enters logs, crash reports, analytics or transport/discovery components.

## Fail-closed rules

- Missing native protected storage => reject production profile unless an explicitly reviewed equivalent exists.
- Unsupported hardware custody => reject that hardware-assurance profile rather than silently relabeling software custody.
- Migration/import must preserve key purpose, generation, epoch and revocation state.
- Destruction must be irreversible at the lifecycle layer and reflected in the native store.


## Eagle crypto contract addition


Define a platform-native custody contract for desktop builds while preserving the same key-domain and lifecycle invariants.

## Required properties

- Prefer the platform-native protected credential/key store over application-managed plaintext files.
- Linux: use Secret Service-compatible secure storage where available; record whether the backend is user-session protected and whether hardware-backed custody exists.
- Windows: use the OS protected credential/cryptographic facilities appropriate to the key operation; do not treat an application file as an equivalent.
- macOS: use Keychain and Secure Enclave only for supported key types/operations.
- The adapter must expose the real custody level to the provider approval gate.
- Protocol private keys, session keys and recovery material remain separate domains.
- No private key material enters logs, crash reports, analytics or transport/discovery components.

## Fail-closed rules

- Missing native protected storage => reject production profile unless an explicitly reviewed equivalent exists.
- Unsupported hardware custody => reject that hardware-assurance profile rather than silently relabeling software custody.
- Migration/import must preserve key purpose, generation, epoch and revocation state.
- Destruction must be irreversible at the lifecycle layer and reflected in the native store.
