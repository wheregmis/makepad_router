use makepad_router::RouterWidgetRef;
use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    use link::theme_desktop_dark::*;
    use makepad_draw::shader::std::*;

    AccentButton = <Button> {
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

    LostCard = <View> {
        width: Fill, height: Fit
        padding: 28
        show_bg: true
        draw_bg: {
            color: #FFEEDB
            uniform color2: #F7DDE0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 18.0);
                let gradient = mix(self.color, self.color2, self.pos.x);
                sdf.fill(gradient);
                sdf.stroke(#E7C9C9, 1.0);
                return sdf.result;
            }
        }
        flow: Down, spacing: 12
    }

    pub NotFoundPage = <View> {
        width: Fill, height: Fill
        flow: Down, spacing: 20, padding: 40

        <LostCard> {
            <Label> {
                text: "404"
                draw_text: { text_style: { font_size: 46 }, color: #3A2B2B }
            }

            <Label> {
                text: "That route does not exist."
                draw_text: { text_style: { font_size: 14 }, color: #6C4F4F }
            }

            home_btn = <AccentButton> { text: "Back to Home" }
        }
    }
}

#[derive(Default)]
pub struct NotFoundController;

impl NotFoundController {
    pub fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, router: &RouterWidgetRef) {
        let Some(to_home) =
            router.with_active_route_widget(|w| w.button(&[live_id!(home_btn)]).clicked(actions))
        else {
            return;
        };
        if to_home {
            router.replace(cx, live_id!(home));
        }
    }
}
