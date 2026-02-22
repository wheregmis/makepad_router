use makepad_router_perf::{
    build_mixed_route_table, router_from_table, run_dispatch_go_to_path_samples,
    run_lookup_samples, run_stack_ops_samples, stats_to_json, summarize_us,
};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let args = Args::parse();

    let table = build_mixed_route_table(1_000);
    let router = router_from_table(&table);

    let mut stats = Vec::new();
    stats.push(summarize_us(
        "route_lookup_exact_static_1k",
        &run_lookup_samples(&table.registry, &table.static_paths, args.iterations),
    ));
    stats.push(summarize_us(
        "route_lookup_dynamic_1k",
        &run_lookup_samples(&table.registry, &table.dynamic_paths, args.iterations),
    ));
    stats.push(summarize_us(
        "route_lookup_mixed_1k",
        &run_lookup_samples(&table.registry, &table.mixed_paths, args.iterations),
    ));
    stats.push(summarize_us(
        "dispatch_go_to_path_1k",
        &run_dispatch_go_to_path_samples(&router, &table.mixed_paths, args.iterations),
    ));
    stats.push(summarize_us(
        "dispatch_go_to_path_nested_1k",
        &run_dispatch_go_to_path_samples(&router, &table.wildcard_paths, args.iterations),
    ));
    stats.push(summarize_us(
        "dispatch_stack_ops",
        &run_stack_ops_samples(&table.route_ids, args.iterations / 2),
    ));

    let json = stats_to_json(&stats);

    if let Some(path) = &args.output {
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        fs::write(path, &json).expect("failed to write output file");
    }

    println!("{}", json);

    if let Some(baseline_path) = &args.baseline {
        let baseline_json = fs::read_to_string(baseline_path)
            .unwrap_or_else(|_| panic!("failed to read baseline file: {}", baseline_path.display()));
        let baseline = parse_p95_map(&baseline_json);

        let mut failures = Vec::new();
        for s in &stats {
            let Some(base_p95) = baseline.get(&s.scenario) else {
                continue;
            };
            let allowed = *base_p95 * (1.0 + args.threshold);
            if s.p95 > allowed {
                failures.push(format!(
                    "{}: current p95 {:.3}us > allowed {:.3}us (baseline {:.3}us, threshold {:.1}%)",
                    s.scenario,
                    s.p95,
                    allowed,
                    base_p95,
                    args.threshold * 100.0
                ));
            }
        }

        if !failures.is_empty() {
            eprintln!("Performance regression(s) detected:");
            for f in failures {
                eprintln!("  - {}", f);
            }
            std::process::exit(1);
        }
    }
}

#[derive(Debug)]
struct Args {
    output: Option<PathBuf>,
    baseline: Option<PathBuf>,
    threshold: f64,
    iterations: usize,
}

impl Args {
    fn parse() -> Self {
        let mut output = None;
        let mut baseline = None;
        let mut threshold = 0.12;
        let mut iterations = 20_000usize;

        let mut it = env::args().skip(1);
        while let Some(arg) = it.next() {
            match arg.as_str() {
                "--output" => {
                    output = it.next().map(PathBuf::from);
                }
                "--baseline" => {
                    baseline = it.next().map(PathBuf::from);
                }
                "--threshold" => {
                    if let Some(v) = it.next() {
                        threshold = v.parse::<f64>().unwrap_or(threshold);
                    }
                }
                "--iterations" => {
                    if let Some(v) = it.next() {
                        iterations = v.parse::<usize>().unwrap_or(iterations);
                    }
                }
                _ => {}
            }
        }

        Self {
            output,
            baseline,
            threshold,
            iterations,
        }
    }
}

fn parse_p95_map(json: &str) -> HashMap<String, f64> {
    let mut out = HashMap::new();
    for chunk in json.split("{\"scenario\":\"").skip(1) {
        let Some((scenario, rest)) = chunk.split_once('"') else {
            continue;
        };
        let Some((_, after)) = rest.split_once("\"p95\":") else {
            continue;
        };
        let num = after
            .split([',', '}'])
            .next()
            .unwrap_or_default()
            .trim();
        if let Ok(p95) = num.parse::<f64>() {
            out.insert(scenario.to_string(), p95);
        }
    }
    out
}
