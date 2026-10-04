# Eagle — Deterministic FeatureVector Contract

Status: Implemented in Kotlin; build verification pending
Implementation: app/src/main/java/com/eagle/app/security/FeatureVector.kt

## Purpose

Convert a bounded, ordered SecurityEvent window into deterministic, privacy-minimized numeric features.

No model is invoked by this component. No policy decision is made here.

## Features

- eventsPerMinuteMicros: events per minute multiplied by 1,000,000;
- rejectionRateBps: non-accepted outcomes as basis points;
- replayFailureRateBps: duplicate/too-old/invalid-sequence events as basis points;
- rateLimitRateBps: rate-limited events as basis points;
- sessionTransitionRejectRateBps: rejected session transitions as basis points.

All rates use integer arithmetic with deterministic floor division.

## Input contract

Events must:
- use schema version 1;
- be ordered by monotonic timestamp;
- fall inside the explicit feature window;
- remain below the bounded 10,000-event window limit.

The extractor rejects malformed or unordered data rather than silently sorting or repairing it.

## Privacy

Only SecurityEvent fields already approved by the telemetry contract enter feature extraction. No message body, private key, authentication secret, or raw challenge material is accepted.

## ML boundary

FeatureVector is evidence. It does not authorize, deny, authenticate, rekey, or bypass a deterministic security primitive.

## Next use

The vector becomes the baseline input for deterministic statistics such as Z-score, EWMA, or Median/MAD evaluation. Thresholds must be evaluated against real event data before production adoption.
