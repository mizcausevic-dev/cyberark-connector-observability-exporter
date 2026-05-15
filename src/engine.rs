use crate::data;
use crate::models::{
    AuditEvent, ConnectorAssessment, ConnectorCollection, ConnectorSnapshot, DashboardSummary,
    ExporterConfig, SamplePayload,
};

pub fn dashboard_summary() -> DashboardSummary {
    let connectors = connectors();
    let connector_count = connectors.len();
    let degraded_connectors = connectors
        .iter()
        .filter(|item| item.verdict != "healthy")
        .count();
    let saturated_pools = connectors
        .iter()
        .filter(|item| item.snapshot.saturation_percent >= 80)
        .count();
    let healthy_exporters = connectors
        .iter()
        .filter(|item| item.snapshot.otel_export_enabled && item.snapshot.pool_status != "critical")
        .count();
    let auth_failures_24h = connectors
        .iter()
        .map(|item| item.snapshot.auth_failures_24h)
        .sum();
    let avg_latency_ms_p95 =
        connectors.iter().map(|item| item.snapshot.latency_ms_p95).sum::<u32>() / connector_count as u32;
    let otel_coverage_percent =
        ((connectors.iter().filter(|item| item.snapshot.otel_export_enabled).count() * 100) / connector_count) as u8;
    let highest_risk_connector = connectors
        .first()
        .map(|item| item.snapshot.name.clone())
        .unwrap_or_else(|| "none".into());

    DashboardSummary {
        connector_count,
        degraded_connectors,
        saturated_pools,
        healthy_exporters,
        auth_failures_24h,
        avg_latency_ms_p95,
        otel_coverage_percent,
        highest_risk_connector,
        lead_recommendation: "Clear saturation and legacy-auth pressure in the vendor and EMEA PSM lanes before those connectors become blind spots in the privileged-access telemetry chain.".into(),
    }
}

pub fn connectors() -> Vec<ConnectorAssessment> {
    let mut rows: Vec<ConnectorAssessment> = data::connector_snapshots()
        .into_iter()
        .map(assess)
        .collect();
    rows.sort_by(|a, b| b.risk_score.cmp(&a.risk_score));
    rows
}

pub fn connector(id: &str) -> Option<ConnectorAssessment> {
    data::connector_snapshots()
        .into_iter()
        .map(assess)
        .find(|item| item.snapshot.id == id)
}

pub fn connector_collection() -> ConnectorCollection {
    ConnectorCollection {
        connectors: connectors(),
    }
}

pub fn exporter_config() -> ExporterConfig {
    data::exporter_config()
}

pub fn audit_log() -> Vec<AuditEvent> {
    data::audit_log()
}

pub fn sample_payload() -> SamplePayload {
    let connectors = connectors();
    SamplePayload {
        summary: dashboard_summary(),
        highest_risk_connector: connectors
            .first()
            .cloned()
            .expect("sample connector set should not be empty"),
        exporter_config: exporter_config(),
        audit_excerpt: audit_log().into_iter().take(3).collect(),
    }
}

pub fn prometheus_metrics() -> String {
    let connectors = connectors();
    let mut lines = vec![
        "# HELP cyberark_connector_health_score Risk-oriented health score for each connector.".to_string(),
        "# TYPE cyberark_connector_health_score gauge".to_string(),
        "# HELP cyberark_connector_auth_failures_total Authentication failures seen in the past 24h.".to_string(),
        "# TYPE cyberark_connector_auth_failures_total gauge".to_string(),
        "# HELP cyberark_connector_latency_p95_ms Observed p95 latency in milliseconds.".to_string(),
        "# TYPE cyberark_connector_latency_p95_ms gauge".to_string(),
        "# HELP cyberark_connector_pool_saturation_percent Session or pool saturation percentage.".to_string(),
        "# TYPE cyberark_connector_pool_saturation_percent gauge".to_string(),
        "# HELP cyberark_connector_otel_enabled Whether OTel export is enabled for the connector.".to_string(),
        "# TYPE cyberark_connector_otel_enabled gauge".to_string(),
    ];

    for item in connectors {
        let labels = format!(
            "connector=\"{}\",pool=\"{}\",region=\"{}\",status=\"{}\"",
            item.snapshot.name, item.snapshot.pool, item.snapshot.region, item.snapshot.pool_status
        );
        lines.push(format!(
            "cyberark_connector_health_score{{{labels}}} {}",
            100u8.saturating_sub(item.risk_score)
        ));
        lines.push(format!(
            "cyberark_connector_auth_failures_total{{{labels}}} {}",
            item.snapshot.auth_failures_24h
        ));
        lines.push(format!(
            "cyberark_connector_latency_p95_ms{{{labels}}} {}",
            item.snapshot.latency_ms_p95
        ));
        lines.push(format!(
            "cyberark_connector_pool_saturation_percent{{{labels}}} {}",
            item.snapshot.saturation_percent
        ));
        lines.push(format!(
            "cyberark_connector_otel_enabled{{{labels}}} {}",
            if item.snapshot.otel_export_enabled { 1 } else { 0 }
        ));
    }

    lines.join("\n")
}

fn assess(snapshot: ConnectorSnapshot) -> ConnectorAssessment {
    let mut risk = 14u8;
    let mut flags = Vec::new();

    if snapshot.saturation_percent >= 90 {
        risk += 26;
        flags.push("pool saturation".into());
    } else if snapshot.saturation_percent >= 75 {
        risk += 16;
        flags.push("high utilization".into());
    }

    if snapshot.auth_failures_24h >= 25 {
        risk += 24;
        flags.push("auth failures".into());
    } else if snapshot.auth_failures_24h >= 10 {
        risk += 14;
        flags.push("auth retries".into());
    }

    if snapshot.latency_ms_p95 >= 500 {
        risk += 18;
        flags.push("latency pressure".into());
    } else if snapshot.latency_ms_p95 >= 250 {
        risk += 10;
        flags.push("latency drift".into());
    }

    if snapshot.stale_minutes >= 15 {
        risk += 14;
        flags.push("stale scrape".into());
    } else if snapshot.stale_minutes >= 6 {
        risk += 8;
    }

    if snapshot.cert_expiry_days == 0 {
        risk += 18;
        flags.push("expired credential".into());
    } else if snapshot.cert_expiry_days <= 14 {
        risk += 12;
        flags.push("certificate nearing expiry".into());
    } else if snapshot.cert_expiry_days <= 30 {
        risk += 6;
    }

    if !snapshot.otel_export_enabled {
        risk += 18;
        flags.push("no otel export".into());
    }

    if snapshot.pool_status == "critical" {
        risk += 18;
    } else if snapshot.pool_status == "degraded" {
        risk += 10;
    } else if snapshot.pool_status == "watch" {
        risk += 6;
    }

    let risk_score = risk.min(100);
    let verdict = if risk_score >= 80 {
        "critical"
    } else if risk_score >= 55 {
        "watch"
    } else {
        "healthy"
    };

    let top_concern = if !snapshot.otel_export_enabled {
        "The connector is still outside the OTel export path, so its failures can become telemetry blind spots."
    } else if snapshot.saturation_percent >= 90 && snapshot.auth_failures_24h >= 25 {
        "Connector saturation and authentication failures are rising together, which makes this lane unstable under privileged-access pressure."
    } else if snapshot.cert_expiry_days == 0 {
        "Credential hygiene already failed, so the connector can no longer be trusted as a clean observability source."
    } else if snapshot.latency_ms_p95 >= 500 {
        "Latency is high enough to distort whether the connector is slow or actually unhealthy."
    } else {
        "The connector is under pressure but still observable enough to keep in the active export lane."
    };

    let recommendation = match verdict {
        "critical" => {
            "Drain excess pool load, rotate credentials if needed, and treat the connector as an urgent exporter-health incident until telemetry is trustworthy again."
        }
        "watch" => {
            "Refresh the connector before the next saturation window by clearing auth retries, validating certificates, and checking the OTel handoff path."
        }
        _ => {
            "Keep the connector in the standard monitoring cadence and preserve current alert thresholds."
        }
    };

    ConnectorAssessment {
        snapshot,
        risk_score,
        verdict: verdict.into(),
        top_concern: top_concern.into(),
        recommendation: recommendation.into(),
        exposure_flags: flags,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vendor_connector_goes_critical() {
        let connector = connector("conn-vendor-access-03").expect("connector should exist");
        assert_eq!(connector.verdict, "critical");
        assert!(!connector.snapshot.otel_export_enabled);
    }

    #[test]
    fn metrics_include_prometheus_labels() {
        let payload = prometheus_metrics();
        assert!(payload.contains("cyberark_connector_health_score"));
        assert!(payload.contains("connector=\"Vendor access relay\""));
    }
}
