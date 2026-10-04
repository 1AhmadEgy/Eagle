# Statistical Baseline and AnomalySignal

Status: Implemented in Kotlin; build verification pending.

## Security boundary

SecurityEvent -> FeatureVector -> statistical baseline -> AnomalySignal -> deterministic policy

AnomalySignal is advisory evidence only. It cannot authenticate, authorize, release keys, alter session state, bypass replay protection, or override rate limiting.

## Implemented methods

Z-score uses population variance and integer square root. Score is scaled by 1000 and bounded to 100,000.

EWMA uses alpha = 0.2 represented as 200/1000, avoiding floating point.

Median/MAD uses median absolute deviation as a robust alternative to standard deviation.

All methods require at least three historical observations and at most 10,000.

## Threshold

Default threshold is 3.000 in milli-score units. This is an engineering default for testing, not a production security threshold.

## Limitations

- Current API evaluates one feature at a time.
- Z-score assumes a stable-enough baseline distribution.
- EWMA can adapt to drift and can also absorb sustained attacks if configured poorly.
- Median/MAD can become degenerate when MAD is zero; a non-median value is treated as maximally anomalous.
- No model training is performed.
- No AI/LLM is involved.

## Promotion gate

Do not promote thresholds to security policy until dataset identity, distribution analysis, resource measurements, and incident/false-positive analysis are recorded.
