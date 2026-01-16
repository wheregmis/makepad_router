use makepad_router::RouterWidgetRef;
use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    use link::theme_desktop_dark::*;
    use makepad_draw::shader::std::*;

    PrimaryButton = <Button> {
        width: Fit, height: 36
        padding: { left: 16, right: 16, top: 6, bottom: 6 }
        draw_bg: {
            color: #2D6CDF
            color_hover: #3D7BE6
            color_down: #245BC4
            color_focus: #2D6CDF
            border_radius: 18.0
        }
        draw_text: {
            color: #FFFFFF
            color_hover: #FFFFFF
            color_down: #F3F6FF
            color_focus: #FFFFFF
            text_style: { font_size: 12 }
        }
    }

    SecondaryButton = <Button> {
        width: Fit, height: 36
        padding: { left: 16, right: 16, top: 6, bottom: 6 }
        draw_bg: {
            color: #EAF0F6
            color_hover: #DDE7F2
            color_down: #CFE0F0
            color_focus: #EAF0F6
            border_radius: 18.0
            border_size: 1.0
            border_color: #C9D6E3
            border_color_focus: #C9D6E3
        }
        draw_text: {
            color: #1C2A3A
            color_hover: #1C2A3A
            color_down: #121A25
            color_focus: #1C2A3A
            text_style: { font_size: 12 }
        }
    }

    HeroCard = <View> {
        width: Fill, height: Fit
        padding: 24
        show_bg: true
        draw_bg: {
            color: #FDF3E6
            uniform color2: #DDEBFA
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 16.0);
                let gradient = mix(self.color, self.color2, self.pos.x);
                sdf.fill(gradient);
                sdf.stroke(#D0D9E2, 1.0);
                return sdf.result;
            }
        }
        flow: Down, spacing: 12
    }

    FeatureCard = <View> {
        width: Fill, height: Fit
        padding: 18
        show_bg: true
        draw_bg: {
            color: #FFFFFF
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 14.0);
                sdf.fill(self.color);
                sdf.stroke(#E2E8F0, 1.0);
                return sdf.result;
            }
        }
        flow: Down, spacing: 10
    }

    pub HomePage = <View> {
        width: Fill, height: Fill
        flow: Down, spacing: 20, padding: 40

        <HeroCard> {
            <Label> {
                text: "Router Example"
                draw_text: { text_style: { font_size: 34 }, color: #1A2233 }
            }
            <Label> {
                text: "A clean, minimal demo of nested routes, params, and not-found handling."
                draw_text: { text_style: { font_size: 14 }, color: #2D3B4F }
            }
        }

        <FeatureCard> {
            <Label> {
                text: "Core routes"
                draw_text: { text_style: { font_size: 13 }, color: #1A2233 }
            }
            <Label> { text: "• / (home)" draw_text: { text_style: { font_size: 12 }, color: #5A6A7D } }
            <Label> { text: "• /settings/* (nested)" draw_text: { text_style: { font_size: 12 }, color: #5A6A7D } }
            <Label> { text: "• /detail/:id (param)" draw_text: { text_style: { font_size: 12 }, color: #5A6A7D } }
            <Label> { text: "• not found (catch-all)" draw_text: { text_style: { font_size: 12 }, color: #5A6A7D } }
        }

        <View> {
            width: Fill, height: Fit
            flow: Right, spacing: 10

            settings_btn = <PrimaryButton> { text: "Open Settings" }
            detail_btn = <SecondaryButton> { text: "Go to Detail" }
        }
    }
}

#[derive(Default)]
pub struct HomeController;

impl HomeController {
    pub fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, router: &RouterWidgetRef) {
        let Some((to_settings, to_detail)) = router.with_active_route_widget(|w| {
            (
                w.button(&[live_id!(settings_btn)]).clicked(actions),
                w.button(&[live_id!(detail_btn)]).clicked(actions),
            )
        }) else {
            return;
        };

        if to_settings {
            router.navigate(cx, live_id!(settings));
        }
        if to_detail {
            router.navigate_by_path(cx, "/detail/42");
        }
    }
}
