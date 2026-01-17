use crate::{pattern::{RouteParams, RoutePatternRef}, url::RouterUrl};
use makepad_widgets::*;

use super::RouterTransitionState;
use crate::route::Route;
use crate::guards::{RouterAsyncGuard, RouterBeforeLeaveAsync, RouterBeforeLeaveSync, RouterSyncGuard};

type RouteChangeCallback = Box<dyn Fn(&mut Cx, Option<Route>, Route) + Send + Sync>;

#[derive(Default)]
pub(crate) struct RouterCallbacks {
    pub(crate) route_change: Vec<RouteChangeCallback>,
}

#[derive(Default)]
pub(crate) struct RouterGuards {
    pub(crate) route_guards: Vec<RouterSyncGuard>,
    pub(crate) route_guards_async: Vec<RouterAsyncGuard>,
    pub(crate) before_leave_hooks: Vec<RouterBeforeLeaveSync>,
    pub(crate) before_leave_hooks_async: Vec<RouterBeforeLeaveAsync>,
}

#[derive(Default)]
pub(crate) struct WebUrlState {
    pub(crate) url_path_override: Option<String>,
    pub(crate) history_index: i32,
    pub(crate) history_initialized: bool,
    pub(crate) suppress_browser_update: bool,
    pub(crate) ignore_next_browser_url_change: bool,
    pub(crate) last_synced_url: Option<String>,
    pub(crate) last_depth: usize,
    pub(crate) last_child_depth: Option<usize>,
    pub(crate) last_child_parent_route: LiveId,
}

#[derive(Default)]
pub(crate) struct RouterCaches {
    pub(crate) route_registry_epoch: u64,
    pub(crate) nested_prefix_cache_epoch: u64,
    pub(crate) nested_prefix_cache_path: String,
    pub(crate) nested_prefix_cache_result: Option<(LiveId, RouteParams, RoutePatternRef, String)>,
    pub(crate) url_parse_cache: Vec<(String, RouterUrl)>,
}

#[derive(Default)]
pub(crate) struct PointerCleanup {
    pub(crate) route: Option<LiveId>,
    pub(crate) budget: u8,
}

pub(crate) struct RouterDrawLists {
    pub(crate) from: DrawList2d,
    pub(crate) to: DrawList2d,
    pub(crate) hero_capture: DrawList2d,
    pub(crate) hero_from: DrawList2d,
    pub(crate) hero_to: DrawList2d,
    pub(crate) inspector: DrawList2d,
}

impl RouterDrawLists {
    pub(crate) fn new(cx: &mut Cx) -> Self {
        Self {
            from: DrawList2d::new(cx),
            to: DrawList2d::new(cx),
            hero_capture: DrawList2d::new(cx),
            hero_from: DrawList2d::new(cx),
            hero_to: DrawList2d::new(cx),
            inspector: DrawList2d::new(cx),
        }
    }
}

#[derive(Default)]
pub(crate) struct TransitionRuntime {
    pub(crate) state: Option<RouterTransitionState>,
    pub(crate) next_frame: NextFrame,
}
