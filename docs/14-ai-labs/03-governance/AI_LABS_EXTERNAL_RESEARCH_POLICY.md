# AI Labs External Research Policy
Status: Proposed / Documentation Only

## Purpose
Define how external research may inform Eagle AI Labs without becoming project authority.

## Rules
1. External web material is untrusted input until source verification.
2. Search-result snippets are not sufficient evidence for security or architecture decisions.
3. Prefer primary specifications, standards bodies, official vendor documentation, and authoritative security projects.
4. Record URL, title, publisher, date, check date, extracted claim, and Eagle impact.
5. Contradictory sources remain contradictory until reconciled.
6. Historical material cannot supersede current main merely because it is older or more detailed.
7. External research cannot directly modify Security Core authority.
8. External research cannot approve an ADR or implementation.
9. A research finding becomes actionable only through the Eagle requirement/ADR/test/review path.
10. Security-critical claims require reproducible or independently reviewable evidence where practical.

## AI Labs authority
AI may collect, summarize, classify, and propose research impacts.
AI may not approve security architecture, approve cryptographic protocols, grant itself permissions, bypass Gatekeeper, merge release changes, redefine canonical authority, or treat a web source as an accepted Eagle decision.

## Evidence states
Discovered → Collected → Verified → Impact Assessed → Proposed → Accepted

Additional states: Partially Verified, Contradicted, Superseded, Expired, Rejected, Unverified.

## Provenance minimum
Every research item should be traceable to Source ID + URL + checked date + claim + affected Eagle path/component + status.