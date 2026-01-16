use crate::guards::{
    RouterAsyncDecision, RouterAsyncGuard, RouterBeforeLeaveAsync, RouterBeforeLeaveDecision,
    RouterBeforeLeaveSync, RouterGuardDecision, RouterNavContext, RouterSyncGuard,
};
use makepad_widgets::Cx;

use super::RouterWidget;

impl RouterWidget {
    pub fn add_route_guard<F>(&mut self, guard: F)
    where
        F: Fn(&mut Cx, &RouterNavContext) -> RouterGuardDecision + Send + Sync + 'static,
    {
        self.guards.route_guards.push(Box::new(guard));
    }

    pub fn add_route_guard_async<F>(&mut self, guard: F)
    where
        F: Fn(&mut Cx, &RouterNavContext) -> RouterAsyncDecision<RouterGuardDecision>
            + Send
            + Sync
            + 'static,
    {
        self.guards.route_guards_async.push(Box::new(guard));
    }

    pub fn add_before_leave_hook<F>(&mut self, hook: F)
    where
        F: Fn(&mut Cx, &RouterNavContext) -> RouterBeforeLeaveDecision + Send + Sync + 'static,
    {
        self.guards.before_leave_hooks.push(Box::new(hook));
    }

    pub fn add_before_leave_hook_async<F>(&mut self, hook: F)
    where
        F: Fn(&mut Cx, &RouterNavContext) -> RouterAsyncDecision<RouterBeforeLeaveDecision>
            + Send
            + Sync
            + 'static,
    {
        self.guards.before_leave_hooks_async.push(Box::new(hook));
    }

    pub(super) fn has_async_before_leave_hooks(&self) -> bool {
        !self.guards.before_leave_hooks_async.is_empty()
    }

    pub(super) fn has_async_route_guards(&self) -> bool {
        !self.guards.route_guards_async.is_empty()
    }

    pub(super) fn before_leave_hooks(&self) -> &[RouterBeforeLeaveSync] {
        &self.guards.before_leave_hooks
    }

    pub(super) fn before_leave_hooks_async(&self) -> &[RouterBeforeLeaveAsync] {
        &self.guards.before_leave_hooks_async
    }

    pub(super) fn route_guards(&self) -> &[RouterSyncGuard] {
        &self.guards.route_guards
    }

    pub(super) fn route_guards_async(&self) -> &[RouterAsyncGuard] {
        &self.guards.route_guards_async
    }
}

