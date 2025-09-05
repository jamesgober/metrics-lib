#!/usr/bin/env bash
set -euo pipefail

# Run a curated set of non-blocking examples sequentially in release mode.
# Skips server-blocking examples (e.g., axum_middleware_metrics).
# Usage:
#   bash tools/run_examples.sh
#   EXAMPLES="quick_start,quick_tour" bash tools/run_examples.sh

ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT_DIR"

IFS=',' read -r -a EX_LIST <<< "${EXAMPLES:-quick_start,quick_tour,async_batch_timing,token_bucket_limiter,custom_exporter_openmetrics,contention_admission,cpu_stats,memory_stats,cache_hit_miss,broker_throughput,benchmark_comparison,streaming_rate_window,axum_registry_integration}"

pass() { printf "\033[32m[PASS]\033[0m %s\n" "$1"; }
fail() { printf "\033[31m[FAIL]\033[0m %s\n" "$1"; }

for ex in "${EX_LIST[@]}"; do
  echo "--- Running example: $ex ---"
  if cargo run --example "$ex" --release -q; then
    pass "$ex"
  else
    fail "$ex"
    exit 1
  fi
  echo
done

echo "All selected examples completed successfully."
