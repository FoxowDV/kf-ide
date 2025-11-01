use eframe::egui;
use egui_file_dialog::FileDialog;

use std::{
    path::PathBuf, 
    sync::Arc
};

use crate::document::Document;

mod menus;
mod ui;
mod tables;

#[derive(Default)]
pub struct App {
    pub documents: Vec<Document>,
    pub active_tab: usize,
    pub next_document_id: usize,
    document_to_save_index: Option<usize>,
    is_modal_open: bool,
    is_closing: bool,
    is_saving: bool,
    file_dialog: FileDialog,
    picked_file: Option<PathBuf>,
    c_line: usize,
    c_col: usize,
}
impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut app = Self::default();

        // archivos
        app.file_dialog = Self::create_file_dialog("Program.kf");
        //app.file_dialog = file_dialog;
        app.picked_file = None;
        app.is_modal_open = false;
        app.add_document("Program1.kf".to_string());
        app
    }

    pub fn add_document(&mut self, name: String) {
        self.documents.push(Document::new(name));
        self.active_tab = self.documents.len() - 1;
    }

    pub fn get_active_document_mut(&mut self) -> Option<&mut Document> {
        self.documents.get_mut(self.active_tab)
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        let mut new_style = (*ctx.style()).clone();
        // fondo
        new_style.visuals.panel_fill = egui::Color32::from_rgb(245, 245, 245);
        // fuente
        new_style.visuals.override_text_color = Some(egui::Color32::from_rgb(0, 0, 0));
        ctx.set_style(new_style);


        self.show_top_panel(ctx);
        self.file_dialog.update(ctx);

        if let Some(path) = self.file_dialog.take_picked() {
            // Si existe abre si no guarda
            if path.exists() && !self.is_saving {
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
                    ui.heading("Unsaved Changes");

                    ui.add_space(10.0);
                    ui.label(format!("Save changes to '{}'?", &self.documents[i].name));
                    ui.add_space(20.0);

                    ui.horizontal(|ui| {
                        if ui.button("Save").clicked() {
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
                        if ui.button("Don't save").clicked() {
                            self.documents[i].is_modified = false;
                            self.is_modal_open = false;
                            self.document_to_save_index = None;
                            self.check_for_close(&ctx);
                        }
                        if ui.button("Cancel").clicked() {
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
    }
}

impl App {
    fn new_file(&mut self) {
        self.next_document_id += 1;
        let name = format!("Program{}.kf", self.next_document_id + 1);
        self.add_document(name);
    }

    fn close_file(&mut self, index: usize) {
        // Permite cambiar automaticamente de tab cuando se cierra un doc
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
        // No permite no tenr archivos, siempre muestra 1
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

                self.file_dialog = Self::create_file_dialog(&doc.name);

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

    fn create_file_dialog(filename: &str)-> FileDialog {
        FileDialog::new()
            .add_file_filter(
                "KF",
                Arc::new(|p| p.extension().unwrap_or_default() == "kf"),
            )
            .default_file_filter("KF")
            .initial_directory(PathBuf::from("/home/wallace/Documents/"))
            .default_file_name(filename)
    }
}
