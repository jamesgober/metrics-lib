<div align="center">
    <img width="90px" height="auto" src="https://raw.githubusercontent.com/jamesgober/jamesgober/main/media/icons/hexagon-3.svg" alt="Triple Hexagon">
    <br>
    <h1>
        <strong>metrics-lib</strong>
        <sup>
            <br>
            <sub>DOCUMENTATION</sub>
            <br>
        </sup>
    </h1>
</div>
<div align="center">
    <sup>
        <a href="../README.md" title="Project Home"><b>HOME</b></a>
        <span>&nbsp;│&nbsp;</span>
        <span>DOCS</span>
        <span>&nbsp;│&nbsp;</span>
        <a href="./API.md" title="API Reference"><b>API</b></a>
        <span>&nbsp;│&nbsp;</span>
        <a href="./GUIDELINES.md" title="Developer Guidelines"><b>GUIDELINES</b></a>
    </sup>
</div>
<br>

## Overview

Welcome to the metrics-lib documentation. This section is the entry point to the full documentation set, including the API reference, developer guidelines, performance notes, and change history.

## Quick Links

- API Reference: [`docs/API.md`](./API.md)
- Developer Guidelines: [`docs/GUIDELINES.md`](./GUIDELINES.md)
- Performance Review: [`docs/PERFORMANCE_REVIEW.md`](./PERFORMANCE_REVIEW.md)
- Changelog: [`CHANGELOG.md`](../CHANGELOG.md)
- License: [`LICENSE`](../LICENSE)

### Integration Examples & Observability Assets
- Integration Examples: see [`API.md` → Integration Examples](./API.md#integration-examples)
- Grafana dashboard (ready to import): [`docs/observability/grafana-dashboard.json`](./observability/grafana-dashboard.json)
- Prometheus recording rules: [`docs/observability/recording-rules.yaml`](./observability/recording-rules.yaml)
- Kubernetes Service: [`docs/k8s/service.yaml`](./k8s/service.yaml)
- Prometheus Operator ServiceMonitor: [`docs/k8s/servicemonitor.yaml`](./k8s/servicemonitor.yaml)
- Secured ServiceMonitor (TLS/Bearer): [`docs/k8s/servicemonitor-secured.yaml`](./k8s/servicemonitor-secured.yaml)

#### Quick Start Commands
- Import Grafana dashboard (via UI): Dashboards → New → Import → Upload `grafana-dashboard.json`.
- Import Grafana dashboard (via API):
```bash
curl -X POST \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer <GRAFANA_API_TOKEN>" \
  http://<grafana-host>/api/dashboards/db \
  -d @docs/observability/grafana-dashboard.json
```

- Validate Prometheus recording rules locally:
```bash
promtool check rules docs/observability/recording-rules.yaml
```

- Apply Kubernetes Service and ServiceMonitor:
```bash
kubectl apply -f docs/k8s/service.yaml
kubectl apply -f docs/k8s/servicemonitor.yaml
# For secured endpoints:
kubectl apply -f docs/k8s/servicemonitor-secured.yaml
```

## Table of Contents

- Installation and Quick Start: see the top-level [`README.md`](../README.md#installation) and [`README.md#quick-start`](../README.md#quick-start)
- Public APIs: [`Counter`](./API.md#counter), [`Gauge`](./API.md#gauge), [`Timer`](./API.md#timer), [`RateMeter`](./API.md#ratemeter), [`SystemHealth`](./API.md#systemhealth)
- Async Support: [`AsyncTimerExt`, `AsyncMetricBatch`](./API.md#async-support)
- Adaptive Controls: sampling, circuit breakers, backpressure in [`API.md`](./API.md#adaptive-controls)
- Contributing: [`CONTRIBUTING.md`](../CONTRIBUTING.md) (if present) and [`GUIDELINES.md`](./GUIDELINES.md)

## Getting Help

- Open an issue: https://github.com/jamesgober/metrics-lib/issues
- Discussions: https://github.com/jamesgober/metrics-lib/discussions

## Version Compatibility

- MSRV (Minimum Supported Rust Version): 1.70+
- See [`Cargo.toml`](../Cargo.toml) for feature flags and defaults


<br>

<!-- FOOT COPYRIGHT
################################################# -->
<div align="center">
  <h2></h2>
  <sup>COPYRIGHT <small>&copy;</small> 2025 <strong>JAMES GOBER.</strong></sup>
</div>
