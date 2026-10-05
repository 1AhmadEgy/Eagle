# Eagle Canonical Serialization Boundary

**Status:** Implemented foundation; production wire authorization remains pending protocol approval and interoperability evidence.

## Format

- Standard: CBOR.
- Current implementation: minicbor 2.3.0.
- Dependency is pinned in Cargo.lock with its registry checksum.
- Wire value is a fixed seven-element definite-length CBOR array.
- Byte strings are definite-length and bounded by the existing protocol limits.

## Security properties

1. Input size is bounded before parsing.
2. The field count is exact.
3. Field order is fixed and schema-owned.
4. Indefinite-length arrays and byte strings are rejected.
5. Identifier and ciphertext limits are enforced.
6. Trailing bytes are rejected.
7. Parsed values are re-encoded and compared byte-for-byte; non-canonical representations are rejected.
8. Protocol version validation remains delegated to the protocol boundary.

## Current seven fields

1. message_id
2. conversation_id
3. sender_device_id
4. recipient_device_id or null
5. ciphertext
6. protocol_version
7. created_at_epoch_ms

## Security limitation

This is a wire-format foundation, not a complete authenticated-message protocol. No claim of end-to-end encryption, forward secrecy, post-compromise security, or cross-language interoperability is granted by this implementation alone.