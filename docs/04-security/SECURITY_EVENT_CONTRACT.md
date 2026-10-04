# Eagle — SecurityEvent Contract

Status: Implemented in Kotlin; build verification pending
Implementation: app/src/main/java/com/eagle/app/security/SecurityEvent.kt

## Purpose

SecurityEvent is the canonical privacy-minimized telemetry record for deterministic security controls and future statistical analysis.

## Contract

Required fields:
- schemaVersion
- type
- monotonicTimestampMillis
- contextId
- peerPseudonym
- protocolState
- outcome
- boundedCount

The type is an explicit enum rather than free-form text. This bounds the event vocabulary and reduces accidental high-cardinality telemetry.

## Privacy boundary

Allowed:
- opaque context identifiers;
- pseudonymous peer identifiers;
- protocol state;
- event type;
- outcome;
- bounded counters.

Forbidden:
- private keys;
- authentication secrets;
- raw challenge material;
- message plaintext;
- unrestricted remote input;
- model prompts or hidden chain-of-thought;
- unnecessary personal data.

## Versioning

schemaVersion is mandatory. A breaking schema change must increment the version and document migration/compatibility behavior.

## ML boundary

SecurityEvent is evidence only. It does not contain a policy decision and cannot directly authorize or deny an operation. Future FeatureVector extraction must be deterministic and versioned.

## Tests

Coverage includes valid construction and rejection of invalid schema version, negative timestamp, oversized identifiers, and oversized counters.

