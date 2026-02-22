mod pages;

use makepad_router::{RouterAction, RouterCommand, RouterWidgetWidgetRefExt};
use makepad_widgets::*;
use pages::{DetailController, HomeController, NotFoundController, SettingsController};

app_main!(App);

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    startup() do #(App::script_component(vm)){
        ui: Root{
            main_window := Window{
                body +: {
                    width: Fill
                    height: Fill
                    flow: Down
                    spacing: 12
                    padding: 20

                    nav_bar := View{
                        width: Fill
                        height: Fit
                        flow: Right
                        spacing: 8

                        home_btn := Button{text: "Home"}
                        settings_btn := Button{text: "Settings"}
                        detail_btn := Button{text: "Detail"}
                        broken_link_btn := Button{text: "404"}

                        filler := View{width: Fill height: Fit}
                        status_label := Label{text: ""}
                        back_btn := Button{text: "Back"}
                    }

                    router := mod.widgets.RouterWidget{
                        width: Fill
                        height: Fill
                        default_route: @home
                        not_found_route: @not_found
                        cap_transitions: true
                        cap_nested: true

                        home := mod.widgets.RouterRoute{
                            route_pattern: "/"
                            mod.widgets.HomePage{}
                        }
                        settings := mod.widgets.RouterRoute{
                            route_pattern: "/settings/*"
                            mod.widgets.SettingsPage{}
                        }
                        detail := mod.widgets.RouterRoute{
                            route_pattern: "/detail/:id"
                            mod.widgets.DetailPage{}
                        }

                        not_found := mod.widgets.RouterRoute{
                            route_transition: @Fade
                            route_transition_duration: 0.20
                            mod.widgets.NotFoundPage{}
                        }
                    }
                }
            }
        }
    }
}

impl App {
    fn run(vm: &mut ScriptVm) -> Self {
        makepad_widgets::script_mod(vm);
        makepad_router::script_mod(vm);
        pages::script_mod(vm);
        App::from_script_mod(vm, self::script_mod)
    }
}

#[derive(Script, ScriptHook)]
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

impl MatchEvent for App {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        let router = self.ui.router_widget(cx, ids!(router));

        // Observe router changes by consuming emitted RouterAction widget-actions.
        for action in actions.filter_widget_actions(router.widget_uid()) {
            if let Some(router_action) = action.action.downcast_ref::<RouterAction>() {
                if let RouterAction::RouteChanged { from, to } = router_action {
                    log!("Route changed: {:?} -> {:?}", from, to);
                }
            }
        }

        // Nav bar
        if self.ui.button(cx, ids!(nav_bar.home_btn)).clicked(actions) {
            let _ = router.dispatch(
                cx,
                RouterCommand::GoToRoute {
                    route_id: live_id!(home),
                    transition: None,
                },
            );
        }
        if self
            .ui
            .button(cx, ids!(nav_bar.settings_btn))
            .clicked(actions)
        {
            let _ = router.dispatch(
                cx,
                RouterCommand::GoToRoute {
                    route_id: live_id!(settings),
                    transition: None,
                },
            );
        }
        if self
            .ui
            .button(cx, ids!(nav_bar.detail_btn))
            .clicked(actions)
        {
            let _ = router.dispatch(
                cx,
                RouterCommand::GoToPath {
                    path: "/detail/42".to_string(),
                },
            );
        }
        if self
            .ui
            .button(cx, ids!(nav_bar.broken_link_btn))
            .clicked(actions)
        {
            let _ = router.dispatch(
                cx,
                RouterCommand::GoToPath {
                    path: "/this/route/does/not/exist".to_string(),
                },
            );
        }
        if self.ui.button(cx, ids!(nav_bar.back_btn)).clicked(actions) {
            let _ = router.dispatch(cx, RouterCommand::Back { transition: None });
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
            .button(cx, ids!(nav_bar.back_btn))
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
            .label(cx, ids!(nav_bar.status_label))
            .set_text(cx, &status);
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}
