# Architecture

CyberArk Connector Observability Exporter is a Rust + Axum service that models connector health as an operator-facing reliability surface instead of leaving it buried in raw metrics.

## Core Flow

1. Sample connector snapshots, audit events, and exporter configuration live in [C:\Users\chaus\dev\repos\cyberark-connector-observability-exporter\src\data.rs](/C:/Users/chaus/dev/repos/cyberark-connector-observability-exporter/src/data.rs).
2. Route handlers in [C:\Users\chaus\dev\repos\cyberark-connector-observability-exporter\src\main.rs](/C:/Users/chaus/dev/repos/cyberark-connector-observability-exporter/src/main.rs) expose HTML proof surfaces, JSON APIs, and the Prometheus metrics endpoint.
3. Assessment logic in [C:\Users\chaus\dev\repos\cyberark-connector-observability-exporter\src\engine.rs](/C:/Users/chaus/dev/repos/cyberark-connector-observability-exporter/src/engine.rs) converts pool saturation, auth failures, latency, stale scrapes, certificate age, and OTel coverage into ranked connector risk.
4. Render helpers in [C:\Users\chaus\dev\repos\cyberark-connector-observability-exporter\src\render.rs](/C:/Users/chaus/dev/repos/cyberark-connector-observability-exporter/src/render.rs) turn those assessments into operator-readable HTML surfaces.
5. Shared API contracts are defined in [C:\Users\chaus\dev\repos\cyberark-connector-observability-exporter\src\models.rs](/C:/Users/chaus/dev/repos/cyberark-connector-observability-exporter/src/models.rs).

## Route Surface

- `/` — exporter overview and connector-health control plane
- `/connectors` — ranked connector board
- `/audit` — replayable exporter activity log
- `/metrics-preview` — proof page for Prometheus and OTel posture
- `/docs` — route and purpose summary
- `/metrics` — Prometheus text format
- `/api/*` — JSON routes for summary, connectors, audit events, configuration, and sample payloads

## Design Notes

- The data stays in memory on purpose so the observability model is inspectable and easy to reason about.
- The exporter is opinionated about connector reliability because generic metric dumping does not tell operators what deserves action first.
- The proof surfaces exist to keep the repo legible to platform, security, and reliability stakeholders who may never run PromQL but still need to understand the operating story.
