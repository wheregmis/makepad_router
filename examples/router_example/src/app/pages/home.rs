use makepad_widgets::*;
use makepad_router::RouterWidgetRef;

live_design! {
    use link::widgets::*;
    use link::theme_desktop_dark::*;

    pub HomePage = <View> {
        width: Fill, height: Fill
        show_bg: true
        draw_bg: { color: #x16213E }

        flow: Down, spacing: 18, padding: 40

        <Label> {
            text: "Router Demo App"
            draw_text: { text_style: { font_size: 32 }, color: #xFFFFFF }
        }

        <Label> {
            text: "A small Makepad app showcasing makepad-router features."
            draw_text: { text_style: { font_size: 16 }, color: #xAAAAAA }
        }

        <View> {
            width: Fill, height: Fit
            flow: Down, spacing: 10

            <Label> {
                text: "Highlights"
                draw_text: { text_style: { font_size: 14 }, color: #xFFFFFF }
            }
            <Label> { text: "• URL sync + deep links (web)" draw_text: { text_style: { font_size: 12 }, color: #xAAAAAA } }
            <Label> { text: "• Nested routers (/admin/*)" draw_text: { text_style: { font_size: 12 }, color: #xAAAAAA } }
            <Label> { text: "• Params + query (/user/:id?tab=...)" draw_text: { text_style: { font_size: 12 }, color: #xAAAAAA } }
            <Label> { text: "• Guards + before-leave hooks" draw_text: { text_style: { font_size: 12 }, color: #xAAAAAA } }
            <Label> { text: "• Per-route transitions + hero transitions" draw_text: { text_style: { font_size: 12 }, color: #xAAAAAA } }
        }

        <View> {
            width: Fill, height: Fit
            flow: Right, spacing: 10

            settings_btn = <Button> { text: "Settings (guards)" }
            hero_btn = <Button> { text: "Hero transition" }
            user_btn = <Button> { text: "User (params/query)" }
            admin_btn = <Button> { text: "Admin (nested)" }
            stack_btn = <Button> { text: "History/Stack demo" }
            about_btn = <Button> { text: "About (async guard)" }
        }
    }
}

#[derive(Default)]
pub struct HomeController;

impl HomeController {
    pub fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, router: &RouterWidgetRef) {
        let Some((to_settings, to_hero, to_user, to_admin, to_stack, to_about)) =
            router.with_active_route_widget(|w| {
                (
                    w.button(&[live_id!(settings_btn)]).clicked(actions),
                    w.button(&[live_id!(hero_btn)]).clicked(actions),
                    w.button(&[live_id!(user_btn)]).clicked(actions),
                    w.button(&[live_id!(admin_btn)]).clicked(actions),
                    w.button(&[live_id!(stack_btn)]).clicked(actions),
                    w.button(&[live_id!(about_btn)]).clicked(actions),
                )
            })
        else {
            return;
        };

        if to_settings {
            router.navigate(cx, live_id!(settings));
        }
        if to_hero {
            router.navigate(cx, live_id!(hero_list));
        }
        if to_user {
            router.navigate_by_path(cx, "/user/12345?tab=posts");
        }
        if to_admin {
            router.navigate_by_path(cx, "/admin/dashboard");
        }
        if to_stack {
            router.navigate(cx, live_id!(stack_demo));
        }
        if to_about {
            router.navigate(cx, live_id!(about));
        }
    }
}
