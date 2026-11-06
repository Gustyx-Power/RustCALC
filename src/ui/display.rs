use super::theme::{BrutalTheme, HERMES_ORANGE};
use egui::{
    vec2, Align, Color32, CornerRadius, FontId, Layout, Rect, Stroke, StrokeKind, Ui,
};

pub struct DisplayState<'a> {
    pub expression: &'a str,
    pub result: &'a str,
    pub is_error: bool,
    pub error_msg: Option<&'a str>,
    pub angle_mode_label: &'a str,
    pub memory_value: f64,
    pub copied_feedback: bool,
}

pub fn render_display(
    ui: &mut Ui,
    state: &DisplayState,
    theme: &BrutalTheme,
) {
    let shadow_offset = vec2(4.0, 4.0);
    let border_stroke = Stroke::new(2.5, theme.border());

    let (outer_rect, _) = ui.allocate_exact_size(vec2(ui.available_width(), 138.0), egui::Sense::hover());

    if ui.is_rect_visible(outer_rect) {
        let painter = ui.painter();

        // Hard drop shadow behind display card
        let shadow_rect = Rect::from_min_size(outer_rect.min + shadow_offset, outer_rect.size());
        painter.rect_filled(shadow_rect, CornerRadius::ZERO, theme.shadow());

        // Display card background
        painter.rect(outer_rect, CornerRadius::ZERO, theme.display_bg(), border_stroke, StrokeKind::Inside);

        let mut content_ui = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(outer_rect.shrink(10.0))
                .layout(Layout::top_down(Align::Min)),
        );

        // Header technical ticker line
        content_ui.horizontal(|ui| {
            let ticker_color = if state.is_error {
                Color32::from_rgb(220, 38, 38)
            } else {
                theme.text_muted()
            };
            ui.label(
                egui::RichText::new("[RustCALC // SCI_ENGINE_V1]")
                    .font(FontId::monospace(9.5))
                    .color(ticker_color),
            );

            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                if state.copied_feedback {
                    ui.label(
                        egui::RichText::new("[COPIED TO CLIPBOARD]")
                            .font(FontId::monospace(9.5))
                            .strong()
                            .color(HERMES_ORANGE),
                    );
                } else if state.is_error {
                    ui.label(
                        egui::RichText::new("[ERR: SYNTAX ERROR]")
                            .font(FontId::monospace(9.5))
                            .color(Color32::from_rgb(220, 38, 38)),
                    );
                } else {
                    ui.label(
                        egui::RichText::new(format!("[{}]", state.angle_mode_label))
                            .font(FontId::monospace(9.5))
                            .color(HERMES_ORANGE),
                    );
                    if state.memory_value.abs() > f64::EPSILON {
                        ui.label(
                            egui::RichText::new(format!("[M: {}]", state.memory_value))
                                .font(FontId::monospace(9.5))
                                .color(theme.text()),
                        );
                    }
                }
            });
        });

        content_ui.add_space(6.0);

        // Expression input line with industrial brutalist block cursor
        content_ui.horizontal(|ui| {
            let expr_display = if state.expression.is_empty() {
                "0 █".to_string()
            } else {
                format!("{} █", state.expression)
            };

            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.label(
                    egui::RichText::new(expr_display)
                        .font(FontId::monospace(15.0))
                        .color(theme.text_muted()),
                );
            });
        });

        content_ui.add_space(4.0);

        // Main Result / Error Output
        content_ui.horizontal(|ui| {
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                if let Some(err) = state.error_msg {
                    ui.label(
                        egui::RichText::new(err)
                            .font(FontId::monospace(13.0))
                            .color(Color32::from_rgb(220, 38, 38)),
                    );
                } else {
                    let result_str = if state.result.is_empty() {
                        "0"
                    } else {
                        state.result
                    };

                    let font_size = if result_str.len() > 16 {
                        22.0
                    } else if result_str.len() > 10 {
                        26.0
                    } else {
                        32.0
                    };

                    ui.label(
                        egui::RichText::new(result_str)
                            .font(FontId::monospace(font_size))
                            .strong()
                            .color(theme.text()),
                    );
                }
            });
        });
    }
}
