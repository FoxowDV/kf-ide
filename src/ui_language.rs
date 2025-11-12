use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum UiLanguage {
    English,
    Spanish,
}

impl UiLanguage {
    pub fn display_name(&self) -> &str {
        match self {
            UiLanguage::English => "English",
            UiLanguage::Spanish => "Español",
        }
    }
    
    pub fn all() -> Vec<UiLanguage> {
        vec![
            UiLanguage::English,
            UiLanguage::Spanish,
        ]
    }
}

impl Default for UiLanguage {
    fn default() -> Self {
        UiLanguage::English
    }
}

