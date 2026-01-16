use makepad_router::RouterWidgetRef;
use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    use link::theme_desktop_dark::*;

    pub UserProfilePage = <View> {
        width: Fill, height: Fill
        show_bg: true
        draw_bg: { color: #x2D5016 }

        flow: Down, spacing: 20, padding: 40

        <Label> { text: "User Profile" draw_text: { text_style: { font_size: 32 }, color: #xFFFFFF } }

        user_id_label = <Label> {
            text: "User ID: (dynamic)"
            draw_text: { text_style: { font_size: 18 }, color: #xAAAAAA }
        }

        tab_label = <Label> {
            text: "Query tab: (none)"
            draw_text: { text_style: { font_size: 16 }, color: #xAAAAAA }
        }

        <View> {
            width: Fill, height: Fit
            flow: Right, spacing: 10
            tab_posts_btn = <Button> { text: "tab=posts" }
            tab_likes_btn = <Button> { text: "tab=likes" }
        }

        home_btn = <Button> { text: "Back to Home" }
    }
}

#[derive(Default)]
pub struct UserProfileController {
    last_user_id: Option<String>,
    last_user_tab: Option<String>,
}

impl UserProfileController {
    pub fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, router: &RouterWidgetRef) {
        if let Some(user_id) = router.get_param_string("id") {
            if self.last_user_id.as_ref() != Some(&user_id) {
                router.bind_param_to_label(cx, "id", live_id!(user_id_label), |id| {
                    format!("User ID: {}", id)
                });
                self.last_user_id = Some(user_id);
            }
        }

        let tab = router
            .get_query_string("tab")
            .unwrap_or_else(|| "(none)".to_string());
        if self.last_user_tab.as_ref() != Some(&tab) {
            router.with_active_route_widget(|w| {
                w.label(&[live_id!(tab_label)])
                    .set_text(cx, &format!("Query tab: {}", tab));
            });
            self.last_user_tab = Some(tab);
        }

        let Some((tab_posts, tab_likes, to_home)) = router.with_active_route_widget(|w| {
            (
                w.button(&[live_id!(tab_posts_btn)]).clicked(actions),
                w.button(&[live_id!(tab_likes_btn)]).clicked(actions),
                w.button(&[live_id!(home_btn)]).clicked(actions),
            )
        }) else {
            return;
        };

        if tab_posts {
            router.navigate_by_path(cx, "/user/12345?tab=posts");
        }
        if tab_likes {
            router.navigate_by_path(cx, "/user/12345?tab=likes");
        }
        if to_home {
            router.navigate(cx, live_id!(home));
        }
    }
}
