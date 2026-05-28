from pathlib import Path
from textwrap import dedent
from html import escape


ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / "screenshots"
OUT.mkdir(exist_ok=True)
LEGACY = [
    "01-hero.png",
    "02-queue-lanes.png",
    "03-escalation-detail.png",
    "04-proof.png",
    "01-overview.svg",
    "02-connector-board.svg",
    "03-audit-log.svg",
    "04-metrics-proof.svg",
    "01-overview-v2.svg",
    "02-connector-board-v2.svg",
    "03-audit-log-v2.svg",
    "04-metrics-proof-v2.svg",
]


CONNECTORS = [
    {
        "name": "Vendor access relay",
        "pool": "vendor-broker-pool",
        "region": "us-central",
        "verdict": "CRITICAL",
        "tag_class": "critical",
        "risk": 100,
        "saturation": 96,
        "auth": 33,
        "latency": 691,
        "freshness": 24,
        "flags": ["legacy auth", "no OTel export", "expired credential"],
        "concern": "Legacy API-key lane is now both the noisiest and the least trustworthy telemetry surface.",
        "recommendation": "Drain pool pressure, rotate credentials, and move this lane into the OTel path before the next review window.",
    },
    {
        "name": "EMEA PSM connector",
        "pool": "psm-emea-primary",
        "region": "emea",
        "verdict": "CRITICAL",
        "tag_class": "warning",
        "risk": 84,
        "saturation": 88,
        "auth": 14,
        "latency": 412,
        "freshness": 70,
        "flags": ["pool saturation", "auth retries", "certificate nearing expiry"],
        "concern": "Session pressure and certificate drift are rising together, which makes this connector unstable under load.",
        "recommendation": "Reduce active pool pressure, refresh the cert, and keep the connector in the priority remediation queue.",
    },
    {
        "name": "LATAM PVWA gateway",
        "pool": "pvwa-latam",
        "region": "latam",
        "verdict": "HEALTHY",
        "tag_class": "healthy",
        "risk": 38,
        "saturation": 69,
        "auth": 7,
        "latency": 264,
        "freshness": 80,
        "flags": ["watch pool", "latency drift", "federation dependency"],
        "concern": "Still stable, but pool pressure and federated-auth drift could make this lane noisy quickly.",
        "recommendation": "Preserve current cadence and keep latency under the watch threshold before traffic spikes.",
    },
]

AUDIT_ROWS = [
    ("2026-05-14 10:01:12", "SCRAPE_STARTED", "cyberark_psm_emea", "Fresh connector-health scrape launched for the EMEA PSM lane.", "success"),
    ("2026-05-14 10:01:41", "POOL_SATURATION_FLAGGED", "vendor-broker-pool", "Active sessions and auth retries crossed the critical threshold for the vendor connector.", "warning"),
    ("2026-05-14 10:02:09", "OTEL_EXPORT_SKIPPED", "cyberark_vendor_access", "Telemetry export was skipped because the connector still uses a legacy API-key lane without OTel wiring.", "failure"),
    ("2026-05-14 10:02:44", "PROMETHEUS_PAYLOAD_EMITTED", "cyberark_connector_*", "Connector health, latency, saturation, and auth-failure metrics emitted into Prometheus format.", "success"),
    ("2026-05-14 10:03:17", "REMEDIATION_RECOMMENDATION", "EMEA PSM connector", "Clear saturation, rotate certificate, and reduce auth-failure pressure before the next review window.", "success"),
]

METRICS_SAMPLE = """# HELP cyberark_connector_health_score Risk-oriented health score for each connector.
# TYPE cyberark_connector_health_score gauge
cyberark_connector_health_score{connector="Vendor access relay",pool="vendor-broker-pool",region="us-central",status="critical"} 0
cyberark_connector_health_score{connector="EMEA PSM connector",pool="psm-emea-primary",region="emea",status="degraded"} 16
cyberark_connector_auth_failures_total{connector="Vendor access relay",pool="vendor-broker-pool",region="us-central",status="critical"} 33
cyberark_connector_latency_p95_ms{connector="Vendor access relay",pool="vendor-broker-pool",region="us-central",status="critical"} 691
cyberark_connector_pool_saturation_percent{connector="Vendor access relay",pool="vendor-broker-pool",region="us-central",status="critical"} 96
cyberark_connector_otel_enabled{connector="Vendor access relay",pool="vendor-broker-pool",region="us-central",status="critical"} 0
cyberark_connector_auth_failures_total{connector="EMEA PSM connector",pool="psm-emea-primary",region="emea",status="degraded"} 14"""


def write(name: str, content: str) -> None:
    (OUT / name).write_text(content, encoding="utf-8")


def wrapped_text(
    text: str,
    x: int,
    y: int,
    max_chars: int,
    line_height: int,
    font_size: int,
    fill: str,
    font_family: str,
    font_weight: str | int = "400",
    letter_spacing: int | None = None,
) -> str:
    words = text.split()
    lines: list[str] = []
    current = ""
    for word in words:
        candidate = word if not current else f"{current} {word}"
        if len(candidate) <= max_chars:
            current = candidate
        else:
            if current:
                lines.append(current)
            current = word
    if current:
        lines.append(current)

    tspans = []
    for index, line in enumerate(lines):
        dy = "0" if index == 0 else str(line_height)
        extra = f' letter-spacing="{letter_spacing}"' if letter_spacing is not None else ""
        tspans.append(
            f'<tspan x="{x}" dy="{dy}"{extra}>{escape(line)}</tspan>'
        )

    return (
        f'<text x="{x}" y="{y}" fill="{fill}" font-size="{font_size}" '
        f'font-family="{font_family}" font-weight="{font_weight}">{"".join(tspans)}</text>'
    )


def svg_shell(title: str, body: str) -> str:
    return dedent(
        f"""\
        <svg xmlns="http://www.w3.org/2000/svg" width="1600" height="980" viewBox="0 0 1600 980">
          <defs>
            <linearGradient id="bgFade" x1="0" x2="1" y1="0" y2="1">
              <stop offset="0%" stop-color="#050b14" />
              <stop offset="100%" stop-color="#091120" />
            </linearGradient>
            <linearGradient id="panelStroke" x1="0" x2="1" y1="0" y2="1">
              <stop offset="0%" stop-color="rgba(117, 200, 255, 0.36)" />
              <stop offset="100%" stop-color="rgba(93, 120, 255, 0.16)" />
            </linearGradient>
            <linearGradient id="blueBar" x1="0" x2="0" y1="0" y2="1">
              <stop offset="0%" stop-color="#74c8ff" />
              <stop offset="100%" stop-color="#5d78ff" />
            </linearGradient>
            <linearGradient id="goodBar" x1="0" x2="1" y1="0" y2="0">
              <stop offset="0%" stop-color="#2f92d6" />
              <stop offset="100%" stop-color="#49d79e" />
            </linearGradient>
            <linearGradient id="warnBar" x1="0" x2="1" y1="0" y2="0">
              <stop offset="0%" stop-color="#4d8cf2" />
              <stop offset="100%" stop-color="#f6c46a" />
            </linearGradient>
            <linearGradient id="hotBar" x1="0" x2="1" y1="0" y2="0">
              <stop offset="0%" stop-color="#d14d6c" />
              <stop offset="100%" stop-color="#ff7987" />
            </linearGradient>
            <filter id="glow">
              <feGaussianBlur stdDeviation="14" result="blur" />
              <feMerge>
                <feMergeNode in="blur" />
                <feMergeNode in="SourceGraphic" />
              </feMerge>
            </filter>
          </defs>
          <rect width="1600" height="980" fill="url(#bgFade)" />
          <circle cx="180" cy="120" r="240" fill="rgba(116,200,255,0.12)" />
          <circle cx="1410" cy="140" r="180" fill="rgba(255,121,135,0.08)" />
          <circle cx="1480" cy="860" r="220" fill="rgba(93,120,255,0.12)" />
          <rect x="0" y="0" width="244" height="980" fill="rgba(0,0,0,0.25)" />
          <rect x="24" y="26" width="196" height="72" rx="18" fill="rgba(11,17,28,0.94)" stroke="rgba(255,255,255,0.06)" />
          <rect x="42" y="42" width="40" height="40" rx="12" fill="url(#blueBar)" filter="url(#glow)" />
          <text x="62" y="67" fill="#ffffff" font-size="18" text-anchor="middle" font-family="Inter, Segoe UI, sans-serif" font-weight="800">CE</text>
          <text x="98" y="57" fill="#f4f7ff" font-size="14" font-family="Inter, Segoe UI, sans-serif" font-weight="700">CyberArk Connector</text>
          <text x="98" y="77" fill="#74c8ff" font-size="10" letter-spacing="3" font-family="Inter, Segoe UI, sans-serif" font-weight="700">OBSERVABILITY EXPORTER</text>
          <text x="42" y="148" fill="#6f83a0" font-size="10" letter-spacing="3" font-family="Inter, Segoe UI, sans-serif" font-weight="700">SURFACES</text>
          <rect x="28" y="168" width="188" height="46" rx="14" fill="rgba(116,200,255,0.08)" stroke="rgba(116,200,255,0.16)" />
          <text x="46" y="196" fill="#bde6ff" font-size="12" letter-spacing="2" font-family="Inter, Segoe UI, sans-serif" font-weight="700">OVERVIEW</text>
          <text x="42" y="262" fill="#8394ad" font-size="11" font-family="Inter, Segoe UI, sans-serif">Exporter lane</text>
          <text x="42" y="282" fill="#f4f7ff" font-size="13" font-family="Inter, Segoe UI, sans-serif" font-weight="700">Prometheus + OTel bridge</text>
          <text x="42" y="328" fill="#8394ad" font-size="11" font-family="Inter, Segoe UI, sans-serif">Degraded connectors</text>
          <text x="42" y="348" fill="#f4f7ff" font-size="28" font-family="Inter, Segoe UI, sans-serif" font-weight="800">2</text>
          <text x="42" y="394" fill="#8394ad" font-size="11" font-family="Inter, Segoe UI, sans-serif">Auth failures / 24h</text>
          <text x="42" y="414" fill="#f4f7ff" font-size="28" font-family="Inter, Segoe UI, sans-serif" font-weight="800">57</text>
          <rect x="278" y="36" width="1288" height="70" rx="18" fill="rgba(0,0,0,0.28)" stroke="rgba(255,255,255,0.06)" />
          <rect x="300" y="55" width="252" height="32" rx="999" fill="rgba(116,200,255,0.08)" stroke="rgba(116,200,255,0.16)" />
          <circle cx="322" cy="71" r="6" fill="#74c8ff" filter="url(#glow)" />
          <text x="338" y="76" fill="#bde6ff" font-size="11" letter-spacing="3" font-family="Inter, Segoe UI, sans-serif" font-weight="700">CONNECTOR SCRAPE LOOP LIVE</text>
          <text x="1095" y="62" fill="#6f83a0" font-size="9" letter-spacing="2" font-family="Inter, Segoe UI, sans-serif" font-weight="700">TELEMETRY COVERAGE</text>
          <text x="1095" y="81" fill="#f4f7ff" font-size="12" font-family="Inter, Segoe UI, sans-serif" font-weight="700">80% OTel enabled</text>
          <text x="1290" y="62" fill="#6f83a0" font-size="9" letter-spacing="2" font-family="Inter, Segoe UI, sans-serif" font-weight="700">EXPORTER HEALTH</text>
          <text x="1290" y="81" fill="#f4f7ff" font-size="12" font-family="Inter, Segoe UI, sans-serif" font-weight="700">4 / 5 lanes stable</text>
          {body}
          <text x="300" y="950" fill="#6f83a0" font-size="10" letter-spacing="2" font-family="Inter, Segoe UI, sans-serif" font-weight="700">DISCIPLINE</text>
          <text x="390" y="950" fill="#c7d5ea" font-size="10" font-family="Inter, Segoe UI, sans-serif" font-weight="700">connector observability</text>
          <text x="650" y="950" fill="#6f83a0" font-size="10" letter-spacing="2" font-family="Inter, Segoe UI, sans-serif" font-weight="700">FOCUS</text>
          <text x="707" y="950" fill="#c7d5ea" font-size="10" font-family="Inter, Segoe UI, sans-serif" font-weight="700">pool health / auth failures / latency / export coverage</text>
          <text x="1220" y="950" fill="#6f83a0" font-size="10" letter-spacing="2" font-family="Inter, Segoe UI, sans-serif" font-weight="700">{title}</text>
        </svg>
        """
    )


def hero(title: str, subtitle: str) -> str:
    return dedent(
        f"""\
        <rect x="278" y="132" width="1288" height="292" rx="28" fill="rgba(9,16,28,0.96)" stroke="rgba(255,255,255,0.06)" />
        <text x="320" y="172" fill="#74c8ff" font-size="11" letter-spacing="5" font-family="Inter, Segoe UI, sans-serif" font-weight="700">CYBERARK CONNECTOR OBSERVABILITY EXPORTER</text>
        {wrapped_text(title, 320, 234, 50, 46, 40, "#f5f7fd", "Georgia, Times New Roman, serif", "700")}
        {wrapped_text(subtitle, 320, 322, 80, 28, 21, "#96a9c6", "Inter, Segoe UI, sans-serif")}
        <rect x="320" y="342" width="248" height="126" rx="18" fill="rgba(255,255,255,0.03)" stroke="rgba(255,255,255,0.06)" />
        <text x="344" y="368" fill="#71839d" font-size="10" letter-spacing="3" font-family="Inter, Segoe UI, sans-serif" font-weight="700">CONNECTORS</text>
        <text x="344" y="414" fill="#f5f7fd" font-size="34" font-family="Inter, Segoe UI, sans-serif" font-weight="800">5</text>
        {wrapped_text("Total modeled fleet across PSM, CPM, PVWA, vendor, and identity lanes.", 344, 436, 30, 18, 13, "#96a9c6", "Inter, Segoe UI, sans-serif")}
        <rect x="586" y="342" width="248" height="126" rx="18" fill="rgba(255,255,255,0.03)" stroke="rgba(255,255,255,0.06)" />
        <text x="610" y="368" fill="#71839d" font-size="10" letter-spacing="3" font-family="Inter, Segoe UI, sans-serif" font-weight="700">DEGRADED CONNECTORS</text>
        <text x="610" y="414" fill="#f5f7fd" font-size="34" font-family="Inter, Segoe UI, sans-serif" font-weight="800">2</text>
        {wrapped_text("Vendor access and EMEA PSM are now carrying the bulk of connector risk.", 610, 436, 30, 18, 13, "#96a9c6", "Inter, Segoe UI, sans-serif")}
        <rect x="852" y="342" width="248" height="126" rx="18" fill="rgba(255,255,255,0.03)" stroke="rgba(255,255,255,0.06)" />
        <text x="876" y="368" fill="#71839d" font-size="10" letter-spacing="3" font-family="Inter, Segoe UI, sans-serif" font-weight="700">OTEL COVERAGE</text>
        <text x="876" y="414" fill="#f5f7fd" font-size="34" font-family="Inter, Segoe UI, sans-serif" font-weight="800">80%</text>
        {wrapped_text("Only one connector still sits outside the modern telemetry export path.", 876, 436, 30, 18, 13, "#96a9c6", "Inter, Segoe UI, sans-serif")}
        <rect x="1118" y="342" width="410" height="126" rx="18" fill="rgba(255,255,255,0.03)" stroke="rgba(255,255,255,0.06)" />
        <text x="1142" y="368" fill="#71839d" font-size="10" letter-spacing="3" font-family="Inter, Segoe UI, sans-serif" font-weight="700">LEAD RECOMMENDATION</text>
        {wrapped_text("Clear vendor saturation and legacy-auth pressure first.", 1142, 398, 33, 24, 16, "#f6c46a", "Georgia, Times New Roman, serif", "700")}
        {wrapped_text("Drain the broker pool, rotate credentials, and move the lane into OTel before the next review window.", 1142, 438, 46, 18, 13, "#dce7fb", "Inter, Segoe UI, sans-serif")}
        """
    )


def bar_chart() -> str:
    bars = [
        ("Vendor", 96, 116),
        ("EMEA PSM", 88, 106),
        ("LATAM PVWA", 69, 84),
        ("CPM", 51, 62),
        ("Identity", 47, 58),
    ]
    pieces = []
    x = 326
    for label, value, height in bars:
        pieces.append(
            f"""
            <rect x="{x}" y="{790-height}" width="90" height="{height}" rx="18" fill="url(#blueBar)" />
            <text x="{x+45}" y="{808-height}" fill="#f5f7fd" font-size="16" text-anchor="middle" font-family="Inter, Segoe UI, sans-serif" font-weight="800">{value}%</text>
            <text x="{x+45}" y="820" fill="#96a9c6" font-size="11" text-anchor="middle" font-family="Inter, Segoe UI, sans-serif" font-weight="700">{label}</text>
            """
        )
        x += 112
    return "".join(pieces)


def connector_row(x: int, y: int, w: int, connector: dict) -> str:
    tag_bg = {
        "critical": "rgba(255,121,135,0.12)",
        "warning": "rgba(246,196,106,0.12)",
        "healthy": "rgba(73,215,158,0.12)",
    }[connector["tag_class"]]
    tag_fg = {
        "critical": "#ff92a2",
        "warning": "#f6c46a",
        "healthy": "#49d79e",
    }[connector["tag_class"]]
    flags = ""
    flag_x = x + 24
    flag_y = y + 146
    for flag in connector["flags"]:
        width = max(126, 12 * len(flag))
        flags += f'<rect x="{flag_x}" y="{flag_y}" width="{width}" height="26" rx="13" fill="rgba(116,200,255,0.08)" /><text x="{flag_x + width/2}" y="{flag_y + 17}" fill="#74c8ff" font-size="10" text-anchor="middle" font-family="Inter, Segoe UI, sans-serif" font-weight="700">{flag.upper()}</text>'
        flag_x += width + 10

    return dedent(
        f"""\
        <rect x="{x}" y="{y}" width="{w}" height="220" rx="22" fill="rgba(4,9,18,0.62)" stroke="rgba(255,255,255,0.06)" />
        <text x="{x+24}" y="{y+36}" fill="#f5f7fd" font-size="24" font-family="Inter, Segoe UI, sans-serif" font-weight="800">{connector["name"]}</text>
        <text x="{x+24}" y="{y+60}" fill="#96a9c6" font-size="13" font-family="Inter, Segoe UI, sans-serif">{connector["pool"]} · {connector["region"]}</text>
        <rect x="{x+w-178}" y="{y+22}" width="96" height="30" rx="15" fill="{tag_bg}" stroke="rgba(255,255,255,0.06)" />
        <text x="{x+w-130}" y="{y+41}" fill="{tag_fg}" font-size="10" text-anchor="middle" font-family="Inter, Segoe UI, sans-serif" font-weight="800">{connector["verdict"]}</text>
        <text x="{x+w-56}" y="{y+30}" fill="#6f83a0" font-size="9" text-anchor="middle" font-family="Inter, Segoe UI, sans-serif" font-weight="700">RISK</text>
        <text x="{x+w-56}" y="{y+56}" fill="#f5f7fd" font-size="24" text-anchor="middle" font-family="Inter, Segoe UI, sans-serif" font-weight="800">{connector["risk"]}</text>
        <text x="{x+24}" y="{y+96}" fill="#6f83a0" font-size="10" letter-spacing="2" font-family="Inter, Segoe UI, sans-serif" font-weight="700">TOP CONCERN</text>
        {wrapped_text(connector["concern"], x + 24, y + 120, 55, 18, 14, "#dce7fb", "Inter, Segoe UI, sans-serif")}
        {flags}
        {wrapped_text(connector["recommendation"], x + 24, y + 198, 64, 16, 11, "#f6c46a", "Inter, Segoe UI, sans-serif", "700")}
        """
    )


def meter_block(x: int, y: int, width: int, label: str, value: int, tone: str) -> str:
    tone_map = {"good": "url(#goodBar)", "watch": "url(#warnBar)", "hot": "url(#hotBar)"}
    fill_width = int((width - 2) * value / 100)
    return dedent(
        f"""\
        <text x="{x}" y="{y}" fill="#dce7fb" font-size="12" font-family="Inter, Segoe UI, sans-serif" font-weight="700">{label}</text>
        <text x="{x+width}" y="{y}" fill="#dce7fb" font-size="12" text-anchor="end" font-family="Inter, Segoe UI, sans-serif" font-weight="700">{value}%</text>
        <rect x="{x}" y="{y+10}" width="{width}" height="10" rx="999" fill="rgba(255,255,255,0.06)" />
        <rect x="{x}" y="{y+10}" width="{fill_width}" height="10" rx="999" fill="{tone_map[tone]}" />
        """
    )


def connector_detail_card(x: int, y: int, connector: dict) -> str:
    tone = "hot" if connector["risk"] >= 80 else "watch" if connector["risk"] >= 55 else "good"
    return dedent(
        f"""\
        <rect x="{x}" y="{y}" width="392" height="352" rx="24" fill="rgba(6,11,20,0.84)" stroke="rgba(255,255,255,0.06)" />
        <text x="{x+24}" y="{y+34}" fill="#74c8ff" font-size="10" letter-spacing="3" font-family="Inter, Segoe UI, sans-serif" font-weight="700">{connector["pool"].upper()}</text>
        <text x="{x+24}" y="{y+68}" fill="#f5f7fd" font-size="28" font-family="Inter, Segoe UI, sans-serif" font-weight="800">{connector["name"]}</text>
        <text x="{x+24}" y="{y+92}" fill="#96a9c6" font-size="13" font-family="Inter, Segoe UI, sans-serif">{connector["region"]} · {connector["verdict"]} posture</text>
        <rect x="{x+278}" y="{y+24}" width="90" height="32" rx="16" fill="rgba(255,255,255,0.04)" stroke="rgba(255,255,255,0.06)" />
        <text x="{x+323}" y="{y+44}" fill="#f5f7fd" font-size="18" text-anchor="middle" font-family="Inter, Segoe UI, sans-serif" font-weight="800">{connector["risk"]}</text>
        {meter_block(x+24, y+132, 344, "Pool saturation", connector["saturation"], tone)}
        {meter_block(x+24, y+178, 344, "Auth failure pressure", min(100, connector["auth"] * 3), tone)}
        {meter_block(x+24, y+224, 344, "Latency pressure", min(100, round(connector["latency"] / 7)), tone)}
        {meter_block(x+24, y+270, 344, "Telemetry freshness", connector["freshness"], "good" if connector["freshness"] > 75 else "watch")}
        {wrapped_text(connector["recommendation"], x + 24, y + 318, 48, 16, 11, "#dce7fb", "Inter, Segoe UI, sans-serif")}
        """
    )


def overview_svg() -> str:
    body = hero(
        "Control-plane summary for CyberArk connector health.",
        "Connector count, pool pressure, auth failures, latency, and exporter coverage at a glance.",
    )
    body += dedent(
        f"""\
        <rect x="278" y="460" width="676" height="440" rx="26" fill="rgba(9,16,28,0.92)" stroke="rgba(255,255,255,0.06)" />
        <text x="314" y="456" fill="#74c8ff" font-size="10" letter-spacing="3" font-family="Inter, Segoe UI, sans-serif" font-weight="700">POOL PRESSURE BY CONNECTOR LANE</text>
        {wrapped_text("Where saturation is starting to outrun comfortable connector health.", 314, 496, 42, 30, 24, "#f5f7fd", "Georgia, Times New Roman, serif", "700")}
        {wrapped_text("A connector can look alive and still be heading toward blind-spot territory if saturation, stale scrapes, and auth failures rise together.", 314, 560, 66, 20, 14, "#96a9c6", "Inter, Segoe UI, sans-serif")}
        <line x1="320" y1="790" x2="910" y2="790" stroke="rgba(255,255,255,0.10)" />
        {bar_chart()}
        <rect x="976" y="460" width="590" height="440" rx="26" fill="rgba(9,16,28,0.92)" stroke="rgba(255,255,255,0.06)" />
        <text x="1012" y="456" fill="#74c8ff" font-size="10" letter-spacing="3" font-family="Inter, Segoe UI, sans-serif" font-weight="700">TOP CONNECTOR BOARD</text>
        {wrapped_text("The riskiest exporter lanes stay visible.", 1012, 496, 32, 30, 24, "#f5f7fd", "Georgia, Times New Roman, serif", "700")}
        {connector_row(1008, 554, 526, CONNECTORS[0])}
        {wrapped_text("The README should show why this exporter matters immediately: connector pressure, exporter trust, and remediation direction on the same surface.", 1012, 796, 58, 20, 13, "#dce7fb", "Inter, Segoe UI, sans-serif")}
        """
    )
    return svg_shell("OVERVIEW SNAPSHOT", body)


def connectors_svg() -> str:
    body = hero(
        "Review queue for connector reliability pressure.",
        "The connectors most likely to need remediation first, with risk posture and export trust on the same board.",
    )
    body += dedent(
        f"""\
        <rect x="278" y="460" width="1288" height="440" rx="26" fill="rgba(9,16,28,0.92)" stroke="rgba(255,255,255,0.06)" />
        <text x="314" y="456" fill="#74c8ff" font-size="10" letter-spacing="3" font-family="Inter, Segoe UI, sans-serif" font-weight="700">CONNECTOR BOARD</text>
        {wrapped_text("Each connector combines pool saturation, auth failures, latency, and telemetry freshness.", 314, 496, 54, 30, 24, "#f5f7fd", "Georgia, Times New Roman, serif", "700")}
        {wrapped_text("This is the practical operator surface for deciding which connector pool needs load relief, auth cleanup, certificate work, or an OTel export fix.", 314, 560, 74, 20, 14, "#96a9c6", "Inter, Segoe UI, sans-serif")}
        {connector_detail_card(314, 548, CONNECTORS[0])}
        {connector_detail_card(734, 548, CONNECTORS[1])}
        {connector_detail_card(1154, 548, CONNECTORS[2])}
        """
    )
    return svg_shell("CONNECTOR BOARD", body)


def audit_svg() -> str:
    log_rows = []
    y = 588
    for timestamp, action, resource, detail, result in AUDIT_ROWS:
        tone = {"success": "#49d79e", "warning": "#f6c46a", "failure": "#ff7987"}[result]
        log_rows.append(
            f"""
            <rect x="332" y="{y-18}" width="1214" height="52" rx="14" fill="rgba(255,255,255,0.02)" />
            <text x="350" y="{y+6}" fill="#6f83a0" font-size="11" font-family="Consolas, 'Courier New', monospace">{timestamp}</text>
            <text x="520" y="{y+6}" fill="#74c8ff" font-size="11" font-family="Consolas, 'Courier New', monospace" font-weight="700">{action}</text>
            <text x="780" y="{y+6}" fill="#f5f7fd" font-size="12" font-family="Inter, Segoe UI, sans-serif" font-weight="700">{resource}</text>
            <text x="960" y="{y+6}" fill="#dce7fb" font-size="12" font-family="Inter, Segoe UI, sans-serif">{detail}</text>
            <text x="1506" y="{y+6}" text-anchor="end" fill="{tone}" font-size="11" font-family="Inter, Segoe UI, sans-serif" font-weight="700">{result.upper()}</text>
            """
        )
        y += 62
    body = hero(
        "Audit evidence for connector-exporter operations.",
        "A replayable log of scrapes, export failures, and remediation recommendations.",
    )
    body += dedent(
        f"""\
        <rect x="278" y="460" width="1288" height="440" rx="26" fill="rgba(9,16,28,0.92)" stroke="rgba(255,255,255,0.06)" />
        <text x="314" y="456" fill="#74c8ff" font-size="10" letter-spacing="3" font-family="Inter, Segoe UI, sans-serif" font-weight="700">AUDIT EVIDENCE</text>
        {wrapped_text("The exporter should leave behind a replayable trail, not just a gauge.", 314, 496, 47, 30, 24, "#f5f7fd", "Georgia, Times New Roman, serif", "700")}
        <rect x="314" y="548" width="1218" height="308" rx="22" fill="rgba(2,6,12,0.90)" stroke="rgba(255,255,255,0.08)" />
        <rect x="314" y="548" width="1218" height="46" rx="22" fill="rgba(255,255,255,0.03)" />
        <circle cx="338" cy="571" r="5" fill="rgba(255,121,135,0.7)" />
        <circle cx="356" cy="571" r="5" fill="rgba(246,196,106,0.7)" />
        <circle cx="374" cy="571" r="5" fill="rgba(73,215,158,0.7)" />
        <text x="398" y="575" fill="#74c8ff" font-size="10" letter-spacing="3" font-family="Inter, Segoe UI, sans-serif" font-weight="700">CONNECTOR-OBSERVABILITY RUNTIME LOG</text>
        {''.join(log_rows)}
        """
    )
    return svg_shell("AUDIT EVIDENCE", body)


def metrics_svg() -> str:
    metric_lines = []
    y = 632
    for line in METRICS_SAMPLE.splitlines():
        metric_lines.append(
            f'<text x="834" y="{y}" fill="#dce8fb" font-size="12" font-family="Consolas, \'Courier New\', monospace">{line}</text>'
        )
        y += 20
    body = hero(
        "Metrics preview for the Prometheus and OTel export path.",
        "The exporter makes connector health scrapeable, alertable, and reviewable in one lane.",
    )
    body += dedent(
        f"""\
        <rect x="278" y="460" width="500" height="440" rx="26" fill="rgba(9,16,28,0.92)" stroke="rgba(255,255,255,0.06)" />
        <text x="314" y="456" fill="#74c8ff" font-size="10" letter-spacing="3" font-family="Inter, Segoe UI, sans-serif" font-weight="700">EXPORTER CONFIGURATION</text>
        {wrapped_text("Prometheus text output plus clear exporter posture.", 314, 496, 42, 30, 24, "#f5f7fd", "Georgia, Times New Roman, serif", "700")}
        <rect x="314" y="548" width="428" height="88" rx="18" fill="rgba(255,255,255,0.03)" stroke="rgba(255,255,255,0.06)" />
        <text x="338" y="574" fill="#6f83a0" font-size="10" letter-spacing="3" font-family="Inter, Segoe UI, sans-serif" font-weight="700">SCRAPE INTERVAL</text>
        <text x="338" y="612" fill="#f5f7fd" font-size="30" font-family="Inter, Segoe UI, sans-serif" font-weight="800">30 seconds</text>
        <rect x="314" y="652" width="428" height="88" rx="18" fill="rgba(255,255,255,0.03)" stroke="rgba(255,255,255,0.06)" />
        <text x="338" y="678" fill="#6f83a0" font-size="10" letter-spacing="3" font-family="Inter, Segoe UI, sans-serif" font-weight="700">PROMETHEUS NAMESPACE</text>
        <text x="338" y="716" fill="#f5f7fd" font-size="22" font-family="Inter, Segoe UI, sans-serif" font-weight="800">cyberark_connector</text>
        <rect x="314" y="756" width="428" height="122" rx="18" fill="rgba(255,255,255,0.03)" stroke="rgba(255,255,255,0.06)" />
        <text x="338" y="782" fill="#6f83a0" font-size="10" letter-spacing="3" font-family="Inter, Segoe UI, sans-serif" font-weight="700">OTEL ENDPOINT</text>
        {wrapped_text("otel-collector.internal:4318/v1/metrics", 338, 820, 32, 18, 17, "#f5f7fd", "Inter, Segoe UI, sans-serif", "700")}
        <rect x="802" y="460" width="764" height="440" rx="26" fill="rgba(2,6,12,0.92)" stroke="rgba(255,255,255,0.08)" />
        <rect x="802" y="460" width="764" height="48" rx="26" fill="rgba(255,255,255,0.03)" />
        <circle cx="826" cy="484" r="5" fill="rgba(255,121,135,0.7)" />
        <circle cx="844" cy="484" r="5" fill="rgba(246,196,106,0.7)" />
        <circle cx="862" cy="484" r="5" fill="rgba(73,215,158,0.7)" />
        <text x="886" y="488" fill="#74c8ff" font-size="10" letter-spacing="3" font-family="Inter, Segoe UI, sans-serif" font-weight="700">/METRICS</text>
        {wrapped_text("The README proof should show the metrics surface is real, queryable, and tied back to connector health.", 834, 548, 58, 18, 13, "#96a9c6", "Inter, Segoe UI, sans-serif")}
        {''.join(metric_lines)}
        """
    )
    return svg_shell("METRICS SURFACE", body)


if __name__ == "__main__":
    for legacy in LEGACY:
        (OUT / legacy).unlink(missing_ok=True)
    write("01-overview-v2.svg", overview_svg())
    write("02-connector-board-v2.svg", connectors_svg())
    write("03-audit-log-v2.svg", audit_svg())
    write("04-metrics-proof-v2.svg", metrics_svg())
    print("rendered screenshots")
