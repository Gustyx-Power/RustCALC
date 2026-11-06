use super::theme::BrutalTheme;
use super::widgets::{brutalist_button, ButtonStyle};
use egui::{vec2, Ui};

#[derive(Debug, Clone, PartialEq)]
pub enum KeyAction {
    Insert(String),
    InsertFunction(String), // inserts "func("
    Backspace,
    Clear,
    Evaluate,
    ToggleSecond,
    ToggleSign,
    MemoryClear,
    MemoryRecall,
    MemoryAdd,
    MemorySub,
    InsertAns,
}

pub struct KeypadState {
    pub is_second: bool,
}

pub fn render_keypad(
    ui: &mut Ui,
    state: &KeypadState,
    theme: &BrutalTheme,
) -> Option<KeyAction> {
    let mut triggered_action: Option<KeyAction> = None;

    let spacing = 6.0;
    let avail_width = ui.available_width() - 4.0;
    let cols = 5.0;
    let btn_width = ((avail_width - (cols - 1.0) * spacing) / cols).floor();
    let btn_height = 42.0;
    let btn_size = vec2(btn_width, btn_height);

    // Memory bar (compact row)
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing = vec2(spacing, spacing);
        let mem_cols = 4.0;
        let mem_width = ((avail_width - (mem_cols - 1.0) * spacing) / mem_cols).floor();
        let mem_size = vec2(mem_width, 28.0);

        if brutalist_button(ui, "MC", mem_size, ButtonStyle::Ghost, theme).clicked() {
            triggered_action = Some(KeyAction::MemoryClear);
        }
        if brutalist_button(ui, "MR", mem_size, ButtonStyle::Ghost, theme).clicked() {
            triggered_action = Some(KeyAction::MemoryRecall);
        }
        if brutalist_button(ui, "M+", mem_size, ButtonStyle::Ghost, theme).clicked() {
            triggered_action = Some(KeyAction::MemoryAdd);
        }
        if brutalist_button(ui, "M-", mem_size, ButtonStyle::Ghost, theme).clicked() {
            triggered_action = Some(KeyAction::MemorySub);
        }
    });

    ui.add_space(6.0);

    // Scientific Row 1: 2nd, Trig, Log
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing = vec2(spacing, spacing);

        let second_style = if state.is_second {
            ButtonStyle::Accent
        } else {
            ButtonStyle::Function
        };
        if brutalist_button(ui, "2nd", btn_size, second_style, theme).clicked() {
            triggered_action = Some(KeyAction::ToggleSecond);
        }

        let sin_label = if state.is_second { "asin" } else { "sin" };
        if brutalist_button(ui, sin_label, btn_size, ButtonStyle::Function, theme).clicked() {
            triggered_action = Some(KeyAction::InsertFunction(format!("{}(", sin_label)));
        }

        let cos_label = if state.is_second { "acos" } else { "cos" };
        if brutalist_button(ui, cos_label, btn_size, ButtonStyle::Function, theme).clicked() {
            triggered_action = Some(KeyAction::InsertFunction(format!("{}(", cos_label)));
        }

        let tan_label = if state.is_second { "atan" } else { "tan" };
        if brutalist_button(ui, tan_label, btn_size, ButtonStyle::Function, theme).clicked() {
            triggered_action = Some(KeyAction::InsertFunction(format!("{}(", tan_label)));
        }

        let ln_label = if state.is_second { "exp" } else { "ln" };
        if brutalist_button(ui, ln_label, btn_size, ButtonStyle::Function, theme).clicked() {
            triggered_action = Some(KeyAction::InsertFunction(format!("{}(", ln_label)));
        }
    });

    ui.add_space(spacing);

    // Scientific Row 2: Powers, Roots, Fact, Log10
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing = vec2(spacing, spacing);

        let log_label = if state.is_second { "10^x" } else { "log" };
        if brutalist_button(ui, log_label, btn_size, ButtonStyle::Function, theme).clicked() {
            if state.is_second {
                triggered_action = Some(KeyAction::Insert("10^(".to_string()));
            } else {
                triggered_action = Some(KeyAction::InsertFunction("log(".to_string()));
            }
        }

        let sqrt_label = if state.is_second { "cbrt" } else { "sqrt" };
        if brutalist_button(ui, sqrt_label, btn_size, ButtonStyle::Function, theme).clicked() {
            triggered_action = Some(KeyAction::InsertFunction(format!("{}(", sqrt_label)));
        }

        if brutalist_button(ui, "^", btn_size, ButtonStyle::Function, theme).clicked() {
            triggered_action = Some(KeyAction::Insert("^".to_string()));
        }

        let fact_label = if state.is_second { "abs" } else { "x!" };
        if brutalist_button(ui, fact_label, btn_size, ButtonStyle::Function, theme).clicked() {
            if state.is_second {
                triggered_action = Some(KeyAction::InsertFunction("abs(".to_string()));
            } else {
                triggered_action = Some(KeyAction::Insert("!".to_string()));
            }
        }

        if brutalist_button(ui, "%", btn_size, ButtonStyle::Function, theme).clicked() {
            triggered_action = Some(KeyAction::Insert("%".to_string()));
        }
    });

    ui.add_space(spacing);

    // Row 3: Constants & Parentheses & Clear
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing = vec2(spacing, spacing);

        if brutalist_button(ui, "π", btn_size, ButtonStyle::Function, theme).clicked() {
            triggered_action = Some(KeyAction::Insert("pi".to_string()));
        }

        if brutalist_button(ui, "e", btn_size, ButtonStyle::Function, theme).clicked() {
            triggered_action = Some(KeyAction::Insert("e".to_string()));
        }

        if brutalist_button(ui, "(", btn_size, ButtonStyle::Function, theme).clicked() {
            triggered_action = Some(KeyAction::Insert("(".to_string()));
        }

        if brutalist_button(ui, ")", btn_size, ButtonStyle::Function, theme).clicked() {
            triggered_action = Some(KeyAction::Insert(")".to_string()));
        }

        if brutalist_button(ui, "AC", btn_size, ButtonStyle::Danger, theme).clicked() {
            triggered_action = Some(KeyAction::Clear);
        }
    });

    ui.add_space(spacing);

    // Row 4: 7, 8, 9, /, DEL
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing = vec2(spacing, spacing);

        if brutalist_button(ui, "7", btn_size, ButtonStyle::Number, theme).clicked() {
            triggered_action = Some(KeyAction::Insert("7".to_string()));
        }
        if brutalist_button(ui, "8", btn_size, ButtonStyle::Number, theme).clicked() {
            triggered_action = Some(KeyAction::Insert("8".to_string()));
        }
        if brutalist_button(ui, "9", btn_size, ButtonStyle::Number, theme).clicked() {
            triggered_action = Some(KeyAction::Insert("9".to_string()));
        }
        if brutalist_button(ui, "÷", btn_size, ButtonStyle::Operator, theme).clicked() {
            triggered_action = Some(KeyAction::Insert(" / ".to_string()));
        }
        if brutalist_button(ui, "⌫", btn_size, ButtonStyle::Operator, theme).clicked() {
            triggered_action = Some(KeyAction::Backspace);
        }
    });

    ui.add_space(spacing);

    // Row 5: 4, 5, 6, *, 1/x
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing = vec2(spacing, spacing);

        if brutalist_button(ui, "4", btn_size, ButtonStyle::Number, theme).clicked() {
            triggered_action = Some(KeyAction::Insert("4".to_string()));
        }
        if brutalist_button(ui, "5", btn_size, ButtonStyle::Number, theme).clicked() {
            triggered_action = Some(KeyAction::Insert("5".to_string()));
        }
        if brutalist_button(ui, "6", btn_size, ButtonStyle::Number, theme).clicked() {
            triggered_action = Some(KeyAction::Insert("6".to_string()));
        }
        if brutalist_button(ui, "×", btn_size, ButtonStyle::Operator, theme).clicked() {
            triggered_action = Some(KeyAction::Insert(" * ".to_string()));
        }
        if brutalist_button(ui, "1/x", btn_size, ButtonStyle::Function, theme).clicked() {
            triggered_action = Some(KeyAction::Insert("1/(".to_string()));
        }
    });

    ui.add_space(spacing);

    // Row 6: 1, 2, 3, -, ANS
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing = vec2(spacing, spacing);

        if brutalist_button(ui, "1", btn_size, ButtonStyle::Number, theme).clicked() {
            triggered_action = Some(KeyAction::Insert("1".to_string()));
        }
        if brutalist_button(ui, "2", btn_size, ButtonStyle::Number, theme).clicked() {
            triggered_action = Some(KeyAction::Insert("2".to_string()));
        }
        if brutalist_button(ui, "3", btn_size, ButtonStyle::Number, theme).clicked() {
            triggered_action = Some(KeyAction::Insert("3".to_string()));
        }
        if brutalist_button(ui, "−", btn_size, ButtonStyle::Operator, theme).clicked() {
            triggered_action = Some(KeyAction::Insert(" - ".to_string()));
        }
        if brutalist_button(ui, "ANS", btn_size, ButtonStyle::Function, theme).clicked() {
            triggered_action = Some(KeyAction::InsertAns);
        }
    });

    ui.add_space(spacing);

    // Row 7: +/-, 0, ., +, and giant =
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing = vec2(spacing, spacing);

        if brutalist_button(ui, "±", btn_size, ButtonStyle::Number, theme).clicked() {
            triggered_action = Some(KeyAction::ToggleSign);
        }
        if brutalist_button(ui, "0", btn_size, ButtonStyle::Number, theme).clicked() {
            triggered_action = Some(KeyAction::Insert("0".to_string()));
        }
        if brutalist_button(ui, ".", btn_size, ButtonStyle::Number, theme).clicked() {
            triggered_action = Some(KeyAction::Insert(".".to_string()));
        }
        if brutalist_button(ui, "+", btn_size, ButtonStyle::Operator, theme).clicked() {
            triggered_action = Some(KeyAction::Insert(" + ".to_string()));
        }

        // Hermès Orange accent '='
        if brutalist_button(ui, "=", btn_size, ButtonStyle::Accent, theme).clicked() {
            triggered_action = Some(KeyAction::Evaluate);
        }
    });

    triggered_action
}
