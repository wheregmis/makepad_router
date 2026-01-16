use crate::route::Route;
use makepad_live_id::LiveId;
use makepad_widgets::{Cx, ToUIReceiver};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RouterNavKind {
    Navigate,
    Replace,
    NavigateByPath,
    ReplaceByPath,
    NavigateByUrl,
    ReplaceByUrl,
    Back,
    Forward,
    Reset,
    SetStack,
    Push,
    Pop,
    PopTo,
    PopToRoot,
    BrowserUrlChanged,
}

#[derive(Clone, Debug)]
pub struct RouterNavContext {
    pub kind: RouterNavKind,
    pub from: Option<Route>,
    pub to: Option<Route>,
    pub to_path: Option<String>,
    pub to_url: Option<String>,
}

#[derive(Clone, Debug)]
pub enum RouterRedirectTarget {
    Route(LiveId),
    Path(String),
    Url(String),
}

#[derive(Clone, Debug)]
pub struct RouterRedirect {
    pub target: RouterRedirectTarget,
    pub replace: bool,
}

#[derive(Clone, Debug)]
pub enum RouterGuardDecision {
    Allow,
    Block,
    Redirect(RouterRedirect),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RouterBeforeLeaveDecision {
    Allow,
    Block,
}

pub enum RouterAsyncDecision<T> {
    Immediate(T),
    Pending(ToUIReceiver<T>),
}

pub type RouterSyncGuard =
    Box<dyn Fn(&mut Cx, &RouterNavContext) -> RouterGuardDecision + Send + Sync>;
pub type RouterAsyncGuard =
    Box<dyn Fn(&mut Cx, &RouterNavContext) -> RouterAsyncDecision<RouterGuardDecision> + Send + Sync>;

pub type RouterBeforeLeaveSync =
    Box<dyn Fn(&mut Cx, &RouterNavContext) -> RouterBeforeLeaveDecision + Send + Sync>;
pub type RouterBeforeLeaveAsync = Box<
    dyn Fn(&mut Cx, &RouterNavContext) -> RouterAsyncDecision<RouterBeforeLeaveDecision> + Send + Sync,
>;

