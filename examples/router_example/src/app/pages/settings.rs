use makepad_router::RouterWidgetRef;
use makepad_widgets::*;

use crate::app::shared::SharedState;

live_design! {
    use link::widgets::*;
    use link::theme_desktop_dark::*;

    pub SettingsPage = <View> {
        width: Fill, height: Fill
        show_bg: true
        draw_bg: { color: #x0F3460 }

        flow: Down, spacing: 20, padding: 40

        <Label> {
            text: "Settings"
            draw_text: { text_style: { font_size: 32 }, color: #xFFFFFF }
        }

        <Label> {
            text: "This page toggles auth + dirty state to exercise guards and before-leave hooks."
            draw_text: { text_style: { font_size: 14 }, color: #xAAAAAA }
        }

        auth_status_label = <Label> {
            text: "Auth: (unknown)"
            draw_text: { text_style: { font_size: 16 }, color: #xFFFFFF }
        }

        dirty_status_label = <Label> {
            text: "Dirty: (unknown)"
            draw_text: { text_style: { font_size: 16 }, color: #xFFFFFF }
        }

        <View> {
            width: Fill, height: Fit
            flow: Right, spacing: 10
            login_toggle_btn = <Button> { text: "Toggle Login" }
            dirty_toggle_btn = <Button> { text: "Toggle Dirty" }
        }

        <View> {
            width: Fill, height: Fit
            flow: Right, spacing: 10
            go_admin_btn = <Button> { text: "Go to /admin/dashboard (guarded)" }
            go_stack_demo_btn = <Button> { text: "Open stack demo" }
        }

        home_btn = <Button> { text: "Back to Home" }
    }
}

#[derive(Default)]
pub struct SettingsController;

impl SettingsController {
    pub fn handle_actions(
        &mut self,
        cx: &mut Cx,
        actions: &Actions,
        router: &RouterWidgetRef,
        shared: &SharedState,
    ) {
        let Some((to_home, toggle_login, toggle_dirty, to_admin, to_stack)) =
            router.with_active_route_widget(|w| {
                (
                    w.button(&[live_id!(home_btn)]).clicked(actions),
                    w.button(&[live_id!(login_toggle_btn)]).clicked(actions),
                    w.button(&[live_id!(dirty_toggle_btn)]).clicked(actions),
                    w.button(&[live_id!(go_admin_btn)]).clicked(actions),
                    w.button(&[live_id!(go_stack_demo_btn)]).clicked(actions),
                )
            })
        else {
            return;
        };

        if toggle_login {
            shared.set_logged_in(!shared.is_logged_in());
        }
        if toggle_dirty {
            shared.set_dirty(!shared.is_dirty());
        }

        router.with_active_route_widget(|w| {
            w.label(&[live_id!(auth_status_label)]).set_text(
                cx,
                if shared.is_logged_in() {
                    "Auth: logged in"
                } else {
                    "Auth: logged out (admin is guarded)"
                },
            );
            w.label(&[live_id!(dirty_status_label)]).set_text(
                cx,
                if shared.is_dirty() {
                    "Dirty: true (before-leave blocks leaving Settings)"
                } else {
                    "Dirty: false"
                },
            );
        });

        if to_admin {
            router.navigate_by_path(cx, "/admin/dashboard");
        }
        if to_stack {
            router.navigate(cx, live_id!(stack_demo));
        }
        if to_home {
            router.navigate(cx, live_id!(home));
        }
    }
}

