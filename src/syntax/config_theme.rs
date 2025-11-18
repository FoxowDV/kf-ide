use egui_code_editor::ColorTheme;
use crate::Config;

pub fn make_theme_from_config(config: &Config) -> ColorTheme {
    let bg = Box::leak(color32_to_hex(config.background).into_boxed_str());
    let fg = Box::leak(color32_to_hex(config.general).into_boxed_str());
    let cursor = Box::leak(color32_to_hex(config.cursor).into_boxed_str());
    let selection = Box::leak(color32_to_hex(config.selection).into_boxed_str());

    let mut theme = ColorTheme::monocolor(false, bg, fg, cursor, selection);

    theme.name = "KF";
    theme.dark = false;

    //theme.bg          = Box::leak(color32_to_hex(config.general).into_boxed_str());
    theme.keywords    = Box::leak(color32_to_hex(config.keywords).into_boxed_str());
    theme.comments    = Box::leak(color32_to_hex(config.comments).into_boxed_str());
    theme.literals    = Box::leak(color32_to_hex(config.identifiers).into_boxed_str());
    theme.numerics    = Box::leak(color32_to_hex(config.numerics).into_boxed_str());
    theme.punctuation = Box::leak(color32_to_hex(config.punctuation).into_boxed_str());
    theme.strs       = Box::leak(color32_to_hex(config.strings).into_boxed_str());
    theme.functions   = Box::leak(color32_to_hex(config.functions).into_boxed_str());
    theme.types       = Box::leak(color32_to_hex(config.types).into_boxed_str());
    theme.special     = Box::leak("#ff0000".to_string().into_boxed_str());

    theme
}

fn color32_to_hex(c: egui::Color32) -> String {
    format!("#{:02x}{:02x}{:02x}", c.r(), c.g(), c.b())
}
