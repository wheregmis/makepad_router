use crate::pattern::{RouteParams, RoutePatternRef};
use crate::route::Route;
use makepad_widgets::*;
// Nested router discovery and child router registration.

use super::{RouterWidget, RouterWidgetWidgetRefExt};

impl RouterWidget {
    pub(super) fn resolve_nested_prefix(
        &mut self,
        path: &str,
    ) -> Option<(LiveId, RouteParams, RoutePatternRef, String)> {
        if self.caches.nested_prefix_cache_epoch == self.caches.route_registry_epoch
            && self.caches.nested_prefix_cache_path == path
        {
            return self.caches.nested_prefix_cache_result.clone();
        }

        let mut best: Option<(LiveId, RouteParams, RoutePatternRef, String, usize)> = None;

        // Support lazy route widget instantiation by using the static Live-scanned child router paths
        // as candidates, even before the child router widgets are fully instantiated.
        for route_id in self.routes.child_router_paths.keys().cloned().chain(self.child_routers.keys().cloned()) {
            let Some(pattern_obj) = self.router.route_registry.get_pattern(route_id) else {
                continue;
            };
            let Some((params, tail)) = pattern_obj.matches_prefix_with_tail(path) else {
                continue;
            };
            let priority = pattern_obj.priority();
            match &best {
                Some((_id, _p, _pat, _tail, best_prio)) if *best_prio <= priority => {}
                _ => {
                    best = Some((route_id, params, pattern_obj.clone(), tail, priority));
                }
            }
        }

        let out = best.map(|(id, params, pattern, tail, _prio)| (id, params, pattern, tail));
        self.caches.nested_prefix_cache_epoch = self.caches.route_registry_epoch;
        self.caches.nested_prefix_cache_path = path.to_string();
        self.caches.nested_prefix_cache_result = out.clone();
        out
    }

    pub(super) fn delegate_tail_to_child(
        &mut self,
        cx: &mut Cx,
        parent_route_id: LiveId,
        tail: &str,
    ) -> bool {
        if tail.is_empty() {
            return true;
        }
        self.detect_child_routers(cx);
        let child_router = self.child_routers.get(&parent_route_id).cloned();
        if let Some(child_router) = child_router {
            if let Some(mut child) = child_router.borrow_mut() {
                return child.navigate_by_path(cx, tail);
            }
        }
        false
    }

    /// Automatically detect and register child routers in route widgets.
    ///
    /// We scan the Live DSL for nested `RouterWidget` instances (and their widget-id paths) in
    /// `apply_value_instance`, then resolve those paths against the instantiated route widgets here.
    pub(super) fn detect_child_routers(&mut self, _cx: &mut Cx) {
        for (route_id, route_widget) in self.routes.widgets.iter() {
            if self.child_routers.contains_key(route_id) {
                continue;
            }
            let Some(paths) = self.routes.child_router_paths.get(route_id) else {
                continue;
            };
            for path in paths {
                let child_widget = route_widget.widget(path);
                if child_widget.borrow::<RouterWidget>().is_some() {
                    let child_router = child_widget.as_router_widget();
                    if let Some(mut inner) = child_router.borrow_mut() {
                        inner.url_sync = false;
                        inner.use_initial_url = false;
                        inner.web.history_initialized = false;
                    }
                    self.child_routers.insert(*route_id, child_router);
                    break;
                }
            }
        }
    }

    pub(super) fn collect_child_router_paths(root_index: usize, nodes: &[LiveNode]) -> Vec<Vec<LiveId>> {
        let router_live_type = LiveType::of::<RouterWidget>();
        let mut out = Vec::new();
        let mut path = Vec::<LiveId>::new();

        let end = nodes.skip_node(root_index);
        let mut i = root_index + 1;
        while i < end {
            i = Self::collect_child_router_paths_recur(i, nodes, router_live_type, &mut path, &mut out);
        }
        out
    }

    fn collect_child_router_paths_recur(
        index: usize,
        nodes: &[LiveNode],
        router_live_type: LiveType,
        path: &mut Vec<LiveId>,
        out: &mut Vec<Vec<LiveId>>,
    ) -> usize {
        let node = &nodes[index];

        if node.origin.has_prop_type(LivePropType::Instance) {
            if let LiveValue::Class { live_type, .. } = &node.value {
                path.push(node.id);
                if *live_type == router_live_type {
                    out.push(path.clone());
                }

                let end = nodes.skip_node(index);
                let mut i = index + 1;
                while i < end {
                    i = Self::collect_child_router_paths_recur(i, nodes, router_live_type, path, out);
                }
                path.pop();
                return end;
            }
        }

        if node.value.is_open() {
            let end = nodes.skip_node(index);
            let mut i = index + 1;
            while i < end {
                i = Self::collect_child_router_paths_recur(i, nodes, router_live_type, path, out);
            }
            return end;
        }

        index + 1
    }

    /// Navigate to a nested route.
    pub fn navigate_nested(&mut self, cx: &mut Cx, path: &[LiveId], route: Route) -> bool {
        if path.is_empty() {
            // Navigate in current router.
            if self.routes.templates.contains_key(&route.id) {
                let old_route = self.router.current_route().cloned();
                self.router.navigate(route.clone());
                self.active_route = route.id;

                self.ensure_route_widget(cx, route.id);
                self.start_transition(
                    cx,
                    old_route.as_ref().map(|r| r.id),
                    route.id,
                    super::RouterActionKind::Push,
                    super::RouterTransitionDirection::Forward,
                    None,
                );

                self.redraw(cx);
                return true;
            }
            return false;
        }

        // Navigate to child router.
        let first = path[0];
        let child_router_opt = self.child_routers.get(&first).cloned();
        if let Some(child_router) = child_router_opt {
            if let Some(mut child) = child_router.borrow_mut() {
                if child.navigate_nested(cx, &path[1..], route) {
                    self.redraw(cx);
                    return true;
                }
            }
        }

        false
    }
}
