# PrivateMesh — Network Acceptance Gate

Status: GATED.

## Required for implementation completion
- MESH-001..015 implementation evidence;
- approved core/protocol contracts;
- ADR-0012 decision and compatibility evidence;
- transport implementation and exact versions recorded;
- direct-path test evidence;
- NAT traversal test evidence where used;
- relay fallback test evidence;
- failure injection evidence;
- resource-bound evidence;
- sanitized telemetry evidence;
- security review;
- independent verification where required.

## Automatic fail conditions
- plaintext crosses Mesh;
- private key is reachable from Mesh;
- relay becomes an application trust boundary;
- silent downgrade;
- unbounded retries/allocations;
- unvalidated network metadata mutates state;
- implementation is claimed complete from documentation alone.

## Completion vocabulary
SPECIFIED = contract exists.
IMPLEMENTED = source exists.
VERIFIED = source + tests + evidence + review.
RELEASE-READY = all required gates pass.

Documentation-only work must never be reported as IMPLEMENTED or VERIFIED.
