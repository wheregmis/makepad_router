# Makepad Router

A standalone routing package for Makepad applications, providing navigation and page management capabilities.

## Features

- **LiveId-Based Routes**: Routes are identified using Makepad's `LiveId` system
- **Navigation History**: Full browser-like back/forward navigation support
- **Route Parameters**: Support for parameterized routes
- **Query + Hash**: Optional query-string map and hash fragment stored per history entry
- **State Persistence**: Optional state persistence using SerRon/DeRon
- **Integration with Makepad Widgets**: Seamless integration with Makepad's widget system
- **Declarative Route Definition**: Define routes directly in Makepad's DSL

## Architecture

The router package consists of several key components:

### Core Components (`libs/router/src/`)

1. **route.rs** - Route and RouteParams structures
   - Define individual routes with optional parameters
   - Macro support for easy route creation

2. **navigation.rs** - NavigationHistory management
   - Stack-based navigation history
   - Back/forward navigation support
   - History depth tracking

3. **router.rs** - Router state management
   - Centralized router logic
   - Navigation actions (Navigate, Replace, Back, Forward)
   - Route change notifications

4. **widget.rs** - RouterWidget implementation
   - Visual widget for rendering routes
   - Event handling for active routes
   - LiveHook integration for DSL support

## Usage

### Basic Example

```rust
use makepad_widgets::*;

live_design! {
    use makepad_router::widget::*;
    
    App = {{App}} {
        ui: <Window> {
            router = <RouterWidget> {
                width: Fill, height: Fill
                default_route: home
                
                home = <HomePage> {}
                settings = <SettingsPage> {}
                profile = <ProfilePage> {}
            }
        }
    }
}

impl MatchEvent for App {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        if self.ui.button(id!(home_btn)).clicked(&actions) {
            self.ui.router_widget(id!(router)).navigate(cx, live_id!(home));
        }
        
        if self.ui.button(id!(back_btn)).clicked(&actions) {
            self.ui.router_widget(id!(router)).back(cx);
        }
    }
}
```

### Route with Parameters

```rust
use makepad_router::prelude::*;

// Create a route with parameters
let profile_route = route!(profile, user_id = john_doe);

// Navigate to the route
router.navigate_with_route(cx, profile_route);

// Access parameters
if let Some(route) = router.current_route() {
    if let Some(user_id) = route.get_param(live_id!(user_id)) {
        // Use the user_id parameter
    }
}
```

### Query + State

```rust
use makepad_router::prelude::*;

// Navigate with query string (works via navigate_by_path or navigate_by_url)
router.navigate_by_path(cx, "/user/123?tab=posts");

// Read query
if let Some(route) = router.current_route() {
    if let Some(tab) = route.query_get("tab") {
        // ...
    }
}

// Persist/restore history stack + current route
let state = router.get_state();
let ron = state.serialize_ron();
let restored = RouterState::deserialize_ron(&ron).unwrap();
router.set_state(cx, restored);
```

### Navigation Methods

```rust
// Navigate to a new route (adds to history)
router.navigate(cx, live_id!(settings));

// Replace current route (doesn't add to history)
router.replace(cx, live_id!(login));

// Go back in history
router.back(cx);

// Go forward in history
router.forward(cx);

// Check navigation availability
if router.can_go_back() {
    // Show back button
}
```

## Design Philosophy

The router is designed to be:

1. **Standalone**: Can be used independently of other Makepad components
2. **Declarative**: Routes defined in Makepad's DSL alongside UI
3. **Type-safe**: Uses LiveId for compile-time route checking
4. **Lightweight**: Minimal overhead, only active routes are rendered
5. **Familiar**: Navigation patterns similar to web routers

## Comparison with Existing Patterns

### vs. PageFlip
- **Router**: Full navigation history, route parameters, centralized routing
- **PageFlip**: Simple page switcher, no history, manual state management

### vs. StackNavigation
- **Router**: Any navigation pattern, declarative routes, LiveId-based
- **StackNavigation**: Mobile push/pop only, imperative, animation-focused

### vs. Dock/Tabs
- **Router**: Application-level routing, shareable routes
- **Dock/Tabs**: IDE-style layout, local state only

## Future Enhancements

- URL synchronization for web targets
- Route guards and middleware
- Nested routers
- Route transitions/animations
- Query parameter support
- Deep linking
- Route aliases
- Debug router inspector overlay (`debug_inspector: true`)

## License

MIT OR Apache-2.0
