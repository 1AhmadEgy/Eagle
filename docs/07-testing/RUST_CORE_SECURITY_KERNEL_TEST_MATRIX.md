# Rust Core / Security Kernel Test Matrix

RK-01 unknown trust cannot authorize
RK-02 trust elevation requires the internal verified seam
RK-03 session establishment requires authenticated state
RK-04 downgrade is rejected without mutation
RK-05 unsupported version is rejected without mutation
RK-06 protocol upgrade is bounded and monotonic
RK-07 rekey is state guarded
RK-08 revocation closes the context
RK-09 revoked and replaced devices fail closed
RK-10 identifier bounds are enforced
RK-11 ciphertext is non-empty and bounded
RK-12 frame payload length is exact
RK-13 administrative capability is denied
RK-14 invalid configuration is rejected
RK-15 unsafe Rust is forbidden

Required CI:
cargo fmt --all -- --check
cargo test --workspace --locked
cargo clippy --workspace --all-targets --locked -- -D warnings

Fuzzing, property tests, protocol vectors and interoperability remain gated on approved cryptographic and protocol contracts.
