## Router Roadmap

### Phase 1: API Completeness + Semantics
- [x] Add `replace`, `forward`, `can_go_forward`, `reset`, `clear_history`, `depth` to `RouterWidget` / `RouterWidgetRef`.
- [x] Add stack semantics: `push(route_id)`, `pop()`, `pop_to(route_id)`, `pop_to_root()`, `set_stack(Vec<Route>)`.
- [x] Emit `RouterAction` into `Actions` on route changes (no callbacks required).

### Phase 2: Nested Routing (Real)
- [x] Delegate remainder paths to child routers (e.g. `/admin/*` passes the tail into `admin_router`) and support optional “base path” composition.
- [x] Add route “not found” handling at each router level (configurable fallback route id).

### Phase 3: Transitions / Animations / View Transitions (which opens the door for hero like element in flutter)
- [x] Implement animated transitions inside `RouterWidget` (keep old+new alive during transition, then drop old).
- [x] Built-in presets: `None`, `Fade`, `SlideLeft/Right`, `Scale`, `SharedAxis` with push/pop direction.
- [x] Per-route and per-navigation overrides (e.g. push slides, replace fades).
- [x] Shared element (“hero”) transitions via `<Hero tag: ...>` (opt-in with `hero_transition: true`).

### Phase 4: URL + Deep Linking (Web + Desktop optional)
- [x] Parse + generate paths (including query + hash), synchronize with web history, and allow initial route from URL.
- [x] Add `navigate_by_url(url)` and `current_url()` helpers.

### Phase 5: Guards, Redirects, Middleware
- [x] Route guards (sync + async) for auth/feature flags; redirect/replace behaviors.
- [x] “Before leave” confirmation hooks.

### Phase 6: State + Data
- [x] Typed params/query support (keep current `LiveId` path, add string map for query).
- [x] Persistence of history stack + current route (extend to params/query).

### Phase 7: Tooling + Testing
- [x] Add unit tests for transition state machine and per-route override parsing.
- [x] Add a small “router inspector” debug overlay for current route/stack/params.
