use makepad_router::{RouterWidgetRef, RouterWidgetWidgetRefExt};
use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    use link::theme_desktop_dark::*;
    use makepad_router::widget::*;
    use makepad_draw::shader::std::*;

    TabButton = <Button> {
        width: Fit, height: 32
        padding: { left: 12, right: 12, top: 4, bottom: 4 }
        draw_bg: {
            color: #FFFFFF
            color_hover: #F2F6FB
            color_down: #E6EEF6
            color_focus: #FFFFFF
            border_radius: 14.0
            border_size: 1.0
            border_color: #D6E0EB
            border_color_focus: #D6E0EB
        }
        draw_text: {
            color: #1C2A3A
            color_hover: #1C2A3A
            color_down: #121A25
            color_focus: #1C2A3A
            text_style: { font_size: 12 }
        }
    }

    GhostButton = <Button> {
        width: Fit, height: 34
        padding: { left: 14, right: 14, top: 6, bottom: 6 }
        draw_bg: {
            color: #FFFFFF
            color_hover: #F2F6FB
            color_down: #E6EEF6
            color_focus: #FFFFFF
            border_radius: 16.0
            border_size: 1.0
            border_color: #D6E0EB
            border_color_focus: #D6E0EB
        }
        draw_text: {
            color: #1C2A3A
            color_hover: #1C2A3A
            color_down: #121A25
            color_focus: #1C2A3A
            text_style: { font_size: 12 }
        }
    }

    SettingsCard = <View> {
        width: Fill, height: Fit
        padding: 24
        show_bg: true
        draw_bg: {
            color: #FFFFFF
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 18.0);
                sdf.fill(self.color);
                sdf.stroke(#E2E8F0, 1.0);
                return sdf.result;
            }
        }
        flow: Down, spacing: 12
    }

    SettingsOverviewPage = <View> {
        width: Fill, height: Fill
        padding: 16
        show_bg: true
        draw_bg: {
            color: #F7FAFD
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 12.0);
                sdf.fill(self.color);
                sdf.stroke(#E2E8F0, 1.0);
                return sdf.result;
            }
        }
        flow: Down, spacing: 8

        <Label> { text: "Overview" draw_text: { text_style: { font_size: 18 }, color: #1A2233 } }
        <Label> { text: "Nested route: /settings/overview" draw_text: { text_style: { font_size: 12 }, color: #5A6A7D } }
    }

    SettingsProfilePage = <View> {
        width: Fill, height: Fill
        padding: 16
        show_bg: true
        draw_bg: {
            color: #F7FAFD
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 12.0);
                sdf.fill(self.color);
                sdf.stroke(#E2E8F0, 1.0);
                return sdf.result;
            }
        }
        flow: Down, spacing: 8

        <Label> { text: "Profile" draw_text: { text_style: { font_size: 18 }, color: #1A2233 } }
        <Label> { text: "Nested route: /settings/profile" draw_text: { text_style: { font_size: 12 }, color: #5A6A7D } }
    }

    pub SettingsPage = <View> {
        width: Fill, height: Fill
        flow: Down, spacing: 20, padding: 40

        <SettingsCard> {
            <Label> {
                text: "Settings"
                draw_text: { text_style: { font_size: 30 }, color: #1A2233 }
            }

            <Label> {
                text: "This route owns a nested RouterWidget."
                draw_text: { text_style: { font_size: 13 }, color: #5A6A7D }
            }

            <View> {
                width: Fill, height: Fit
                flow: Right, spacing: 10
                overview_btn = <TabButton> { text: "Overview" }
                profile_btn = <TabButton> { text: "Profile" }
            }

            settings_router = <RouterWidget> {
                width: Fill, height: 220
                default_route: settings_overview
                not_found_route: settings_overview
                push_transition: SlideLeft
                pop_transition: SlideRight
                transition_duration: 0.20

                settings_overview = <SettingsOverviewPage> { route_pattern: "/overview" }
                settings_profile = <SettingsProfilePage> { route_pattern: "/profile" }
            }

            home_btn = <GhostButton> { text: "Back to Home" }
        }
    }
}

#[derive(Default)]
pub struct SettingsController;

impl SettingsController {
    pub fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, router: &RouterWidgetRef) {
        let Some((to_home, to_overview, to_profile, settings_router)) =
            router.with_active_route_widget(|w| {
                (
                    w.button(&[live_id!(home_btn)]).clicked(actions),
                    w.button(&[live_id!(overview_btn)]).clicked(actions),
                    w.button(&[live_id!(profile_btn)]).clicked(actions),
                    w.widget(&[live_id!(settings_router)]).as_router_widget(),
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
