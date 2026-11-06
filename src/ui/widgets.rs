use super::theme::{
    BrutalTheme, HERMES_ORANGE, HERMES_ORANGE_HOVER, NOIR_JET, PURE_WHITE,
};
use egui::{
    vec2, Align2, Color32, CornerRadius, FontId, Rect, Response, Sense, Stroke, StrokeKind, Ui,
    Vec2,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonStyle {
    Number,
    Function,
    Operator,
    Accent, // Hermès Orange
    Danger, // AC / Clear
    Ghost,
}

pub fn brutalist_button(
    ui: &mut Ui,
    label: &str,
    size: Vec2,
    style: ButtonStyle,
    theme: &BrutalTheme,
) -> Response {
    let (rect, response) = ui.allocate_exact_size(size, Sense::click());

    if ui.is_rect_visible(rect) {
        let is_pressed = response.is_pointer_button_down_on();
        let is_hovered = response.hovered();

        let shadow_offset = vec2(3.0, 3.0);
        let press_offset = vec2(2.0, 2.0);

        // Determine background and text colors
        let (bg_color, text_color) = match style {
            ButtonStyle::Number => (theme.num_btn_bg(), theme.text()),
            ButtonStyle::Function => {
                if is_hovered {
                    (HERMES_ORANGE, NOIR_JET)
                } else {
                    (theme.fn_btn_bg(), theme.text())
                }
            }
            ButtonStyle::Operator => {
                if is_hovered {
                    (HERMES_ORANGE_HOVER, NOIR_JET)
                } else {
                    (theme.fn_btn_bg(), theme.text())
                }
            }
            ButtonStyle::Accent => {
                if is_hovered {
                    (HERMES_ORANGE_HOVER, NOIR_JET)
                } else {
                    (HERMES_ORANGE, NOIR_JET)
                }
            }
            ButtonStyle::Danger => {
                if is_hovered {
                    (Color32::from_rgb(239, 68, 68), PURE_WHITE)
                } else {
                    (Color32::from_rgb(220, 38, 38), PURE_WHITE)
                }
            }
            ButtonStyle::Ghost => (Color32::TRANSPARENT, theme.text()),
        };

        let painter = ui.painter();

        // Draw hard drop shadow if not fully pressed
        if !is_pressed && style != ButtonStyle::Ghost {
            let shadow_rect = Rect::from_min_size(rect.min + shadow_offset, rect.size() - vec2(1.0, 1.0));
            painter.rect_filled(shadow_rect, CornerRadius::ZERO, theme.shadow());
        }

        // Offset the button rectangle when pressed for tactile mechanical feel
        let active_rect = if is_pressed {
            Rect::from_min_size(rect.min + press_offset, rect.size() - vec2(1.0, 1.0))
        } else {
            Rect::from_min_size(rect.min, rect.size() - vec2(1.0, 1.0))
        };

        // Button body
        let border_stroke = Stroke::new(2.0, theme.border());
        painter.rect(active_rect, CornerRadius::ZERO, bg_color, border_stroke, StrokeKind::Inside);

        // Typography
        let font_size = if label.len() > 5 {
            13.0
        } else if label.len() > 3 {
            15.0
        } else {
            18.0
        };

        let font_id = FontId::monospace(font_size);
        painter.text(
            active_rect.center(),
            Align2::CENTER_CENTER,
            label,
            font_id,
            text_color,
        );
    }

    response
}

pub fn brutalist_badge(
    ui: &mut Ui,
    label: &str,
    is_active: bool,
    theme: &BrutalTheme,
) -> Response {
    let font_id = FontId::monospace(11.0);
    let text_size = ui.painter().layout_no_wrap(label.to_string(), font_id.clone(), Color32::WHITE).size();
    let pad = vec2(10.0, 6.0);
    let size = text_size + pad * 2.0;

    let (rect, response) = ui.allocate_exact_size(size, Sense::click());

    if ui.is_rect_visible(rect) {
        let is_pressed = response.is_pointer_button_down_on();
        let is_hovered = response.hovered();

        let shadow_offset = vec2(2.0, 2.0);
        let press_offset = vec2(1.5, 1.5);

        let (bg_color, text_color) = if is_active {
            (HERMES_ORANGE, NOIR_JET)
        } else if is_hovered {
            (theme.fn_btn_bg(), theme.text())
        } else {
            (theme.card_bg(), theme.text_muted())
        };

        let painter = ui.painter();

        if !is_pressed {
            let shadow_rect = Rect::from_min_size(rect.min + shadow_offset, rect.size());
            painter.rect_filled(shadow_rect, CornerRadius::ZERO, theme.shadow());
        }

        let active_rect = if is_pressed {
            Rect::from_min_size(rect.min + press_offset, rect.size())
        } else {
            rect
        };

        let stroke = Stroke::new(1.8, theme.border());
        painter.rect(active_rect, CornerRadius::ZERO, bg_color, stroke, StrokeKind::Inside);

        painter.text(
            active_rect.center(),
            Align2::CENTER_CENTER,
            label,
            font_id,
            text_color,
        );
    }

    response
}
