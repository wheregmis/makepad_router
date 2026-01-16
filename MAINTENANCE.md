# Router Maintenance Plan

This package is split into two conceptual layers:

- **Core (pure logic):** `route.rs`, `url.rs`, `navigation.rs`, `router.rs`, `state.rs`
- **UI integration:** `widget.rs` (+ submodules), `guards.rs`, `hero.rs`

The goal of this document is to keep future changes incremental and safe, with tests that catch regressions early.

## Refactor Goals

- Keep pure logic isolated (testable without a `Cx`/GPU).
- Keep `RouterWidget` readable by pushing feature-specific code into submodules.

## Testing Strategy

### Unit Tests (fast)

- Keep unit tests close to the logic they test (e.g. `route.rs` pattern parsing/matching).
- Prefer deterministic “table-style” cases over UI-driven tests.

Run:

- `cargo test --manifest-path libs/router/Cargo.toml`

### Integration/Regression Tests (public API)

- Use `libs/router/tests/*` to exercise the public surface and persistence formats.
- Keep at least one roundtrip test for each serialization format we rely on.

Currently covered:

- URL parsing/query encoding (`libs/router/tests/url_regression.rs`)
- State persistence roundtrip (`libs/router/tests/state_persistence_regression.rs`)

## Suggested Next Refactors (in order)

1. **Split `widget.rs` further**
   - `widget/inspector.rs` is already extracted.
   - `widget/transitions.rs` and `widget/url_sync.rs` are already extracted.
   - `widget/nested.rs` is already extracted.
   - `widget/guard_flow.rs` is already extracted.
   - `widget/hero.rs` is already extracted (hero transition state/pairing data).
   - `widget/hero_render.rs` is already extracted.
   - `widget/api.rs` is already extracted (public `RouterWidget` methods).
   - `widget/path_nav.rs` is already extracted (path resolution + not-found handling).
   - `widget/route_widgets.rs` is already extracted (route widget construction + metadata silencing).
   - `widget/live_apply.rs` is already extracted (LiveHook apply plumbing + route template scanning).
   - `widget/actions.rs` is already extracted (action queue + emission plumbing).
   - `widget/fields.rs` is already extracted (group RouterWidget state into sub-structs).
   - `widget/persistence.rs` is already extracted (widget-level state save/restore helpers).
   - `widget/callbacks.rs` is already extracted (route-change callback registration + dispatch).
   - `widget/guards.rs` is already extracted (guard + before-leave registration + storage).
   - Next candidates:
      - `widget/actions.rs` (RouterWidgetRef convenience helpers + action routing)
      
2. **Reduce cross-module coupling**
   - Keep `Router` and `RouteRegistry` free of `makepad_widgets` types.
   - If UI-only types are needed, contain them under `widget` submodules.

3. **Expand regression coverage for router semantics**
   - Nested prefix matching edge cases (dynamic + wildcards).
   - URL sync: path/query/hash roundtrip through `RouterUrl`.
   - History semantics: push/replace/back/forward/stack APIs.


## Performance Notes

- `RouterWidget` now lazily instantiates only the active route widget and only the active route receives events.

## Performance TODOs

- [x] Replace params/query storage with `HashMap` for O(1) lookups.
- [x] Optimize `NavigationHistory::pop_to` with a reverse index.
- [x] Lazy route widget instantiation (only active route).
- [x] Active-route-only event handling + active-widget fast paths.
- [x] Optimize `RouteRegistry::resolve_path` with indices (exact static + first-segment buckets).
- [x] URL parsing cache (small LRU or last-value cache).
- [x] Nested routing cache for repeated prefix matches.
