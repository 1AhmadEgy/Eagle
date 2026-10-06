# The Eagle — Performance, Efficiency and Size Gates

- Status: CANONICAL internal policy
- Priority: subordinate to security and correctness
- Basis: reviewed Android performance requirements
- Measurement state: UNKNOWN until benchmarks are executed

## 1. Principle

Performance is measured, not assumed.

No optimization is accepted when it:

- weakens a security control
- introduces plaintext persistence
- introduces a forbidden runtime dependency
- creates a new uncontrolled network path
- increases instability

## 2. Metrics

Measure at minimum:

- cold startup
- warm/hot startup where relevant
- TTID
- TTFD
- first secure frame
- unlock latency
- Rust call latency
- encryption/decryption latency
- p50/p95/p99 where meaningful
- frame timing
- memory footprint
- native memory
- battery impact
- background wakeups
- WebRTC resource behavior
- APK size
- AAB size

## 3. Initial internal regression gates

- cold startup regression ≤ 10%
- TTID regression ≤ 10%
- p95 frame regression ≤ 15%
- memory regression ≤ 15%
- APK size regression ≤ 5%

These values are internal gates from the reviewed project documentation. They must be validated against actual target-device measurements before being treated as final production budgets.

## 4. Required tooling

- Baseline Profiles
- Macrobenchmark
- Perfetto/tracing
- memory profiling
- release R8 analysis
- APK/AAB size analysis
- representative physical devices

## 5. Approved optimization directions

- lazy startup initialization
- bounded memory usage
- streaming/chunked encryption
- reduced ByteArray copying
- pagination
- indexed database access
- strict main-thread protection
- cancellation of stale work
- limited concurrency
- WebRTC adaptation
- R8 and resource shrinking
- native ABI reduction when compatible with supported devices

## 6. Performance/security coupling

Every performance change must record:

- before measurement
- after measurement
- security impact
- binary-size impact
- battery impact
- affected modules
- test evidence

## 7. Release block conditions

Block release when:

- an established gate regresses without approved evidence;
- a main-thread security-sensitive operation is introduced;
- memory behavior becomes unbounded;
- a new runtime dependency appears;
- plaintext is introduced into persistent or diagnostic artifacts;
- a benchmark lacks reproducible evidence.

## 8. Privacy of performance artifacts

Traces and benchmark artifacts must not contain real sensitive user content or private keys.

Use synthetic, representative data with production-like sizes.
