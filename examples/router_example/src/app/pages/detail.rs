use makepad_router::RouterWidgetRef;
use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    use link::theme_desktop_dark::*;
    use makepad_draw::shader::std::*;

    Chip = <View> {
        width: Fit, height: Fit
        padding: { left: 10, right: 10, top: 4, bottom: 4 }
        show_bg: true
        draw_bg: {
            color: #EAF0F6
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 12.0);
                sdf.fill(self.color);
                sdf.stroke(#C9D6E3, 1.0);
                return sdf.result;
            }
        }
    }

    ActionButton = <Button> {
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

    SecondaryButton = <Button> {
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

    PageCard = <View> {
        width: Fill, height: Fit
        padding: 26
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
        flow: Down, spacing: 14
    }

    pub DetailPage = <View> {
        width: Fill, height: Fill
        flow: Down, spacing: 20, padding: 40

        <PageCard> {
            <Label> { text: "Detail" draw_text: { text_style: { font_size: 30 }, color: #1A2233 } }

            <View> {
                width: Fill, height: Fit
                flow: Right, spacing: 10

                <Chip> {
                    detail_id_label = <Label> {
                        text: "ID: (dynamic)"
                        draw_text: { text_style: { font_size: 12 }, color: #1C2A3A }
                    }
                }
            }

            <View> {
                width: Fill, height: Fit
                flow: Right, spacing: 10
                id_1_btn = <ActionButton> { text: "Go /detail/1" }
                id_42_btn = <SecondaryButton> { text: "Go /detail/42" }
            }

            home_btn = <SecondaryButton> { text: "Back to Home" }
        }
    }
}

#[derive(Default)]
pub struct DetailController {
    last_id: Option<String>,
}

impl DetailController {
    pub fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, router: &RouterWidgetRef) {
        if let Some(id) = router.get_param_string("id") {
            if self.last_id.as_ref() != Some(&id) {
                router.bind_param_to_label(cx, "id", live_id!(detail_id_label), |id| {
                    format!("ID: {}", id)
                });
                self.last_id = Some(id);
            }
        }

        let Some((to_id_1, to_id_42, to_home)) = router.with_active_route_widget(|w| {
            (
                w.button(&[live_id!(id_1_btn)]).clicked(actions),
                w.button(&[live_id!(id_42_btn)]).clicked(actions),
                w.button(&[live_id!(home_btn)]).clicked(actions),
            )
        }) else {
            return;
        };

        if to_id_1 {
            router.navigate_by_path(cx, "/detail/1");
        }
        if to_id_42 {
            router.navigate_by_path(cx, "/detail/42");
        }
        if to_home {
            router.navigate(cx, live_id!(home));
        }
    }
}
