# Eagle — Relay / Infrastructure Security Baseline — 2026-10-05

## Threat model

Assume an attacker can:

- fully control the network;
- spoof or poison discovery data;
- delay, drop, replay, reorder, or inject packets;
- exhaust connections, CPU, memory, file descriptors, queues, and bandwidth;
- compromise an infrastructure node;
- read infrastructure logs;
- force reconnect and downgrade attempts.

## Assets

- peer identity binding;
- authorization/session state;
- opaque message envelopes;
- endpoint metadata;
- infrastructure credentials;
- deployment artifacts and configuration;
- operational telemetry.

## Controls

| Threat | Control | Current |
|---|---|---|
| Relay becomes message path | hard policy rejection + CI gate | Implemented |
| Discovery becomes trust root | explicit non-authoritative boundary | Implemented |
| Connection exhaustion | quotas, deadlines, backoff | Required for future runtime |
| Resource amplification | bounded parsing and byte limits | Partially present in protocol candidate |
| Version downgrade | authenticated version policy | Pending approved transport |
| Metadata correlation | minimal endpoint metadata | Pending privacy review |
| Credential theft | short-lived secret delivery + no repo secrets | Policy established |
| Compromised infrastructure | minimize trust + opaque application frames | Architecture |
| Log leakage | sanitization/redaction | Policy established |
| Supply-chain compromise | pinned actions/dependencies + provenance | Partial |

## Required release evidence

- adversarial connection-limit tests;
- oversized input tests;
- downgrade/reconnect tests;
- discovery poisoning tests;
- packet-loss/reordering tests;
- no-relay application-content proof;
- artifact/SBOM provenance;
- infrastructure configuration review;
- independent security review.

## Non-negotiable

No relay implementation that forwards application data may be introduced while the current P2P-only requirement is authoritative.
