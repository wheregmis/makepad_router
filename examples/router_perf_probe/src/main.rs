use makepad_router_perf::{
    build_mixed_route_table, router_from_table, run_dispatch_go_to_path_samples,
    run_lookup_samples, run_stack_ops_samples, stats_to_json, summarize_us,
};

fn main() {
    let iterations = std::env::var("ROUTER_PERF_ITERATIONS")
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(12_000);

    let mut all = Vec::new();

    for scale in [100usize, 500usize, 1_000usize] {
        let table = build_mixed_route_table(scale);
        let router = router_from_table(&table);

        all.push(summarize_us(
            &format!("lookup_exact_static_{}", scale),
            &run_lookup_samples(&table.registry, &table.static_paths, iterations),
        ));
        all.push(summarize_us(
            &format!("lookup_dynamic_{}", scale),
            &run_lookup_samples(&table.registry, &table.dynamic_paths, iterations),
        ));
        all.push(summarize_us(
            &format!("lookup_mixed_{}", scale),
            &run_lookup_samples(&table.registry, &table.mixed_paths, iterations),
        ));
        all.push(summarize_us(
            &format!("dispatch_go_to_path_{}", scale),
            &run_dispatch_go_to_path_samples(&router, &table.mixed_paths, iterations),
        ));
        all.push(summarize_us(
            &format!("dispatch_go_to_path_nested_{}", scale),
            &run_dispatch_go_to_path_samples(&router, &table.wildcard_paths, iterations),
        ));
        all.push(summarize_us(
            &format!("dispatch_stack_ops_{}", scale),
            &run_stack_ops_samples(&table.route_ids, iterations / 2),
        ));
    }

    println!("{}", stats_to_json(&all));
}
