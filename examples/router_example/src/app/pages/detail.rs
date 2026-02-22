use makepad_router::{RouterCommand, RouterWidgetRef};
use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*

    mod.widgets.DetailPage = View{
        width: Fill
        height: Fill
        flow: Down
        spacing: 16
        padding: 32

        Label{text: "Detail" draw_text.text_style.font_size: 30}

        detail_id_label := Label{text: "ID: (dynamic)"}

        View{
            width: Fill
            height: Fit
            flow: Right
            spacing: 10
            id_1_btn := Button{text: "Go /detail/1"}
            id_42_btn := Button{text: "Go /detail/42"}
        }

        home_btn := Button{text: "Back to Home"}
    }
}

#[derive(Default)]
pub struct DetailController {
    last_id: Option<String>,
}

impl DetailController {
    pub fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, router: &RouterWidgetRef) {
        if let Some(id) = router.get_param_string("id") {
            if self.last_id.as_ref() != Some(&id) {
                router.bind_param_to_label(cx, "id", live_id!(detail_id_label), |id| {
                    format!("ID: {}", id)
                });
                self.last_id = Some(id);
            }
        }

        let Some((to_id_1, to_id_42, to_home)) = router.with_active_route_widget(|w| {
            (
                w.button(cx, &[live_id!(id_1_btn)]).clicked(actions),
                w.button(cx, &[live_id!(id_42_btn)]).clicked(actions),
                w.button(cx, &[live_id!(home_btn)]).clicked(actions),
            )
        }) else {
            return;
        };

        if to_id_1 {
            let _ = router.dispatch(
                cx,
                RouterCommand::GoToPath {
                    path: "/detail/1".to_string(),
                },
            );
        }
        if to_id_42 {
            let _ = router.dispatch(
                cx,
                RouterCommand::GoToPath {
                    path: "/detail/42".to_string(),
                },
            );
        }
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
