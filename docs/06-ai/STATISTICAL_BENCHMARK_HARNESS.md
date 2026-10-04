# Eagle — Statistical Benchmark Harness

Status: Implemented as deterministic JVM evaluation harness; Android/device performance verification pending.

## Purpose

Evaluate Z-score, EWMA, and Median/MAD on exactly the same bounded synthetic dataset and feature.

Implementation:
app/src/test/java/com/eagle/app/security/StatisticalBaselineBenchmarkTest.kt

## Dataset

The fixture contains 300 deterministic samples for:
SecurityFeature.REJECTION_RATE_BPS

Each sample contains:
- 32 historical observations;
- one current observation;
- NORMAL, ANOMALY, or DRIFT label.

The fixture is synthetic and must never be presented as production telemetry.

The canonical dataset receives a SHA-256 digest. The deterministic decision/score sequence also receives a SHA-256 digest for repeatability checks.

## Fair comparison

All methods receive exactly the same history/current values and feature.

The score semantics are explicitly different:
- Z-score: absolute z-score multiplied by 1000.
- EWMA: relative deviation multiplied by 1000.
- Median/MAD: absolute MAD units multiplied by 1000.

Therefore the harness has two modes of interpretation:

1. shared engineering threshold smoke test;
2. threshold sweep per method, which is the primary comparison.

A numeric threshold of 3000 does not imply equivalent statistical operating points between the three methods.

## Metrics

Classification:
- precision
- recall
- false-positive rate
- false-negative rate
- drift sensitivity

Performance diagnostic:
- median nanoseconds per inference
- p95 nanoseconds per inference

Repeatability:
- dataset SHA-256
- method decision/score SHA-256

## Timing qualification

The JVM harness is useful for deterministic regression detection and relative diagnostics. It is not an authoritative Android performance measurement.

For device-level CPU and allocation measurements, use AndroidX Microbenchmark after the security code is moved into a benchmarkable library module. AndroidX Benchmark 1.5.0 is the current stable release as of 2026-09-09; its Microbenchmark tooling is designed for isolated in-process code and reports timing/allocation information.

Authoritative references:
- https://developer.android.com/jetpack/androidx/releases/benchmark
- https://developer.android.com/topic/performance/benchmarking/microbenchmark-overview

## Dataset limitations

The current fixture is deliberately synthetic. No production threshold should be selected from it. The next evidence gate is a sanitized, representative dataset with a documented provenance/hash and, where possible, security labels.

## Gate

Do not introduce Isolation Forest, One-Class SVM, neural sequence models, or LLM decision logic until this baseline comparison is complete and the simpler methods demonstrate a material limitation.
