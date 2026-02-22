use makepad_router::{Route, RouterCommand, RouterWidgetRef};
use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*

    mod.widgets.StackDemoPage = View{
        width: Fill
        height: Fill
        flow: Down
        spacing: 16
        padding: 32

        Label{text: "History / Stack Demo" draw_text.text_style.font_size: 32}
        Label{text: "Exercises stack APIs: set_stack, pop_to, pop_to_root, replace."}

        View{
            width: Fill
            height: Fit
            flow: Right
            spacing: 10
            set_stack_btn := Button{text: "Set stack: Home > Settings > About"}
            pop_to_settings_btn := Button{text: "Pop to Settings"}
            pop_to_root_btn := Button{text: "Pop to Root"}
        }

        View{
            width: Fill
            height: Fit
            flow: Right
            spacing: 10
            replace_about_btn := Button{text: "Replace -> About"}
            home_btn := Button{text: "Back to Home"}
        }
    }
}

#[derive(Default)]
pub struct StackDemoController;

impl StackDemoController {
    pub fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, router: &RouterWidgetRef) {
        let Some((set_stack, pop_to_settings, pop_to_root, replace_about, to_home)) =
            router.with_active_route_widget(|w| {
                (
                    w.button(cx, &[live_id!(set_stack_btn)]).clicked(actions),
                    w.button(cx, &[live_id!(pop_to_settings_btn)]).clicked(actions),
                    w.button(cx, &[live_id!(pop_to_root_btn)]).clicked(actions),
                    w.button(cx, &[live_id!(replace_about_btn)]).clicked(actions),
                    w.button(cx, &[live_id!(home_btn)]).clicked(actions),
                )
            })
        else {
            return;
        };

        if set_stack {
            let _ = router.dispatch(
                cx,
                RouterCommand::SetStack {
                    stack: vec![
                        Route::new(live_id!(home)),
                        Route::new(live_id!(settings)),
                        Route::new(live_id!(about)),
                    ],
                },
            );
        }
        if pop_to_settings {
            let _ = router.dispatch(
                cx,
                RouterCommand::PopTo {
                    route_id: live_id!(settings),
                },
            );
        }
        if pop_to_root {
            let _ = router.dispatch(cx, RouterCommand::PopToRoot);
        }
        if replace_about {
            let _ = router.dispatch(
                cx,
                RouterCommand::ReplaceRoute {
                    route_id: live_id!(about),
                    transition: None,
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
