# Changelog

All notable changes to this project are documented here.

## [1.1.0] - 2026-05-28

### Changed

- **Visual overhaul of every HTML route.** Pulled in the Gemini-built reference design: Instrument Serif headlines with italic accent words, JetBrains Mono labels with leading colored dots, brighter brand glow (Camunda-blue style `#74c8ff` with `0 0 14-18px` shadows on active nav, action pills, and status chips), pulsing live-loop dot on the status chip, and a leading colored side-rail on the active sidebar entry.
- Status pills (`healthy` / `watch` / `critical`) now carry leading per-status SVG icons (check / clock / triangle) instead of color-only differentiation.
- Eyebrow chips, KPI labels, stat-card labels, and section-head labels all use JetBrains Mono with leading dots — the consistent metadata language that ties the surfaces together.
- Added OG / Twitter / theme-color meta on every route for share-card rendering on LinkedIn / Slack / Mastodon.
- `shell()` signature split into `(page_title, h1_html, subtitle, current, body)` so the h1 can carry HTML markup (italic accent word) while the `<title>` and meta tags stay plain-text — preventing raw HTML from leaking into the page header.

### Screenshots

- **Replaced the Python SVG-mock generator** (`scripts/generate_screenshots.py` + `screenshots/*-v3.svg`) with **real browser-rendered PNG captures** via headless Microsoft Edge: `scripts/render_readme_assets.ps1`. The script builds the release binary, starts it on port 4978, walks the four screenshot routes at 1600px-wide viewports, and writes `01-overview-proof.png` / `02-connector-board-proof.png` / `03-audit-log-proof.png` / `04-metrics-proof.png`.
- README updated to inline the new PNG screenshots with descriptive alt text + a short caption under each, replacing the collapsed `<details>` block from v1.0.

### Preserved

- `src/data.rs`, `src/engine.rs`, `src/models.rs` — connector fleet, scoring engine, and types are unchanged.
- All five HTML routes, all seven JSON APIs, the Prometheus `/metrics` payload shape.
- 5/5 `cargo test` green.

## [1.0.0] - 2026-05-14

### Released
- Published **cyberark-connector-observability-exporter** as a public Rust exporter focused on connector health, telemetry coverage, and privileged-access observability.
- Packaged Prometheus metrics, OTel coverage posture, audit-friendly proof surfaces, documentation, screenshots, and CI into a reviewable portfolio-grade repo.
- Clarified the core problem: CyberArk deployments often monitor vault outcomes without giving operators a clean view into the connectors that make those outcomes possible.

### Why this mattered
- Existing monitoring stacks could collect numbers, but they rarely made connector saturation, auth failures, legacy export gaps, and remediation direction visible in one lane.
- This release turns connector observability into something platform, security, and reliability teams can actually discuss and act on.

## [0.1.0] - 2026-01-24

### Shipped
- Cut the first coherent internal version of the connector health model with pool saturation, auth-failure pressure, scrape freshness, and latency scoring.
- Established the first reviewable architecture for a Rust exporter that feels closer to an operator control surface than a naked metrics endpoint.

## [Prototype] - 2025-06-02

### Built
- Built the first runnable prototype to test whether connector telemetry could be ranked into meaningful operational lanes instead of raw exporter noise.
- Used the prototype to validate whether Prometheus-style metrics could still be paired with human-legible review surfaces and audit context.

## [Design Phase] - 2025-10-09

### Designed
- Framed the exporter around operator-first and audit-legible outputs instead of generic dashboarding.
- Chose a shape that would still make sense to platform reliability teams, PAM operators, and security reviewers reading the repo cold.

## [Idea Origin] - 2024-11-18

### Observed
- The idea surfaced from the recurring gap between privileged-access control and privileged-access observability.
- Teams could usually explain what the vault was supposed to do, but not whether connector lanes were saturating, retrying, or silently falling out of telemetry coverage.

## [Background Signals] - 2022-09-12

### Context
- Earlier work around security platforms, operational reviews, and evidence pipelines made one pattern obvious: partial observability around critical connectors creates more drag than most teams admit until a review or outage forces the issue.
