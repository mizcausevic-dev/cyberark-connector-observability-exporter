use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ConnectorSnapshot {
    pub id: String,
    pub name: String,
    pub pool: String,
    pub region: String,
    pub target: String,
    pub auth_mode: String,
    pub pool_status: String,
    pub active_sessions: u32,
    pub saturation_percent: u8,
    pub auth_failures_24h: u32,
    pub latency_ms_p95: u32,
    pub stale_minutes: u32,
    pub cert_expiry_days: u16,
    pub otel_export_enabled: bool,
    pub prometheus_job: String,
    pub last_sync_minutes: u16,
}

#[derive(Debug, Clone, Serialize)]
pub struct ConnectorAssessment {
    #[serde(flatten)]
    pub snapshot: ConnectorSnapshot,
    pub risk_score: u8,
    pub verdict: String,
    pub top_concern: String,
    pub recommendation: String,
    pub exposure_flags: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DashboardSummary {
    pub connector_count: usize,
    pub degraded_connectors: usize,
    pub saturated_pools: usize,
    pub healthy_exporters: usize,
    pub auth_failures_24h: u32,
    pub avg_latency_ms_p95: u32,
    pub otel_coverage_percent: u8,
    pub highest_risk_connector: String,
    pub lead_recommendation: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ConnectorCollection {
    pub connectors: Vec<ConnectorAssessment>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ExporterConfig {
    pub scrape_interval_seconds: u32,
    pub otel_endpoint: String,
    pub prometheus_namespace: String,
    pub alert_targets: Vec<String>,
    pub retention_hours: u32,
}

#[derive(Debug, Clone, Serialize)]
pub struct AuditEvent {
    pub timestamp: String,
    pub action: String,
    pub resource: String,
    pub result: String,
    pub detail: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct SamplePayload {
    pub summary: DashboardSummary,
    pub highest_risk_connector: ConnectorAssessment,
    pub exporter_config: ExporterConfig,
    pub audit_excerpt: Vec<AuditEvent>,
}
