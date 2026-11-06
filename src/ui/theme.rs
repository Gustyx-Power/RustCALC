use egui::Color32;

pub const HERMES_ORANGE: Color32 = Color32::from_rgb(243, 112, 33);
pub const HERMES_ORANGE_HOVER: Color32 = Color32::from_rgb(250, 130, 58);
pub const HERMES_ORANGE_DARK: Color32 = Color32::from_rgb(215, 90, 15);

pub const NOIR_JET: Color32 = Color32::from_rgb(10, 10, 10);
pub const ALABASTER: Color32 = Color32::from_rgb(247, 245, 240);
pub const CONCRETE: Color32 = Color32::from_rgb(228, 225, 218);
pub const PURE_WHITE: Color32 = Color32::from_rgb(255, 255, 255);
pub const MUTED_GRAY: Color32 = Color32::from_rgb(105, 105, 100);

pub const DARK_BG: Color32 = Color32::from_rgb(18, 18, 18);
pub const DARK_CARD: Color32 = Color32::from_rgb(28, 28, 28);
pub const DARK_BUTTON_FN: Color32 = Color32::from_rgb(40, 40, 40);
pub const DARK_BUTTON_NUM: Color32 = Color32::from_rgb(52, 52, 52);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BrutalTheme {
    AlabasterLight,
    OnyxDark,
}

impl BrutalTheme {
    pub fn bg(&self) -> Color32 {
        match self {
            BrutalTheme::AlabasterLight => ALABASTER,
            BrutalTheme::OnyxDark => DARK_BG,
        }
    }

    pub fn card_bg(&self) -> Color32 {
        match self {
            BrutalTheme::AlabasterLight => CONCRETE,
            BrutalTheme::OnyxDark => DARK_CARD,
        }
    }

    pub fn display_bg(&self) -> Color32 {
        match self {
            BrutalTheme::AlabasterLight => PURE_WHITE,
            BrutalTheme::OnyxDark => Color32::from_rgb(10, 10, 10),
        }
    }

    pub fn num_btn_bg(&self) -> Color32 {
        match self {
            BrutalTheme::AlabasterLight => PURE_WHITE,
            BrutalTheme::OnyxDark => DARK_BUTTON_NUM,
        }
    }

    pub fn fn_btn_bg(&self) -> Color32 {
        match self {
            BrutalTheme::AlabasterLight => CONCRETE,
            BrutalTheme::OnyxDark => DARK_BUTTON_FN,
        }
    }

    pub fn accent_btn_bg(&self) -> Color32 {
        HERMES_ORANGE
    }

    pub fn border(&self) -> Color32 {
        match self {
            BrutalTheme::AlabasterLight => NOIR_JET,
            BrutalTheme::OnyxDark => Color32::from_rgb(230, 230, 230),
        }
    }

    pub fn shadow(&self) -> Color32 {
        match self {
            BrutalTheme::AlabasterLight => NOIR_JET,
            BrutalTheme::OnyxDark => Color32::from_rgb(0, 0, 0),
        }
    }

    pub fn text(&self) -> Color32 {
        match self {
            BrutalTheme::AlabasterLight => NOIR_JET,
            BrutalTheme::OnyxDark => PURE_WHITE,
        }
    }

    pub fn text_muted(&self) -> Color32 {
        match self {
            BrutalTheme::AlabasterLight => MUTED_GRAY,
            BrutalTheme::OnyxDark => Color32::from_rgb(160, 160, 155),
        }
    }

    pub fn toggle(&self) -> Self {
        match self {
            BrutalTheme::AlabasterLight => BrutalTheme::OnyxDark,
            BrutalTheme::OnyxDark => BrutalTheme::AlabasterLight,
        }
    }
}

