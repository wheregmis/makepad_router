use makepad_router::RouterWidgetRef;
use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*

    mod.widgets.HomePage = View{
        width: Fill
        height: Fill
        flow: Down
        spacing: 16
        padding: 32

        Label{
            text: "Router Example"
            draw_text.text_style.font_size: 32
        }

        Label{text: "A clean demo of nested routes, params, and not-found handling."}

        View{
            width: Fill
            height: Fit
            flow: Right
            spacing: 10

            settings_btn := Button{text: "Open Settings"}
            detail_btn := Button{text: "Go to Detail"}
        }
    }
}

#[derive(Default)]
pub struct HomeController;

impl HomeController {
    pub fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, router: &RouterWidgetRef) {
        let Some((to_settings, to_detail)) = router.with_active_route_widget(|w| {
            (
                w.button(cx, &[live_id!(settings_btn)]).clicked(actions),
                w.button(cx, &[live_id!(detail_btn)]).clicked(actions),
            )
        }) else {
            return;
        };

        if to_settings {
            router.navigate(cx, live_id!(settings));
        }
        if to_detail {
            router.navigate_by_path(cx, "/detail/42");
        }
    }
}
