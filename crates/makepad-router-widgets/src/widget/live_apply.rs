use crate::registry::RouteRegistry;
use crate::route::Route;
use makepad_widgets::*;

use super::RouterWidget;

impl LiveHook for RouterWidget {
    fn before_apply(
        &mut self,
        _cx: &mut Cx,
        apply: &mut Apply,
        _index: usize,
        _nodes: &[LiveNode],
    ) {
        if let ApplyFrom::UpdateFromDoc { .. } = apply.from {
            self.caches.route_registry_epoch = self.caches.route_registry_epoch.wrapping_add(1);
            self.caches.nested_prefix_cache_epoch = 0;
            self.caches.nested_prefix_cache_path.clear();
            self.caches.nested_prefix_cache_result = None;
            self.caches.url_parse_cache.clear();
            self.routes.templates.clear();
            self.routes.patterns.clear();
            self.routes.transition_overrides.clear();
            self.routes.transition_duration_overrides.clear();
            self.routes.child_router_paths.clear();
            self.child_routers.clear();
            self.router.route_registry = RouteRegistry::default();
            self.transition_rt.state = None;
        }
    }

    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        match apply.from {
            ApplyFrom::NewFromDoc { .. } | ApplyFrom::UpdateFromDoc { .. } => {
                if self.router.current_route().is_none() {
                    let initial_route = if self.active_route.0 != 0 {
                        self.active_route
                    } else {
                        self.default_route
                    };

                    if initial_route.0 != 0 {
                        self.router.persist_state = self.persist_state;
                        self.router.reset(Route::new(initial_route));
                        self.active_route = initial_route;
                    }
                }

                // Performance-first: lazily instantiate only the active route.
                if self.active_route.0 != 0 {
                    self.ensure_route_widget(cx, self.active_route);
                }

                self.detect_child_routers(cx);
            }
            _ => (),
        }
    }

    fn apply_value_instance(
        &mut self,
        cx: &mut Cx,
        apply: &mut Apply,
        index: usize,
        nodes: &[LiveNode],
    ) -> usize {
        let id = nodes[index].id;
        match apply.from {
            ApplyFrom::NewFromDoc { file_id } | ApplyFrom::UpdateFromDoc { file_id, .. } => {
                if nodes[index].origin.has_prop_type(LivePropType::Instance) {
                    let live_ptr = cx
                        .live_registry
                        .borrow()
                        .file_id_index_to_live_ptr(file_id, index);
                    self.routes.templates.insert(id, live_ptr);

                    // Scan for route_pattern property in child nodes and register it.
                    if let Some(pattern_node_idx) = nodes.child_by_name(
                        index,
                        LiveProp(live_id!(route_pattern), LivePropType::Field),
                    ) {
                        let pattern_node = &nodes[pattern_node_idx];
                        if let LiveValue::Str(pattern) = &pattern_node.value {
                            let pattern_str = pattern.to_string();
                            self.routes.patterns.insert(id, pattern_str.clone());
                            if let Err(e) = self.router.register_route_pattern(&pattern_str, id) {
                                log!("Failed to register route pattern {}: {}", pattern_str, e);
                            } else {
                                self.caches.route_registry_epoch =
                                    self.caches.route_registry_epoch.wrapping_add(1);
                            }
                        } else if let LiveValue::String(pattern) = &pattern_node.value {
                            let pattern_str = pattern.as_str().to_string();
                            self.routes.patterns.insert(id, pattern_str.clone());
                            if let Err(e) = self.router.register_route_pattern(&pattern_str, id) {
                                log!("Failed to register route pattern {}: {}", pattern_str, e);
                            } else {
                                self.caches.route_registry_epoch =
                                    self.caches.route_registry_epoch.wrapping_add(1);
                            }
                        }
                    }

                    // Optional per-route transition override (DSL metadata).
                    if let Some(transition_node_idx) = nodes.child_by_name(
                        index,
                        LiveProp(live_id!(route_transition), LivePropType::Field),
                    ) {
                        let transition_node = &nodes[transition_node_idx];
                        let transition_id = match &transition_node.value {
                            LiveValue::Id(id) => *id,
                            LiveValue::Str(s) => LiveId::from_str(s),
                            LiveValue::String(s) => LiveId::from_str(s.as_str()),
                            _ => LiveId(0),
                        };
                        if transition_id.0 != 0 {
                            self.routes.transition_overrides.insert(id, transition_id);
                        }
                    }

                    if let Some(duration_node_idx) = nodes.child_by_name(
                        index,
                        LiveProp(live_id!(route_transition_duration), LivePropType::Field),
                    ) {
                        let duration_node = &nodes[duration_node_idx];
                        let duration = match &duration_node.value {
                            LiveValue::Float64(v) => Some(*v),
                            LiveValue::Float32(v) => Some(*v as f64),
                            LiveValue::Int64(v) => Some(*v as f64),
                            _ => None,
                        };
                        if let Some(duration) = duration {
                            self.routes.transition_duration_overrides.insert(id, duration);
                        }
                    }

                    // Scan for nested RouterWidget instances inside this route.
                    self.routes.child_router_paths
                        .insert(id, Self::collect_child_router_paths(index, nodes));
                    self.caches.route_registry_epoch = self.caches.route_registry_epoch.wrapping_add(1);

                    // Create/update the route widget instance only if it already exists
                    // (e.g. after navigation / hot reload). Otherwise it will be lazily created.
                    if self.routes.widgets.contains_key(&id) {
                        let route_pattern_idx = nodes.child_by_name(
                            index,
                            LiveProp(live_id!(route_pattern), LivePropType::Field),
                        );
                        let route_transition_idx = nodes.child_by_name(
                            index,
                            LiveProp(live_id!(route_transition), LivePropType::Field),
                        );
                        let route_transition_duration_idx = nodes.child_by_name(
                            index,
                            LiveProp(live_id!(route_transition_duration), LivePropType::Field),
                        );

                        let widget = self
                            .routes
                            .widgets
                            .get_or_insert(cx, id, |_cx| WidgetRef::empty());

                        Self::apply_widget_silencing_route_metadata(
                            cx,
                            apply,
                            index,
                            nodes,
                            widget,
                            &[
                                route_pattern_idx,
                                route_transition_idx,
                                route_transition_duration_idx,
                            ],
                        );
                    }
                } else {
                    cx.apply_error_no_matching_field(live_error_origin!(), index, nodes);
                }
            }
            _ => (),
        }
        nodes.skip_node(index)
    }
}
