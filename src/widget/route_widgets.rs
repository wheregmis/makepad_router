use makepad_widgets::*;

use super::RouterWidget;

impl RouterWidget {
    pub(super) fn new_route_widget_from_ptr(cx: &mut Cx, ptr: LivePtr) -> WidgetRef {
        let mut widget = WidgetRef::empty();
        cx.get_nodes_from_live_ptr(ptr, |cx, file_id, index, nodes| {
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
            let mut apply = ApplyFrom::NewFromDoc { file_id }.into();
            Self::apply_widget_silencing_route_metadata(
                cx,
                &mut apply,
                index,
                nodes,
                &mut widget,
                &[
                    route_pattern_idx,
                    route_transition_idx,
                    route_transition_duration_idx,
                ],
            );
            nodes.skip_node(index)
        });
        widget
    }

    pub(super) fn ensure_route_widget(&mut self, cx: &mut Cx, route_id: LiveId) {
        if self.route_widgets.contains_key(&route_id) {
            return;
        }
        let Some(ptr) = self.route_templates.get(&route_id).copied() else {
            return;
        };
        self.route_widgets
            .get_or_insert(cx, route_id, |cx| Self::new_route_widget_from_ptr(cx, ptr));
    }

    /// Apply a route widget while silencing router-only DSL metadata.
    ///
    /// `route_pattern` / `route_transition` / `route_transition_duration` are router-level metadata
    /// fields, not properties of the route page widgets.
    /// The Live apply system forwards all instance children into the instantiated widget. Instead
    /// of attempting to surgically re-run the apply process without this field (which would require
    /// reconstructing parts of the apply engine), we mark the node as "prefixed". The default
    /// `LiveHook::apply_value_unknown` handler does not warn on prefixed unknown properties, so the
    /// page widget ignores it without logging.
    pub(super) fn apply_widget_silencing_route_metadata(
        cx: &mut Cx,
        apply: &mut Apply,
        instance_index: usize,
        nodes: &[LiveNode],
        widget: &mut WidgetRef,
        silence_node_indices: &[Option<usize>],
    ) {
        if silence_node_indices.iter().all(|i| i.is_none()) {
            widget.apply(cx, apply, instance_index, nodes);
            return;
        }

        let mut patched_nodes = nodes.to_vec();
        for idx in silence_node_indices.iter().flatten().copied() {
            patched_nodes[idx].origin = patched_nodes[idx].origin.with_node_has_prefix(true);
        }
        widget.apply(cx, apply, instance_index, &patched_nodes);
    }
}

