//! Shared-element ("hero") rendering hooks for route transitions.

use crate::hero::{HeroGlobals, HeroPhase};
use makepad_draw::draw_list_2d::DrawListExt;
use makepad_widgets::*;

use super::{RouterTransitionDirection, RouterTransitionPreset, RouterWidget};

impl RouterWidget {
    pub(super) fn draw_routes_with_hero(&mut self, cx: &mut Cx2d, scope: &mut Scope, rect: Rect) {
        let router_uid = self.widget_uid();
        let hero_enabled = self.hero_transition && self.transition_rt.state.is_some();

        if hero_enabled {
            cx.global::<HeroGlobals>().push_router(router_uid);
        }

        if let Some(state_snapshot) = self.transition_rt.state.clone() {
            if hero_enabled && !state_snapshot.hero.capture_done {
                cx.global::<HeroGlobals>().clear_capture();
                cx.global::<HeroGlobals>().set_hide_tags(&[]);

                self.draw_lists.hero_capture.begin_always(cx);
                let draw_list_id = self.draw_lists.hero_capture.id();
                {
                    let dl = &mut cx.cx.cx.draw_lists[draw_list_id];
                    dl.draw_list_uniforms.view_shift = vec2(0.0, 0.0);
                    dl.draw_list_uniforms.view_transform = Mat4f::identity();
                }

                cx.global::<HeroGlobals>().set_phase(HeroPhase::CaptureFrom);
                if let Some(widget) = self.routes.widgets.get_mut(&state_snapshot.from_route) {
                    let _ = widget.draw_walk(cx, scope, Walk::fill().with_abs_pos(rect.pos));
                }

                cx.global::<HeroGlobals>().set_phase(HeroPhase::CaptureTo);
                if let Some(widget) = self.routes.widgets.get_mut(&state_snapshot.to_route) {
                    let _ = widget.draw_walk(cx, scope, Walk::fill().with_abs_pos(rect.pos));
                }

                cx.global::<HeroGlobals>().set_phase(HeroPhase::Idle);
                self.draw_lists.hero_capture.end(cx);

                let hero_pairs = cx.global::<HeroGlobals>().take_pairs();
                if let Some(state) = self.transition_rt.state.as_mut() {
                    state.hero.pairs = hero_pairs;
                    state.hero.capture_done = true;
                }
            }

            let state = self.transition_rt.state.clone().unwrap_or(state_snapshot);

            let hide_tags: Vec<LiveId> = state.hero.pairs.iter().map(|p| p.tag).collect();
            let has_hero_pairs = hero_enabled && !hide_tags.is_empty();
            let route_preset = if has_hero_pairs {
                RouterTransitionPreset::Fade
            } else {
                state.preset
            };

            if has_hero_pairs {
                cx.global::<HeroGlobals>().set_hide_tags(&hide_tags);
                cx.global::<HeroGlobals>().set_phase(HeroPhase::VisibleFrom);
            } else {
                cx.global::<HeroGlobals>().set_hide_tags(&[]);
                cx.global::<HeroGlobals>().set_phase(HeroPhase::Idle);
            }

            let from_effect =
                Self::compute_effect(route_preset, state.direction, state.progress, false, rect);
            Self::draw_route_into_draw_list(
                cx,
                scope,
                &mut self.draw_lists.from,
                &mut self.routes.widgets,
                state.from_route,
                from_effect,
                true,
            );

            if has_hero_pairs {
                cx.global::<HeroGlobals>().set_phase(HeroPhase::VisibleTo);
            } else {
                cx.global::<HeroGlobals>().set_phase(HeroPhase::Idle);
            }

            let to_effect =
                Self::compute_effect(route_preset, state.direction, state.progress, true, rect);
            Self::draw_route_into_draw_list(
                cx,
                scope,
                &mut self.draw_lists.to,
                &mut self.routes.widgets,
                state.to_route,
                to_effect,
                true,
            );

            if has_hero_pairs {
                cx.global::<HeroGlobals>().set_hide_tags(&[]);
                cx.global::<HeroGlobals>().set_phase(HeroPhase::Overlay);

                let t = Self::ease_in_out(state.progress);

                let pairs = state.hero.pairs.clone();
                let lerp = |a: f64, b: f64| a + (b - a) * t;
                let lerp_rect = |a: Rect, b: Rect| Rect {
                    pos: dvec2(lerp(a.pos.x, b.pos.x), lerp(a.pos.y, b.pos.y)),
                    size: dvec2(lerp(a.size.x, b.size.x), lerp(a.size.y, b.size.y)),
                };

                self.draw_lists.hero_from.begin_always(cx);
                let draw_list_id = self.draw_lists.hero_from.id();
                {
                    let dl = &mut cx.cx.cx.draw_lists[draw_list_id];
                    dl.draw_list_uniforms.view_shift = vec2(0.0, 0.0);
                    dl.draw_list_uniforms.view_transform = Mat4f::identity();
                }
                for pair in &pairs {
                    let r = lerp_rect(pair.from_rect, pair.to_rect);
                    let hero = cx.widget_tree().widget(pair.from_uid);
                    let _ = hero.draw_walk(cx, scope, Walk::abs_rect(r));
                }
                self.draw_lists.hero_from.end(cx);

                self.draw_lists.hero_to.begin_always(cx);
                let draw_list_id = self.draw_lists.hero_to.id();
                {
                    let dl = &mut cx.cx.cx.draw_lists[draw_list_id];
                    dl.draw_list_uniforms.view_shift = vec2(0.0, 0.0);
                    dl.draw_list_uniforms.view_transform = Mat4f::identity();
                }
                for pair in &pairs {
                    let r = lerp_rect(pair.from_rect, pair.to_rect);
                    let hero = cx.widget_tree().widget(pair.to_uid);
                    let _ = hero.draw_walk(cx, scope, Walk::abs_rect(r));
                }
                self.draw_lists.hero_to.end(cx);

                cx.global::<HeroGlobals>().set_phase(HeroPhase::Idle);
            } else if hero_enabled {
                cx.global::<HeroGlobals>().set_phase(HeroPhase::Idle);
            }
        } else {
            let effect = Self::compute_effect(
                RouterTransitionPreset::None,
                RouterTransitionDirection::Forward,
                0.0,
                true,
                rect,
            );
            Self::draw_route_into_draw_list(
                cx,
                scope,
                &mut self.draw_lists.to,
                &mut self.routes.widgets,
                self.active_route,
                effect,
                false,
            );
        }

        if hero_enabled {
            cx.global::<HeroGlobals>().pop_router(router_uid);
        }
    }
}
