use makepad_widgets::*;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Shared-element ("hero") widget and state tracking.

pub(crate) enum HeroPhase {
    Idle,
    CaptureFrom,
    CaptureTo,
    VisibleFrom,
    VisibleTo,
    Overlay,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct HeroEntry {
    pub uid: WidgetUid,
    pub rect: Rect,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct HeroPair {
    pub tag: LiveId,
    pub from_uid: WidgetUid,
    pub to_uid: WidgetUid,
    pub from_rect: Rect,
    pub to_rect: Rect,
}

#[derive(Default)]
pub(crate) struct HeroGlobals {
    stack: Vec<HeroRouterContext>,
}

struct HeroRouterContext {
    router_uid: WidgetUid,
    phase: HeroPhase,
    hide_tags: HashSet<LiveId>,
    from: HashMap<LiveId, HeroEntry>,
    to: HashMap<LiveId, HeroEntry>,
}

impl HeroGlobals {
    pub(crate) fn push_router(&mut self, router_uid: WidgetUid) {
        self.stack.push(HeroRouterContext {
            router_uid,
            phase: HeroPhase::Idle,
            hide_tags: HashSet::new(),
            from: HashMap::new(),
            to: HashMap::new(),
        });
    }

    pub(crate) fn pop_router(&mut self, router_uid: WidgetUid) {
        if self
            .stack
            .last()
            .map(|ctx| ctx.router_uid == router_uid)
            .unwrap_or(false)
        {
            self.stack.pop();
        }
    }

    pub(crate) fn set_phase(&mut self, phase: HeroPhase) {
        if let Some(ctx) = self.stack.last_mut() {
            ctx.phase = phase;
        }
    }

    pub(crate) fn set_hide_tags(&mut self, tags: &[LiveId]) {
        if let Some(ctx) = self.stack.last_mut() {
            ctx.hide_tags.clear();
            ctx.hide_tags.extend(tags.iter().copied());
        }
    }

    pub(crate) fn clear_capture(&mut self) {
        if let Some(ctx) = self.stack.last_mut() {
            ctx.from.clear();
            ctx.to.clear();
        }
    }

    pub(crate) fn phase_and_hide(&self) -> Option<(HeroPhase, &HashSet<LiveId>)> {
        let ctx = self.stack.last()?;
        Some((ctx.phase, &ctx.hide_tags))
    }

    pub(crate) fn capture(&mut self, tag: LiveId, entry: HeroEntry) {
        if let Some(ctx) = self.stack.last_mut() {
            match ctx.phase {
                HeroPhase::CaptureFrom => {
                    ctx.from.insert(tag, entry);
                }
                HeroPhase::CaptureTo => {
                    ctx.to.insert(tag, entry);
                }
                _ => {}
            }
        }
    }

    pub(crate) fn take_pairs(&mut self) -> Vec<HeroPair> {
        let Some(ctx) = self.stack.last_mut() else {
            return Vec::new();
        };
        let mut pairs = Vec::new();
        for (tag, from) in ctx.from.iter() {
            if let Some(to) = ctx.to.get(tag) {
                pairs.push(HeroPair {
                    tag: *tag,
                    from_uid: from.uid,
                    to_uid: to.uid,
                    from_rect: from.rect,
                    to_rect: to.rect,
                });
            }
        }
        pairs
    }
}

#[derive(Script, ScriptHook, Widget)]
pub struct Hero {
    #[deref]
    view: View,
    /// Tag used to match heroes between routes during transitions.
    #[live]
    tag: LiveId,
}

impl Widget for Hero {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let tag = self.tag;
        if tag.0 == 0 {
            return self.view.draw_walk(cx, scope, walk);
        }

        let (phase, should_hide) = {
            let globals = cx.global::<HeroGlobals>();
            if let Some((phase, hide_tags)) = globals.phase_and_hide() {
                (phase, hide_tags.contains(&tag))
            } else {
                (HeroPhase::Idle, false)
            }
        };

        let should_hide = matches!(phase, HeroPhase::VisibleFrom | HeroPhase::VisibleTo) && should_hide;
        let capture_rect = if matches!(phase, HeroPhase::CaptureFrom | HeroPhase::CaptureTo) {
            Some(cx.peek_walk_turtle(walk))
        } else {
            None
        };

        let step = if should_hide {
            cx.walk_turtle(walk);
            DrawStep::done()
        } else {
            self.view.draw_walk(cx, scope, walk)
        };

        if let Some(rect) = capture_rect {
            cx.global::<HeroGlobals>().capture(tag, HeroEntry {
                uid: self.widget_uid(),
                rect,
            });
        }

        step
    }
}
