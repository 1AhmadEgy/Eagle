# Protocol Implementation Evidence — 2026-10-05

Branch: execution/protocol-implementation-baseline-2026-10-05
Latest commit: 050c06202135db2a6cbf1d3c83c7d609246a62cf
Draft PR: #72

## Implemented
- bounded replay window with duplicate and sequence-collision rejection
- bounded out-of-order acceptance
- explicit monotonic epoch transition with rollback rejection
- freshness and expiry validation
- inbound delivery guard combining freshness and replay checks
- Rust Core boundary keeps epoch transition internal
- integration tests for replay/freshness behavior

## Security properties
- stale messages are rejected before replay-state mutation
- duplicate delivery is deterministic
- sequence reuse with altered message identity is rejected
- replay state is isolated per explicit authenticated session epoch boundary
- epoch rollback is rejected
- parser/message size controls remain fail-closed in the existing envelope contract

## Deliberate non-implementation
- no cryptographic primitive
- no custom ratchet
- no serialization selection encoded in runtime
- no unauthenticated capability upgrade
- no transport/relay fallback

## Verification
GitHub Actions are running against the latest protocol branch commit.
Latest observed workflow states: Rust Security Kernel = queued; Eagle Test Lab = queued; CI = queued.
The earlier run on commit f7316b2c... failed before Rust tests because cargo fmt detected formatting drift in inherited security-core files. Those files were normalized in this branch.
The earlier Android Test Lab failure was an independent AndroidKeyStoreStorageTest failure and is not treated as protocol evidence.

## Gate
Protocol implementation status: STRUCTURAL/DELIVERY BASELINE COMPLETE FOR THIS SLICE.
Protocol production approval remains blocked pending approved crypto/session/serialization/transport decisions, complete conformance evidence, and independent security review.