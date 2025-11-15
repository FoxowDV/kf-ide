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
            "compile" => "Compile",
            "tools" => "Tools",
            "about" => "About",
            "new file" => "New File",
            "open" => "Open",
            "save" => "Save",
            "save all" => "Save All",
            "save as" => "Save As",
            "close file" => "Close File",
            "close all" => "Close All",
            "exit" => "Exit",
            "copy" => "Copy",
            "cut" => "Cut",
            "paste" => "Paste",
            "compile and run" => "Compile And Run",
            "open config" => "Open Config",
            "unsaved changes" => "Unsaved Changes",
            "save changes to" => "Save changes to",
            "dont save" => "Don't Save",
            "cancel" => "Cancel",
            _ => key,
        }
    }
    
    fn translate_es<'a>(&self, key: &'a str) -> &'a str {
        match key {
            "file" => "Archivo",
            "edit" => "Editar",
            "compile" => "Compilar",
            "tools" => "Herramientas",
            "about" => "Acerca de",
            "new file" => "Nuevo Archivo",
            "open" => "Abrir",
            "save" => "Guardar",
            "save all" => "Guardar Todo",
            "save as" => "Guardar Como",
            "close file" => "Cerrar Archivo",
            "close all" => "Cerrar Todo",
            "exit" => "Salir",
            "copy" => "Copiar",
            "cut" => "Cortar",
            "paste" => "Pegar",
            "compile and run" => "Compilar y Ejecutar",
            "open config" => "Abrir Configuración",
            "unsaved changes" => "Cambios Sin Guardar",
            "save changes to" => "Guardar cambios en",
            "dont save" => "No Guardar",
            "cancel" => "Cancelar",
            _ => self.translate_en(key),
        }
    }
}
