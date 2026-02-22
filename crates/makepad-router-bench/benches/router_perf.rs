use criterion::{black_box, criterion_group, criterion_main, Criterion};
use makepad_router_core::Router;
use makepad_router_perf::{build_mixed_route_table, router_from_table};
use std::collections::VecDeque;

fn route_lookup_exact_static_1k(c: &mut Criterion) {
    let table = build_mixed_route_table(1_000);
    let registry = table.registry;
    let paths = table.static_paths;

    c.bench_function("route_lookup_exact_static_1k", |b| {
        let mut i = 0usize;
        b.iter(|| {
            let path = &paths[i % paths.len()];
            i = i.wrapping_add(1);
            black_box(registry.resolve_path(black_box(path)));
        });
    });
}

fn route_lookup_dynamic_1k(c: &mut Criterion) {
    let table = build_mixed_route_table(1_000);
    let registry = table.registry;
    let paths = table.dynamic_paths;

    c.bench_function("route_lookup_dynamic_1k", |b| {
        let mut i = 0usize;
        b.iter(|| {
            let path = &paths[i % paths.len()];
            i = i.wrapping_add(1);
            black_box(registry.resolve_path(black_box(path)));
        });
    });
}

fn route_lookup_mixed_1k(c: &mut Criterion) {
    let table = build_mixed_route_table(1_000);
    let registry = table.registry;
    let paths = table.mixed_paths;

    c.bench_function("route_lookup_mixed_1k", |b| {
        let mut i = 0usize;
        b.iter(|| {
            let path = &paths[i % paths.len()];
            i = i.wrapping_add(1);
            black_box(registry.resolve_path(black_box(path)));
        });
    });
}

fn dispatch_go_to_path_1k(c: &mut Criterion) {
    let table = build_mixed_route_table(1_000);
    let mut router = router_from_table(&table);
    let paths = table.mixed_paths;

    c.bench_function("dispatch_go_to_path_1k", |b| {
        let mut i = 0usize;
        b.iter(|| {
            let path = &paths[i % paths.len()];
            i = i.wrapping_add(1);
            let _ = black_box(router.navigate_by_path(black_box(path)));
            if router.depth() > 64 {
                router.clear_history();
            }
        });
    });
}

fn dispatch_go_to_path_nested_1k(c: &mut Criterion) {
    let table = build_mixed_route_table(1_000);
    let mut router = router_from_table(&table);
    let paths = table.wildcard_paths;

    c.bench_function("dispatch_go_to_path_nested_1k", |b| {
        let mut i = 0usize;
        b.iter(|| {
            let path = &paths[i % paths.len()];
            i = i.wrapping_add(1);
            let _ = black_box(router.navigate_by_path(black_box(path)));
            if router.depth() > 64 {
                router.clear_history();
            }
        });
    });
}

fn dispatch_go_to_route_1k(c: &mut Criterion) {
    let table = build_mixed_route_table(1_000);
    let mut router = Router::new(makepad_router_core::Route::new(makepad_live_id::LiveId::from_str(
        "home_bench",
    )));
    let route_ids = table.route_ids;

    c.bench_function("dispatch_go_to_route_1k", |b| {
        let mut i = 0usize;
        b.iter(|| {
            let route_id = route_ids[i % route_ids.len()];
            i = i.wrapping_add(1);
            router.navigate_to(black_box(route_id));
            if router.depth() > 64 {
                router.clear_history();
            }
        });
    });
}

fn dispatch_stack_ops(c: &mut Criterion) {
    let table = build_mixed_route_table(1_000);
    let route_ids = table.route_ids;
    let mut router = Router::new(makepad_router_core::Route::new(makepad_live_id::LiveId::from_str(
        "stack_root_bench",
    )));
    router.set_stack(
        route_ids
            .iter()
            .take(24)
            .map(|id| makepad_router_core::Route::new(*id))
            .collect(),
    );

    c.bench_function("dispatch_stack_ops", |b| {
        let mut i = 0usize;
        b.iter(|| {
            if router.depth() <= 2 {
                router.set_stack(
                    route_ids
                        .iter()
                        .take(24)
                        .map(|id| makepad_router_core::Route::new(*id))
                        .collect(),
                );
            }
            match i % 4 {
                0 => {
                    let _ = router.pop();
                }
                1 => {
                    let _ = router.pop_to(route_ids[i % route_ids.len()]);
                }
                2 => {
                    let _ = router.pop_to_root();
                }
                _ => {
                    router.set_stack(
                        route_ids
                            .iter()
                            .skip(i % 64)
                            .take(12)
                            .map(|id| makepad_router_core::Route::new(*id))
                            .collect(),
                    );
                }
            }
            i = i.wrapping_add(1);
        });
    });
}

fn dispatch_with_sync_guards(c: &mut Criterion) {
    let table = build_mixed_route_table(1_000);
    let mut router = router_from_table(&table);
    let paths = table.dynamic_paths;
    let redirect_path = "/s/0";

    c.bench_function("dispatch_with_sync_guards", |b| {
        let mut i = 0usize;
        b.iter(|| {
            let path = &paths[i % paths.len()];
            let allow = (i % 5) != 0;
            if allow {
                let _ = router.navigate_by_path(path);
            } else {
                let _ = router.navigate_by_path(redirect_path);
            }
            if router.depth() > 64 {
                router.clear_history();
            }
            i = i.wrapping_add(1);
        });
    });
}

fn dispatch_with_async_guard_pending(c: &mut Criterion) {
    let table = build_mixed_route_table(1_000);
    let mut router = router_from_table(&table);
    let paths = table.mixed_paths;

    c.bench_function("dispatch_with_async_guard_pending", |b| {
        let mut i = 0usize;
        let mut pending: VecDeque<&str> = VecDeque::with_capacity(16);

        b.iter(|| {
            let path = paths[i % paths.len()].as_str();
            i = i.wrapping_add(1);

            pending.push_back(path);
            if pending.len() >= 2 {
                if let Some(ready) = pending.pop_front() {
                    let _ = router.navigate_by_path(ready);
                }
            }
            if router.depth() > 64 {
                router.clear_history();
            }
        });
    });
}

criterion_group!(
    router_perf,
    route_lookup_exact_static_1k,
    route_lookup_dynamic_1k,
    route_lookup_mixed_1k,
    dispatch_go_to_path_1k,
    dispatch_go_to_path_nested_1k,
    dispatch_go_to_route_1k,
    dispatch_stack_ops,
    dispatch_with_sync_guards,
    dispatch_with_async_guard_pending,
);
criterion_main!(router_perf);
