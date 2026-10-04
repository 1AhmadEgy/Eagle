# Statistical Baseline and AnomalySignal

Status: Implemented in Kotlin; build verification pending.

## Security boundary

SecurityEvent -> FeatureVector -> statistical baseline -> AnomalySignal -> deterministic policy

AnomalySignal is advisory evidence only. It cannot authenticate, authorize, release keys, alter session state, bypass replay protection, or override rate limiting.

## Implemented methods

Z-score uses population variance and integer square root. Score is scaled by 1000 and bounded to 100,000. The feature identity is explicit and carried through AnomalySignal.

EWMA uses alpha = 0.2 represented as 200/1000, avoiding floating point. Its score semantics are relative deviation x1000.

Median/MAD uses median absolute deviation as a robust alternative to standard deviation. Its score semantics are absolute MAD units x1000.

All methods require at least three historical observations and at most 10,000.

## Threshold

Default threshold is 3.000 in milli-score units. This is an engineering default for testing, not a production security threshold.

## Limitations

- Current API evaluates one explicit SecurityFeature at a time.
- Z-score assumes a stable-enough baseline distribution.
- EWMA can adapt to drift and can also absorb sustained attacks if configured poorly.
- Median/MAD can become degenerate when MAD is zero; a non-median value is treated as maximally anomalous.
- No model training is performed.
- No AI/LLM is involved. Benchmarking is external to the security decision path.

## Promotion gate

Do not promote thresholds to security policy until dataset identity, distribution analysis, resource measurements, and incident/false-positive analysis are recorded.


## Benchmark

The reproducible JVM benchmark/evaluation harness is `app/src/test/java/com/eagle/app/security/StatisticalBaselineBenchmarkTest.kt`. It uses the same synthetic dataset, feature, histories, current values, and threshold for all methods, plus threshold sweeps. See `docs/06-ai/STATISTICAL_BENCHMARK_HARNESS.md`.
