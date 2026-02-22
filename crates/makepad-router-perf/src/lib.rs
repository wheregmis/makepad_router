use makepad_live_id::LiveId;
use makepad_router_core::{Route, RouteRegistry, Router};
use std::time::Instant;

#[derive(Clone, Debug)]
pub struct PerfRouteTable {
    pub registry: RouteRegistry,
    pub route_ids: Vec<LiveId>,
    pub static_paths: Vec<String>,
    pub dynamic_paths: Vec<String>,
    pub wildcard_paths: Vec<String>,
    pub miss_paths: Vec<String>,
    pub mixed_paths: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct PerfStats {
    pub scenario: String,
    pub iterations: usize,
    pub p50: f64,
    pub p95: f64,
    pub p99: f64,
    pub mean: f64,
}

pub fn build_mixed_route_table(route_count: usize) -> PerfRouteTable {
    let mut registry = RouteRegistry::new();
    let mut route_ids = Vec::with_capacity(route_count);
    let mut static_paths = Vec::new();
    let mut dynamic_paths = Vec::new();
    let mut wildcard_paths = Vec::new();
    let mut miss_paths = Vec::with_capacity(route_count);
    let mut mixed_paths = Vec::with_capacity(route_count * 2);

    for i in 0..route_count {
        let route_id = LiveId::from_str(&format!("route_{}", i));
        route_ids.push(route_id);

        match i % 4 {
            0 => {
                let pattern = format!("/s/{}", i);
                registry.register_pattern(&pattern, route_id).unwrap();
                static_paths.push(pattern.clone());
                mixed_paths.push(pattern);
            }
            1 => {
                let pattern = format!("/u/:id/r{}", i);
                registry.register_pattern(&pattern, route_id).unwrap();
                let concrete = format!("/u/{}/r{}", i.saturating_mul(3).saturating_add(7), i);
                dynamic_paths.push(concrete.clone());
                mixed_paths.push(concrete);
            }
            2 => {
                let pattern = format!("/w/{i}/*");
                registry.register_pattern(&pattern, route_id).unwrap();
                let concrete = format!("/w/{i}/leaf{}", i % 17);
                wildcard_paths.push(concrete.clone());
                mixed_paths.push(concrete);
            }
            _ => {
                let pattern = format!("/m/{i}/**");
                registry.register_pattern(&pattern, route_id).unwrap();
                let concrete = format!("/m/{i}/a/b/{}", i % 19);
                wildcard_paths.push(concrete.clone());
                mixed_paths.push(concrete);
            }
        }

        let miss = format!("/x/{}/nope", i);
        miss_paths.push(miss.clone());
        if i % 8 == 0 {
            mixed_paths.push(miss);
        }
    }

    if mixed_paths.is_empty() {
        mixed_paths.push("/".to_string());
    }

    PerfRouteTable {
        registry,
        route_ids,
        static_paths,
        dynamic_paths,
        wildcard_paths,
        miss_paths,
        mixed_paths,
    }
}

pub fn router_from_table(table: &PerfRouteTable) -> Router {
    let mut router = Router::new(Route::new(LiveId::from_str("perf_home")));
    router.route_registry = table.registry.clone();
    router
}

pub fn build_stack(route_ids: &[LiveId], depth: usize) -> Vec<Route> {
    let mut out = Vec::with_capacity(depth.max(1));
    if route_ids.is_empty() {
        out.push(Route::new(LiveId::from_str("empty")));
        return out;
    }
    for i in 0..depth.max(1) {
        out.push(Route::new(route_ids[i % route_ids.len()]));
    }
    out
}

pub fn run_lookup_samples(registry: &RouteRegistry, paths: &[String], iterations: usize) -> Vec<u128> {
    let mut out = Vec::with_capacity(iterations);
    if paths.is_empty() {
        return out;
    }
    for i in 0..iterations {
        let path = &paths[i % paths.len()];
        let start = Instant::now();
        let _ = registry.resolve_path(path);
        out.push(start.elapsed().as_nanos() as u128);
    }
    out
}

pub fn run_dispatch_go_to_path_samples(router: &Router, paths: &[String], iterations: usize) -> Vec<u128> {
    let mut out = Vec::with_capacity(iterations);
    if paths.is_empty() {
        return out;
    }
    let mut router = router.clone();
    for i in 0..iterations {
        let path = &paths[i % paths.len()];
        let start = Instant::now();
        let _ = router.navigate_by_path(path);
        out.push(start.elapsed().as_nanos() as u128);

        if router.depth() > 64 {
            router.clear_history();
        }
    }
    out
}

pub fn run_stack_ops_samples(route_ids: &[LiveId], iterations: usize) -> Vec<u128> {
    let mut out = Vec::with_capacity(iterations);
    let mut router = Router::new(Route::new(LiveId::from_str("stack_root")));

    let seed = build_stack(route_ids, 24);
    router.set_stack(seed.clone());

    for i in 0..iterations {
        if router.depth() <= 2 {
            router.set_stack(seed.clone());
        }
        let start = Instant::now();
        match i % 4 {
            0 => {
                let _ = router.pop();
            }
            1 => {
                if !route_ids.is_empty() {
                    let target = route_ids[i % route_ids.len()];
                    let _ = router.pop_to(target);
                }
            }
            2 => {
                let _ = router.pop_to_root();
            }
            _ => {
                router.set_stack(build_stack(route_ids, 12));
            }
        }
        out.push(start.elapsed().as_nanos() as u128);
    }

    out
}

pub fn summarize_us(scenario: &str, samples_ns: &[u128]) -> PerfStats {
    if samples_ns.is_empty() {
        return PerfStats {
            scenario: scenario.to_string(),
            iterations: 0,
            p50: 0.0,
            p95: 0.0,
            p99: 0.0,
            mean: 0.0,
        };
    }

    let mut sorted = samples_ns.to_vec();
    sorted.sort_unstable();

    let p50 = percentile_us(&sorted, 0.50);
    let p95 = percentile_us(&sorted, 0.95);
    let p99 = percentile_us(&sorted, 0.99);
    let mean = (samples_ns.iter().sum::<u128>() as f64 / samples_ns.len() as f64) / 1_000.0;

    PerfStats {
        scenario: scenario.to_string(),
        iterations: samples_ns.len(),
        p50,
        p95,
        p99,
        mean,
    }
}

pub fn stats_to_json(stats: &[PerfStats]) -> String {
    let body = stats
        .iter()
        .map(|s| {
            format!(
                "{{\"scenario\":\"{}\",\"iterations\":{},\"p50\":{:.3},\"p95\":{:.3},\"p99\":{:.3},\"mean\":{:.3}}}",
                s.scenario, s.iterations, s.p50, s.p95, s.p99, s.mean
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    format!("{{\"results\":[{}]}}", body)
}

fn percentile_us(sorted_samples_ns: &[u128], q: f64) -> f64 {
    let max_idx = sorted_samples_ns.len().saturating_sub(1);
    let idx = ((max_idx as f64) * q).round() as usize;
    sorted_samples_ns[idx.min(max_idx)] as f64 / 1_000.0
}
