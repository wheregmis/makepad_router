use crate::{
    route::{Route, RouteQuery},
    router::RouterAction,
};
use makepad_widgets::*;

use super::{RouterActionKind, RouterTransitionDirection, RouterWidget};

impl RouterWidget {
    pub(super) fn navigate_by_path_internal(
        &mut self,
        cx: &mut Cx,
        path: &str,
        clear_extras: bool,
    ) -> bool {
        let parsed = self.parse_url_cached(path);
        let query = RouteQuery::from_query_string(&parsed.query);
        let hash = parsed.hash.clone();
        let path = parsed.path;

        if clear_extras {
            self.clear_url_extras();
        } else {
            self.url_path_override = None;
        }

        // 1) Full match in this router.
        if let Some(mut route) = self.router.route_registry.resolve_path(&path) {
            if self.routes.templates.contains_key(&route.id) {
                let old_route = self.router.current_route().cloned();
                route.query = query.clone();
                route.hash = hash.clone();
                self.router.navigate(route.clone());
                self.active_route = route.id;

                self.ensure_route_widget(cx, route.id);
                self.start_transition(
                    cx,
                    old_route.as_ref().map(|r| r.id),
                    route.id,
                    RouterActionKind::Push,
                    RouterTransitionDirection::Forward,
                    None,
                );

                self.dispatch_route_change(cx, old_route.clone(), route.clone());
                self.queue_route_actions(
                    Some(RouterAction::Navigate(route.clone())),
                    old_route.as_ref().map(|r| r.id),
                    &route,
                );

                // If this route owns a child router, delegate the tail to it.
                if self.child_routers.contains_key(&route.id) {
                    if let Some(pattern) = &route.pattern {
                        if let Some((_params, tail)) = pattern.matches_prefix_with_tail(&path) {
                            let _ = self.delegate_tail_to_child(cx, route.id, &tail);
                        }
                    }
                }

                self.redraw(cx);
                return true;
            }
        }

        // 2) Prefix match for nested routing: activate a parent route and delegate the tail.
        if let Some((route_id, params, pattern, tail)) = self.resolve_nested_prefix(&path) {
            if self.routes.templates.contains_key(&route_id) {
                let old_route = self.router.current_route().cloned();
                let parent_route = Route {
                    id: route_id,
                    params,
                    query: query.clone(),
                    hash: hash.clone(),
                    pattern: Some(pattern),
                };
                self.router.navigate(parent_route.clone());
                self.active_route = route_id;
                self.ensure_route_widget(cx, route_id);
                self.start_transition(
                    cx,
                    old_route.as_ref().map(|r| r.id),
                    route_id,
                    RouterActionKind::Push,
                    RouterTransitionDirection::Forward,
                    None,
                );

                self.dispatch_route_change(cx, old_route.clone(), parent_route.clone());
                self.queue_route_actions(
                    Some(RouterAction::Navigate(parent_route.clone())),
                    old_route.as_ref().map(|r| r.id),
                    &parent_route,
                );

                let _ = self.delegate_tail_to_child(cx, route_id, &tail);
                self.redraw(cx);
                return true;
            }
        }

        // 3) Not-found fallback.
        if self.not_found_route.0 != 0 && self.routes.templates.contains_key(&self.not_found_route)
        {
            // Push not-found so the user can navigate back to the previous page.
            // Preserve the attempted path in the address bar.
            if self.current_route_id() != Some(self.not_found_route) {
                self.url_path_override = Some(path);
                let old_route = self.router.current_route().cloned();
                let mut nf = Route::new(self.not_found_route);
                nf.query = query;
                nf.hash = hash;
                self.router.navigate(nf);
                self.active_route = self.not_found_route;
                self.ensure_route_widget(cx, self.not_found_route);
                self.start_transition(
                    cx,
                    old_route.as_ref().map(|r| r.id),
                    self.not_found_route,
                    RouterActionKind::Push,
                    RouterTransitionDirection::Forward,
                    None,
                );
                if let Some(new_route) = self.router.current_route().cloned() {
                    self.dispatch_route_change(cx, old_route.clone(), new_route.clone());
                    self.queue_route_actions(
                        Some(RouterAction::Navigate(new_route.clone())),
                        old_route.as_ref().map(|r| r.id),
                        &new_route,
                    );
                }
                self.redraw(cx);
                return true;
            }
            return false;
        }

        log!("Router: No route found for path: {}", path);
        false
    }

    pub(super) fn replace_by_path_internal(
        &mut self,
        cx: &mut Cx,
        path: &str,
        clear_extras: bool,
    ) -> bool {
        let parsed = self.parse_url_cached(path);
        let query = RouteQuery::from_query_string(&parsed.query);
        let hash = parsed.hash.clone();
        let path = parsed.path;

        if clear_extras {
            self.clear_url_extras();
        } else {
            self.url_path_override = None;
        }

        if let Some(mut route) = self.router.route_registry.resolve_path(&path) {
            if self.routes.templates.contains_key(&route.id) {
                let old_route = self.router.current_route().cloned();
                route.query = query.clone();
                route.hash = hash.clone();
                self.router.replace(route.clone());
                self.active_route = route.id;
                self.ensure_route_widget(cx, route.id);
                self.start_transition(
                    cx,
                    old_route.as_ref().map(|r| r.id),
                    route.id,
                    RouterActionKind::Replace,
                    RouterTransitionDirection::Forward,
                    None,
                );
                self.dispatch_route_change(cx, old_route.clone(), route.clone());
                self.queue_route_actions(
                    Some(RouterAction::Replace(route.clone())),
                    old_route.as_ref().map(|r| r.id),
                    &route,
                );

                // If this route owns a child router, delegate the tail to it.
                if self.child_routers.contains_key(&route.id) {
                    if let Some(pattern) = &route.pattern {
                        if let Some((_params, tail)) = pattern.matches_prefix_with_tail(&path) {
                            let _ = self.delegate_tail_to_child(cx, route.id, &tail);
                        }
                    }
                }

                self.redraw(cx);
                return true;
            }
        }

        if let Some((route_id, params, pattern, tail)) = self.resolve_nested_prefix(&path) {
            if self.routes.templates.contains_key(&route_id) {
                let old_route = self.router.current_route().cloned();
                let parent_route = Route {
                    id: route_id,
                    params,
                    query: query.clone(),
                    hash: hash.clone(),
                    pattern: Some(pattern),
                };
                self.router.replace(parent_route.clone());
                self.active_route = route_id;
                self.ensure_route_widget(cx, route_id);
                self.start_transition(
                    cx,
                    old_route.as_ref().map(|r| r.id),
                    route_id,
                    RouterActionKind::Replace,
                    RouterTransitionDirection::Forward,
                    None,
                );

                self.dispatch_route_change(cx, old_route.clone(), parent_route.clone());
                self.queue_route_actions(
                    Some(RouterAction::Replace(parent_route.clone())),
                    old_route.as_ref().map(|r| r.id),
                    &parent_route,
                );

                let _ = self.delegate_tail_to_child(cx, route_id, &tail);
                self.redraw(cx);
                return true;
            }
        }

        if self.not_found_route.0 != 0 && self.routes.templates.contains_key(&self.not_found_route)
        {
            self.url_path_override = Some(path);
            let old_route = self.router.current_route().cloned();
            let mut nf = Route::new(self.not_found_route);
            nf.query = query;
            nf.hash = hash;
            self.router.replace(nf);
            self.active_route = self.not_found_route;
            self.ensure_route_widget(cx, self.not_found_route);
            self.start_transition(
                cx,
                old_route.as_ref().map(|r| r.id),
                self.not_found_route,
                RouterActionKind::Replace,
                RouterTransitionDirection::Forward,
                None,
            );
            if let Some(new_route) = self.router.current_route().cloned() {
                self.dispatch_route_change(cx, old_route.clone(), new_route.clone());
                self.queue_route_actions(
                    Some(RouterAction::Replace(new_route.clone())),
                    old_route.as_ref().map(|r| r.id),
                    &new_route,
                );
            }
            self.redraw(cx);
            return true;
        }

        false
    }
}
