# Statistical Baseline Benchmark Report — 2026-10-04

## Scope

This report records the first local JVM benchmark of the Eagle deterministic statistical baseline.

It is a **synthetic fixture result**, not production telemetry and not an Android/device performance result.

## Reproducibility

Dataset:
- size: 300 samples
- feature: REJECTION_RATE_BPS
- history per sample: 32 values
- labels: NORMAL, ANOMALY, DRIFT
- dataset SHA-256: f7d1562ab594d0d1459f73a1c91fbff671d3c8db9f82d9c4491b5f956069012b

Threshold smoke test:
- 3000 milli-score units

Toolchain:
- kotlinc-jvm 1.9.0
- OpenJDK 21.0.11
- host: x86_64
- available processors: 3

## Smoke-test results

| Method | TP | TN | FP | FN | Precision | Recall | FPR | FNR | Drift sensitivity | Median wall ns/inference | P95 wall ns/inference | Median CPU ns/inference | P95 CPU ns/inference |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| Z-score | 60 | 240 | 0 | 0 | 1.000000 | 1.000000 | 0.000000 | 0.000000 | 1.000000 | 4076 | 6228 | 4074 | 7057 |
| EWMA | 30 | 240 | 0 | 30 | 1.000000 | 0.500000 | 0.000000 | 0.000000 | 0.000000 | 3054 | 7978 | 2923 | 8425 |
| Median/MAD | 60 | 240 | 0 | 0 | 1.000000 | 1.000000 | 0.000000 | 0.000000 | 1.000000 | 5316 | 10350 | 5266 | 11078 |

These timing and CPU-time numbers are JVM diagnostics only. They vary with JIT, host load, and runtime state and are not Android acceptance numbers.

## Threshold sweep

| Method | Thresholds tested | Observation |
|---|---|---|
| Z-score | 1000, 2000, 3000, 4000, 5000, 7500 | At 1000, FP increased to 67; at 2000+ the fixture produced 0 FP / 0 FN. |
| EWMA | 1000, 2000, 3000, 4000, 5000, 7500 | At 1000–4000, TP remained 30 and FN 30; at 5000+ all 60 labeled anomalies were missed. |
| Median/MAD | 1000, 2000, 3000, 4000, 5000, 7500 | At 1000, FP increased to 80; at 2000+ the fixture produced 0 FP / 0 FN. |

## Interpretation

The fixture is easy for the current Z-score and Median/MAD implementations and exposes a limitation of the current EWMA scoring semantics for gradual drift.

This is useful as a regression fixture, not as evidence that Z-score or Median/MAD are superior in real Eagle traffic.

No production threshold is selected from these results.

## Verification status

The Eagle security/statistical Kotlin source was checked with kotlinc-jvm 1.9.0 successfully after the feature-aware and overflow-hardening changes.

This is a source-level compilation check only. Full Gradle build, Android Lint, unit-test execution under Gradle, and device benchmarks remain pending.

## Next evidence gate

Run the same harness against sanitized representative Eagle telemetry, preserve dataset hash/provenance, then compare:
- threshold operating points;
- distribution drift;
- FP/FN where labels exist;
- resource usage;
- stability across devices.

Only after that comparison should a production baseline or an ML replacement be considered.
