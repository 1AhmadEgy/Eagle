# Rust Core / Security Kernel Test Matrix

**Date:** 2026-10-05  
**Scope:** deterministic Rust Security Kernel only

| ID | Security property | Evidence |
|---|---|---|
| RK-01 | unknown trust cannot authorize | unit + integration |
| RK-02 | public callers cannot elevate trust | compile-visible API boundary + negative tests |
| RK-03 | session establishment requires trusted authenticated state | unit + integration |
| RK-04 | downgrade rejected without state mutation | unit + integration |
| RK-05 | unsupported version rejected without state mutation | unit + integration |
| RK-06 | negotiated protocol cannot exceed configured maximum/current version | configuration + negotiation tests |
| RK-07 | rekey requires established trusted session | unit |
| RK-08 | incomplete rekey closes the session | unit |
| RK-09 | authentication cancellation returns to untrusted/idle state | unit + integration |
| RK-10 | revocation closes the context | unit |
| RK-11 | revoked/replaced devices fail closed and cannot transition back | unit + integration |
| RK-12 | identifier bounds enforced at construction | unit + integration |
| RK-13 | envelope ciphertext non-empty and bounded | unit + integration |
| RK-14 | frame payload length is exact and header version is checked | unit + integration |
| RK-15 | administrative capability denied | integration |
| RK-16 | invalid protocol configuration rejected | unit + integration |
| RK-17 | unsafe Rust forbidden | crate-level `forbid(unsafe_code)` |
| RK-18 | authority-bearing values are not Copy/Clone | compile/API boundary + source regression |
| RK-19 | bounded frame decoder rejects truncation before slicing | unit | 
| RK-20 | bounded frame decoder returns borrowed payload without allocation | unit | 
| RK-21 | send sequence never wraps | test-only sequence seam | 
| RK-22 | receive window rejects duplicates | test-only replay seam |
| RK-23 | receive window rejects sequences outside window | test-only replay seam |
| RK-24 | replay state is advanced only through authenticated seam | API boundary + test-only constructor |
| RK-25 | effective trust reflects current device authority | unit |
| RK-26 | revoked device cannot mutate negotiated protocol | unit |

## CI gate

```text
cargo fmt --all -- --check
cargo test --workspace --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
```

The repository CI result is the independent execution evidence for the current branch.

## Deferred categories

Cryptographic vectors, property/fuzz testing over cryptographic state, protocol interoperability, key-storage tests, FFI ABI/security tests, platform-native verification, and production authenticated replay integration remain **PENDING** until their governing technical decisions and implementations exist.

A missing category is never treated as PASS.
