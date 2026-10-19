<div align="center">
        <img width="99" alt="Rust logo" src="https://raw.githubusercontent.com/jamesgober/rust-collection/72baabd71f00e14aa9184efcb16fa3deddda3a0a/assets/rust-logo.svg">
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
        <a href="./GUIDELINES.md" title="Guidelines"><b>GUIDELINES</b></a>
    </sup>
</div>
<br>

## Guides

- [API reference](./API.md): every type and exporter with runnable examples.
- [Migrating from 1.x to 2.0](./MIGRATION-2.0.md)
- [Migrating from the `metrics` crate](./migrating-from-metrics-rs.md)
- [Performance](./PERFORMANCE.md): measured figures and how they were taken.
- [Performance tuning](./performance-tuning.md)
- [Verifying the hot-path cost](./zero-overhead-proof.md)
- [API stability](./STABILITY.md): the 2.x promise.
- [Development guidelines](./GUIDELINES.md)
- [Release notes](./release/)
- [Changelog](../CHANGELOG.md)

## Observability assets

Examples that match the metric names this crate exports:

- [Grafana dashboard](./observability/grafana-dashboard.json):
  request rate and p95 latency from the `tower` middleware's
  `http_requests_total` and `http_request_duration_seconds`.
- [Prometheus recording rules](./observability/recording-rules.yaml)
- Kubernetes: [Service](./k8s/service.yaml),
  [ServiceMonitor](./k8s/servicemonitor.yaml),
  [secured ServiceMonitor](./k8s/servicemonitor-secured.yaml), and a
  [Helm chart fragment](./k8s/helm/) wiring both plus the rules.

Validate the rules and import the dashboard:

```bash
promtool check rules docs/observability/recording-rules.yaml
curl -X POST -H "Content-Type: application/json" \
  -H "Authorization: Bearer <GRAFANA_API_TOKEN>" \
  http://<grafana-host>/api/dashboards/db \
  -d @docs/observability/grafana-dashboard.json
kubectl apply -f docs/k8s/service.yaml -f docs/k8s/servicemonitor.yaml
```

## Help

- Issues: <https://github.com/jamesgober/metrics-lib/issues>
- MSRV: Rust 1.85. Feature flags: see the crate documentation or
  [`Cargo.toml`](../Cargo.toml).

<br>

<div align="center">
  <h2></h2>
  <sup>COPYRIGHT <small>&copy;</small> 2026 <strong>JAMES GOBER.</strong></sup>
</div>
