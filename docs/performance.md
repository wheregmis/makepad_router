# Router Performance

This repository uses a benchmark-first performance workflow focused on the route lookup + dispatch hot path.

## Goals

- Primary target: `GoToPath` dispatch p95 under `100us` at `1k` routes (native release build, guards off, transitions off).
- Route lookup and dispatch regressions are checked locally with snapshot baselines.
- Native (`criterion`) and wasm-friendly (`router_perf_probe`) runs use the same scenario generators from `makepad-router-perf`.

## Packages

- `crates/makepad-router-perf`: shared route-table/scenario generators and percentile summaries.
- `crates/makepad-router-bench`: native criterion benchmarks + `perf_snapshot` binary.
- `examples/router_perf_probe`: deterministic perf probe executable for native/wasm-style runs.

## Commands

### Native criterion suite

```bash
cargo bench -p makepad-router-bench --bench router_perf
```

Bench groups:

- `route_lookup_exact_static_1k`
- `route_lookup_dynamic_1k`
- `route_lookup_mixed_1k`
- `dispatch_go_to_path_1k`
- `dispatch_go_to_path_nested_1k`
- `dispatch_go_to_route_1k`
- `dispatch_stack_ops`
- `dispatch_with_sync_guards`
- `dispatch_with_async_guard_pending`

Note: dispatch benchmarks run on the headless/core dispatch path (route resolve + history mutation), which isolates lookup/dispatch cost from draw/event/render overhead.

### Snapshot + local regression check

```bash
scripts/perf_check.sh
```

Behavior:

- Captures current snapshot to `perf/current.json`.
- Creates `perf/baseline.json` on first run.
- Fails locally if current p95 exceeds baseline by more than `PERF_THRESHOLD` (default `0.12` = 12%).
- Runs criterion after snapshot comparison unless `PERF_SKIP_CRITERION=1`.

Useful env vars:

- `PERF_THRESHOLD=0.15` (15% regression budget)
- `PERF_ITERATIONS=30000` (more stable snapshots)
- `PERF_SKIP_CRITERION=1` (snapshot-only check)

### Perf probe (native/wasm-style)

```bash
cargo run -p router_perf_probe --release
```

Optional iteration override:

```bash
ROUTER_PERF_ITERATIONS=20000 cargo run -p router_perf_probe --release
```

Output: JSON summary with `scenario`, `iterations`, `p50`, `p95`, `p99`, `mean`.

## Notes

- Snapshot metrics are microseconds (`us`).
- Compare results only on the same machine/profile.
- For wasm workflows, run the probe in your existing wasm runtime setup and collect the JSON output for comparison.
