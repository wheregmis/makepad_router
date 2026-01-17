# Makepad Router

A standalone routing package for Makepad applications, providing navigation, deep links, and page management for both widget-based and headless use.

## Features

- **LiveId routes + DSL integration** (define routes directly in `live_design!`)
- **Path patterns** with params and wildcards (`/user/:id`, `/admin/*`, `/docs/**`)
- **Navigation history** with back/forward semantics
- **Query + hash support** per history entry
- **URL sync (web)** with `use_initial_url` for deep linking
- **Nested routers** for sub-navigation
- **Transitions** + optional hero (shared element) transitions
- **Guards + before-leave hooks** (sync + async)
- **State persistence** via SerRon/DeRon
- **Debug inspector overlay** for dev diagnostics

## RouterWidget Toggles

These Live properties control optional subsystems. All default to behavior shown in the example app.

- `url_sync` (bool): sync route changes into browser URL/history on web builds.
- `use_initial_url` (bool): apply the initial browser URL on startup (web only).
- `persist_state` (bool): serialize/restore router state via `RouterState`.
- `hero_transition` (bool): enable shared-element transitions (requires `Hero` widgets).
- `debug_inspector` (bool): show a small overlay with route/stack/params.
- `push_transition`, `pop_transition`, `replace_transition`, `transition_duration`: configure route transitions.

## Quick Start

```rust
use makepad_widgets::*;
use makepad_router::{RouterWidgetWidgetRefExt, RouterAction};

live_design! {
    use makepad_router::widget::*;

    App = {{App}} {
        ui: <Window> {
            router = <RouterWidget> {
                width: Fill, height: Fill
                default_route: home
                not_found_route: not_found

                home = <HomePage> { route_pattern: "/" }
                settings = <SettingsPage> { route_pattern: "/settings/*" }
                detail = <DetailPage> { route_pattern: "/detail/:id" }
                not_found = <NotFoundPage> {}
            }
        }
    }
}

impl MatchEvent for App {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        let router = self.ui.router_widget(ids!(router));

        if self.ui.button(ids!(home_btn)).clicked(actions) {
            router.navigate(cx, live_id!(home));
        }

        if self.ui.button(ids!(back_btn)).clicked(actions) {
            router.back(cx);
        }

        for action in actions.filter_widget_actions(router.widget_uid()) {
            if let Some(RouterAction::RouteChanged { from, to }) = action.action.downcast_ref() {
                log!("Route changed: {:?} -> {:?}", from, to);
            }
        }
    }
}
```

## Path Parameters + Query

```rust
use makepad_router::prelude::*;

// Navigate by path (matches registered patterns)
router.navigate_by_path(cx, "/detail/42?tab=posts");

// Read params/query from current route
if let Some(route) = router.current_route() {
    if let Some(id) = route.get_param_string(live_id!(id)) {
        log!("id={}", id);
    }
    if let Some(tab) = route.query_get("tab") {
        log!("tab={}", tab);
    }
}
```

## Guards + Before-Leave Hooks

```rust
use makepad_router::{RouterGuardDecision, RouterRedirect, RouterRedirectTarget, RouterBeforeLeaveDecision};

router.add_route_guard(|_cx, nav| {
    if nav.to.as_ref().map(|r| r.id) == Some(live_id!(admin)) {
        return RouterGuardDecision::Redirect(RouterRedirect {
            target: RouterRedirectTarget::Route(live_id!(login)),
            replace: true,
        });
    }
    RouterGuardDecision::Allow
});

router.add_before_leave_hook(|_cx, nav| {
    if nav.from.as_ref().map(|r| r.id) == Some(live_id!(settings)) {
        return RouterBeforeLeaveDecision::Block;
    }
    RouterBeforeLeaveDecision::Allow
});
```

## Example App

Run the example app:

```
cargo run -p router_example
```

Routes in the example:
- `/` (home)
- `/settings/*` (nested router)
- `/detail/:id` (param)
- `not_found` (catch-all)

## Architecture

Key components in `src/`:

1. **route.rs** – Route + pattern parsing, params, and query helpers
2. **navigation.rs** – History stack and back/forward logic
3. **router.rs** – Core router state + pattern registry
4. **widget.rs** – RouterWidget implementation and DSL integration
5. **guards.rs** – Guard/before-leave types
6. **url.rs** – URL parsing + query helpers
7. **state.rs** – Serializable router state

## License

MIT OR Apache-2.0
