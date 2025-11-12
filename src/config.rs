use crate::ui_language::UiLanguage;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    font: String,
    style: String,
    size: u32,
    language: UiLanguage,
    general: egui::Color32,
    keywords: egui::Color32,
    comments: egui::Color32,
    identifiers: egui::Color32,
    integers: egui::Color32,
    floats: egui::Color32,
    punctuation: egui::Color32,
    work_path: String,
    assember_path: String,
    dos_emulator_path: String,
}

impl ::std::default::Default for Config {
    fn default() -> Self { 
        Self { 
            font: "Consolas".into(),
            style: "plain".into(),
            size: 12,
            language: UiLanguage::default(),
            general: egui::Color32::from_rgb(220, 220, 220),
            keywords: egui::Color32::from_rgb(86, 156, 214),
            comments: egui::Color32::from_rgb(106, 153, 85),
            identifiers: egui::Color32::from_rgb(156, 220, 254),
            integers: egui::Color32::from_rgb(181, 206, 168),
            floats: egui::Color32::from_rgb(181, 206, 168),
            punctuation: egui::Color32::from_rgb(212, 212, 212),
            work_path: "./".into(),
            assember_path: "./TASM".into(),
            dos_emulator_path: "./DOSBox".into(),
        } 
    }
}


impl Config {
    pub fn save(&mut self, new: Config) {
        confy::store("kf-ide", &new).ok();
        *self = new;
    }

    pub fn get_lang(&self) -> UiLanguage {
        self.language
    }
}

