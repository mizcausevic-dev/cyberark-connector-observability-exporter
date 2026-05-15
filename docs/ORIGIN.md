# Why We Built This

**cyberark-connector-observability-exporter** exists because connector health is usually the least glamorous part of privileged-access architecture right up until it becomes the reason observability breaks down.

The recurring enterprise problem here was not a lack of dashboards. It was a lack of operational coherence around the connectors that move data, sessions, rotations, and identity context between CyberArk and the rest of the environment. Teams could usually tell you whether the vault itself was strategically important. They were much less comfortable explaining whether the PSM, CPM, PVWA, vendor, or identity bridge lanes were saturating, retrying, aging, or quietly falling out of telemetry coverage.

That gap matters more than it looks. Once connector health becomes fuzzy, the downstream consequences pile up quickly. Pool saturation starts looking like generic session noise. Authentication retries get normalized as “temporary.” Legacy API-key paths linger outside modern telemetry exports. Latency gets blamed on the wrong system. By the time someone asks for evidence during an incident, quarterly review, or remediation conversation, the metrics might exist, but the operating story is missing.

That is the problem this repo is trying to make visible. It is not pretending to be a production-ready CyberArk exporter for every deployment shape. It is a deliberate model of what a better connector-observability layer should feel like: one that keeps Prometheus metrics, OpenTelemetry coverage posture, auditability, and operator recommendations close together.

Existing tools did parts of the job. Monitoring stacks could ingest raw numbers. Security platforms could alert on downstream symptoms. Governance workflows could document remediation after the fact. But those surfaces usually left out the connective tissue: which connector is actually under pressure, what kind of pressure it is, and whether the telemetry path itself can still be trusted.

That shaped the design philosophy:

- **operator-first** so the riskiest connector lanes rise immediately instead of hiding behind aggregate exporter success
- **reliability-legible** so auth failures, latency drift, saturation, and stale scrapes tell a clear operational story
- **audit-friendly** so exporter events can be reused as evidence instead of disappearing into metrics backends
- **CI-native** so the repo reads like a real engineering artifact, not a static observability mock

The roadmap is practical. Historical baselines, more explicit alert policy modeling, richer OpenTelemetry attribute strategy, and broader connector families all belong in future iterations. But the core point will stay the same: observability work is strongest when it makes the uncomfortable lanes obvious before they become review or incident debt.
