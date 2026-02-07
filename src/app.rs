use egui_code_editor::{Completer, Syntax};
use crate::syntax::kf::SyntaxExt;

mod menus;
mod ui;
mod tables;
mod config_window;

use config_window::ConfigTab;

use eframe::egui;
use egui_file_dialog::FileDialog;

use std::{
    path::PathBuf, 
    sync::Arc,
};

use crate::document::Document;

use crate::Config;
use crate::translator::Translator;

use kf_compiler::parser::parser::ParseError;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PathType {
    WorkPath,
    AssemblerPath,
    DosEmulatorPath,
}


#[derive(Default)]
pub struct App {
    pub documents: Vec<Document>,
    pub active_tab: usize,
    pub config_tab: ConfigTab,
    pub next_document_id: usize,
    pub compile_errors: Vec<ParseError>,  
    pub output_content: String, 
    document_to_save_index: Option<usize>,
    is_modal_open: bool,
    is_closing: bool,
    is_saving: bool,
    file_dialog: FileDialog,
    picked_file: Option<PathBuf>,
    c_line: usize,
    c_col: usize,
    is_config_open: bool,
    config: Config,
    translator: Translator,
    completer: Completer,
    selected_text: String,
    error_line: Option<usize>,
    current_path_type: Option<PathType>,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>, cfg: Config) -> Self {

        Self::setup_custom_fonts(&cc.egui_ctx);
        
        let syntax = Syntax::kf();
        let mut app = Self::default();
        app.config = cfg;
        app.file_dialog = Self::create_file_dialog("Program.kf", &app.config.work_path);
        app.picked_file = None;
        app.is_modal_open = false;
        app.add_document("Program1.kf".to_string());
        app.translator = Translator::new(app.config.language);
        app.completer = Completer::new_with_syntax(&syntax).with_user_words();
        app.compile_errors = Vec::new();
        app.output_content = String::new();
        app
    }

    fn setup_custom_fonts(ctx: &egui::Context) {
        let mut fonts = egui::FontDefinitions::default();

        fonts.font_data.insert(
            "Arial-Regular".to_owned(),
            egui::FontData::from_static(include_bytes!("../fonts/arial.ttf")).into(),
        );

        fonts.font_data.insert(
            "Arial-Bold".to_owned(),
            egui::FontData::from_static(include_bytes!("../fonts/arialbd.ttf")).into(),
        );

        fonts.font_data.insert(
            "Arial-Italic".to_owned(),
            egui::FontData::from_static(include_bytes!("../fonts/ariali.ttf")).into(),
        );

        fonts.font_data.insert(
            "Consola-Regular".to_owned(),
            egui::FontData::from_static(include_bytes!("../fonts/CONSOLA.ttf")).into(),
        );

        fonts.font_data.insert(
            "Consola-Bold".to_owned(),
            egui::FontData::from_static(include_bytes!("../fonts/CONSOLAB.ttf")).into(),
        );

        fonts.font_data.insert(
            "Consola-Italic".to_owned(),
            egui::FontData::from_static(include_bytes!("../fonts/CONSOLAI.ttf")).into(),
        );

        fonts.font_data.insert(
            "Ubuntu-Regular".to_owned(),
            egui::FontData::from_static(include_bytes!("../fonts/ubuntu.regular.ttf")).into(),
        );

        fonts.font_data.insert(
            "Ubuntu-Bold".to_owned(),
            egui::FontData::from_static(include_bytes!("../fonts/ubuntu.bold.ttf")).into(),
        );

        fonts.font_data.insert(
            "Ubuntu-Italic".to_owned(),
            egui::FontData::from_static(include_bytes!("../fonts/ubuntu.italic.ttf")).into(),
        );

        fonts.font_data.insert(
            "Hack-Regular".to_owned(),
            egui::FontData::from_static(include_bytes!("../fonts/Hack-Regular.ttf")).into(),
        );

        fonts.font_data.insert(
            "Hack-Bold".to_owned(),
            egui::FontData::from_static(include_bytes!("../fonts/Hack-Bold.ttf")).into(),
        );

        fonts.font_data.insert(
            "Hack-Italic".to_owned(),
            egui::FontData::from_static(include_bytes!("../fonts/Hack-Italic.ttf")).into(),
        );

        ctx.set_fonts(fonts);
    }

    fn update_font_p(&mut self, ctx: &egui::Context) {
        let mut fonts = ctx.fonts(|f| f.definitions().clone());
        
        let font_key = format!("{}-{}", self.config.font, self.config.style);
        
        fonts.families.get_mut(&egui::FontFamily::Monospace).unwrap().clear();
        fonts.families.get_mut(&egui::FontFamily::Monospace).unwrap().push(font_key);
        
        ctx.set_fonts(fonts);
    }



    pub fn add_document(&mut self, name: String) {
        self.documents.push(Document::new(name));
        self.active_tab = self.documents.len() - 1;
    }

}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        let mut new_style = (*ctx.style()).clone();
        new_style.visuals.panel_fill = egui::Color32::from_rgb(245, 245, 245);
        new_style.visuals.override_text_color = Some(egui::Color32::from_rgb(0, 0, 0));
        ctx.set_style(new_style);


        self.show_top_panel(ctx);
        self.file_dialog.update(ctx);

        if let Some(path) = self.file_dialog.take_picked() {
            if path.exists() && !self.is_saving {
                
                if path.is_dir() {
                    let path_str = path.to_string_lossy().to_string();
                    if let Some(path_type) = self.current_path_type.take() {
                        match path_type {
                            PathType::WorkPath => self.config.work_path = path_str,
                            PathType::AssemblerPath => self.config.assembler_path = path_str,
                            PathType::DosEmulatorPath => self.config.dos_emulator_path = path_str,
                        }
                    }
                }
                else {
                    match std::fs::read_to_string(&path) {
                        Ok(content) => {
                            let mut new_doc = Document::new(
                                path.file_name()
                                    .and_then(|n| n.to_str())
                                    .unwrap()
                                    .to_string(),
                            );
                            new_doc.content = content;
                            new_doc.file_path = Some(path.clone());
                            new_doc.is_modified = false;
                            self.documents.push(new_doc);
                            self.active_tab = self.documents.len() - 1;
                        }
                        Err(e) => {
                            eprintln!("Error al abrir el archivo: {}", e);
                        }
                    }
                }
            } else {
                self.is_saving = false;
                let index = self.document_to_save_index.unwrap_or(self.active_tab);
                if let Some(doc) = self.documents.get_mut(index) {
                    if let Err(e) = std::fs::write(&path, &doc.content) {
                        eprintln!("Error guardando {}", e);
                    } else {
                        doc.is_modified = false;
                        doc.file_path = Some(path.clone());
                        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                            doc.name = name.to_string();
                        }
                        self.document_to_save_index = None;
                        if self.is_closing {
                            self.check_for_close(ctx);
                        }
                    }
                }
            }
        }

        if self.is_modal_open {
            if let Some(i) = self.document_to_save_index {
                egui::Window::new("save_modal").show(ctx, |ui| {
                    ui.heading(self.translator.t("unsaved changes"));

                    ui.add_space(10.0);
                    ui.label(format!("{} '{}'?", self.translator.t("save changes to"), &self.documents[i].name));
                    ui.add_space(20.0);

                    ui.horizontal(|ui| {
                        if ui.button(self.translator.t("save")).clicked() {
                            if self.documents[i].file_path.is_some() {
                                if self.save_file(i) {
                                    self.documents[i].is_modified = false;
                                }
                                self.is_modal_open = false;
                                self.document_to_save_index = None;
                            } else {
                                self.is_modal_open = false;
                                self.is_saving = true;
                                self.file_dialog.save_file();
                            }
                        }
                        if ui.button(self.translator.t("dont save")).clicked() {
                            self.documents[i].is_modified = false;
                            self.is_modal_open = false;
                            self.document_to_save_index = None;
                            self.check_for_close(&ctx);
                        }
                        if ui.button(self.translator.t("cancel")).clicked() {
                            self.is_modal_open = false;
                            self.document_to_save_index = None;
                            self.is_closing = false;
                        }
                    });
                });
            }
        }

        if self.is_closing && !self.is_modal_open {
            if !self.check_for_close(ctx) {}
        }


        self.show_tables(ctx);

        self.show_bottom(ctx);

        self.show_tabs(ctx);
        self.show_central_panel(ctx);

        if self.is_config_open {
            self.show_config_window(ctx)
        }

    }
}

impl App {
    fn new_file(&mut self) {
        self.next_document_id += 1;
        let name = format!("Program{}.kf", self.next_document_id + 1);
        self.add_document(name);
    }

    fn close_file(&mut self, index: usize) {
        if self.documents.get_mut(index).unwrap().is_modified {
            self.document_to_save_index = Some(index); 
            self.is_modal_open = true;
        }else{
        if self.documents.len() > 1 && index < self.documents.len() {
            self.documents.remove(index);
            if self.active_tab >= self.documents.len() {
                self.active_tab = self.documents.len() - 1;
            } else if self.active_tab > index {
                self.active_tab -= 1;
            }
        } else if index < self.documents.len() {
            self.documents.remove(index);
            self.next_document_id += 1;
            let name = format!("Program{}.kf", self.next_document_id + 1);
            self.documents.push(Document::new(name));
            self.active_tab = self.documents.len() - 1;
        }
        }
    }

    fn open_file(&mut self) {
        self.file_dialog.pick_file();
    }

    fn save_file(&mut self, index: usize) -> bool {
        if let Some(doc) = self.documents.get_mut(index) {
            if let Some(path) = &doc.file_path {
                if let Err(e) = std::fs::write(path, &doc.content) {
                    eprintln!("Error guardando {}", e);
                    return false;
                } else {
                    doc.is_modified = false;
                    return true;
                }
            } else {
                self.is_saving = true;

                self.file_dialog = Self::create_file_dialog(&doc.name, &self.config.work_path);

                self.file_dialog.save_file();
                return false;
            }
        }
        return false;
    }

    fn check_for_close(&mut self, ctx: &egui::Context) -> bool {
        for (i, doc) in self.documents.iter().enumerate() {
            if doc.is_modified {
                self.document_to_save_index = Some(i);
                self.is_modal_open = true;
                return false;
            }
        }
        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        true
    }

    fn create_file_dialog(filename: &str, work_path: &str)-> FileDialog {
        FileDialog::new()
            .add_file_filter(
                "KF",
                Arc::new(|p| p.extension().unwrap_or_default() == "kf"),
            )
            .default_file_filter("KF")
            .initial_directory(PathBuf::from(work_path))
            .default_file_name(filename)
    }
}
