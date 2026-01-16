use makepad_router::{Route, RouterWidgetRef};
use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    use link::theme_desktop_dark::*;

    pub StackDemoPage = <View> {
        width: Fill, height: Fill
        show_bg: true
        draw_bg: { color: #x101A24 }
        flow: Down, spacing: 20, padding: 40

        <Label> {
            text: "History / Stack Demo"
            draw_text: { text_style: { font_size: 32 }, color: #xFFFFFF }
        }

        <Label> {
            text: "Exercises stack APIs: set_stack, pop_to, pop_to_root, replace."
            draw_text: { text_style: { font_size: 14 }, color: #xAAAAAA }
        }

        <View> {
            width: Fill, height: Fit
            flow: Right, spacing: 10
            set_stack_btn = <Button> { text: "Set stack: Home > Settings > About" }
            pop_to_settings_btn = <Button> { text: "Pop to Settings" }
            pop_to_root_btn = <Button> { text: "Pop to Root" }
        }

        <View> {
            width: Fill, height: Fit
            flow: Right, spacing: 10
            replace_about_btn = <Button> { text: "Replace → About" }
            home_btn = <Button> { text: "Back to Home" }
        }
    }
}

#[derive(Default)]
pub struct StackDemoController;

impl StackDemoController {
    pub fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, router: &RouterWidgetRef) {
        let Some((set_stack, pop_to_settings, pop_to_root, replace_about, to_home)) =
            router.with_active_route_widget(|w| {
                (
                    w.button(&[live_id!(set_stack_btn)]).clicked(actions),
                    w.button(&[live_id!(pop_to_settings_btn)]).clicked(actions),
                    w.button(&[live_id!(pop_to_root_btn)]).clicked(actions),
                    w.button(&[live_id!(replace_about_btn)]).clicked(actions),
                    w.button(&[live_id!(home_btn)]).clicked(actions),
                )
            })
        else {
            return;
        };

        if set_stack {
            router.set_stack(
                cx,
                vec![
                    Route::new(live_id!(home)),
                    Route::new(live_id!(settings)),
                    Route::new(live_id!(about)),
                ],
            );
        }
        if pop_to_settings {
            router.pop_to(cx, live_id!(settings));
        }
        if pop_to_root {
            router.pop_to_root(cx);
        }
        if replace_about {
            router.replace(cx, live_id!(about));
        }
        if to_home {
            router.navigate(cx, live_id!(home));
        }
    }
}

