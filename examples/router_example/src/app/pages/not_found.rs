use makepad_router::RouterWidgetRef;
use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    use link::theme_desktop_dark::*;

    pub NotFoundPage = <View> {
        width: Fill, height: Fill
        show_bg: true
        draw_bg: { color: #x4A0E4E }

        flow: Down, spacing: 20, padding: 40

        <Label> {
            text: "404 - Not Found"
            draw_text: { text_style: { font_size: 32 }, color: #xFFFFFF }
        }

        <Label> {
            text: "This is a wildcard catch-all route."
            draw_text: { text_style: { font_size: 16 }, color: #xAAAAAA }
        }

        home_btn = <Button> { text: "Back to Home" }
    }
}

#[derive(Default)]
pub struct NotFoundController;

impl NotFoundController {
    pub fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, router: &RouterWidgetRef) {
        let Some(to_home) =
            router.with_active_route_widget(|w| w.button(&[live_id!(home_btn)]).clicked(actions))
        else {
            return;
        };
        if to_home {
            router.replace(cx, live_id!(home));
        }
    }
}

