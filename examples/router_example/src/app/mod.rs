mod pages;

live_design! {
    use link::widgets::*;
    use link::theme_desktop_dark::*;
    use makepad_router::widget::*;
    use makepad_draw::shader::std::*;
    use crate::app::pages::home::*;
    use crate::app::pages::settings::*;
    use crate::app::pages::detail::*;
    use crate::app::pages::not_found::*;

    NavButton = <Button> {
        width: Fit, height: 34
        padding: { left: 14, right: 14, top: 6, bottom: 6 }
        draw_bg: {
            color: #FFFFFF
            color_hover: #F5F2EC
            color_down: #E9E3DA
            color_focus: #FFFFFF
            border_radius: 16.0
            border_size: 1.0
            border_color: #D7D2C8
            border_color_hover: #C9C2B6
            border_color_down: #B7AF9F
            border_color_focus: #D7D2C8
        }
        draw_text: {
            color: #1C2A3A
            color_hover: #1C2A3A
            color_down: #121A25
            color_focus: #1C2A3A
            text_style: { font_size: 12 }
        }
    }

    AccentButton = <Button> {
        width: Fit, height: 34
        padding: { left: 14, right: 14, top: 6, bottom: 6 }
        draw_bg: {
            color: #2D6CDF
            color_hover: #3D7BE6
            color_down: #245BC4
            color_focus: #2D6CDF
            border_radius: 16.0
        }
        draw_text: {
            color: #FFFFFF
            color_hover: #FFFFFF
            color_down: #F3F6FF
            color_focus: #FFFFFF
            text_style: { font_size: 12 }
        }
    }

    NavShell = <View> {
        width: Fill, height: Fit
        padding: 12
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
    }

    StatusPill = <View> {
        width: Fit, height: Fit
        padding: { left: 10, right: 10, top: 4, bottom: 4 }
        show_bg: true
        draw_bg: {
            color: #F7FAFD
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 12.0);
                sdf.fill(self.color);
                sdf.stroke(#D6E0EB, 1.0);
                return sdf.result;
            }
        }
    }

    App = {{App}} {
        ui: <Window> {
            show_bg: true
            draw_bg: { color: #F4F2EC }
            width: Fill, height: Fill

            body = <View> {
                width: Fill, height: Fill
                flow: Down
                padding: 24
                spacing: 16
                show_bg: true
                draw_bg: {
                    color: #F7F5F0
                    uniform color2: #E7EFF6
                    fn pixel(self) -> vec4 {
                        let gradient = mix(self.color, self.color2, self.pos.y);
                        return gradient;
                    }
                }

                nav_shell = <NavShell> {
                    nav_bar = <View> {
                        width: Fill, height: Fit
                        flow: Right, spacing: 8

                        home_btn = <NavButton> { text: "Home" }
                        settings_btn = <NavButton> { text: "Settings" }
                        detail_btn = <NavButton> { text: "Detail" }
                        broken_link_btn = <NavButton> { text: "404" }

                        <View> { width: Fill, height: Fit }

                        status_container = <StatusPill> {
                            status_label = <Label> {
                                text: ""
                                draw_text: { text_style: { font_size: 10 }, color: #52647A }
                            }
                        }

                        back_btn = <AccentButton> { text: "← Back" }
                    }
                }

                router = <RouterWidget> {
                    width: Fill, height: Fill
                    default_route: home
                    not_found_route: not_found
                    use_initial_url: true

                    home = <HomePage> { route_pattern: "/" }
                    settings = <SettingsPage> { route_pattern: "/settings/*" }
                    detail = <DetailPage> { route_pattern: "/detail/:id" }

                    not_found = <NotFoundPage> {
                        route_transition: Fade
                        route_transition_duration: 0.20
                    }
                }
            }
        }
    }
}

use makepad_router::{RouterAction, RouterWidgetWidgetRefExt};
use makepad_widgets::*;
use pages::{DetailController, HomeController, NotFoundController, SettingsController};

#[derive(Live, LiveHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
    #[rust]
    home: HomeController,
    #[rust]
    settings: SettingsController,
    #[rust]
    detail: DetailController,
    #[rust]
    not_found: NotFoundController,
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        makepad_router::live_design(cx);
        makepad_widgets::live_design(cx);
        pages::live_design(cx);
    }
}

impl MatchEvent for App {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        let router = self.ui.router_widget(ids!(router));

        // Observe router changes by consuming emitted RouterAction widget-actions.
        for action in actions.filter_widget_actions(router.widget_uid()) {
            if let Some(router_action) = action.action.downcast_ref::<RouterAction>() {
                if let RouterAction::RouteChanged { from, to } = router_action {
                    log!("Route changed: {:?} -> {:?}", from, to);
                }
            }
        }

        // Nav bar
        if self.ui.button(ids!(nav_bar.home_btn)).clicked(actions) {
            router.navigate(cx, live_id!(home));
        }
        if self.ui.button(ids!(nav_bar.settings_btn)).clicked(actions) {
            router.navigate(cx, live_id!(settings));
        }
        if self.ui.button(ids!(nav_bar.detail_btn)).clicked(actions) {
            router.navigate_by_path(cx, "/detail/42");
        }
        if self
            .ui
            .button(ids!(nav_bar.broken_link_btn))
            .clicked(actions)
        {
            router.navigate_by_path(cx, "/this/route/does/not/exist");
        }
        if self.ui.button(ids!(nav_bar.back_btn)).clicked(actions) {
            router.back(cx);
        }

        // Routed buttons: delegate to the active page controller (route-scoped widget access).
        match router.current_route_id() {
            Some(live_id!(home)) => self.home.handle_actions(cx, actions, &router),
            Some(live_id!(settings)) => self.settings.handle_actions(cx, actions, &router),
            Some(live_id!(detail)) => self.detail.handle_actions(cx, actions, &router),
            Some(live_id!(not_found)) => self.not_found.handle_actions(cx, actions, &router),
            _ => {}
        }

        // Update back button state + status label.
        self.ui
            .button(ids!(nav_bar.back_btn))
            .set_enabled(cx, router.can_go_back());
        let status = format!(
            "{}  {}",
            router
                .current_route_id()
                .map(|id: LiveId| id.to_string())
                .unwrap_or_else(|| "(none)".to_string()),
            router
                .current_url()
                .unwrap_or_else(|| "(no url)".to_string())
        );
        self.ui
            .label(ids!(nav_bar.status_label))
            .set_text(cx, &status);
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}

app_main!(App);
