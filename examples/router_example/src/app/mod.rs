mod pages;
mod shared;
live_design! {
    use link::widgets::*;
    use link::theme_desktop_dark::*;
    use makepad_router::widget::*;
    use makepad_draw::shader::std::*;
    use crate::app::pages::home::*;
    use crate::app::pages::settings::*;
    use crate::app::pages::about::*;
    use crate::app::pages::stack_demo::*;
    use crate::app::pages::hero::*;
    use crate::app::pages::user_profile::*;
    use crate::app::pages::admin::*;
    use crate::app::pages::not_found::*;

    App = {{App}} {
        ui: <Window> {
            show_bg: true
            draw_bg: {
                fn pixel(self) -> vec4 {
                    return mix(#x1A1A2E, #x0F0F1E, self.pos.y);
                }
            }
            width: Fill, height: Fill

            body = <View> {
                width: Fill, height: Fill
                flow: Down

                nav_bar = <View> {
                    width: Fill, height: Fit
                    show_bg: true
                    draw_bg: { color: #x252545 }
                    padding: 15
                    flow: Right, spacing: 10

                    home_btn = <Button> { text: "Home" }
                    settings_btn = <Button> { text: "Settings" }
                    hero_btn = <Button> { text: "Hero" }
                    admin_btn = <Button> { text: "Admin" }
                    stack_btn = <Button> { text: "Stack" }
                    broken_link_btn = <Button> { text: "404" }

                    <View> { width: Fill, height: Fit }

                    status_label = <Label> {
                        text: ""
                        draw_text: { text_style: { font_size: 10 }, color: #xAAAAAA }
                    }

                    back_btn = <Button> { text: "← Back" }
                }

                router = <RouterWidget> {
                    width: Fill, height: Fill
                    default_route: home
                    not_found_route: not_found
                    push_transition: SlideLeft
                    pop_transition: SlideRight
                    replace_transition: Fade
                    transition_duration: 0.30
                    hero_transition: true
                    debug_inspector: true
                    use_initial_url: true

                    home = <HomePage> { route_pattern: "/" }
                    settings = <SettingsPage> { route_pattern: "/settings" }
                    about = <AboutPage> { route_pattern: "/about" }
                    stack_demo = <StackDemoPage> { route_pattern: "/stack" }

                    hero_list = <HeroListPage> { route_pattern: "/hero" }
                    hero_detail = <HeroDetailPage> { route_pattern: "/hero/detail" }

                    user_profile = <UserProfilePage> { route_pattern: "/user/:id" }

                    admin = <AdminDashboard> { route_pattern: "/admin/*" }

                    not_found = <NotFoundPage> {
                        route_transition: Fade
                        route_transition_duration: 0.20
                    }
                }
            }
        }
    }
}

use makepad_router::{
    RouterAction, RouterAsyncDecision, RouterBeforeLeaveDecision, RouterGuardDecision, RouterRedirect,
    RouterRedirectTarget, RouterWidgetWidgetRefExt,
};
use makepad_widgets::*;
use pages::{
    AboutController, AdminController, HeroDetailController, HeroListController, HomeController,
    NotFoundController, SettingsController, StackDemoController, UserProfileController,
};
use shared::SharedState;

#[cfg(not(target_arch = "wasm32"))]
use std::time::Duration;

#[derive(Live, LiveHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
    #[rust]
    shared: SharedState,
    #[rust]
    home: HomeController,
    #[rust]
    settings: SettingsController,
    #[rust]
    about: AboutController,
    #[rust]
    stack_demo: StackDemoController,
    #[rust]
    hero_list: HeroListController,
    #[rust]
    hero_detail: HeroDetailController,
    #[rust]
    user_profile: UserProfileController,
    #[rust]
    admin: AdminController,
    #[rust]
    not_found: NotFoundController,
    #[rust]
    hooks_installed: bool,
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
        if self.ui.button(ids!(nav_bar.hero_btn)).clicked(actions) {
            router.navigate(cx, live_id!(hero_list));
        }
        if self.ui.button(ids!(nav_bar.admin_btn)).clicked(actions) {
            router.navigate_by_path(cx, "/admin/dashboard");
        }
        if self.ui.button(ids!(nav_bar.stack_btn)).clicked(actions) {
            router.navigate(cx, live_id!(stack_demo));
        }
        if self.ui.button(ids!(nav_bar.broken_link_btn)).clicked(actions) {
            router.navigate_by_path(cx, "/this/route/does/not/exist");
        }
        if self.ui.button(ids!(nav_bar.back_btn)).clicked(actions) {
            router.back(cx);
        }

        // Routed buttons: delegate to the active page controller (route-scoped widget access).
        match router.current_route_id() {
            Some(route_id) if route_id == live_id!(home) => self.home.handle_actions(cx, actions, &router),
            Some(route_id) if route_id == live_id!(settings) => {
                self.settings.handle_actions(cx, actions, &router, &self.shared)
            }
            Some(route_id) if route_id == live_id!(about) => self.about.handle_actions(cx, actions, &router),
            Some(route_id) if route_id == live_id!(stack_demo) => {
                self.stack_demo.handle_actions(cx, actions, &router)
            }
            Some(route_id) if route_id == live_id!(hero_list) => {
                self.hero_list.handle_actions(cx, actions, &router)
            }
            Some(route_id) if route_id == live_id!(hero_detail) => {
                self.hero_detail.handle_actions(cx, actions, &router)
            }
            Some(route_id) if route_id == live_id!(user_profile) => {
                self.user_profile.handle_actions(cx, actions, &router)
            }
            Some(route_id) if route_id == live_id!(admin) => self.admin.handle_actions(cx, actions, &router),
            Some(route_id) if route_id == live_id!(not_found) => {
                self.not_found.handle_actions(cx, actions, &router)
            }
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
            router.current_url().unwrap_or_else(|| "(no url)".to_string())
        );
        self.ui.label(ids!(nav_bar.status_label)).set_text(cx, &status);

        // Settings labels are updated by the Settings controller.
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        if matches!(event, Event::Startup) && !self.hooks_installed {
            self.hooks_installed = true;
            self.install_router_hooks(cx);
        }
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}

impl App {
    fn install_router_hooks(&mut self, _cx: &mut Cx) {
        let router = self.ui.router_widget(ids!(router));
        let auth = self.shared.auth_logged_in.clone();
        let dirty = self.shared.settings_dirty.clone();

        // Guard: admin requires auth; redirect to settings.
        router.add_route_guard(move |_cx, nav| {
            let to = nav.to.as_ref().map(|r| r.id);
            if to == Some(live_id!(admin)) && !auth.load(std::sync::atomic::Ordering::SeqCst) {
                return RouterGuardDecision::Redirect(RouterRedirect {
                    target: RouterRedirectTarget::Route(live_id!(settings)),
                    replace: true,
                });
            }
            RouterGuardDecision::Allow
        });

        // Before leave: block leaving Settings when dirty.
        router.add_before_leave_hook(move |_cx, nav| {
            let from = nav.from.as_ref().map(|r| r.id);
            let to = nav.to.as_ref().map(|r| r.id);
            if from == Some(live_id!(settings))
                && to != Some(live_id!(settings))
                && dirty.load(std::sync::atomic::Ordering::SeqCst)
            {
                return RouterBeforeLeaveDecision::Block;
            }
            RouterBeforeLeaveDecision::Allow
        });

        // Async guard: about is delayed on native to demo async decisions.
        router.add_route_guard_async(move |_cx, nav| {
            if nav.to.as_ref().map(|r| r.id) != Some(live_id!(about)) {
                return RouterAsyncDecision::Immediate(RouterGuardDecision::Allow);
            }

            #[cfg(target_arch = "wasm32")]
            {
                RouterAsyncDecision::Immediate(RouterGuardDecision::Allow)
            }

            #[cfg(not(target_arch = "wasm32"))]
            {
                let rx: ToUIReceiver<RouterGuardDecision> = Default::default();
                let tx = rx.sender();
                std::thread::spawn(move || {
                    std::thread::sleep(Duration::from_millis(200));
                    let _ = tx.send(RouterGuardDecision::Allow);
                });
                RouterAsyncDecision::Pending(rx)
            }
        });
    }
}

app_main!(App);
