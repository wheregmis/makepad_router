use crate::{route::Route, router::RouterAction, state::RouterState};
use makepad_widgets::*;

use super::{RouterNavRequest, RouterWidget, RouterActionKind, RouterTransitionDirection, RouterTransitionSpec};

impl RouterWidget {
    pub fn navigate_by_url(&mut self, cx: &mut Cx, url: &str) -> bool {
        if !self.guard_bypass {
            return self.request_navigation(
                cx,
                RouterNavRequest::NavigateByUrl {
                    url: url.to_string(),
                },
            );
        }
        self.ensure_web_history_initialized(cx);
        let ok = self.navigate_by_path_internal(cx, url, false);

        if ok {
            self.web_push_current_url(cx);
        }
        ok
    }

    pub fn navigate(&mut self, cx: &mut Cx, route_id: LiveId) -> bool {
        if !self.guard_bypass {
            return self.request_navigation(cx, RouterNavRequest::Navigate { route_id });
        }
        if self.route_templates.contains_key(&route_id) {
            self.clear_url_extras();
            let old_route = self.router.current_route().cloned();
            self.router.navigate_to(route_id);
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

            if let Some(new_route) = self.router.current_route().cloned() {
                self.dispatch_route_change(cx, old_route.clone(), new_route.clone());
                self.queue_route_actions(
                    Some(RouterAction::Navigate(new_route.clone())),
                    old_route.as_ref().map(|r| r.id),
                    &new_route,
                );
            }

            self.web_push_current_url(cx);
            self.redraw(cx);
            true
        } else {
            log!("Router: Route template not found for {:?}", route_id);
            false
        }
    }

    pub fn navigate_with_transition(
        &mut self,
        cx: &mut Cx,
        route_id: LiveId,
        transition: RouterTransitionSpec,
    ) -> bool {
        if !self.guard_bypass {
            return self.request_navigation(
                cx,
                RouterNavRequest::NavigateWithTransition {
                    route_id,
                    transition,
                },
            );
        }
        if self.route_templates.contains_key(&route_id) {
            self.clear_url_extras();
            let old_route = self.router.current_route().cloned();
            self.router.navigate_to(route_id);
            self.active_route = route_id;

            self.ensure_route_widget(cx, route_id);
            self.start_transition(
                cx,
                old_route.as_ref().map(|r| r.id),
                route_id,
                RouterActionKind::Push,
                RouterTransitionDirection::Forward,
                Some(transition),
            );

            if let Some(new_route) = self.router.current_route().cloned() {
                self.dispatch_route_change(cx, old_route.clone(), new_route.clone());
                self.queue_route_actions(
                    Some(RouterAction::Navigate(new_route.clone())),
                    old_route.as_ref().map(|r| r.id),
                    &new_route,
                );
            }

            self.web_push_current_url(cx);
            self.redraw(cx);
            true
        } else {
            log!("Router: Route template not found for {:?}", route_id);
            false
        }
    }

    pub fn replace(&mut self, cx: &mut Cx, route_id: LiveId) -> bool {
        if !self.guard_bypass {
            return self.request_navigation(cx, RouterNavRequest::Replace { route_id });
        }
        if self.route_templates.contains_key(&route_id) {
            self.clear_url_extras();
            let old_route = self.router.current_route().cloned();
            self.router.replace_with(route_id);
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

            if let Some(new_route) = self.router.current_route().cloned() {
                self.dispatch_route_change(cx, old_route.clone(), new_route.clone());
                self.queue_route_actions(
                    Some(RouterAction::Replace(new_route.clone())),
                    old_route.as_ref().map(|r| r.id),
                    &new_route,
                );
            }

            self.web_replace_current_url(cx);
            self.redraw(cx);
            true
        } else {
            log!("Router: Route template not found for {:?}", route_id);
            false
        }
    }

    pub fn replace_with_transition(
        &mut self,
        cx: &mut Cx,
        route_id: LiveId,
        transition: RouterTransitionSpec,
    ) -> bool {
        if !self.guard_bypass {
            return self.request_navigation(
                cx,
                RouterNavRequest::ReplaceWithTransition {
                    route_id,
                    transition,
                },
            );
        }
        if self.route_templates.contains_key(&route_id) {
            self.clear_url_extras();
            let old_route = self.router.current_route().cloned();
            self.router.replace_with(route_id);
            self.active_route = route_id;

            self.ensure_route_widget(cx, route_id);
            self.start_transition(
                cx,
                old_route.as_ref().map(|r| r.id),
                route_id,
                RouterActionKind::Replace,
                RouterTransitionDirection::Forward,
                Some(transition),
            );

            if let Some(new_route) = self.router.current_route().cloned() {
                self.dispatch_route_change(cx, old_route.clone(), new_route.clone());
                self.queue_route_actions(
                    Some(RouterAction::Replace(new_route.clone())),
                    old_route.as_ref().map(|r| r.id),
                    &new_route,
                );
            }

            self.web_replace_current_url(cx);
            self.redraw(cx);
            true
        } else {
            log!("Router: Route template not found for {:?}", route_id);
            false
        }
    }

    pub fn back(&mut self, cx: &mut Cx) -> bool {
        if !self.guard_bypass {
            return self.request_navigation(cx, RouterNavRequest::Back { transition: None });
        }
        let old_route = self.router.current_route().cloned();
        if self.router.back() {
            if let Some(route) = self.router.current_route().cloned() {
                self.clear_url_extras();
                self.active_route = route.id;
                self.start_transition(
                    cx,
                    old_route.as_ref().map(|r| r.id),
                    route.id,
                    RouterActionKind::Pop,
                    RouterTransitionDirection::Backward,
                    None,
                );

                self.dispatch_route_change(cx, old_route.clone(), route.clone());

                self.ensure_route_widget(cx, route.id);
                self.queue_route_actions(
                    Some(RouterAction::Back),
                    old_route.as_ref().map(|r| r.id),
                    &route,
                );

                self.web_go(cx, -1);
                self.redraw(cx);
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn back_with_transition(&mut self, cx: &mut Cx, transition: RouterTransitionSpec) -> bool {
        if !self.guard_bypass {
            return self.request_navigation(
                cx,
                RouterNavRequest::Back {
                    transition: Some(transition),
                },
            );
        }
        let old_route = self.router.current_route().cloned();
        if self.router.back() {
            if let Some(route) = self.router.current_route().cloned() {
                self.clear_url_extras();
                self.active_route = route.id;
                self.start_transition(
                    cx,
                    old_route.as_ref().map(|r| r.id),
                    route.id,
                    RouterActionKind::Pop,
                    RouterTransitionDirection::Backward,
                    Some(transition),
                );
                self.dispatch_route_change(cx, old_route.clone(), route.clone());
                self.ensure_route_widget(cx, route.id);
                self.queue_route_actions(
                    Some(RouterAction::Back),
                    old_route.as_ref().map(|r| r.id),
                    &route,
                );
                self.web_go(cx, -1);
                self.redraw(cx);
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn forward(&mut self, cx: &mut Cx) -> bool {
        if !self.guard_bypass {
            return self.request_navigation(cx, RouterNavRequest::Forward { transition: None });
        }
        let old_route = self.router.current_route().cloned();
        if self.router.forward() {
            if let Some(route) = self.router.current_route().cloned() {
                self.clear_url_extras();
                self.active_route = route.id;
                self.start_transition(
                    cx,
                    old_route.as_ref().map(|r| r.id),
                    route.id,
                    RouterActionKind::Push,
                    RouterTransitionDirection::Forward,
                    None,
                );
                self.dispatch_route_change(cx, old_route.clone(), route.clone());
                self.ensure_route_widget(cx, route.id);
                self.queue_route_actions(
                    Some(RouterAction::Forward),
                    old_route.as_ref().map(|r| r.id),
                    &route,
                );
                self.web_go(cx, 1);
                self.redraw(cx);
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn forward_with_transition(
        &mut self,
        cx: &mut Cx,
        transition: RouterTransitionSpec,
    ) -> bool {
        if !self.guard_bypass {
            return self.request_navigation(
                cx,
                RouterNavRequest::Forward {
                    transition: Some(transition),
                },
            );
        }
        let old_route = self.router.current_route().cloned();
        if self.router.forward() {
            if let Some(route) = self.router.current_route().cloned() {
                self.clear_url_extras();
                self.active_route = route.id;
                self.start_transition(
                    cx,
                    old_route.as_ref().map(|r| r.id),
                    route.id,
                    RouterActionKind::Push,
                    RouterTransitionDirection::Forward,
                    Some(transition),
                );
                self.dispatch_route_change(cx, old_route.clone(), route.clone());
                self.ensure_route_widget(cx, route.id);
                self.queue_route_actions(
                    Some(RouterAction::Forward),
                    old_route.as_ref().map(|r| r.id),
                    &route,
                );
                self.web_go(cx, 1);
                self.redraw(cx);
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn can_go_back(&self) -> bool {
        self.router.can_go_back()
    }

    pub fn can_go_forward(&self) -> bool {
        self.router.can_go_forward()
    }

    pub fn depth(&self) -> usize {
        self.router.depth()
    }

    pub fn current_route_id(&self) -> Option<LiveId> {
        self.router.current_route_id()
    }

    pub fn get_state(&self) -> RouterState {
        self.build_state()
    }

    pub fn set_state(&mut self, cx: &mut Cx, state: RouterState) -> bool {
        self.apply_state(cx, state)
    }

    pub fn clear_history(&mut self, cx: &mut Cx) {
        self.router.clear_history();
        self.web_replace_current_url(cx);
        self.redraw(cx);
    }

    pub fn reset(&mut self, cx: &mut Cx, route: Route) -> bool {
        if !self.guard_bypass {
            return self.request_navigation(cx, RouterNavRequest::Reset { route });
        }
        if !self.route_templates.contains_key(&route.id) {
            log!("Router: Route template not found for {:?}", route.id);
            return false;
        }
        self.clear_url_extras();
        let old_route = self.router.current_route().cloned();
        self.router.reset(route.clone());
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

        if let Some(new_route) = self.router.current_route().cloned() {
            self.dispatch_route_change(cx, old_route.clone(), new_route.clone());
            self.queue_route_actions(
                Some(RouterAction::Reset(new_route.clone())),
                old_route.as_ref().map(|r| r.id),
                &new_route,
            );
        }

        self.web_replace_current_url(cx);
        self.redraw(cx);
        true
    }

    pub fn push(&mut self, cx: &mut Cx, route_id: LiveId) -> bool {
        self.navigate(cx, route_id)
    }

    pub fn pop(&mut self, cx: &mut Cx) -> bool {
        if !self.guard_bypass {
            return self.request_navigation(cx, RouterNavRequest::Pop);
        }
        let old_route = self.router.current_route().cloned();
        if self.router.pop() {
            if let Some(new_route) = self.router.current_route().cloned() {
                self.clear_url_extras();
                self.active_route = new_route.id;
                self.ensure_route_widget(cx, new_route.id);
                self.start_transition(
                    cx,
                    old_route.as_ref().map(|r| r.id),
                    new_route.id,
                    RouterActionKind::Pop,
                    RouterTransitionDirection::Backward,
                    None,
                );
                self.dispatch_route_change(cx, old_route.clone(), new_route.clone());
                self.queue_route_actions(None, old_route.as_ref().map(|r| r.id), &new_route);
                self.web_go(cx, -1);
                self.redraw(cx);
                return true;
            }
        }
        false
    }

    pub fn pop_to(&mut self, cx: &mut Cx, route_id: LiveId) -> bool {
        if !self.guard_bypass {
            return self.request_navigation(cx, RouterNavRequest::PopTo { route_id });
        }
        let before_depth = self.router.depth() as i32;
        let old_route = self.router.current_route().cloned();
        if self.router.pop_to(route_id) {
            if let Some(new_route) = self.router.current_route().cloned() {
                self.clear_url_extras();
                self.active_route = new_route.id;
                self.ensure_route_widget(cx, new_route.id);
                self.start_transition(
                    cx,
                    old_route.as_ref().map(|r| r.id),
                    new_route.id,
                    RouterActionKind::Pop,
                    RouterTransitionDirection::Backward,
                    None,
                );
                self.dispatch_route_change(cx, old_route.clone(), new_route.clone());
                self.queue_route_actions(None, old_route.as_ref().map(|r| r.id), &new_route);
                let after_depth = self.router.depth() as i32;
                let delta = after_depth - before_depth;
                if delta != 0 {
                    self.web_go(cx, delta);
                }
                self.redraw(cx);
                return true;
            }
        }
        false
    }

    pub fn pop_to_root(&mut self, cx: &mut Cx) -> bool {
        if !self.guard_bypass {
            return self.request_navigation(cx, RouterNavRequest::PopToRoot);
        }
        let before_depth = self.router.depth() as i32;
        let old_route = self.router.current_route().cloned();
        if self.router.pop_to_root() {
            if let Some(new_route) = self.router.current_route().cloned() {
                self.clear_url_extras();
                self.active_route = new_route.id;
                self.ensure_route_widget(cx, new_route.id);
                self.start_transition(
                    cx,
                    old_route.as_ref().map(|r| r.id),
                    new_route.id,
                    RouterActionKind::Pop,
                    RouterTransitionDirection::Backward,
                    None,
                );
                self.dispatch_route_change(cx, old_route.clone(), new_route.clone());
                self.queue_route_actions(None, old_route.as_ref().map(|r| r.id), &new_route);
                let after_depth = self.router.depth() as i32;
                let delta = after_depth - before_depth;
                if delta != 0 {
                    self.web_go(cx, delta);
                }
                self.redraw(cx);
                return true;
            }
        }
        false
    }

    pub fn set_stack(&mut self, cx: &mut Cx, stack: Vec<Route>) -> bool {
        if !self.guard_bypass {
            return self.request_navigation(cx, RouterNavRequest::SetStack { stack });
        }
        let filtered: Vec<Route> = stack
            .into_iter()
            .filter(|r| self.route_templates.contains_key(&r.id))
            .collect();
        if filtered.is_empty() {
            return false;
        }
        self.clear_url_extras();
        let old_route = self.router.current_route().cloned();
        self.router.set_stack(filtered);
        let Some(new_route) = self.router.current_route().cloned() else {
            return false;
        };
        self.active_route = new_route.id;
        self.ensure_route_widget(cx, new_route.id);
        self.start_transition(
            cx,
            old_route.as_ref().map(|r| r.id),
            new_route.id,
            RouterActionKind::Replace,
            RouterTransitionDirection::Forward,
            None,
        );
        self.dispatch_route_change(cx, old_route.clone(), new_route.clone());
        self.queue_route_actions(
            Some(RouterAction::Reset(new_route.clone())),
            old_route.as_ref().map(|r| r.id),
            &new_route,
        );
        self.web_replace_current_url(cx);
        self.redraw(cx);
        true
    }

    /// Navigate by path string
    pub fn navigate_by_path(&mut self, cx: &mut Cx, path: &str) -> bool {
        if !self.guard_bypass {
            return self.request_navigation(
                cx,
                RouterNavRequest::NavigateByPath {
                    path: path.to_string(),
                },
            );
        }
        let ok = self.navigate_by_path_internal(cx, path, true);
        if ok {
            self.web_push_current_url(cx);
        }
        ok
    }
}
