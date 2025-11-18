use crate::ui_language::UiLanguage;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub font: String,
    pub style: String,
    pub size: u32,
    pub language: UiLanguage,
    pub background: egui::Color32,
    pub cursor: egui::Color32,
    pub selection: egui::Color32,
    pub general: egui::Color32,
    pub keywords: egui::Color32,
    pub comments: egui::Color32,
    pub identifiers: egui::Color32,
    pub numerics: egui::Color32,
    pub punctuation: egui::Color32,
    pub strings: egui::Color32,
    pub functions: egui::Color32,
    pub types: egui::Color32,
    pub special: egui::Color32,
    pub work_path: String,
    pub assembler_path: String,
    pub dos_emulator_path: String,
}

impl ::std::default::Default for Config {
    fn default() -> Self { 
        Self { 
            font: "Consola".into(),
            style: "Regular".into(),
            size: 12,
            language: UiLanguage::default(),

            background: egui::Color32::WHITE,
            cursor: egui::Color32::BLACK,
            selection: egui::Color32::from_rgb(200, 200, 255),

            general: egui::Color32::BLACK,
            keywords: egui::Color32::from_rgb(0, 102, 204),
            comments: egui::Color32::from_rgb(106, 153, 85),
            identifiers: egui::Color32::from_rgb(0, 0, 0),
            numerics: egui::Color32::from_rgb(180, 0, 0),
            punctuation: egui::Color32::from_rgb(50, 50, 50),
            strings: egui::Color32::from_rgb(180, 0, 180),
            functions: egui::Color32::from_rgb(128, 0, 128),
            types: egui::Color32::from_rgb(0, 128, 128),
            special: egui::Color32::from_rgb(255, 0, 0),

            work_path: "./".into(),
            assembler_path: "./TASM".into(),
            dos_emulator_path: "./DOSBox".into(),
        } 
    }
}


impl Config {
    pub fn save(&mut self, new: Config) {
        confy::store("kf-ide", &new).ok();
        *self = new;
    }
}

