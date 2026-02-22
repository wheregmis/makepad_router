use makepad_router::RouterWidgetRef;
use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.HeroListPage = View{
        width: Fill
        height: Fill
        flow: Down
        spacing: 16
        padding: 32

        Label{text: "Hero Transition" draw_text.text_style.font_size: 32}
        Label{text: "Shared element tags match across routes (tag: hero_card)."}

        hero_card := mod.widgets.Hero{
            tag: @hero_card
            width: 96
            height: 96
            View{
                width: Fill
                height: Fill
                show_bg: true
                draw_bg.color: #xFFB000
            }
        }

        View{
            width: Fill
            height: Fit
            flow: Right
            spacing: 10
            detail_btn := Button{text: "Open Detail"}
            home_btn := Button{text: "Back to Home"}
        }
    }

    mod.widgets.HeroDetailPage = View{
        width: Fill
        height: Fill
        flow: Down
        spacing: 16
        padding: 32

        Label{text: "Hero Detail" draw_text.text_style.font_size: 32}

        hero_card := mod.widgets.Hero{
            tag: @hero_card
            width: 320
            height: 200
            View{
                width: Fill
                height: Fill
                show_bg: true
                draw_bg.color: #xFFB000
            }
        }

        back_btn := Button{text: "Back"}
    }
}

#[derive(Default)]
pub struct HeroListController;

impl HeroListController {
    pub fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, router: &RouterWidgetRef) {
        let Some((to_detail, to_home)) = router.with_active_route_widget(|w| {
            (
                w.button(cx, &[live_id!(detail_btn)]).clicked(actions),
                w.button(cx, &[live_id!(home_btn)]).clicked(actions),
            )
        }) else {
            return;
        };

        if to_detail {
            router.navigate(cx, live_id!(hero_detail));
        }
        if to_home {
            router.navigate(cx, live_id!(home));
        }
    }
}

#[derive(Default)]
pub struct HeroDetailController;

impl HeroDetailController {
    pub fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, router: &RouterWidgetRef) {
        let Some(to_back) =
            router.with_active_route_widget(|w| w.button(cx, &[live_id!(back_btn)]).clicked(actions))
        else {
            return;
        };
        if to_back {
            router.back(cx);
        }
    }
}
