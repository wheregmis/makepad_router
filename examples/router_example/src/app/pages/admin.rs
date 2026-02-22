use makepad_router::{RouterCommand, RouterWidgetRef, RouterWidgetWidgetRefExt};
use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.AdminUsersPage = View{
        width: Fill
        height: Fill
        flow: Down
        spacing: 16
        padding: 24

        Label{text: "Admin - Users" draw_text.text_style.font_size: 26}
        Label{text: "Nested router route: /admin/dashboard"}

        View{
            width: Fill
            height: Fit
            flow: Right
            spacing: 10
            admin_settings_btn := Button{text: "Admin Settings"}
            admin_user_42_btn := Button{text: "User 42 (nested param)"}
        }
    }

    mod.widgets.AdminSettingsPage = View{
        width: Fill
        height: Fill
        flow: Down
        spacing: 16
        padding: 24

        Label{text: "Admin - Settings" draw_text.text_style.font_size: 26}
        Label{text: "Nested route: /admin/settings"}

        admin_users_btn := Button{text: "Back to Users"}
    }

    mod.widgets.AdminUserDetailPage = View{
        width: Fill
        height: Fill
        flow: Down
        spacing: 16
        padding: 24

        Label{text: "Admin - User Detail" draw_text.text_style.font_size: 26}
        admin_user_id_label := Label{text: "User ID: (dynamic)"}
        back_btn := Button{text: "Back"}
    }

    mod.widgets.AdminDashboard = View{
        width: Fill
        height: Fill
        flow: Down
        spacing: 16
        padding: 24

        Label{text: "Admin" draw_text.text_style.font_size: 32}
        Label{text: "This route owns a nested RouterWidget."}

        admin_router := mod.widgets.RouterWidget{
            width: Fill
            height: Fill
            default_route: @admin_users
            not_found_route: @admin_users
            push_transition: @SharedAxis
            pop_transition: @SharedAxis
            replace_transition: @Fade
            transition_duration: 0.25
            cap_transitions: true
            cap_nested: true

            admin_users := mod.widgets.RouterRoute{
                route_pattern: "/dashboard"
                mod.widgets.AdminUsersPage{}
            }
            admin_settings := mod.widgets.RouterRoute{
                route_pattern: "/settings"
                mod.widgets.AdminSettingsPage{}
            }
            admin_user_detail := mod.widgets.RouterRoute{
                route_pattern: "/users/:id"
                mod.widgets.AdminUserDetailPage{}
            }
        }

        home_btn := Button{text: "Back to Home"}
    }
}

#[derive(Default)]
pub struct AdminController {
    last_detail_id: Option<String>,
}

impl AdminController {
    pub fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, router: &RouterWidgetRef) {
        let Some((to_home, admin_router)) = router.with_active_route_widget(|w| {
            let to_home = w.button(cx, &[live_id!(home_btn)]).clicked(actions);
            let admin_router = w.widget(cx, &[live_id!(admin_router)]).as_router_widget();
            (to_home, admin_router)
        }) else {
            return;
        };

        if !admin_router.is_empty() {
            self.handle_nested(cx, actions, &admin_router);
        } else {
            self.last_detail_id = None;
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

    fn handle_nested(&mut self, cx: &mut Cx, actions: &Actions, admin_router: &RouterWidgetRef) {
        match admin_router.current_route_id() {
            Some(id) if id == live_id!(admin_users) => {
                let Some((to_settings, to_user_42)) = admin_router.with_active_route_widget(|w| {
                    (
                        w.button(cx, &[live_id!(admin_settings_btn)]).clicked(actions),
                        w.button(cx, &[live_id!(admin_user_42_btn)]).clicked(actions),
                    )
                }) else {
                    return;
                };
                if to_settings {
                    let _ = admin_router.dispatch(
                        cx,
                        RouterCommand::GoToPath {
                            path: "/settings".to_string(),
                        },
                    );
                }
                if to_user_42 {
                    let _ = admin_router.dispatch(
                        cx,
                        RouterCommand::GoToPath {
                            path: "/users/42".to_string(),
                        },
                    );
                }
            }
            Some(id) if id == live_id!(admin_settings) => {
                let Some(to_users) = admin_router.with_active_route_widget(|w| {
                    w.button(cx, &[live_id!(admin_users_btn)]).clicked(actions)
                }) else {
                    return;
                };
                if to_users {
                    let _ = admin_router.dispatch(
                        cx,
                        RouterCommand::GoToPath {
                            path: "/dashboard".to_string(),
                        },
                    );
                }
            }
            Some(id) if id == live_id!(admin_user_detail) => {
                if let Some(user_id) = admin_router.get_param_string("id") {
                    if self.last_detail_id.as_ref() != Some(&user_id) {
                        admin_router.with_active_route_widget(|w| {
                            w.label(cx, &[live_id!(admin_user_id_label)])
                                .set_text(cx, &format!("User ID: {}", user_id));
                        });
                        self.last_detail_id = Some(user_id);
                    }
                }

                let Some(to_back) = admin_router.with_active_route_widget(|w| {
                    w.button(cx, &[live_id!(back_btn)]).clicked(actions)
                }) else {
                    return;
                };
                if to_back {
                    let _ = admin_router.dispatch(cx, RouterCommand::Back { transition: None });
                }
            }
            _ => {}
        }
    }
}
