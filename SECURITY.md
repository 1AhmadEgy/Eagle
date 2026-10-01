# Security Policy

## Scope

Eagle is developed with security as a first-class requirement. Until a release-specific security policy is published, security issues should not be disclosed publicly with exploit details.

## Reporting

Report suspected vulnerabilities privately to the project maintainers through the repository's configured private security reporting channel. Do not commit secrets, credentials, private keys, exploit payloads, or personal data to the repository.

## Development security requirements

- Never commit credentials, API keys, tokens, private keys, or production data.
- Validate all untrusted input at trust boundaries.
- Prefer maintained, well-reviewed dependencies over custom cryptographic or security primitives.
- Pin or constrain CI actions and dependencies where practical.
- Require automated tests and security checks before merging implementation code.
- Document security assumptions and threat-model changes with the affected component.

## Status

This document is a foundation policy. It must be expanded with the project's final contact channel, supported versions, disclosure SLA, and component-specific security requirements before a production release.
