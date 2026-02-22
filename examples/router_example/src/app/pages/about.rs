use makepad_router::{RouterCommand, RouterWidgetRef};
use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*

    mod.widgets.AboutPage = View{
        width: Fill
        height: Fill
        flow: Down
        spacing: 16
        padding: 32

        Label{text: "About" draw_text.text_style.font_size: 32}
        Label{text: "This route is behind an async guard (native adds a small delay)."}
        Label{text: "Router demo app"}

        home_btn := Button{text: "Back to Home"}
    }
}

#[derive(Default)]
pub struct AboutController;

impl AboutController {
    pub fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, router: &RouterWidgetRef) {
        let Some(to_home) =
            router.with_active_route_widget(|w| w.button(cx, &[live_id!(home_btn)]).clicked(actions))
        else {
            return;
        };
        if to_home {
            let _ = router.dispatch(
                cx,
                RouterCommand::GoToRoute {
                    route_id: live_id!(home),
                    transition: None,
                },
            );
        }
    }
}
