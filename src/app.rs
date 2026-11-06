use crate::engine::{calculate, format_result, AngleMode, EvalContext};
use crate::ui::display::{render_display, DisplayState};
use crate::ui::history::{render_drawer, DrawerAction, DrawerTab, HistoryItem};
use crate::ui::keypad::{render_keypad, KeyAction, KeypadState};
use crate::ui::theme::BrutalTheme;
use crate::ui::widgets::brutalist_badge;
use egui::{Align, CornerRadius, Key, Layout, Ui};

pub struct CalculatorApp {
    pub expression: String,
    pub result: String,
    pub is_error: bool,
    pub error_msg: Option<String>,
    pub eval_ctx: EvalContext,
    pub keypad_state: KeypadState,
    pub theme: BrutalTheme,
    pub history: Vec<HistoryItem>,
    pub drawer_tab: DrawerTab,
    pub is_drawer_open: bool,
    pub copied_timer: f32,
    pub just_evaluated: bool,
}

impl Default for CalculatorApp {
    fn default() -> Self {
        Self {
            expression: String::new(),
            result: "0".to_string(),
            is_error: false,
            error_msg: None,
            eval_ctx: EvalContext::default(),
            keypad_state: KeypadState { is_second: false },
            theme: BrutalTheme::AlabasterLight,
            history: Vec::new(),
            drawer_tab: DrawerTab::History,
            is_drawer_open: false,
            copied_timer: 0.0,
            just_evaluated: false,
        }
    }
}

impl CalculatorApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }

    pub fn execute_action(&mut self, action: KeyAction) {
        match action {
            KeyAction::Insert(s) => {
                if self.is_error || (self.just_evaluated && s.chars().all(|c| c.is_ascii_digit() || c == '.')) {
                    self.expression.clear();
                    self.is_error = false;
                    self.error_msg = None;
                } else if self.just_evaluated && matches!(s.trim(), "+" | "-" | "*" | "/" | "^" | "%") {
                    self.expression = format_result(self.eval_ctx.ans);
                }
                self.just_evaluated = false;
                self.expression.push_str(&s);
                self.live_preview();
            }
            KeyAction::InsertFunction(s) => {
                if self.is_error || self.just_evaluated {
                    self.expression.clear();
                    self.is_error = false;
                    self.error_msg = None;
                }
                self.just_evaluated = false;
                self.expression.push_str(&s);
                self.keypad_state.is_second = false;
                self.live_preview();
            }
            KeyAction::Backspace => {
                self.just_evaluated = false;
                if self.is_error {
                    self.expression.clear();
                    self.is_error = false;
                    self.error_msg = None;
                } else if !self.expression.is_empty() {
                    self.expression.pop();
                    self.live_preview();
                }
            }
            KeyAction::Clear => {
                self.expression.clear();
                self.result = "0".to_string();
                self.is_error = false;
                self.error_msg = None;
                self.just_evaluated = false;
                crate::trim_memory();
            }
            KeyAction::Evaluate => {
                self.evaluate_final();
            }
            KeyAction::ToggleSecond => {
                self.keypad_state.is_second = !self.keypad_state.is_second;
            }
            KeyAction::ToggleSign => {
                self.just_evaluated = false;
                if !self.expression.is_empty() {
                    if self.expression.starts_with("-(") && self.expression.ends_with(')') {
                        self.expression = self.expression[2..self.expression.len() - 1].to_string();
                    } else {
                        self.expression = format!("-({})", self.expression);
                    }
                    self.live_preview();
                }
            }
            KeyAction::MemoryClear => {
                self.eval_ctx.memory = 0.0;
            }
            KeyAction::MemoryRecall => {
                self.just_evaluated = false;
                let mem_str = format_result(self.eval_ctx.memory);
                self.expression.push_str(&mem_str);
                self.live_preview();
            }
            KeyAction::MemoryAdd => {
                if let Ok(val) = self.eval_current() {
                    self.eval_ctx.memory += val;
                }
            }
            KeyAction::MemorySub => {
                if let Ok(val) = self.eval_current() {
                    self.eval_ctx.memory -= val;
                }
            }
            KeyAction::InsertAns => {
                self.just_evaluated = false;
                self.expression.push_str("ans");
                self.live_preview();
            }
        }
    }

    fn eval_current(&self) -> Result<f64, String> {
        let input = if self.expression.trim().is_empty() {
            "0"
        } else {
            self.expression.as_str()
        };
        calculate(input, &self.eval_ctx)
    }

    fn live_preview(&mut self) {
        let input = self.expression.trim();
        if input.is_empty() {
            self.result = "0".to_string();
            self.is_error = false;
            self.error_msg = None;
            return;
        }

        match calculate(input, &self.eval_ctx) {
            Ok(val) => {
                self.result = format_result(val);
                self.is_error = false;
                self.error_msg = None;
            }
            Err(_) => {
                // In live preview mode while typing, ignore partial syntax errors
            }
        }
    }

    fn evaluate_final(&mut self) {
        let input = self.expression.trim();
        if input.is_empty() {
            return;
        }

        // Auto-close unclosed parentheses before evaluating
        let open_count = input.chars().filter(|&c| c == '(').count();
        let close_count = input.chars().filter(|&c| c == ')').count();
        if open_count > close_count {
            for _ in 0..(open_count - close_count) {
                self.expression.push(')');
            }
        }

        let final_input = self.expression.trim();

        match calculate(final_input, &self.eval_ctx) {
            Ok(val) => {
                let formatted = format_result(val);
                self.result = formatted.clone();
                self.eval_ctx.ans = val;
                self.is_error = false;
                self.error_msg = None;
                self.just_evaluated = true;

                // Add to history
                self.history.push(HistoryItem {
                    expression: self.expression.clone(),
                    result: formatted,
                });

                // Immediate memory purge after computation
                crate::trim_memory();
            }
            Err(e) => {
                self.is_error = true;
                self.error_msg = Some(e);
            }
        }
    }

    fn handle_keyboard_input(&mut self, ctx: &egui::Context) {
        let mut actions: Vec<KeyAction> = Vec::new();

        ctx.input(|i| {
            for event in &i.events {
                match event {
                    egui::Event::Text(text) => {
                        for c in text.chars() {
                            match c {
                                '0'..='9' => actions.push(KeyAction::Insert(c.to_string())),
                                'a'..='z' | 'A'..='Z' => actions.push(KeyAction::Insert(c.to_lowercase().to_string())),
                                '.' | ',' => actions.push(KeyAction::Insert(".".to_string())),
                                '+' => actions.push(KeyAction::Insert(" + ".to_string())),
                                '-' => actions.push(KeyAction::Insert(" - ".to_string())),
                                '*' => actions.push(KeyAction::Insert(" * ".to_string())),
                                '/' => actions.push(KeyAction::Insert(" / ".to_string())),
                                '^' => actions.push(KeyAction::Insert("^".to_string())),
                                '%' => actions.push(KeyAction::Insert("%".to_string())),
                                '!' => actions.push(KeyAction::Insert("!".to_string())),
                                '(' => actions.push(KeyAction::Insert("(".to_string())),
                                ')' => actions.push(KeyAction::Insert(")".to_string())),
                                '=' => actions.push(KeyAction::Evaluate),
                                _ => {}
                            }
                        }
                    }
                    egui::Event::Paste(pasted) => {
                        let sanitized: String = pasted
                            .chars()
                            .filter(|c| c.is_ascii_alphanumeric() || matches!(c, '+' | '-' | '*' | '/' | '^' | '%' | '!' | '(' | ')' | '.' | ' ' | 'π' | 'τ' | 'φ'))
                            .collect();
                        if !sanitized.is_empty() {
                            actions.push(KeyAction::Insert(sanitized));
                        }
                    }
                    egui::Event::Key { key, pressed: true, .. } => {
                        match key {
                            Key::Enter | Key::Equals => actions.push(KeyAction::Evaluate),
                            Key::Backspace => actions.push(KeyAction::Backspace),
                            Key::Escape => actions.push(KeyAction::Clear),
                            Key::Delete => actions.push(KeyAction::Clear),
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
        });

        for act in actions {
            self.execute_action(act);
        }
    }
}

impl eframe::App for CalculatorApp {
    fn ui(&mut self, ui: &mut Ui, _frame: &mut eframe::Frame) {
        // Decrease copied timer
        if self.copied_timer > 0.0 {
            self.copied_timer -= ui.ctx().input(|i| i.stable_dt);
            ui.ctx().request_repaint();
        }

        // Handle physical keyboard and text input events
        self.handle_keyboard_input(ui.ctx());

        // Paint background covering whole window
        let max_rect = ui.max_rect();
        ui.painter().rect_filled(max_rect, CornerRadius::ZERO, self.theme.bg());

        // Content padding inside brutalist window
        let content_rect = max_rect.shrink(14.0);
        let mut padded_ui = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(content_rect)
                .layout(Layout::top_down(Align::Min)),
        );

        let ui = &mut padded_ui;

        // Top Brutalist Header
        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new("RustCALC")
                    .font(egui::FontId::monospace(14.0))
                    .strong()
                    .color(self.theme.text()),
            );

            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                // Theme toggle badge
                let theme_label = match self.theme {
                    BrutalTheme::AlabasterLight => "ALABASTER",
                    BrutalTheme::OnyxDark => "ONYX",
                };
                if brutalist_badge(ui, theme_label, false, &self.theme).clicked() {
                    self.theme = self.theme.toggle();
                }

                // Drawer toggle badge
                let drawer_label = if self.is_drawer_open { "[HIDE REC]" } else { "[REC/CONST]" };
                if brutalist_badge(ui, drawer_label, self.is_drawer_open, &self.theme).clicked() {
                    self.is_drawer_open = !self.is_drawer_open;
                }

                // Angle mode badge
                let is_deg = self.eval_ctx.angle_mode == AngleMode::Deg;
                if brutalist_badge(ui, self.eval_ctx.angle_mode.label(), is_deg, &self.theme).clicked() {
                    self.eval_ctx.angle_mode = self.eval_ctx.angle_mode.toggle();
                    self.live_preview();
                }

                // Copy button
                if brutalist_badge(ui, "COPY", false, &self.theme).clicked() {
                    ui.ctx().copy_text(self.result.clone());
                    self.copied_timer = 1.5;
                }
            });
        });

        ui.add_space(10.0);

        // Screen Display
        let display_state = DisplayState {
            expression: &self.expression,
            result: &self.result,
            is_error: self.is_error,
            error_msg: self.error_msg.as_deref(),
            angle_mode_label: self.eval_ctx.angle_mode.label(),
            memory_value: self.eval_ctx.memory,
            copied_feedback: self.copied_timer > 0.0,
        };
        render_display(ui, &display_state, &self.theme);

        ui.add_space(12.0);

        // Optional Collapsible Drawer (Tape Archive, Constants, and Unit Converter)
        if self.is_drawer_open {
            let cur_val = self.eval_ctx.ans;
            if let Some(action) = render_drawer(ui, &self.history, &self.drawer_tab, cur_val, &self.theme) {
                match action {
                    DrawerAction::SwitchTab(t) => {
                        self.drawer_tab = t;
                    }
                    DrawerAction::SelectHistory(res) => {
                        self.expression.push_str(&res);
                        self.live_preview();
                    }
                    DrawerAction::InsertConstant(c) => {
                        self.expression.push_str(&c);
                        self.live_preview();
                    }
                    DrawerAction::ClearHistory => {
                        self.history.clear();
                    }
                }
            }
            ui.add_space(10.0);
        }

        // Main Keypad Matrix
        if let Some(action) = render_keypad(ui, &self.keypad_state, &self.theme) {
            self.execute_action(action);
        }
    }
}
