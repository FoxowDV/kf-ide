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
            "save changes to" => "Save Changes To",
            "dont save" => "Don't Save",
            "cancel" => "Cancel",
            "accept" => "Accept",
            "style" => "Style",
            "font" => "Font",
            "compilation" => "Compilation",
            "configuration" => "Configuration",
            "example" => "Example",
            "size" => "Size",
            "language" => "Language",
            "color" => "Color",
            "background" => "Background",
            "cursor" => "Cursor",
            "selection" => "Selection",
            "general" => "General",
            "reserved words" => "Reserved Words",
            "coments" => "Coments",
            "identifiers" => "Identifiers",
            "identifier" => "Identifier",
            "numbers" => "Numbers",
            "chains" => "Chains",
            "punctuation marks" => "Punctuation Marks",
            "hello" => "Hello",
            "world" => "World",
            "line" => "Line",
            "column" => "Column",
            "data type" => "Data Type",
            "value" => "Value",
            "program" => "program",
            "example1" => "Example1",
            "var" => "var",
            "age" => "age",
            "byte" => "byte",
            "begin" => "begin",
            "writeln" => "writeln",
            "welcome" => "Welcome...",
            "end" => "end",
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
            "save changes to" => "Guardar Cambios En",
            "dont save" => "No Guardar",
            "cancel" => "Cancelar",
            "accept" => "Aceptar",
            "style" => "Estilo",
            "font" => "Fuente",
            "compilation" => "Compilacion",
            "configuration" => "Configuracion",
            "example" => "Ejemplo",
            "size" => "Tamaño",
            "language" => "Idioma",
            "color" => "Color",
            "background" => "Fondo",
            "cursor" => "Cursor",
            "selection" => "Seleccion",
            "general" => "General",
            "reserved words" => "Palabras Reservadas",
            "coments" => "Comentarios",
            "identifiers" => "Identificadores",
            "identifier" => "Identificador",
            "numbers" => "Numeros",
            "chains" => "Cadenas",
            "punctuation marks" => "Signos De Puntuacion",
            "hello" => "Hola",
            "world" => "Mundo",
            "line" => "Linea",
            "column" => "Columna",
            "data type" => "Tipo De Dato",
            "value" => "Valor",
            "program" => "programa",
            "example1" => "Ejemplo1",
            "var" => "var",
            "age" => "edad",
            "byte" => "byte",
            "begin" => "begin",
            "writeln" => "writeln",
            "welcome" => "Bienvenido...",
            "end" => "end",
            _ => self.translate_en(key),
        }
    }
}
