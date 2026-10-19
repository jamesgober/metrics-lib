#!/usr/bin/env bash
set -euo pipefail

# Run every example (or a chosen subset) in release mode with all features.
# Every example terminates on its own; `prometheus_endpoint` serves for
# SERVE_FOR_SECS seconds (default 0).
# Usage:
#   bash tools/run_examples.sh
#   EXAMPLES="quick_start,quick_tour" bash tools/run_examples.sh

ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT_DIR"

if [ -n "${EXAMPLES:-}" ]; then
  IFS=',' read -r -a EX_LIST <<< "$EXAMPLES"
else
  EX_LIST=()
  for f in examples/*.rs; do
    EX_LIST+=("$(basename "$f" .rs)")
  done
fi

pass() { printf "\033[32m[PASS]\033[0m %s\n" "$1"; }
fail() { printf "\033[31m[FAIL]\033[0m %s\n" "$1"; }

for ex in "${EX_LIST[@]}"; do
  echo "--- Running example: $ex ---"
  if cargo run --example "$ex" --release --all-features -q; then
    pass "$ex"
  else
    fail "$ex"
    exit 1
  fi
  echo
done

echo "All selected examples completed successfully."
