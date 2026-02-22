use makepad_router::{RouterWidgetRef, RouterWidgetWidgetRefExt};
use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.SettingsOverviewPage = View{
        width: Fill
        height: Fill
        flow: Down
        spacing: 8
        padding: 16

        Label{text: "Overview" draw_text.text_style.font_size: 18}
        Label{text: "Nested route: /settings/overview"}
    }

    mod.widgets.SettingsProfilePage = View{
        width: Fill
        height: Fill
        flow: Down
        spacing: 8
        padding: 16

        Label{text: "Profile" draw_text.text_style.font_size: 18}
        Label{text: "Nested route: /settings/profile"}
    }

    mod.widgets.SettingsPage = View{
        width: Fill
        height: Fill
        flow: Down
        spacing: 16
        padding: 32

        Label{text: "Settings" draw_text.text_style.font_size: 30}
        Label{text: "This route owns a nested RouterWidget."}

        View{
            width: Fill
            height: Fit
            flow: Right
            spacing: 10
            overview_btn := Button{text: "Overview"}
            profile_btn := Button{text: "Profile"}
        }

        settings_router := mod.widgets.RouterWidget{
            width: Fill
            height: 220
            default_route: @settings_overview
            not_found_route: @settings_overview
            push_transition: @SlideLeft
            pop_transition: @SlideRight
            transition_duration: 0.20

            settings_overview := mod.widgets.RouterRoute{
                route_pattern: "/overview"
                mod.widgets.SettingsOverviewPage{}
            }
            settings_profile := mod.widgets.RouterRoute{
                route_pattern: "/profile"
                mod.widgets.SettingsProfilePage{}
            }
        }

        home_btn := Button{text: "Back to Home"}
    }
}

#[derive(Default)]
pub struct SettingsController;

impl SettingsController {
    pub fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, router: &RouterWidgetRef) {
        let Some((to_home, to_overview, to_profile, settings_router)) =
            router.with_active_route_widget(|w| {
                (
                    w.button(cx, &[live_id!(home_btn)]).clicked(actions),
                    w.button(cx, &[live_id!(overview_btn)]).clicked(actions),
                    w.button(cx, &[live_id!(profile_btn)]).clicked(actions),
                    w.widget(cx, &[live_id!(settings_router)]).as_router_widget(),
                )
            })
        else {
            return;
        };

        if !settings_router.is_empty() {
            if to_overview {
                settings_router.navigate_by_path(cx, "/overview");
            }
            if to_profile {
                settings_router.navigate_by_path(cx, "/profile");
            }
        }

        if to_home {
            router.navigate(cx, live_id!(home));
        }
    }
}
