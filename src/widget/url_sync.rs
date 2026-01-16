use makepad_widgets::*;

use super::{RouterNavRequest, RouterWidget};

impl RouterWidget {
    pub(super) fn clear_url_extras(&mut self) {
        self.web.url_path_override = None;
    }

    fn web_enabled(&self, cx: &Cx) -> bool {
        self.url_sync && cx.os_type().is_web()
    }

    pub(super) fn ensure_web_history_initialized(&mut self, cx: &mut Cx) {
        if !self.web_enabled(cx) || self.web.history_initialized {
            return;
        }
        self.web.history_initialized = true;
        self.web.history_index = 0;

        // Stamp an initial history state so `popstate` can report an index.
        if !self.web.suppress_browser_update {
            let url = self.current_url();
            CxOsApi::set_browser_url(cx, &url, true, self.web.history_index as f64);
            self.web_mark_synced(cx);
        }
    }

    fn web_active_child_depth(&mut self, cx: &mut Cx) -> Option<usize> {
        self.detect_child_routers(cx);
        let child = self.child_routers.get(&self.active_route)?.borrow()?;
        Some(child.router.depth())
    }

    fn web_mark_synced(&mut self, cx: &mut Cx) {
        self.web.last_synced_url = Some(self.current_url());
        self.web.last_depth = self.router.depth();
        self.web.last_child_depth = self.web_active_child_depth(cx);
        self.web.last_child_parent_route = self.active_route;
    }

    pub(super) fn web_push_current_url(&mut self, cx: &mut Cx) {
        if !self.web_enabled(cx) {
            return;
        }
        self.ensure_web_history_initialized(cx);
        self.web.history_index = self.web.history_index.saturating_add(1);
        if self.web.suppress_browser_update {
            return;
        }
        let url = self.current_url();
        CxOsApi::set_browser_url(cx, &url, false, self.web.history_index as f64);
        self.web_mark_synced(cx);
    }

    pub(super) fn web_replace_current_url(&mut self, cx: &mut Cx) {
        if !self.web_enabled(cx) {
            return;
        }
        self.ensure_web_history_initialized(cx);
        if self.web.suppress_browser_update {
            return;
        }
        let url = self.current_url();
        CxOsApi::set_browser_url(cx, &url, true, self.web.history_index as f64);
        self.web_mark_synced(cx);
    }

    pub(super) fn web_go(&mut self, cx: &mut Cx, delta: i32) {
        if !self.web_enabled(cx) {
            return;
        }
        self.ensure_web_history_initialized(cx);

        if delta < 0 {
            self.web.history_index = self.web.history_index.saturating_sub((-delta) as i32);
        } else {
            self.web.history_index = self.web.history_index.saturating_add(delta as i32);
        }

        if self.web.suppress_browser_update {
            return;
        }
        self.web.ignore_next_browser_url_change = true;
        CxOsApi::browser_history_go(cx, delta);
        self.web_mark_synced(cx);
    }

    pub(super) fn sync_web_url_if_needed(&mut self, cx: &mut Cx) {
        if !self.web_enabled(cx) || self.web.suppress_browser_update {
            return;
        }
        self.ensure_web_history_initialized(cx);

        let current_url = self.current_url();
        let Some(last_url) = self.web.last_synced_url.clone() else {
            self.web_mark_synced(cx);
            return;
        };

        if current_url == last_url {
            self.web_mark_synced(cx);
            return;
        }

        let current_depth = self.router.depth();
        let last_depth = self.web.last_depth;
        if current_depth != last_depth {
            if current_depth > last_depth {
                self.web_push_current_url(cx);
            } else {
                self.web_replace_current_url(cx);
            }
            return;
        }

        if self.web.last_child_parent_route == self.active_route {
            let current_child_depth = self.web_active_child_depth(cx);
            if let (Some(prev), Some(now)) = (self.web.last_child_depth, current_child_depth) {
                if prev != now {
                    let delta = now as i32 - prev as i32;
                    if delta > 0 {
                        self.web_push_current_url(cx);
                    } else {
                        self.web_go(cx, delta);
                    }
                    return;
                }
            }
        }

        self.web_replace_current_url(cx);
    }

    fn join_paths(base: &str, tail: &str) -> String {
        let base = base.trim();
        let tail = tail.trim();

        let base = if base.is_empty() { "/" } else { base };
        let base_trim = base.trim_end_matches('/');
        let tail_trim = tail.trim_start_matches('/');

        if tail_trim.is_empty() {
            if base_trim.is_empty() {
                "/".to_string()
            } else if base_trim == "/" {
                "/".to_string()
            } else {
                base_trim.to_string()
            }
        } else if base_trim.is_empty() || base_trim == "/" {
            format!("/{}", tail_trim)
        } else {
            format!("{}/{}", base_trim, tail_trim)
        }
    }

    fn current_path_for_route(&self, route: &crate::route::Route) -> String {
        // Keep unknown path in the address bar while showing the configured not-found route.
        if self.not_found_route.0 != 0 && route.id == self.not_found_route {
            if let Some(path) = &self.web.url_path_override {
                let mut p = path.trim().to_string();
                if p.is_empty() {
                    p = "/".to_string();
                } else if !p.starts_with('/') {
                    p.insert(0, '/');
                }
                return p;
            }
        }

        let pattern = route
            .pattern
            .as_ref()
            .or_else(|| self.router.route_registry.get_pattern(route.id));

        let base = if let Some(pattern) = pattern {
            pattern
                .format_path(&route.params)
                .unwrap_or_else(|| pattern.format_base_path(&route.params))
        } else {
            let s = route.id.to_string();
            if s.is_empty() {
                "/".to_string()
            } else {
                format!("/{}", s)
            }
        };

        let Some(child_router) = self.child_routers.get(&route.id) else {
            return base;
        };
        let Some(child) = child_router.borrow() else {
            return base;
        };
        let tail = child.current_path();
        Self::join_paths(&base, &tail)
    }

    fn current_path(&self) -> String {
        let Some(route) = self.router.current_route() else {
            return "/".to_string();
        };
        self.current_path_for_route(route)
    }

    pub fn current_url(&self) -> String {
        let Some(route) = self.router.current_route() else {
            return self.current_path();
        };
        format!(
            "{}{}{}",
            self.current_path_for_route(route),
            route.query_string(),
            route.hash
        )
    }

    pub(super) fn apply_initial_url_if_needed(&mut self, cx: &mut Cx) {
        if !self.web_enabled(cx) || !self.use_initial_url || self.web.history_initialized {
            return;
        }
        let OsType::Web(params) = cx.os_type() else {
            return;
        };
        let browser_url = format!("{}{}{}", &params.pathname, &params.search, &params.hash);

        self.web.suppress_browser_update = true;
        let _ = self.request_navigation_internal(
            cx,
            RouterNavRequest::ReplaceByUrl {
                url: browser_url.clone(),
            },
            true,
            0,
        );
        self.web.suppress_browser_update = false;

        self.web.history_initialized = true;
        self.web.history_index = 0;
        if self.pending_navigation.is_none() {
            self.web_replace_current_url(cx);
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub(super) fn handle_browser_url_changed(&mut self, cx: &mut Cx, url: &str, state_index: i32) {
        if !self.guard_bypass {
            let ok = self.request_navigation_internal(
                cx,
                RouterNavRequest::BrowserUrlChanged {
                    url: url.to_string(),
                    state_index,
                },
                true,
                0,
            );
            if !ok {
                self.web.ignore_next_browser_url_change = true;
                self.web_replace_current_url(cx);
            }
            return;
        }

        if !self.web_enabled(cx) {
            return;
        }
        if self.web.ignore_next_browser_url_change {
            self.web.ignore_next_browser_url_change = false;
            return;
        }

        if state_index >= 0 {
            self.web.history_initialized = true;
            self.web.history_index = state_index;
        } else {
            self.ensure_web_history_initialized(cx);
        }

        self.web.suppress_browser_update = true;
        let _ = self.replace_by_path_internal(cx, url, false);
        self.web.suppress_browser_update = false;
        self.web_mark_synced(cx);
        self.redraw(cx);
    }
}
