use eframe::egui;
use egui_file_dialog::FileDialog;

use std::{path::PathBuf, sync::Arc};

use crate::panels::{right_panel};
use crate::document::Document;

mod ui;
mod menus;


#[derive(Default)]
pub struct App {
    pub documents: Vec<Document>,
    pub active_tab: usize,
    pub next_document_id: usize,
    file_dialog: FileDialog,
    picked_file: Option<PathBuf>,
    c_line: usize,
    c_col: usize,
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut app = Self::default();

        // archivos
        let file_dialog = FileDialog::new()
        .add_file_filter("KF", Arc::new(|p| p.extension().unwrap_or_default() == "kf"));
        app.file_dialog = file_dialog;
        app.picked_file = None;
        app.add_document("Program1.kf".to_string());
        app
    }

    pub fn add_document(&mut self, name: String) {
        self.documents.push(Document::new(name));
        self.active_tab = self.documents.len() -1;
    }


    pub fn get_active_document_mut(&mut self) -> Option<&mut Document> {
        self.documents.get_mut(self.active_tab)
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.show_top_panel(ctx);

        self.file_dialog.update(ctx);

        if let Some(path) = self.file_dialog.take_picked() {
            if path.exists() {
            match std::fs::read_to_string(&path) {
                    Ok(content) => {
                        if let Some(doc) = self.get_active_document_mut() {
                            doc.content = content;
                            doc.is_modified = false;
                            doc.name = path.file_name()
                                .and_then(|n| n.to_str())
                                .unwrap_or("Unnamed")
                                .to_string();
                        } else {
                            let mut new_doc = Document::new(
                                path.file_name()
                                    .and_then(|n| n.to_str())
                                    .unwrap_or("Unnamed")
                                    .to_string(),
                            );
                            new_doc.content = content;
                            self.documents.push(new_doc);
                            self.active_tab = self.documents.len() - 1;
                        }    
                        self.picked_file = Some(path.to_path_buf());
                    }
                    Err(e) => {
                        eprintln!("Error al abrir el archivo: {}", e);
                    }
            }
                } else {
                if let Some(doc) = self.get_active_document_mut() {
                    if let Err(e) = std::fs::write(&path, &doc.content) {
                        eprintln!("Error guardando {}", e);
                    } else {
                        dbg!(&path);

                        doc.is_modified = false;
                        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                            doc.name = name.to_string();
                        }
                    }
                }
            }
        }

        right_panel::show(ctx);

        self.show_bottom(ctx);

        self.show_tabs(ctx);
        self.show_central_panel(ctx);




    }
}


impl App {
    fn new_file(&mut self) {
        self.next_document_id += 1;
        let name = format!("Program{}.kf", self.next_document_id+1);
        self.add_document(name);
    }

    fn close_file(&mut self, index: usize) {
        // If not the only document close the tab and change the active tab depending of the current
        // active tab
        if self.documents.len() > 1 && index < self.documents.len() {
            self.documents.remove(index);
            if self.active_tab >= self.documents.len() {
                self.active_tab = self.documents.len() - 1;
            } else if self.active_tab > index {
                self.active_tab -= 1;
            }
        // If the tab is the only one, open a new one always
        } else if index < self.documents.len() {
            self.documents.remove(index);
            self.next_document_id += 1;
            let name = format!("Program{}.kf", self.next_document_id+1);
            self.documents.push(Document::new(name));
            self.active_tab = self.documents.len() - 1;

        }
    }

    fn open_file(&mut self) {
        self.file_dialog.pick_file();
 
        if let Some(path) = self.file_dialog.take_picked() {
            if path.exists() {
                match std::fs::read_to_string(&path) {
                    Ok(content) => {
                        // Si ya hay un documento activo, reemplazamos su contenido
                        if let Some(doc) = self.get_active_document_mut() {
                            doc.content = content;
                            doc.is_modified = false;
                            doc.name = path.file_name()
                                .and_then(|n| n.to_str())
                                .unwrap_or("Unnamed")
                                .to_string();
                        } else {
                            // Si no hay documento activo, creamos uno nuevo
                            let mut new_doc = Document::new(
                                path.file_name()
                                    .and_then(|n| n.to_str())
                                    .unwrap_or("Unnamed")
                                    .to_string(),
                            );
                            new_doc.content = content;
                            self.documents.push(new_doc);
                            self.active_tab = self.documents.len() - 1;
                        }

                        self.picked_file = Some(path.to_path_buf());
                    }
                    Err(e) => {
                        eprintln!("Error al abrir el archivo: {}", e);
                    }
                }
            }
        }
    }


    fn save_file(&mut self) {
        self.file_dialog.save_file();
    }

}

