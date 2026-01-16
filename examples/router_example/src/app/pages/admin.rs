use makepad_router::RouterWidgetRef;
use makepad_router::RouterWidgetWidgetRefExt;
use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    use link::theme_desktop_dark::*;
    use makepad_router::widget::*;

    AdminUsersPage = <View> {
        width: Fill, height: Fill
        show_bg: true
        draw_bg: { color: #x2A3A4E }
        flow: Down, spacing: 20, padding: 30

        <Label> { text: "Admin - Users" draw_text: { text_style: { font_size: 26 }, color: #xFFFFFF } }
        <Label> { text: "Nested router route: /admin/dashboard" draw_text: { text_style: { font_size: 14 }, color: #xAAAAAA } }

        <View> {
            width: Fill, height: Fit
            flow: Right, spacing: 10
            admin_settings_btn = <Button> { text: "Admin Settings" }
            admin_user_42_btn = <Button> { text: "User 42 (nested param)" }
        }
    }

    AdminSettingsPage = <View> {
        width: Fill, height: Fill
        show_bg: true
        draw_bg: { color: #x3A4A5E }
        flow: Down, spacing: 20, padding: 30

        <Label> { text: "Admin - Settings" draw_text: { text_style: { font_size: 26 }, color: #xFFFFFF } }
        <Label> { text: "Nested route: /admin/settings" draw_text: { text_style: { font_size: 14 }, color: #xAAAAAA } }

        admin_users_btn = <Button> { text: "Back to Users" }
    }

    AdminUserDetailPage = <View> {
        width: Fill, height: Fill
        show_bg: true
        draw_bg: { color: #x23324A }
        flow: Down, spacing: 20, padding: 30

        <Label> { text: "Admin - User Detail" draw_text: { text_style: { font_size: 26 }, color: #xFFFFFF } }
        admin_user_id_label = <Label> { text: "User ID: (dynamic)" draw_text: { text_style: { font_size: 16 }, color: #xAAAAAA } }
        back_btn = <Button> { text: "Back" }
    }

    pub AdminDashboard = <View> {
        width: Fill, height: Fill
        show_bg: true
        draw_bg: { color: #x1A1A3E }
        flow: Down, spacing: 16, padding: 40

        <Label> { text: "Admin" draw_text: { text_style: { font_size: 32 }, color: #xFFFFFF } }
        <Label> { text: "This route owns a nested RouterWidget." draw_text: { text_style: { font_size: 14 }, color: #xAAAAAA } }

        admin_router = <RouterWidget> {
            width: Fill, height: Fill
            default_route: admin_users
            not_found_route: admin_users
            push_transition: SharedAxis
            pop_transition: SharedAxis
            replace_transition: Fade
            transition_duration: 0.25

            admin_users = <AdminUsersPage> { route_pattern: "/dashboard" }
            admin_settings = <AdminSettingsPage> { route_pattern: "/settings" }
            admin_user_detail = <AdminUserDetailPage> { route_pattern: "/users/:id" }
        }

        home_btn = <Button> { text: "Back to Home" }
    }
}

#[derive(Default)]
pub struct AdminController {
    last_detail_id: Option<String>,
}

impl AdminController {
    pub fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, router: &RouterWidgetRef) {
        let Some((to_home, admin_router)) = router.with_active_route_widget(|w| {
            let to_home = w.button(&[live_id!(home_btn)]).clicked(actions);
            let admin_router = w.widget(&[live_id!(admin_router)]).as_router_widget();
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
            router.navigate(cx, live_id!(home));
        }
    }

    fn handle_nested(&mut self, cx: &mut Cx, actions: &Actions, admin_router: &RouterWidgetRef) {
        match admin_router.current_route_id() {
            Some(id) if id == live_id!(admin_users) => {
                let Some((to_settings, to_user_42)) = admin_router.with_active_route_widget(|w| {
                    (
                        w.button(&[live_id!(admin_settings_btn)]).clicked(actions),
                        w.button(&[live_id!(admin_user_42_btn)]).clicked(actions),
                    )
                }) else {
                    return;
                };
                if to_settings {
                    admin_router.navigate_by_path(cx, "/settings");
                }
                if to_user_42 {
                    admin_router.navigate_by_path(cx, "/users/42");
                }
            }
            Some(id) if id == live_id!(admin_settings) => {
                let Some(to_users) = admin_router.with_active_route_widget(|w| {
                    w.button(&[live_id!(admin_users_btn)]).clicked(actions)
                }) else {
                    return;
                };
                if to_users {
                    admin_router.navigate_by_path(cx, "/dashboard");
                }
            }
            Some(id) if id == live_id!(admin_user_detail) => {
                if let Some(user_id) = admin_router.get_param_string("id") {
                    if self.last_detail_id.as_ref() != Some(&user_id) {
                        admin_router.with_active_route_widget(|w| {
                            w.label(&[live_id!(admin_user_id_label)])
                                .set_text(cx, &format!("User ID: {}", user_id));
                        });
                        self.last_detail_id = Some(user_id);
                    }
                }

                let Some(to_back) =
                    admin_router.with_active_route_widget(|w| w.button(&[live_id!(back_btn)]).clicked(actions))
                else {
                    return;
                };
                if to_back {
                    admin_router.back(cx);
                }
            }
            _ => {}
        }
    }
}
