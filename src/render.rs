use crate::engine;
use crate::models::ConnectorAssessment;

fn shell(title: &str, subtitle: &str, current: &str, body: &str) -> String {
    let summary = engine::dashboard_summary();
    let nav = [
        ("/", "Overview", "overview"),
        ("/connectors", "Connector Board", "connectors"),
        ("/audit", "Audit Log", "audit"),
        ("/metrics-preview", "Metrics", "metrics"),
        ("/docs", "Docs", "docs"),
    ];

    let side_links = nav
        .iter()
        .map(|(href, label, key)| {
            format!(
                r#"<a class="side-link {}" href="{}">{}</a>"#,
                if current == *key { "active" } else { "" },
                href,
                label
            )
        })
        .collect::<Vec<_>>()
        .join("");

    let tab_links = nav
        .iter()
        .map(|(href, label, key)| {
            format!(
                r#"<a class="tab-pill {}" href="{}">{}</a>"#,
                if current == *key { "active" } else { "" },
                href,
                label
            )
        })
        .collect::<Vec<_>>()
        .join("");

    format!(
        r#"<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>{title}</title>
    <style>
      :root {{
        color-scheme: dark;
        --bg: #04070d;
        --panel: rgba(9, 16, 28, 0.92);
        --line: rgba(255,255,255,0.07);
        --text: #f5f7fd;
        --muted: #96a9c6;
        --soft: #6d809b;
        --blue: #74c8ff;
        --indigo: #5d78ff;
        --green: #49d79e;
        --amber: #f6c46a;
        --red: #ff7987;
      }}
      * {{ box-sizing: border-box; }}
      body {{
        margin: 0;
        font-family: Inter, "Segoe UI", system-ui, sans-serif;
        color: var(--text);
        background:
          radial-gradient(circle at top left, rgba(116,200,255,0.14), transparent 24%),
          radial-gradient(circle at top right, rgba(255,121,135,0.08), transparent 16%),
          linear-gradient(180deg, #02050a 0%, #050912 100%);
      }}
      a {{ color: inherit; text-decoration: none; }}
      .shell {{ min-height: 100vh; display: grid; grid-template-columns: 248px minmax(0,1fr); }}
      .sidebar {{
        background: rgba(0,0,0,0.3);
        border-right: 1px solid rgba(255,255,255,0.06);
        backdrop-filter: blur(16px);
        padding: 24px 18px;
        display: flex;
        flex-direction: column;
      }}
      .brand {{
        display: flex; align-items: center; gap: 12px; padding: 8px 10px 18px;
        border-bottom: 1px solid rgba(255,255,255,0.06);
      }}
      .brand-mark {{
        width: 40px; height: 40px; border-radius: 12px; display:grid; place-items:center;
        background: linear-gradient(135deg, #0c97c2, #5d78ff); color:white; font-weight:900;
        box-shadow: 0 0 18px rgba(93,120,255,0.28);
      }}
      .brand strong {{ display:block; font-size:14px; }}
      .brand span {{ display:block; margin-top:4px; color:var(--blue); font-size:10px; letter-spacing:.18em; text-transform:uppercase; }}
      nav {{ margin-top: 18px; }}
      .side-link {{
        display:block; padding:13px 14px; border-radius:14px; color:#8195b4; font-size:12px;
        font-weight:700; text-transform:uppercase; letter-spacing:.12em; transition:all 150ms ease;
      }}
      .side-link.active {{ color:var(--blue); background:rgba(116,200,255,0.08); border:1px solid rgba(116,200,255,0.16); }}
      .side-link:hover {{ color:var(--text); background:rgba(255,255,255,0.04); }}
      .meta {{ margin-top:auto; padding:16px 12px 8px; border-top:1px solid rgba(255,255,255,0.06); }}
      .meta dt {{ color:#687c98; font-size:10px; text-transform:uppercase; letter-spacing:.14em; margin-bottom:4px; }}
      .meta dd {{ margin:0 0 14px; font-size:12px; font-weight:700; }}
      .topbar {{
        height:72px; position:sticky; top:0; z-index:2; display:flex; align-items:center; justify-content:space-between;
        padding:0 34px; background:rgba(0,0,0,0.34); border-bottom:1px solid rgba(255,255,255,0.06); backdrop-filter: blur(16px);
      }}
      .status-chip {{
        display:inline-flex; align-items:center; gap:10px; padding:9px 14px; border-radius:999px;
        border:1px solid rgba(116,200,255,0.14); background:rgba(116,200,255,0.05); color:#b9e1ff;
        font-size:10px; font-weight:800; text-transform:uppercase; letter-spacing:.18em;
      }}
      .status-dot {{ width:8px; height:8px; border-radius:50%; background:var(--blue); box-shadow:0 0 12px rgba(116,200,255,0.84); }}
      .topbar-right {{ display:flex; align-items:center; gap:22px; }}
      .meta-block {{ display:flex; flex-direction:column; align-items:flex-end; }}
      .meta-block span {{ color:#6d809b; font-size:9px; text-transform:uppercase; letter-spacing:.15em; }}
      .meta-block strong {{ margin-top:4px; font-size:11px; text-transform:uppercase; letter-spacing:.12em; }}
      .action-pill {{
        display:inline-flex; align-items:center; padding:12px 16px; border-radius:999px; color:white;
        background:linear-gradient(135deg, #0f8fbf, #5d78ff); box-shadow:0 0 20px rgba(93,120,255,0.24);
        font-size:10px; font-weight:900; letter-spacing:.18em; text-transform:uppercase;
      }}
      .wrap {{ max-width: 1280px; margin:0 auto; padding:34px; }}
      .hero {{
        border:1px solid var(--line); border-radius:28px; padding:28px;
        background: linear-gradient(180deg, rgba(9,16,28,0.96), rgba(6,11,20,0.94));
        box-shadow: 0 26px 60px rgba(0,0,0,0.34);
      }}
      .hero-eyebrow {{ margin-bottom:18px; color:var(--blue); font-size:11px; letter-spacing:.28em; text-transform:uppercase; font-weight:800; }}
      h1 {{ margin:0; font-size:clamp(38px,5vw,70px); line-height:.92; font-family:Georgia, "Times New Roman", serif; letter-spacing:-.04em; }}
      .hero-subtitle {{ margin-top:14px; max-width:860px; color:var(--muted); font-size:19px; line-height:1.55; }}
      .hero-strip {{ display:flex; flex-wrap:wrap; gap:14px; margin-top:24px; }}
      .hero-kpi {{ min-width:180px; padding:14px 16px; border-radius:18px; border:1px solid rgba(255,255,255,0.06); background:rgba(255,255,255,0.03); }}
      .hero-kpi .k {{ color:#6f83a0; font-size:10px; text-transform:uppercase; letter-spacing:.14em; font-weight:800; }}
      .hero-kpi .v {{ margin-top:6px; font-size:28px; font-weight:800; }}
      .hero-callout {{
        margin-top:18px; padding:18px 20px; border-radius:18px; border:1px solid rgba(255,255,255,0.06); background:rgba(2,8,17,0.62);
      }}
      .hero-callout strong {{ display:block; color:var(--amber); font-size:10px; text-transform:uppercase; letter-spacing:.18em; margin-bottom:8px; }}
      .hero-callout p {{ margin:0; color:#dce7fb; font-size:17px; line-height:1.5; }}
      .tab-row {{ display:flex; gap:10px; flex-wrap:wrap; margin-top:20px; }}
      .tab-pill {{
        display:inline-flex; align-items:center; padding:10px 14px; border-radius:999px; border:1px solid rgba(255,255,255,0.08);
        background:rgba(255,255,255,0.03); color:#afc0d8; font-size:11px; font-weight:800; text-transform:uppercase; letter-spacing:.12em;
      }}
      .tab-pill.active {{ color:var(--amber); border-color:rgba(246,196,106,0.18); background:rgba(246,196,106,0.08); }}
      .page-section {{ margin-top:24px; border-radius:26px; border:1px solid var(--line); background:var(--panel); overflow:hidden; box-shadow:0 24px 54px rgba(0,0,0,0.24); }}
      .section-head {{ padding:20px 24px 14px; border-bottom:1px solid rgba(255,255,255,0.05); }}
      .section-head strong {{ display:block; color:var(--blue); font-size:10px; text-transform:uppercase; letter-spacing:.2em; margin-bottom:10px; }}
      .section-head h2 {{ margin:0; font-family:Georgia, "Times New Roman", serif; font-size:24px; letter-spacing:-.03em; }}
      .section-head p {{ margin:10px 0 0; color:var(--muted); font-size:15px; line-height:1.55; }}
      .section-body {{ padding:24px; }}
      .stats-grid, .three-col {{ display:grid; gap:18px; grid-template-columns:repeat(4,minmax(0,1fr)); }}
      .three-col {{ grid-template-columns:repeat(3,minmax(0,1fr)); }}
      .stat-card, .metric-card {{
        border-radius:20px; padding:18px 18px 20px; border:1px solid rgba(255,255,255,0.06);
        background:linear-gradient(180deg, rgba(255,255,255,0.04), rgba(0,0,0,0.08));
      }}
      .stat-card .label, .micro {{
        color:#71839d; font-size:10px; text-transform:uppercase; letter-spacing:.16em; font-weight:800;
      }}
      .stat-card .value {{ margin-top:10px; font-size:36px; font-weight:900; }}
      .stat-card .sub, .metric-card .desc {{ margin-top:10px; color:var(--muted); font-size:14px; line-height:1.45; }}
      .metric-card .title {{ margin-top:8px; font-size:16px; font-weight:800; }}
      .panel-grid {{ display:grid; gap:14px; }}
      .insight-grid {{ display:grid; gap:18px; grid-template-columns:1.2fr 1fr; }}
      .panel {{
        border-radius:22px; border:1px solid rgba(255,255,255,0.06); background:rgba(4,9,18,0.55); padding:22px;
      }}
      .panel h3 {{ margin:0 0 14px; font-size:18px; }}
      .chart-shell {{
        border-radius:22px; border:1px solid rgba(255,255,255,0.06); background:rgba(4,9,18,0.55); padding:22px;
      }}
      .bars {{ display:flex; align-items:flex-end; justify-content:space-between; gap:14px; height:180px; margin-top:18px; }}
      .bar-col {{ flex:1; text-align:center; }}
      .bar-wrap {{ height:124px; display:flex; align-items:flex-end; justify-content:center; }}
      .bar {{
        width:100%; max-width:76px; border-radius:18px 18px 8px 8px; background:linear-gradient(180deg, var(--blue), var(--indigo));
        box-shadow:0 0 24px rgba(93,120,255,0.24);
      }}
      .bar-label {{ margin-top:8px; color:var(--muted); font-size:11px; text-transform:uppercase; letter-spacing:.12em; }}
      .bar-value {{ margin-top:6px; font-size:15px; font-weight:800; }}
      .connector-grid {{ display:grid; gap:16px; }}
      .connector-card {{
        border-radius:22px; border:1px solid rgba(255,255,255,0.06); background:rgba(4,9,18,0.6); overflow:hidden;
      }}
      .connector-top {{ display:grid; grid-template-columns:minmax(0,1fr) auto auto; gap:18px; align-items:center; padding:20px 22px; }}
      .connector-card h3 {{ margin:0; font-size:22px; font-weight:800; letter-spacing:-.03em; }}
      .meta-text {{ margin-top:8px; color:var(--muted); font-size:13px; }}
      .tag {{
        display:inline-flex; align-items:center; justify-content:center; padding:8px 12px; border-radius:999px; font-size:10px; font-weight:900; letter-spacing:.16em; text-transform:uppercase;
      }}
      .healthy {{ color:var(--green); background:rgba(73,215,158,0.12); border:1px solid rgba(73,215,158,0.14); }}
      .watch {{ color:var(--amber); background:rgba(246,196,106,0.12); border:1px solid rgba(246,196,106,0.14); }}
      .critical {{ color:var(--red); background:rgba(255,121,135,0.12); border:1px solid rgba(255,121,135,0.14); }}
      .score-stack {{ text-align:right; }}
      .score-stack .label {{ color:#6f83a0; font-size:9px; text-transform:uppercase; letter-spacing:.16em; font-weight:800; }}
      .score-stack .value {{ margin-top:6px; font-size:28px; font-weight:900; }}
      .connector-bottom {{ padding:18px 22px 22px; border-top:1px solid rgba(255,255,255,0.05); background:rgba(255,255,255,0.02); }}
      .two-col {{ display:grid; grid-template-columns:1fr 1fr; gap:18px; }}
      .signal-pill {{
        display:inline-flex; align-items:center; padding:8px 10px; border-radius:999px; background:rgba(116,200,255,0.09); color:var(--blue);
        font-size:10px; font-weight:800; letter-spacing:.12em; text-transform:uppercase;
      }}
      .pill-stack {{ display:flex; flex-wrap:wrap; gap:10px; }}
      .meter-row + .meter-row {{ margin-top:14px; }}
      .meter-head {{ display:flex; justify-content:space-between; gap:16px; margin-bottom:8px; color:#cfe0f7; font-size:12px; font-weight:700; }}
      .meter-track {{ height:10px; border-radius:999px; background:rgba(255,255,255,0.05); overflow:hidden; }}
      .meter-fill {{ height:100%; border-radius:999px; }}
      .meter-fill.good {{ background:linear-gradient(90deg, #1e7fc7, #49d79e); }}
      .meter-fill.watch {{ background:linear-gradient(90deg, #2f82ff, #f6c46a); }}
      .meter-fill.hot {{ background:linear-gradient(90deg, #d14d6c, #ff7987); }}
      .log-shell {{
        border-radius:22px; border:1px solid rgba(255,255,255,0.08); background:rgba(2,6,12,0.88); overflow:hidden;
      }}
      .log-head {{
        padding:16px 18px; display:flex; align-items:center; gap:12px; border-bottom:1px solid rgba(255,255,255,0.08); background:rgba(255,255,255,0.03);
      }}
      .lights {{ display:flex; gap:8px; }}
      .lights i {{ width:11px; height:11px; border-radius:50%; display:block; }}
      .lights i:nth-child(1) {{ background:rgba(255,121,135,0.55); }}
      .lights i:nth-child(2) {{ background:rgba(246,196,106,0.55); }}
      .lights i:nth-child(3) {{ background:rgba(73,215,158,0.55); }}
      .log-head strong {{ color:var(--blue); font-size:10px; letter-spacing:.18em; text-transform:uppercase; }}
      .log-body {{ padding:18px 18px 8px; }}
      .log-line {{ display:grid; grid-template-columns:170px 180px minmax(0,1fr) 90px; gap:14px; align-items:start; padding:10px 12px; border-radius:14px; }}
      .log-line + .log-line {{ margin-top:8px; }}
      .log-line:hover {{ background:rgba(255,255,255,0.03); }}
      .log-time {{ color:#6f83a0; font-size:11px; font-family:"Cascadia Code", Consolas, monospace; }}
      .log-action {{ color:var(--blue); font-size:11px; font-family:"Cascadia Code", Consolas, monospace; font-weight:800; letter-spacing:.08em; }}
      .log-resource strong {{ display:block; font-size:12px; }}
      .log-resource span {{ display:block; margin-top:4px; color:var(--muted); font-size:12px; line-height:1.45; }}
      .result-good {{ color:var(--green); }}
      .result-warning {{ color:var(--amber); }}
      .result-bad {{ color:var(--red); }}
      .code-panel {{
        border-radius:22px; border:1px solid rgba(255,255,255,0.08); background:rgba(2,6,12,0.92); padding:18px 20px 20px;
      }}
      .code-head {{ display:flex; align-items:center; justify-content:space-between; padding-bottom:12px; margin-bottom:16px; border-bottom:1px solid rgba(255,255,255,0.08); }}
      .code-head span {{ color:var(--blue); font-size:10px; font-weight:800; text-transform:uppercase; letter-spacing:.18em; }}
      pre {{
        margin:0; white-space:pre-wrap; overflow:auto; color:#dce8fb; font-size:13px; line-height:1.6; font-family:"Cascadia Code", Consolas, monospace;
      }}
      .footer-strip {{ display:flex; justify-content:space-between; gap:16px; margin-top:18px; padding:4px 2px 10px; color:#6d809b; font-size:10px; text-transform:uppercase; letter-spacing:.16em; }}
      .footer-strip strong {{ color:#b8c9de; }}
      @media (max-width:1080px) {{
        .shell {{ grid-template-columns:1fr; }}
        .sidebar {{ display:none; }}
        .stats-grid, .three-col, .insight-grid, .two-col {{ grid-template-columns:1fr; }}
        .connector-top {{ grid-template-columns:1fr; align-items:start; }}
        .topbar {{ padding:0 18px; height:auto; min-height:72px; flex-wrap:wrap; gap:12px; }}
        .topbar-right, .footer-strip {{ flex-wrap:wrap; }}
        .log-line {{ grid-template-columns:1fr; }}
      }}
    </style>
  </head>
  <body>
    <div class="shell">
      <aside class="sidebar">
        <div class="brand">
          <div class="brand-mark">CE</div>
          <div>
            <strong>CyberArk Connector Observability Exporter</strong>
            <span>Instance: CONNECTOR-OBS</span>
          </div>
        </div>
        <nav>{side_links}</nav>
        <dl class="meta">
          <dt>Exporter lane</dt>
          <dd>Prometheus + OTel bridge</dd>
          <dt>Degraded connectors</dt>
          <dd>{degraded_connectors} connectors</dd>
          <dt>Saturated pools</dt>
          <dd>{saturated_pools} pools</dd>
        </dl>
      </aside>
      <main>
        <header class="topbar">
          <div class="status-chip"><span class="status-dot"></span>Connector scrape loop live</div>
          <div class="topbar-right">
            <div class="meta-block"><span>Telemetry coverage</span><strong>{otel_coverage}% OTel enabled</strong></div>
            <div class="meta-block"><span>Exporter health</span><strong>{healthy_exporters}/{connector_count} lanes stable</strong></div>
            <a class="action-pill" href="/metrics">Open Prometheus metrics</a>
          </div>
        </header>
        <div class="wrap">
          <section class="hero">
            <div class="hero-eyebrow">CyberArk Connector Observability Exporter</div>
            <h1>{title}</h1>
            <p class="hero-subtitle">{subtitle}</p>
            <div class="hero-strip">
              <div class="hero-kpi"><div class="k">Connectors</div><div class="v">{connector_count}</div></div>
              <div class="hero-kpi"><div class="k">Auth failures / 24h</div><div class="v">{auth_failures}</div></div>
              <div class="hero-kpi"><div class="k">Avg latency p95</div><div class="v">{avg_latency}ms</div></div>
              <div class="hero-kpi"><div class="k">Highest-risk connector</div><div class="v" style="font-size:20px">{highest_risk}</div></div>
            </div>
            <div class="hero-callout">
              <strong>Lead recommendation</strong>
              <p>{lead_recommendation}</p>
            </div>
            <div class="tab-row">{tab_links}</div>
          </section>
          {body}
          <div class="footer-strip">
            <span><strong>Discipline:</strong> connector observability</span>
            <span><strong>Focus:</strong> pool health / auth failures / latency / export coverage</span>
            <span><strong>Surface:</strong> operator-first / reliability-legible</span>
          </div>
        </div>
      </main>
    </div>
  </body>
</html>"#,
        title = title,
        subtitle = subtitle,
        body = body,
        side_links = side_links,
        tab_links = tab_links,
        connector_count = summary.connector_count,
        degraded_connectors = summary.degraded_connectors,
        saturated_pools = summary.saturated_pools,
        healthy_exporters = summary.healthy_exporters,
        auth_failures = summary.auth_failures_24h,
        avg_latency = summary.avg_latency_ms_p95,
        otel_coverage = summary.otel_coverage_percent,
        highest_risk = summary.highest_risk_connector,
        lead_recommendation = summary.lead_recommendation,
    )
}

fn score_bars(connector: &ConnectorAssessment) -> String {
    let metrics = vec![
        ("Pool saturation", connector.snapshot.saturation_percent as usize),
        ("Auth failures", ((connector.snapshot.auth_failures_24h.min(35) * 100) / 35) as usize),
        ("Latency p95", ((connector.snapshot.latency_ms_p95.min(700) * 100) / 700) as usize),
        (
            "Telemetry freshness",
            100usize.saturating_sub((connector.snapshot.stale_minutes.min(30) * 100 / 30) as usize),
        ),
    ];

    metrics
        .into_iter()
        .map(|(label, value)| {
            let tone = if value < 45 {
                "good"
            } else if value < 75 {
                "watch"
            } else {
                "hot"
            };
            format!(
                r#"<div class="meter-row"><div class="meter-head"><span>{}</span><span>{}%</span></div><div class="meter-track"><div class="meter-fill {}" style="width:{}%"></div></div></div>"#,
                label, value, tone, value
            )
        })
        .collect::<Vec<_>>()
        .join("")
}

pub fn render_overview() -> String {
    let summary = engine::dashboard_summary();
    let connectors = engine::connectors();
    let chart_bars = connectors
        .iter()
        .take(5)
        .map(|connector| {
            let height = (connector.snapshot.saturation_percent as f32 * 1.1).max(18.0);
            format!(
                r#"<div class="bar-col"><div class="bar-wrap"><div class="bar" style="height:{:.0}px"></div></div><div class="bar-value">{}</div><div class="bar-label">{}</div></div>"#,
                height, connector.snapshot.saturation_percent, connector.snapshot.region
            )
        })
        .collect::<Vec<_>>()
        .join("");

    let board = connectors
        .iter()
        .take(3)
        .map(connector_card)
        .collect::<Vec<_>>()
        .join("");

    let body = format!(
        r#"
      <section class="page-section">
        <div class="section-head">
          <strong>Exporter overview</strong>
          <h2>Connector telemetry should make privileged-access risk legible before it becomes review debt.</h2>
          <p>This exporter treats CyberArk connector health as an operating problem: are pools saturating, are auth failures climbing, and will Prometheus and OpenTelemetry still see the lane clearly when pressure hits?</p>
        </div>
        <div class="section-body">
          <div class="stats-grid">
            <div class="stat-card"><div class="label">Degraded connectors</div><div class="value">{}</div><div class="sub">Connectors already carrying enough friction to deserve operator review.</div></div>
            <div class="stat-card"><div class="label">Saturated pools</div><div class="value">{}</div><div class="sub">Pool lanes above the threshold where session pressure turns into reliability risk.</div></div>
            <div class="stat-card"><div class="label">OTel coverage</div><div class="value">{}%</div><div class="sub">How much of the connector estate is still inside the modern telemetry export path.</div></div>
            <div class="stat-card"><div class="label">Auth failures</div><div class="value">{}</div><div class="sub">Authentication failure pressure observed across the past 24 hours.</div></div>
          </div>
          <div class="insight-grid" style="margin-top:20px;">
            <div class="chart-shell">
              <div class="micro">Pool pressure by connector lane</div>
              <h3 style="margin:10px 0 0;">Where saturation is starting to outrun comfortable connector health.</h3>
              <div class="bars">{}</div>
              <p style="margin-top:16px;color:var(--muted);font-size:13px;line-height:1.5;">A connector can look alive and still be heading toward blind-spot territory if saturation, stale scrapes, and auth failures rise together.</p>
            </div>
            <div class="panel">
              <h3>What the exporter is really for</h3>
              <div class="panel-grid">
                <div class="metric-card">
                  <div class="micro">Prometheus path</div>
                  <div class="title">Keep raw connector health queryable.</div>
                  <div class="desc">Latency, pool load, auth failures, and export coverage all show up in one scrape surface.</div>
                </div>
                <div class="metric-card">
                  <div class="micro">OpenTelemetry path</div>
                  <div class="title">Do not let legacy lanes disappear from traces.</div>
                  <div class="desc">If a connector is still outside the OTel path, the exporter should make that absence visible immediately.</div>
                </div>
                <div class="metric-card">
                  <div class="micro">Operator lane</div>
                  <div class="title">Turn alerts into remediation direction.</div>
                  <div class="desc">The repo is strongest when it explains which connector deserves attention and why.</div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </section>
      <section class="page-section">
        <div class="section-head">
          <strong>Connector board</strong>
          <h2>The highest-risk connectors stay visible as a working board, not an afterthought.</h2>
          <p>Every connector card combines saturation, auth, latency, and telemetry freshness so the riskiest export lanes are obvious at a glance.</p>
        </div>
        <div class="section-body">
          <div class="connector-grid">{}</div>
        </div>
      </section>
        "#,
        summary.degraded_connectors,
        summary.saturated_pools,
        summary.otel_coverage_percent,
        summary.auth_failures_24h,
        chart_bars,
        board
    );

    shell(
        "Control-plane summary for CyberArk connector health.",
        "Connector count, pool pressure, auth failures, latency, and exporter coverage at a glance.",
        "overview",
        &body,
    )
}

pub fn render_connectors() -> String {
    let rows = engine::connectors()
        .iter()
        .map(connector_card)
        .collect::<Vec<_>>()
        .join("");

    let body = format!(
        r#"
      <section class="page-section">
        <div class="section-head">
          <strong>Connector board</strong>
          <h2>The lanes most likely to drift from healthy telemetry into privileged-access blind spots.</h2>
          <p>This is the practical operator surface for deciding which connector pool needs load relief, auth cleanup, certificate work, or an OTel export fix before the next failure storm arrives.</p>
        </div>
        <div class="section-body">
          <div class="connector-grid">{}</div>
        </div>
      </section>
        "#,
        rows
    );

    shell(
        "Review queue for connector reliability pressure.",
        "The connectors most likely to need containment or remediation first.",
        "connectors",
        &body,
    )
}

pub fn render_audit() -> String {
    let rows = engine::audit_log()
        .iter()
        .map(|event| {
            let result_class = match event.result.as_str() {
                "success" => "result-good",
                "warning" => "result-warning",
                _ => "result-bad",
            };
            format!(
                r#"<div class="log-line"><div class="log-time">{}</div><div class="log-action">{}</div><div class="log-resource"><strong>{}</strong><span>{}</span></div><div class="{}">{}</div></div>"#,
                event.timestamp, event.action, event.resource, event.detail, result_class, event.result
            )
        })
        .collect::<Vec<_>>()
        .join("");

    let body = format!(
        r#"
      <section class="page-section">
        <div class="section-head">
          <strong>Audit evidence</strong>
          <h2>The exporter should leave behind a replayable trail, not just a gauge.</h2>
          <p>The useful part is not only that metrics were scraped. It is that connector events, skipped exports, and remediation recommendations stay legible to humans later.</p>
        </div>
        <div class="section-body">
          <div class="log-shell">
            <div class="log-head">
              <div class="lights"><i></i><i></i><i></i></div>
              <strong>connector-observability runtime log</strong>
            </div>
            <div class="log-body">{}</div>
          </div>
          <div class="three-col" style="margin-top:18px;">
            <div class="metric-card">
              <div class="micro">Alert fidelity</div>
              <div class="title">Warnings explain their operational context.</div>
              <div class="desc">A high auth-failure count means more when the exporter also tells you which connector and which pool triggered it.</div>
            </div>
            <div class="metric-card">
              <div class="micro">Compliance legibility</div>
              <div class="title">Exporter events become audit evidence.</div>
              <div class="desc">This is how observability work becomes review-friendly instead of living only inside a metrics backend.</div>
            </div>
            <div class="metric-card">
              <div class="micro">Operator trust</div>
              <div class="title">Skipped or legacy lanes stay visible.</div>
              <div class="desc">If a connector is outside the expected telemetry path, the repo should make that explicit and uncomfortable.</div>
            </div>
          </div>
        </div>
      </section>
        "#,
        rows
    );

    shell(
        "Audit evidence for connector-exporter operations.",
        "A replayable log of scrapes, export failures, and remediation recommendations.",
        "audit",
        &body,
    )
}

pub fn render_metrics_preview() -> String {
    let metrics = engine::prometheus_metrics();
    let config = engine::exporter_config();
    let body = format!(
        r#"
      <section class="page-section">
        <div class="section-head">
          <strong>Metrics surface</strong>
          <h2>The Prometheus and OpenTelemetry handoff should be obvious before anyone wires alerts on top.</h2>
          <p>This page makes the metric namespace, export posture, and raw payload preview visible in one place so the exporter feels concrete.</p>
        </div>
        <div class="section-body">
          <div class="insight-grid">
            <div class="panel">
              <h3>Exporter configuration</h3>
              <div class="panel-grid">
                <div class="metric-card">
                  <div class="micro">Scrape interval</div>
                  <div class="title">{} seconds</div>
                  <div class="desc">Frequent enough to catch pool stress before the connector becomes stale telemetry.</div>
                </div>
                <div class="metric-card">
                  <div class="micro">Prometheus namespace</div>
                  <div class="title">{}</div>
                  <div class="desc">Metrics stay grouped under a clear namespace instead of blending into generic exporter noise.</div>
                </div>
                <div class="metric-card">
                  <div class="micro">OTel endpoint</div>
                  <div class="title">{}</div>
                  <div class="desc">The exporter also models where modern telemetry should flow once connectors are instrumented correctly.</div>
                </div>
              </div>
            </div>
            <div class="code-panel">
              <div class="code-head"><span>/metrics</span><div class="lights"><i></i><i></i><i></i></div></div>
              <pre><code>{}</code></pre>
            </div>
          </div>
        </div>
      </section>
        "#,
        config.scrape_interval_seconds,
        config.prometheus_namespace,
        config.otel_endpoint,
        metrics
    );

    shell(
        "Metrics preview for the Prometheus and OTel export path.",
        "The exporter makes connector health scrapeable, alertable, and reviewable in one lane.",
        "metrics",
        &body,
    )
}

pub fn render_docs() -> String {
    let body = r#"
      <section class="page-section">
        <div class="section-head">
          <strong>Docs</strong>
          <h2>What the exporter exposes and why each route exists.</h2>
          <p>This service is opinionated: it does not just emit generic health data. It frames CyberArk connector pressure as something operators can review and act on quickly.</p>
        </div>
        <div class="section-body">
          <div class="three-col">
            <div class="metric-card">
              <div class="micro">GET /api/dashboard/summary</div>
              <div class="title">Fleet posture</div>
              <div class="desc">Returns connector count, degraded lanes, auth failures, latency posture, and the lead recommendation.</div>
            </div>
            <div class="metric-card">
              <div class="micro">GET /api/connectors</div>
              <div class="title">Connector board</div>
              <div class="desc">Returns the connector fleet ordered by risk score with flags and operator recommendations attached.</div>
            </div>
            <div class="metric-card">
              <div class="micro">GET /metrics</div>
              <div class="title">Prometheus payload</div>
              <div class="desc">Exposes connector health score, auth failures, latency, saturation, and OTel coverage in text format.</div>
            </div>
          </div>
        </div>
      </section>
    "#;

    shell(
        "Rust exporter documentation for connector observability.",
        "Prometheus metrics, OpenTelemetry export posture, and a review-friendly connector-health model.",
        "docs",
        body,
    )
}

fn connector_card(connector: &ConnectorAssessment) -> String {
    let flags = if connector.exposure_flags.is_empty() {
        r#"<span class="signal-pill">Stable telemetry lane</span>"#.to_string()
    } else {
        connector
            .exposure_flags
            .iter()
            .map(|flag| format!(r#"<span class="signal-pill">{}</span>"#, flag))
            .collect::<Vec<_>>()
            .join("")
    };

    format!(
        r#"<div class="connector-card">
          <div class="connector-top">
            <div>
              <h3>{name}</h3>
              <div class="meta-text">{pool} · {region} · {target} · {auth_mode}</div>
            </div>
            <span class="tag {verdict_class}">{verdict}</span>
            <div class="score-stack"><div class="label">Risk score</div><div class="value">{risk}</div></div>
          </div>
          <div class="connector-bottom">
            <div class="two-col">
              <div>{bars}</div>
              <div class="panel-grid">
                <div class="metric-card">
                  <div class="micro">Top concern</div>
                  <div class="title">{top_concern}</div>
                  <div class="desc">{recommendation}</div>
                </div>
                <div class="metric-card">
                  <div class="micro">Exposure flags</div>
                  <div class="pill-stack">{flags}</div>
                </div>
              </div>
            </div>
          </div>
        </div>"#,
        name = connector.snapshot.name,
        pool = connector.snapshot.pool,
        region = connector.snapshot.region,
        target = connector.snapshot.target,
        auth_mode = connector.snapshot.auth_mode,
        verdict_class = connector.verdict,
        verdict = connector.verdict,
        risk = connector.risk_score,
        bars = score_bars(connector),
        top_concern = connector.top_concern,
        recommendation = connector.recommendation,
        flags = flags
    )
}
