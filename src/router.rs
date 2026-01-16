use crate::{
    navigation::NavigationHistory,
    route::{Route, RoutePattern, RouteSegment},
};
use makepad_live_id::*;
use makepad_micro_serde::*;
use std::collections::HashMap;

/// Route registry entry
#[derive(Clone, Debug, SerBin, DeBin, SerRon, DeRon)]
struct RouteEntry {
    route_id: LiveId,
    pattern: Option<RoutePattern>,
    priority: usize,
}

/// Registry for pattern-based routes
#[derive(Clone, Debug, Default)]
pub struct RouteRegistry {
    /// Routes by LiveId (for exact matches)
    by_id: HashMap<LiveId, RouteEntry>,
    /// Routes by pattern (for path-based matching)
    by_pattern: Vec<RouteEntry>,
    /// Exact lookup for fully-static patterns (normalized path -> route_id).
    exact_static: HashMap<String, LiveId>,
    /// Candidate pattern indices keyed by first static segment.
    by_first_segment: HashMap<String, Vec<usize>>,
    /// Candidate pattern indices for patterns without a static first segment.
    fallback_first_segment: Vec<usize>,
}

impl RouteRegistry {
    /// Create an empty registry.
    pub fn new() -> Self {
        Self {
            by_id: HashMap::new(),
            by_pattern: Vec::new(),
            exact_static: HashMap::new(),
            by_first_segment: HashMap::new(),
            fallback_first_segment: Vec::new(),
        }
    }

    /// Register a route by LiveId
    pub fn register_by_id(&mut self, route_id: LiveId) {
        let entry = RouteEntry {
            route_id,
            pattern: None,
            priority: 0, // Highest priority
        };
        self.by_id.insert(route_id, entry);
    }

    /// Register a route pattern
    pub fn register_pattern(&mut self, pattern: &str, route_id: LiveId) -> Result<(), String> {
        let route_pattern = RoutePattern::parse(pattern)?;
        let priority = route_pattern.priority();
        let entry = RouteEntry {
            route_id,
            pattern: Some(route_pattern),
            priority,
        };

        self.by_id.insert(route_id, entry.clone());
        
        // Insert in sorted order by priority (lower priority value = higher priority)
        // Find insertion point
        let pos = self.by_pattern.iter()
            .position(|e| e.priority > priority)
            .unwrap_or(self.by_pattern.len());
        self.by_pattern.insert(pos, entry);
        self.rebuild_indices();
        Ok(())
    }

    /// Resolve a path to a route (exact static match first, then pattern match).
    pub fn resolve_path(&self, path: &str) -> Option<Route> {
        let normalized = Self::normalize_path(path);

        if let Some(route_id) = self.exact_static.get(&normalized).copied() {
            let pattern = self.by_id.get(&route_id).and_then(|e| e.pattern.clone());
            return Some(Route {
                id: route_id,
                params: Default::default(),
                query: Default::default(),
                hash: String::new(),
                pattern,
            });
        }

        let first = normalized
            .trim_start_matches('/')
            .split('/')
            .next()
            .unwrap_or("")
            .to_string();

        if let Some(candidates) = self.by_first_segment.get(&first) {
            for &idx in candidates {
                let entry = self.by_pattern.get(idx)?;
                let Some(ref pattern) = entry.pattern else { continue };
                if let Some(params) = pattern.matches(&normalized) {
                    return Some(Route {
                        id: entry.route_id,
                        params,
                        query: Default::default(),
                        hash: String::new(),
                        pattern: Some(pattern.clone()),
                    });
                }
            }
        }

        for &idx in &self.fallback_first_segment {
            let entry = self.by_pattern.get(idx)?;
            let Some(ref pattern) = entry.pattern else { continue };
            if let Some(params) = pattern.matches(&normalized) {
                return Some(Route {
                    id: entry.route_id,
                    params,
                    query: Default::default(),
                    hash: String::new(),
                    pattern: Some(pattern.clone()),
                });
            }
        }

        None
    }

    /// Check if a route ID is registered
    pub fn has_route(&self, route_id: LiveId) -> bool {
        self.by_id.contains_key(&route_id)
    }

    /// Get pattern for a route ID (if registered with a pattern).
    pub fn get_pattern(&self, route_id: LiveId) -> Option<&RoutePattern> {
        self.by_id.get(&route_id).and_then(|e| e.pattern.as_ref())
    }

    fn normalize_path(path: &str) -> String {
        let mut p = path.trim().to_string();
        if p.is_empty() {
            return "/".to_string();
        }
        // Accept full URLs too (same behavior as RouterUrl::parse).
        if let Some((_, after_scheme)) = p.split_once("://") {
            let mut rest = after_scheme;
            if let Some((_, after_host_slash)) = rest.split_once('/') {
                rest = after_host_slash;
                p = format!("/{}", rest);
            } else {
                p = "/".to_string();
            }
        }
        if !p.starts_with('/') {
            p.insert(0, '/');
        }
        // Strip query/hash for matching.
        if let Some((before_hash, _)) = p.split_once('#') {
            p = before_hash.to_string();
        }
        if let Some((before_q, _)) = p.split_once('?') {
            p = before_q.to_string();
        }
        // Collapse trailing slashes.
        while p.len() > 1 && p.ends_with('/') {
            p.pop();
        }
        p
    }

    fn rebuild_indices(&mut self) {
        self.exact_static.clear();
        self.by_first_segment.clear();
        self.fallback_first_segment.clear();

        for (idx, entry) in self.by_pattern.iter().enumerate() {
            let Some(pattern) = entry.pattern.as_ref() else {
                continue;
            };

            // Build exact static lookup.
            if pattern
                .segments
                .iter()
                .all(|s| matches!(s, RouteSegment::Static(_)))
            {
                let path = format!(
                    "/{}",
                    pattern
                        .segments
                        .iter()
                        .filter_map(|s| match s {
                            RouteSegment::Static(v) => Some(v.as_str()),
                            _ => None,
                        })
                        .collect::<Vec<_>>()
                        .join("/")
                );
                self.exact_static.entry(path).or_insert(entry.route_id);
            }

            // Index by first segment (if static).
            match pattern.segments.first() {
                Some(RouteSegment::Static(first)) => {
                    self.by_first_segment
                        .entry(first.clone())
                        .or_default()
                        .push(idx);
                }
                _ => self.fallback_first_segment.push(idx),
            }
        }
    }
}

impl SerBin for RouteRegistry {
    fn ser_bin(&self, s: &mut Vec<u8>) {
        self.by_id.ser_bin(s);
        self.by_pattern.ser_bin(s);
    }
}

impl DeBin for RouteRegistry {
    fn de_bin(o: &mut usize, d: &[u8]) -> Result<Self, DeBinErr> {
        let by_id = <HashMap<LiveId, RouteEntry>>::de_bin(o, d)?;
        let by_pattern = <Vec<RouteEntry>>::de_bin(o, d)?;
        let mut out = Self {
            by_id,
            by_pattern,
            exact_static: HashMap::new(),
            by_first_segment: HashMap::new(),
            fallback_first_segment: Vec::new(),
        };
        out.rebuild_indices();
        Ok(out)
    }
}

impl SerRon for RouteRegistry {
    fn ser_ron(&self, d: usize, s: &mut SerRonState) {
        s.st_pre();
        s.field(d + 1, "by_id");
        self.by_id.ser_ron(d + 1, s);
        s.conl();
        s.field(d + 1, "by_pattern");
        self.by_pattern.ser_ron(d + 1, s);
        s.out.push('\n');
        s.st_post(d);
    }
}

impl DeRon for RouteRegistry {
    fn de_ron(s: &mut DeRonState, i: &mut std::str::Chars) -> Result<Self, DeRonErr> {
        s.paren_open(i)?;
        let mut by_id: Option<HashMap<LiveId, RouteEntry>> = None;
        let mut by_pattern: Option<Vec<RouteEntry>> = None;
        loop {
            match s.tok {
                DeRonTok::ParenClose => {
                    s.paren_close(i)?;
                    break;
                }
                DeRonTok::Ident => {
                    let key = s.identbuf.clone();
                    s.ident(i)?;
                    s.colon(i)?;
                    match key.as_str() {
                        "by_id" => by_id = Some(HashMap::<LiveId, RouteEntry>::de_ron(s, i)?),
                        "by_pattern" => by_pattern = Some(Vec::<RouteEntry>::de_ron(s, i)?),
                        _ => return Err(s.err_token("by_id or by_pattern")),
                    }
                    s.eat_comma_paren(i)?;
                }
                _ => return Err(s.err_token("Identifier or )")),
            }
        }
        let mut out = Self {
            by_id: by_id.unwrap_or_default(),
            by_pattern: by_pattern.unwrap_or_default(),
            exact_static: HashMap::new(),
            by_first_segment: HashMap::new(),
            fallback_first_segment: Vec::new(),
        };
        out.rebuild_indices();
        Ok(out)
    }
}

/// Router configuration and state
#[derive(Clone, Debug, SerBin, DeBin, SerRon, DeRon)]
pub struct Router {
    /// Navigation history
    pub history: NavigationHistory,
    /// Whether to persist router state
    pub persist_state: bool,
    /// Registry for pattern-based navigation (non-visual, usable headless).
    pub route_registry: RouteRegistry,
}

impl Default for Router {
    fn default() -> Self {
        Self {
            history: NavigationHistory::empty(),
            persist_state: false,
            route_registry: RouteRegistry::default(),
        }
    }
}

impl Router {
    /// Create a new router with an initial route
    pub fn new(initial_route: Route) -> Self {
        Self {
            history: NavigationHistory::new(initial_route),
            persist_state: false,
            route_registry: RouteRegistry::default(),
        }
    }

    /// Create a router with state persistence enabled
    pub fn with_persistence(initial_route: Route) -> Self {
        Self {
            history: NavigationHistory::new(initial_route),
            persist_state: true,
            route_registry: RouteRegistry::default(),
        }
    }

    /// Navigate to a new route
    pub fn navigate(&mut self, route: Route) {
        self.history.push(route);
    }

    /// Navigate to a route by ID
    pub fn navigate_to(&mut self, route_id: LiveId) {
        self.navigate(Route::new(route_id));
    }

    /// Replace the current route
    pub fn replace(&mut self, route: Route) {
        self.history.replace(route);
    }

    /// Replace the current route by ID
    pub fn replace_with(&mut self, route_id: LiveId) {
        self.replace(Route::new(route_id));
    }

    /// Go back in history
    pub fn back(&mut self) -> bool {
        self.history.back()
    }

    /// Go forward in history
    pub fn forward(&mut self) -> bool {
        self.history.forward()
    }

    /// Get the current route
    pub fn current_route(&self) -> Option<&Route> {
        self.history.current()
    }

    /// Get the current route ID
    pub fn current_route_id(&self) -> Option<LiveId> {
        self.current_route().map(|r| r.id)
    }

    /// Check if we can navigate back
    pub fn can_go_back(&self) -> bool {
        self.history.can_go_back()
    }

    /// Check if we can navigate forward
    pub fn can_go_forward(&self) -> bool {
        self.history.can_go_forward()
    }

    /// Reset to a specific route, clearing all history
    pub fn reset(&mut self, route: Route) {
        self.history.reset(route);
    }

    /// Clear all history except current route
    pub fn clear_history(&mut self) {
        self.history.clear();
    }

    /// Get the depth of the navigation history
    pub fn depth(&self) -> usize {
        self.history.depth()
    }

    /// Register a pattern-based route for `navigate_by_path`.
    pub fn register_route_pattern(&mut self, pattern: &str, route_id: LiveId) -> Result<(), String> {
        self.route_registry.register_pattern(pattern, route_id)
    }

    /// Navigate using a path string, using the registered route patterns.
    /// Returns the resolved `Route` on success.
    pub fn navigate_by_path(&mut self, path: &str) -> Result<Route, String> {
        let route = self
            .route_registry
            .resolve_path(path)
            .ok_or_else(|| format!("No route found for path: {}", path))?;
        self.navigate(route.clone());
        Ok(route)
    }

    /// Push a route onto the stack (alias of `navigate`).
    pub fn push(&mut self, route: Route) {
        self.navigate(route);
    }

    /// Pop the current route (stack-style semantics).
    pub fn pop(&mut self) -> bool {
        self.history.pop()
    }

    /// Pop to the given route id (stack-style semantics).
    pub fn pop_to(&mut self, route_id: LiveId) -> bool {
        self.history.pop_to(route_id)
    }

    /// Pop to the root route (stack-style semantics).
    pub fn pop_to_root(&mut self) -> bool {
        self.history.pop_to_root()
    }

    /// Set the entire stack (stack-style semantics).
    pub fn set_stack(&mut self, stack: Vec<Route>) {
        self.history.set_stack(stack);
    }

}

/// Router actions for event handling
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RouterAction {
    /// Navigate to a route
    Navigate(Route),
    /// Replace the current route
    Replace(Route),
    /// Go back in history
    Back,
    /// Go forward in history
    Forward,
    /// Reset to a route
    Reset(Route),
    /// Route changed notification
    RouteChanged { from: Option<LiveId>, to: LiveId },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_router_navigate() {
        let mut router = Router::new(Route::new(live_id!(home)));
        assert_eq!(router.current_route_id(), Some(live_id!(home)));

        router.navigate_to(live_id!(settings));
        assert_eq!(router.current_route_id(), Some(live_id!(settings)));
        assert_eq!(router.depth(), 2);
    }

    #[test]
    fn test_router_back() {
        let mut router = Router::new(Route::new(live_id!(home)));
        router.navigate_to(live_id!(settings));
        router.navigate_to(live_id!(profile));

        assert!(router.can_go_back());
        assert!(router.back());
        assert_eq!(router.current_route_id(), Some(live_id!(settings)));

        assert!(router.back());
        assert_eq!(router.current_route_id(), Some(live_id!(home)));
        assert!(!router.can_go_back());
    }

    #[test]
    fn test_router_replace() {
        let mut router = Router::new(Route::new(live_id!(home)));
        router.replace_with(live_id!(settings));

        assert_eq!(router.current_route_id(), Some(live_id!(settings)));
        assert_eq!(router.depth(), 1);
        assert!(!router.can_go_back());
    }

    #[test]
    fn test_route_registry_register_pattern() {
        let mut registry = RouteRegistry::new();
        registry.register_pattern("/user/:id", live_id!(user_profile)).unwrap();
        assert!(registry.has_route(live_id!(user_profile)));
    }

    #[test]
    fn test_route_registry_resolve_path() {
        let mut registry = RouteRegistry::new();
        registry.register_pattern("/user/:id", live_id!(user_profile)).unwrap();
        
        let route = registry.resolve_path("/user/123").unwrap();
        assert_eq!(route.id, live_id!(user_profile));
        assert_eq!(route.get_param(LiveId::from_str("id")), Some(LiveId::from_str("123")));
    }

    #[test]
    fn test_route_registry_priority() {
        let mut registry = RouteRegistry::new();
        // Register in reverse priority order
        registry.register_pattern("/user/**", live_id!(user_wildcard)).unwrap();
        registry.register_pattern("/user/*", live_id!(user_single)).unwrap();
        registry.register_pattern("/user/:id", live_id!(user_dynamic)).unwrap();
        registry.register_pattern("/user/profile", live_id!(user_static)).unwrap();

        // Most specific should match first
        let route = registry.resolve_path("/user/profile").unwrap();
        assert_eq!(route.id, live_id!(user_static));

        let route = registry.resolve_path("/user/123").unwrap();
        assert_eq!(route.id, live_id!(user_dynamic));

        let route = registry.resolve_path("/user/other").unwrap();
        assert_eq!(route.id, live_id!(user_dynamic));

        let route = registry.resolve_path("/user/123/posts").unwrap();
        assert_eq!(route.id, live_id!(user_wildcard));
    }

    #[test]
    fn test_router_navigate_by_path() {
        let mut router = Router::new(Route::new(live_id!(home)));
        router.register_route_pattern("/user/:id", live_id!(user_profile)).unwrap();
        
        router.navigate_by_path("/user/123").unwrap();
        let route = router.current_route().unwrap();
        assert_eq!(route.id, live_id!(user_profile));
        assert_eq!(route.get_param(LiveId::from_str("id")), Some(LiveId::from_str("123")));
    }
}
