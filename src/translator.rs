use crate::ui_language::UiLanguage;

pub struct Translator {
    language: UiLanguage
}

impl ::std::default::Default for Translator {
    fn default() -> Self { 
        Self { language: UiLanguage::English }
    }
}


impl Translator {
    pub fn new(language: UiLanguage) -> Self {
        Self { language }
    }

    pub fn t<'a>(&self, key: &'a str) -> &'a str {
        match self.language {
            UiLanguage::English => self.translate_en(key),
            UiLanguage::Spanish => self.translate_es(key),
        }
    }
    
    fn translate_en<'a>(&self, key: &'a str) -> &'a str {
        match key {
            "file" => "File",
            "edit" => "Edit",
            "view" => "View",
            "settings" => "Settings",
            "save" => "Save",
            "open" => "Open",
            "close" => "Close",
            "new file" => "New File",
            _ => key,
        }
    }
    
    fn translate_es<'a>(&self, key: &'a str) -> &'a str {
        match key {
            "file" => "Archivo",
            "edit" => "Editar",
            "view" => "Ver",
            "settings" => "Configuración",
            "save" => "Guardar",
            "open" => "Abrir",
            "close" => "Cerrar",
            "new file" => "Nuevo Archivo",
            _ => self.translate_en(key),
        }
    }
}
