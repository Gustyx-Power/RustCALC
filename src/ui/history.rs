use super::theme::{BrutalTheme, HERMES_ORANGE};
use super::widgets::{brutalist_button, ButtonStyle};
use egui::{vec2, Align, CornerRadius, FontId, Layout, Rect, ScrollArea, Stroke, StrokeKind, Ui};

#[derive(Debug, Clone)]
pub struct HistoryItem {
    pub expression: String,
    pub result: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrawerTab {
    History,
    Constants,
    Convert,
}

pub fn render_drawer(
    ui: &mut Ui,
    history: &[HistoryItem],
    active_tab: &DrawerTab,
    current_value: f64,
    theme: &BrutalTheme,
) -> Option<DrawerAction> {
    let mut action: Option<DrawerAction> = None;

    let shadow_offset = vec2(3.0, 3.0);
    let border_stroke = Stroke::new(2.0, theme.border());

    ui.vertical(|ui| {
        // Tab switcher (3 equal columns)
        ui.horizontal(|ui| {
            let tab_w = (ui.available_width() - 12.0) / 3.0;

            let hist_style = if *active_tab == DrawerTab::History {
                ButtonStyle::Accent
            } else {
                ButtonStyle::Function
            };
            if brutalist_button(ui, "ARCHIVE", vec2(tab_w, 30.0), hist_style, theme).clicked() {
                action = Some(DrawerAction::SwitchTab(DrawerTab::History));
            }

            let const_style = if *active_tab == DrawerTab::Constants {
                ButtonStyle::Accent
            } else {
                ButtonStyle::Function
            };
            if brutalist_button(ui, "CONSTANTS", vec2(tab_w, 30.0), const_style, theme).clicked() {
                action = Some(DrawerAction::SwitchTab(DrawerTab::Constants));
            }

            let conv_style = if *active_tab == DrawerTab::Convert {
                ButtonStyle::Accent
            } else {
                ButtonStyle::Function
            };
            if brutalist_button(ui, "CONVERT", vec2(tab_w, 30.0), conv_style, theme).clicked() {
                action = Some(DrawerAction::SwitchTab(DrawerTab::Convert));
            }
        });

        ui.add_space(8.0);

        match active_tab {
            DrawerTab::History => {
                if history.is_empty() {
                    ui.add_space(20.0);
                    ui.vertical_centered(|ui| {
                        ui.label(
                            egui::RichText::new("[NO ARCHIVED RECORDS]")
                                .font(FontId::monospace(12.0))
                                .color(theme.text_muted()),
                        );
                    });
                } else {
                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new(format!("RECORDS: {}", history.len()))
                                .font(FontId::monospace(10.0))
                                .color(theme.text_muted()),
                        );
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            if brutalist_button(ui, "CLEAR", vec2(50.0, 20.0), ButtonStyle::Danger, theme).clicked() {
                                action = Some(DrawerAction::ClearHistory);
                            }
                        });
                    });

                    ui.add_space(4.0);

                    ScrollArea::vertical()
                        .max_height(200.0)
                        .show(ui, |ui| {
                            for (idx, item) in history.iter().enumerate().rev() {
                                let (rect, response) = ui.allocate_exact_size(
                                    vec2(ui.available_width() - 6.0, 52.0),
                                    egui::Sense::click(),
                                );

                                if ui.is_rect_visible(rect) {
                                    let painter = ui.painter();
                                    let shadow_rect = Rect::from_min_size(rect.min + shadow_offset, rect.size());
                                    painter.rect_filled(shadow_rect, CornerRadius::ZERO, theme.shadow());

                                    let bg = if response.hovered() {
                                        theme.fn_btn_bg()
                                    } else {
                                        theme.card_bg()
                                    };

                                    painter.rect(rect, CornerRadius::ZERO, bg, border_stroke, StrokeKind::Inside);

                                    // Content
                                    let label_idx = format!("[REC_{:03}]", idx + 1);
                                    painter.text(
                                        rect.min + vec2(8.0, 6.0),
                                        egui::Align2::LEFT_TOP,
                                        label_idx,
                                        FontId::monospace(9.0),
                                        theme.text_muted(),
                                    );

                                    painter.text(
                                        rect.min + vec2(8.0, 20.0),
                                        egui::Align2::LEFT_TOP,
                                        &item.expression,
                                        FontId::monospace(11.0),
                                        theme.text(),
                                    );

                                    painter.text(
                                        rect.max - vec2(8.0, 8.0),
                                        egui::Align2::RIGHT_BOTTOM,
                                        format!("= {}", item.result),
                                        FontId::monospace(13.0),
                                        HERMES_ORANGE,
                                    );
                                }

                                if response.clicked() {
                                    action = Some(DrawerAction::SelectHistory(item.result.clone()));
                                }

                                ui.add_space(6.0);
                            }
                        });
                }
            }
            DrawerTab::Constants => {
                let constants: &[(&str, &str, &str)] = &[
                    ("π (pi)", "3.14159265", "pi"),
                    ("e (Euler)", "2.71828182", "e"),
                    ("φ (Golden)", "1.61803398", "phi"),
                    ("c (Light)", "299,792,458 m/s", "c"),
                    ("h (Planck)", "6.62607e-34", "h"),
                    ("g (Gravity)", "9.80665 m/s²", "g"),
                    ("NA (Avogadro)", "6.02214e23", "na"),
                ];

                ScrollArea::vertical()
                    .max_height(200.0)
                    .show(ui, |ui| {
                        for (name, val, insert_token) in constants {
                            let (rect, response) = ui.allocate_exact_size(
                                vec2(ui.available_width() - 6.0, 38.0),
                                egui::Sense::click(),
                            );

                            if ui.is_rect_visible(rect) {
                                let painter = ui.painter();
                                let shadow_rect = Rect::from_min_size(rect.min + shadow_offset, rect.size());
                                painter.rect_filled(shadow_rect, CornerRadius::ZERO, theme.shadow());

                                let bg = if response.hovered() {
                                    theme.fn_btn_bg()
                                } else {
                                    theme.card_bg()
                                };

                                painter.rect(rect, CornerRadius::ZERO, bg, border_stroke, StrokeKind::Inside);

                                painter.text(
                                    rect.min + vec2(8.0, 10.0),
                                    egui::Align2::LEFT_TOP,
                                    *name,
                                    FontId::monospace(12.0),
                                    theme.text(),
                                );

                                painter.text(
                                    rect.max - vec2(8.0, 10.0),
                                    egui::Align2::RIGHT_BOTTOM,
                                    *val,
                                    FontId::monospace(11.0),
                                    theme.text_muted(),
                                );
                            }

                            if response.clicked() {
                                action = Some(DrawerAction::InsertConstant(insert_token.to_string()));
                            }

                            ui.add_space(4.0);
                        }
                    });
            }
            DrawerTab::Convert => {
                let val = current_value;
                let conversions: Vec<(&str, String, String)> = vec![
                    ("DEG → RAD", format!("{:.6} rad", val.to_radians()), format!("{:.6}", val.to_radians())),
                    ("RAD → DEG", format!("{:.4}°", val.to_degrees()), format!("{:.4}", val.to_degrees())),
                    ("°C → °F", format!("{:.2} °F", val * 9.0 / 5.0 + 32.0), format!("{:.4}", val * 9.0 / 5.0 + 32.0)),
                    ("°F → °C", format!("{:.2} °C", (val - 32.0) * 5.0 / 9.0), format!("{:.4}", (val - 32.0) * 5.0 / 9.0)),
                    ("°C → K", format!("{:.2} K", val + 273.15), format!("{:.4}", val + 273.15)),
                    ("Meters → Feet", format!("{:.4} ft", val * 3.28084), format!("{:.6}", val * 3.28084)),
                    ("Feet → Meters", format!("{:.4} m", val / 3.28084), format!("{:.6}", val / 3.28084)),
                    ("Km → Miles", format!("{:.4} mi", val * 0.621371), format!("{:.6}", val * 0.621371)),
                    ("Kg → Pounds", format!("{:.4} lbs", val * 2.20462), format!("{:.6}", val * 2.20462)),
                ];

                ui.label(
                    egui::RichText::new(format!("BASE VALUE: {}", val))
                        .font(FontId::monospace(10.0))
                        .color(HERMES_ORANGE),
                );
                ui.add_space(4.0);

                ScrollArea::vertical()
                    .max_height(200.0)
                    .show(ui, |ui| {
                        for (label, display_val, insert_val) in conversions {
                            let (rect, response) = ui.allocate_exact_size(
                                vec2(ui.available_width() - 6.0, 38.0),
                                egui::Sense::click(),
                            );

                            if ui.is_rect_visible(rect) {
                                let painter = ui.painter();
                                let shadow_rect = Rect::from_min_size(rect.min + shadow_offset, rect.size());
                                painter.rect_filled(shadow_rect, CornerRadius::ZERO, theme.shadow());

                                let bg = if response.hovered() {
                                    theme.fn_btn_bg()
                                } else {
                                    theme.card_bg()
                                };

                                painter.rect(rect, CornerRadius::ZERO, bg, border_stroke, StrokeKind::Inside);

                                painter.text(
                                    rect.min + vec2(8.0, 10.0),
                                    egui::Align2::LEFT_TOP,
                                    label,
                                    FontId::monospace(11.0),
                                    theme.text(),
                                );

                                painter.text(
                                    rect.max - vec2(8.0, 10.0),
                                    egui::Align2::RIGHT_BOTTOM,
                                    &display_val,
                                    FontId::monospace(11.5),
                                    HERMES_ORANGE,
                                );
                            }

                            if response.clicked() {
                                action = Some(DrawerAction::SelectHistory(insert_val));
                            }

                            ui.add_space(4.0);
                        }
                    });
            }
        }
    });

    action
}

pub enum DrawerAction {
    SwitchTab(DrawerTab),
    SelectHistory(String),
    InsertConstant(String),
    ClearHistory,
}
