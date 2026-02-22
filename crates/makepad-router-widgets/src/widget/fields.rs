use crate::{
    pattern::{RouteParams, RoutePatternRef},
    url::RouterUrl,
};
use makepad_widgets::*;

use super::RouterTransitionState;
use crate::guards::{
    RouterAsyncGuard, RouterBeforeLeaveAsync, RouterBeforeLeaveSync, RouterSyncGuard,
};
use crate::route::Route;

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
pub(crate) struct RouterCaches {
    pub(crate) route_registry_epoch: u64,
    pub(crate) nested_prefix_cache_epoch: u64,
    pub(crate) nested_prefix_cache_path: String,
    pub(crate) nested_prefix_cache_result: Option<(LiveId, RouteParams, RoutePatternRef, String)>,
    pub(crate) url_parse_cache: Vec<(String, RouterUrl)>,
}

#[derive(Default)]
pub(crate) struct RouterRouteMaps {
    pub(crate) templates: ComponentMap<LiveId, ScriptObjectRef>,
    pub(crate) widgets: ComponentMap<LiveId, WidgetRef>,
    pub(crate) patterns: ComponentMap<LiveId, String>,
    pub(crate) transition_overrides: ComponentMap<LiveId, LiveId>,
    pub(crate) transition_duration_overrides: ComponentMap<LiveId, f64>,
}

#[derive(Default)]
pub(crate) struct PointerCleanup {
    pub(crate) route: Option<LiveId>,
    pub(crate) budget: u8,
}

pub(crate) struct RouterDrawLists {
    pub(crate) from: DrawList2d,
    pub(crate) to: DrawList2d,
    pub(crate) inspector: DrawList2d,
}

impl RouterDrawLists {
    pub(crate) fn new(cx: &mut Cx) -> Self {
        Self {
            from: DrawList2d::new(cx),
            to: DrawList2d::new(cx),
            inspector: DrawList2d::new(cx),
        }
    }
}

impl ScriptNew for RouterDrawLists {
    fn script_new(vm: &mut ScriptVm) -> Self {
        Self::new(vm.cx_mut())
    }
}

impl ScriptApply for RouterDrawLists {}
impl ScriptHook for RouterDrawLists {}

#[derive(Default)]
pub(crate) struct TransitionRuntime {
    pub(crate) state: Option<RouterTransitionState>,
    pub(crate) next_frame: NextFrame,
}
