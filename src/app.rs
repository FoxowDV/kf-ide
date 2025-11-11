use eframe::egui;
use egui_file_dialog::FileDialog;

use std::{
    path::PathBuf, 
    sync::Arc,
    collections::HashMap,
};

use crate::document::Document;

mod menus;
mod ui;
mod tables;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Language {
    Spanish,
    English,
}

pub struct I18n {
    translations: HashMap<String, HashMap<Language, String>>,
}

impl I18n {
    pub fn new() -> Self {
        let mut translations = HashMap::new();
        
        let mut example_code = HashMap::new();
        example_code.insert(
            Language::Spanish,
            "program ejemplo1;\nvar\n  edad:byte\nbegin\n  writeln(\"Bienvenido al Lenguaje\n  MicroPascual...\");\nend.".to_string()
        );
        example_code.insert(
            Language::English,
            "program example1;\nvar\n  age:byte\nbegin\n  writeln(\"Welcome to the\n  MicroPascal Language...\");\nend.".to_string()
        );
        translations.insert("example_code".to_string(), example_code);
        
        Self { translations }
    }
    
    pub fn get(&self, key: &str, language: &Language) -> String {
        self.translations
            .get(key)
            .and_then(|map| map.get(language))
            .cloned()
            .unwrap_or_else(|| format!("Missing translation: {}", key))
    }
}

#[derive(PartialEq)]
enum ConfigTab {
    Font,
    Compilation,
}

struct ConfigWindow {
    open: bool,
    active_tab: ConfigTab,
    selected_font: String,
    selected_style: String,
    selected_size: usize,
    selected_language: String,
    selected_color_category: String,
    selected_color: egui::Color32,
    hue: f32,
    picker_pos: egui::Pos2,
    work_path: String,
    assembler_path: String,
    dos_emulator_path: String,
    i18n: I18n,
}

impl Default for ConfigWindow {
    fn default() -> Self {
        Self {
            open: false,
            active_tab: ConfigTab::Font,
            selected_font: "Proportional".to_string(),
            selected_style: "PLAIN".to_string(),
            selected_size: 11,
            selected_language: "Español".to_string(),
            selected_color_category: "General".to_string(),
            selected_color: egui::Color32::BLUE,
            hue: 240.0,
            picker_pos: egui::Pos2::new(128.0, 0.0),
            work_path: "C:\\".to_string(),
            assembler_path: "C:\\TASM".to_string(),
            dos_emulator_path: "C:\\DOSBox".to_string(),
            i18n: I18n::new(),
        }
    }
}

impl ConfigWindow {
    fn get_language_enum(&self) -> Language {
        match self.selected_language.as_str() {
            "Inglés" => Language::English,
            _ => Language::Spanish,
        }
    }

    fn show(&mut self, ctx: &egui::Context) {
        if !self.open {
            return;
        }

        egui::Window::new("Configuración")
            .collapsible(false)
            .resizable(true)
            .default_width(700.0)
            .default_height(500.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut self.active_tab, ConfigTab::Font, "Fuente");
                    ui.selectable_value(&mut self.active_tab, ConfigTab::Compilation, "Compilación");
                });

                ui.separator();

                match self.active_tab {
                    ConfigTab::Font => self.show_font_tab(ui),
                    ConfigTab::Compilation => self.show_compilation_tab(ui),
                }

                ui.add_space(10.0);
                ui.separator();

                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("Cancelar").clicked() {
                            self.open = false;
                        }
                        if ui.button("Aceptar").clicked() {
                            self.open = false;
                        }
                    });
                });
            });
    }

    fn show_font_tab(&mut self, ui: &mut egui::Ui) {
        ui.horizontal_top(|ui| {
            ui.vertical(|ui| {
                ui.label("Fuente:");
                ui.add_space(5.0);
                egui::ScrollArea::vertical()
                    .id_salt("font_list")
                    .max_height(180.0)
                    .show(ui, |ui| {
                        ui.set_width(180.0);
                        let fonts = vec![
                            "Proportional",
                            "Monospace",
                        ];
                        for font in fonts {
                            ui.selectable_value(&mut self.selected_font, font.to_string(), font);
                        }
                    });
            });

            ui.add_space(15.0);

            ui.vertical(|ui| {
                ui.label("Estilo:");
                ui.add_space(5.0);
                egui::ScrollArea::vertical()
                    .id_salt("style_list")
                    .max_height(70.0)
                    .show(ui, |ui| {
                        ui.set_width(100.0);
                        ui.selectable_value(&mut self.selected_style, "PLAIN".to_string(), "PLAIN");
                        ui.selectable_value(&mut self.selected_style, "BOLD".to_string(), "BOLD");
                        ui.selectable_value(&mut self.selected_style, "ITALIC".to_string(), "ITALIC");
                    });

                ui.add_space(10.0);

                ui.label("Ejemplo:");
                ui.add_space(5.0);
                egui::Frame::new()
                    .fill(egui::Color32::WHITE)
                    .stroke(egui::Stroke::new(1.0, egui::Color32::GRAY))
                    .inner_margin(8.0)
                    .show(ui, |ui| {
                        ui.set_width(280.0);
                        ui.set_height(90.0);
                        
                        let current_language = self.get_language_enum();
                        let example_text = self.i18n.get("example_code", &current_language);
                        
                        let font_family = match self.selected_font.as_str() {
                            "Monospace" => egui::FontFamily::Monospace,
                            _ => egui::FontFamily::Proportional,
                        };
                        
                        let mut text = egui::RichText::new(example_text)
                            .size(self.selected_size as f32)
                            .family(font_family);
                        
                        text = match self.selected_style.as_str() {
                            "BOLD" => text.strong(),
                            "ITALIC" => text.italics(),
                            _ => text,
                        };
                        
                        text = text.color(self.selected_color);
                        
                        ui.label(text);
                    });
            });

            ui.add_space(15.0);

            ui.vertical(|ui| {
                ui.label("Tamaño:");
                ui.add_space(5.0);
                egui::ScrollArea::vertical()
                    .id_salt("size_list")
                    .max_height(100.0)
                    .show(ui, |ui| {
                        ui.set_width(50.0);
                        for size in 10..=16 {
                            ui.selectable_value(&mut self.selected_size, size, size.to_string());
                        }
                    });

                ui.add_space(15.0);

                ui.label("Idioma:");
                ui.add_space(5.0);
                ui.radio_value(&mut self.selected_language, "Español".to_string(), "Español");
                ui.radio_value(&mut self.selected_language, "Inglés".to_string(), "Inglés");
            });
        });

        ui.add_space(15.0);
        ui.separator();
        ui.add_space(10.0);

        ui.label("Color:");
        ui.add_space(5.0);
        
        ui.horizontal_top(|ui| {
            ui.vertical(|ui| {
                egui::ScrollArea::vertical()
                    .id_salt("color_categories")
                    .max_height(200.0)
                    .show(ui, |ui| {
                        ui.set_width(180.0);
                        let categories = vec![
                            "General", "Palabras reservadas", "Comentarios", "Cadenas",
                            "Identificadores", "Números enteros", "Números flotantes",
                            "Signos de puntuación",
                        ];
                        for category in categories {
                            ui.selectable_value(
                                &mut self.selected_color_category,
                                category.to_string(),
                                category
                            );
                        }
                    });
            });

            ui.add_space(15.0);

            ui.vertical(|ui| {
                ui.label("Color:");
                ui.add_space(5.0);

                ui.horizontal(|ui| {
                    let picker_size = egui::vec2(256.0, 256.0);
                    let (response, painter) = ui.allocate_painter(picker_size, egui::Sense::click_and_drag());
                    
                    let rect = response.rect;
                    
                    let mesh = self.create_sv_gradient_mesh(rect);
                    painter.add(mesh);
                    
                    if response.dragged() || response.clicked() {
                        if let Some(pos) = response.interact_pointer_pos() {
                            let local_x = (pos.x - rect.min.x).clamp(0.0, picker_size.x);
                            let local_y = (pos.y - rect.min.y).clamp(0.0, picker_size.y);
                            self.picker_pos = egui::Pos2::new(local_x, local_y);
                            
                            let saturation = local_x / picker_size.x;
                            let value = 1.0 - (local_y / picker_size.y);
                            self.selected_color = self.hsv_to_rgb(self.hue, saturation, value);
                        }
                    }
                    
                    let circle_pos = rect.min + self.picker_pos.to_vec2();
                    painter.circle_stroke(
                        circle_pos,
                        8.0,
                        egui::Stroke::new(2.0, egui::Color32::WHITE)
                    );
                    painter.circle_stroke(
                        circle_pos,
                        7.0,
                        egui::Stroke::new(1.0, egui::Color32::BLACK)
                    );
                    
                    ui.add_space(10.0);
                    
                    ui.vertical(|ui| {
                        let hue_bar_size = egui::vec2(20.0, 256.0);
                        let (hue_response, hue_painter) = ui.allocate_painter(hue_bar_size, egui::Sense::click_and_drag());
                        
                        let hue_rect = hue_response.rect;
                        
                        let hue_mesh = self.create_hue_bar_mesh(hue_rect);
                        hue_painter.add(hue_mesh);
                        
                        if hue_response.dragged() || hue_response.clicked() {
                            if let Some(pos) = hue_response.interact_pointer_pos() {
                                let local_y = (pos.y - hue_rect.min.y).clamp(0.0, hue_bar_size.y);
                                self.hue = (local_y / hue_bar_size.y) * 360.0;
                                
                                let saturation = self.picker_pos.x / 256.0;
                                let value = 1.0 - (self.picker_pos.y / 256.0);
                                self.selected_color = self.hsv_to_rgb(self.hue, saturation, value);
                            }
                        }
                        
                        let indicator_y = hue_rect.min.y + (self.hue / 360.0) * hue_bar_size.y;
                        hue_painter.hline(
                            hue_rect.min.x..=hue_rect.max.x,
                            indicator_y,
                            egui::Stroke::new(2.0, egui::Color32::WHITE)
                        );
                        hue_painter.hline(
                            hue_rect.min.x..=hue_rect.max.x,
                            indicator_y + 1.0,
                            egui::Stroke::new(1.0, egui::Color32::BLACK)
                        );
                    });
                });
            });
        });
    }
    
    fn create_sv_gradient_mesh(&self, rect: egui::Rect) -> egui::epaint::Mesh {
        let mut mesh = egui::epaint::Mesh::default();
        let steps = 32;
        
        for y in 0..steps {
            for x in 0..steps {
                let x_norm = x as f32 / (steps - 1) as f32;
                let y_norm = y as f32 / (steps - 1) as f32;
                
                let saturation = x_norm;
                let value = 1.0 - y_norm;
                
                let color = self.hsv_to_rgb(self.hue, saturation, value);
                
                let pos = rect.min + egui::vec2(
                    x_norm * rect.width(),
                    y_norm * rect.height()
                );
                
                mesh.colored_vertex(pos, color);
                
                if x < steps - 1 && y < steps - 1 {
                    let idx = (y * steps + x) as u32;
                    mesh.add_triangle(idx, idx + 1, idx + steps);
                    mesh.add_triangle(idx + 1, idx + steps + 1, idx + steps);
                }
            }
        }
        
        mesh
    }
    
    fn create_hue_bar_mesh(&self, rect: egui::Rect) -> egui::epaint::Mesh {
        let mut mesh = egui::epaint::Mesh::default();
        let steps = 360;
        
        for i in 0..steps {
            let y_norm = i as f32 / (steps - 1) as f32;
            let hue = y_norm * 360.0;
            let color = self.hsv_to_rgb(hue, 1.0, 1.0);
            
            let top = rect.min + egui::vec2(0.0, y_norm * rect.height());
            let bottom = top + egui::vec2(rect.width(), 0.0);
            
            mesh.colored_vertex(top, color);
            mesh.colored_vertex(bottom, color);
            
            if i < steps - 1 {
                let idx = (i * 2) as u32;
                mesh.add_triangle(idx, idx + 1, idx + 2);
                mesh.add_triangle(idx + 1, idx + 3, idx + 2);
            }
        }
        
        mesh
    }
    
    fn hsv_to_rgb(&self, h: f32, s: f32, v: f32) -> egui::Color32 {
        let c = v * s;
        let h_prime = h / 60.0;
        let x = c * (1.0 - ((h_prime % 2.0) - 1.0).abs());
        let m = v - c;
        
        let (r, g, b) = if h_prime < 1.0 {
            (c, x, 0.0)
        } else if h_prime < 2.0 {
            (x, c, 0.0)
        } else if h_prime < 3.0 {
            (0.0, c, x)
        } else if h_prime < 4.0 {
            (0.0, x, c)
        } else if h_prime < 5.0 {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };
        
        egui::Color32::from_rgb(
            ((r + m) * 255.0) as u8,
            ((g + m) * 255.0) as u8,
            ((b + m) * 255.0) as u8,
        )
    }

    fn show_compilation_tab(&mut self, ui: &mut egui::Ui) {
        ui.add_space(20.0);

        ui.horizontal(|ui| {
            ui.label("Ruta de Trabajo:");
        });
        ui.horizontal(|ui| {
            ui.add(
                egui::TextEdit::singleline(&mut self.work_path)
                    .desired_width(500.0)
            );
            if ui.button("Buscar").clicked() {
            }
        });

        ui.add_space(20.0);

        ui.horizontal(|ui| {
            ui.label("Ruta del Ensamblador:");
        });
        ui.horizontal(|ui| {
            ui.add(
                egui::TextEdit::singleline(&mut self.assembler_path)
                    .desired_width(500.0)
            );
            if ui.button("Buscar").clicked() {
            }
        });

        ui.add_space(20.0);

        ui.horizontal(|ui| {
            ui.label("Ruta Emulador DOS:");
        });
        ui.horizontal(|ui| {
            ui.add(
                egui::TextEdit::singleline(&mut self.dos_emulator_path)
                    .desired_width(500.0)
            );
            if ui.button("Buscar").clicked() {
            }
        });
    }
}

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
    config_window: ConfigWindow,
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut app = Self::default();

        app.file_dialog = Self::create_file_dialog("Program.kf");
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

    pub fn open_config_window(&mut self) {
        self.config_window.open = true;
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

        self.config_window.show(ctx);
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