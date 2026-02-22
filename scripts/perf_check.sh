#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BASELINE_FILE="${1:-$ROOT_DIR/perf/baseline.json}"
CURRENT_FILE="$ROOT_DIR/perf/current.json"
THRESHOLD="${PERF_THRESHOLD:-0.12}"
ITERATIONS="${PERF_ITERATIONS:-20000}"

mkdir -p "$(dirname "$BASELINE_FILE")" "$ROOT_DIR/perf"

echo "[perf] collecting current snapshot..."
cargo run -p makepad-router-bench --release --bin perf_snapshot -- \
  --output "$CURRENT_FILE" \
  --iterations "$ITERATIONS"

if [[ ! -f "$BASELINE_FILE" ]]; then
  cp "$CURRENT_FILE" "$BASELINE_FILE"
  echo "[perf] baseline created at $BASELINE_FILE"
  echo "[perf] rerun scripts/perf_check.sh to compare against baseline"
  exit 0
fi

echo "[perf] comparing against baseline..."
cargo run -p makepad-router-bench --release --bin perf_snapshot -- \
  --output "$CURRENT_FILE" \
  --baseline "$BASELINE_FILE" \
  --threshold "$THRESHOLD" \
  --iterations "$ITERATIONS"

echo "[perf] snapshot check passed (threshold=${THRESHOLD})"

if [[ "${PERF_SKIP_CRITERION:-0}" != "1" ]]; then
  echo "[perf] running criterion benchmark suite..."
  cargo bench -p makepad-router-bench --bench router_perf -- --noplot
else
  echo "[perf] skipping criterion run (PERF_SKIP_CRITERION=1)"
fi

echo "[perf] done"
