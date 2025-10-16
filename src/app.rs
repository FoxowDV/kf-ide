use eframe::egui;
use egui_file_dialog::FileDialog;
use std::path::PathBuf;

use crate::panels::{right_panel};
use crate::document::Document;

use crate::code_editor::CodeEditor;

use chrono::Local;


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
        let file_dialog = FileDialog::new();
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

        right_panel::show(ctx);

        self.show_bottom(ctx);

        self.show_tabs(ctx);
        self.show_central_panel(ctx);
    }
}

impl App {
    fn show_top_panel(&mut self, ctx: &egui::Context) {
        let mut new_style = (*ctx.style()).clone();
        new_style.visuals.panel_fill = egui::Color32::from_rgb(230, 230, 230);
        ctx.set_style(new_style);

        egui::TopBottomPanel::top("top_menu_bar").show(ctx, |ui| {
            self.show_menu_bar(ui);
            self.show_tool_bar(ui);
        });
    }


    fn show_tabs(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("tabs_panel")
            .resizable(false)
            .min_height(32.0)
            .frame(
                egui::Frame::default()
                .outer_margin(0.0)
                .inner_margin(0.0)
                .fill(egui::Color32::from_rgb(230, 230, 230))
            )
            .show(ctx, |ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                let mut tab_to_close = None;

                let tab_width = 120.0;
                let tab_height = ui.available_height();

                for (i, doc) in self.documents.iter().enumerate() {
                    // The name of each tab
                    let tab_text = if doc.is_modified {
                        format!("*{}", doc.name)
                    } else {
                        doc.name.clone()
                    };
                
                    let is_active = self.active_tab == i;

                    // The tab group
                    ui.allocate_ui_with_layout(
                        egui::Vec2::new(tab_width, tab_height),
                        egui::Layout::left_to_right(egui::Align::Center),
                        |ui| {
                            let tab_rect = ui.available_rect_before_wrap();
                            let tab_color = if is_active {
                                egui::Color32::WHITE
                            } else {
                                egui::Color32::from_rgb(230, 230, 230)
                            };
                        
                            ui.painter().rect_filled(
                                tab_rect,
                                egui::CornerRadius::same(4),
                                tab_color
                            );

                            let button_response = ui.add_sized(
                                [tab_width - 25.0, tab_height],
                                egui::Button::new(&tab_text)
                                    .fill(egui::Color32::TRANSPARENT)
                                    .stroke(egui::Stroke::NONE)
                                    .corner_radius(egui::CornerRadius::same(4))
                            );

                            if button_response.clicked() {
                                self.active_tab = i;
                            }
                        
                            // Close button
                            let close_response = ui.add_sized(
                                [16.0, tab_height],
                                egui::Button::new("×")
                                    .fill(egui::Color32::TRANSPARENT)
                                    .stroke(egui::Stroke::NONE)
                                    .small()
                            );

                            if close_response.clicked() {
                                tab_to_close = Some(i);
                            }
                        }
                    );
                    
                    ui.add_space(2.0);
                }
                    
                // Handle close tab
                if let Some(index) = tab_to_close {
                    self.close_file(index);
                }

                if ui.button("+").clicked() {
                    self.new_file();
                }
            })
        });
    }

    fn show_central_panel(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().
            frame(egui::Frame::default()
                .outer_margin(0.0)
                .inner_margin(0.0)
                .fill(egui::Color32::WHITE)
                )
            .show(ctx, |ui| {
                if let Some(doc) = self.get_active_document_mut() {
                    let prev_content = doc.content.clone();
                    
                    let (response, line, col) = CodeEditor::default()
                        .id_source("code_editor")
                        .with_ui_fontsize(ui)
                        .with_rows(1)
                        .show(ui, &mut doc.content);



                    if response.response.changed() {
                        if doc.content != prev_content {
                            doc.is_modified = true;
                        }

                    }

                    response.response.context_menu(|ui| {
                        if ui.button("Copy").clicked() {
                            println!("Copy");
                            ui.close();
                        }
                        if ui.button("Cut").clicked() {
                            println!("Cut");
                            ui.close();
                        }
                        if ui.button("Paste").clicked() {
                            println!("Paste");
                            ui.close();
                        }

                    });


                    self.c_line = line;
                    self.c_col = col;
                }
        });
    }

    fn show_bottom(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("bottom_panel")
            .resizable(true)
            .default_height(200.0)
            .min_height(64.0)
            .show(ctx, |ui| {
                 ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(format!("Line: {}, Column: {}", self.c_line, self.c_col));

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let now = Local::now();
                        let time_str = now.format("%H:%M:%S").to_string();
                        ui.label(time_str);
                    });
                });


                let mut content = String::from("** Generación correcta del analizador léxico **\nArchivo guardado en /home/wallace/Documents/");
                _ = egui::TextEdit::multiline(&mut content)
                    .code_editor()
                    .min_size(ui.available_size())
                    .desired_width(f32::INFINITY)
                    .show(ui)
                });

            });
    }
}


impl App {
    fn show_menu_bar(&mut self, ui: &mut egui::Ui) {
        egui::MenuBar::new().ui(ui, |ui| {
            self.show_file_menu(ui);
            self.show_edit_menu(ui);
            self.show_compile_menu(ui);
            self.show_tools_menu(ui);
            self.show_about_menu(ui);
        });
    }

    fn show_file_menu(&mut self, ui: &mut egui::Ui) {
        ui.menu_button("File", |ui| {
            if ui.add(egui::Button::new("New File").shortcut_text("CTRL+N")).clicked() {
                self.new_file();
                ui.close();
            }
            if ui.add(egui::Button::new("Open").shortcut_text("CTRL+O")).clicked() {
                println!("Abrir");
                ui.close();
            }

            ui.separator();

            if ui.add(egui::Button::new("Save").shortcut_text("CTRL+S")).clicked() {
                println!("Guardar");
                ui.close();
            }
            if ui.add(egui::Button::new("Save all").shortcut_text("CTRL+ALT+S")).clicked() {
                println!("Guardar todo");
                ui.close();
            }
            if ui.add(egui::Button::new("Save as").shortcut_text("CTRL+SHIFT+S")).clicked() {
                println!("Guardar como");
                ui.close();
            }

            ui.separator();

            if ui.add(egui::Button::new("Close file").shortcut_text("CTRL+SHIFT+W")).clicked() {
                self.close_file(self.active_tab);
                ui.close();
            }


            if ui.button("Close all").clicked() {
                println!("Cerrar todo");
                ui.close();
            }

            ui.separator();

            if ui.add(egui::Button::new("Exit").shortcut_text("ALT+X")).clicked() {
                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
            }
        });
    }

    fn show_edit_menu(&mut self, ui: &mut egui::Ui) {
        ui.menu_button("Edit", |ui| {
            if ui.add(egui::Button::new("Copy").shortcut_text("CTRL+C")).clicked() {
                println!("Copiar");
                ui.close();
            }
            if ui.add(egui::Button::new("Cut").shortcut_text("CTRL+X")).clicked() {
                println!("Cortar");
                ui.close();
            }
            if ui.add(egui::Button::new("Paste").shortcut_text("CTRL+V")).clicked() {
                println!("Pegar");
                ui.close();
            }
        });
    }

    fn show_compile_menu(&mut self, ui: &mut egui::Ui) {
        ui.menu_button("Compile", |ui| {
            if ui.add(egui::Button::new("Compile").shortcut_text("CTRL+SHIFT+B")).clicked() {
                println!("Compilar");
                ui.close();
            }
            if ui.add(egui::Button::new("Compile and run").shortcut_text("CTRL+F6")).clicked() {
                println!("Compilar y correr");
                ui.close();
            }
        });
    }

    fn show_tools_menu(&mut self, ui: &mut egui::Ui) {
        ui.menu_button("Tools", |_ui| {
            // Tools menu content
        });
    }

    fn show_about_menu(&mut self, ui: &mut egui::Ui) {
        ui.menu_button("About", |_ui| {
            // About menu content
        });
    }

    fn show_tool_bar(&mut self, ui: &mut egui::Ui) {
        let image_size = egui::vec2(24.0, 24.0);
        
        // Load all images
        let new_file_image = egui::include_image!("../resources/new-file.svg");
        let open_file_image = egui::include_image!("../resources/open-file.svg");
        let save_all_file_image = egui::include_image!("../resources/save-all.svg");
        let save_file_image = egui::include_image!("../resources/save.svg");
        let copy_image = egui::include_image!("../resources/copy.svg");
        let cut_image = egui::include_image!("../resources/cut.svg");
        let paste_image = egui::include_image!("../resources/paste.svg");
        let compile_image = egui::include_image!("../resources/compliance.svg");
        let compile_and_run_image = egui::include_image!("../resources/run-all.svg");

        ui.horizontal(|ui| {
            if ui.add(egui::Button::image(egui::Image::new(new_file_image).fit_to_exact_size(image_size))
                .min_size(image_size)).clicked() {
                self.new_file();
            }
            if ui.add(egui::Button::image(egui::Image::new(open_file_image).fit_to_exact_size(image_size))
                .min_size(image_size)).clicked() {
                self.open_file();
            }
            if ui.add(egui::Button::image(egui::Image::new(save_file_image).fit_to_exact_size(image_size))
                .min_size(image_size)).clicked() {
                println!("save file");    
            }
            if ui.add(egui::Button::image(egui::Image::new(save_all_file_image).fit_to_exact_size(image_size))
                .min_size(image_size)).clicked() {
                println!("save all file");    
            }
            if ui.add(egui::Button::image(egui::Image::new(copy_image).fit_to_exact_size(image_size))
                .min_size(image_size)).clicked() {
                println!("copy");    
            }
            if ui.add(egui::Button::image(egui::Image::new(cut_image).fit_to_exact_size(image_size))
                .min_size(image_size)).clicked() {
                println!("cut");    
            }
            if ui.add(egui::Button::image(egui::Image::new(paste_image).fit_to_exact_size(image_size))
                .min_size(image_size)).clicked() {
                println!("paste");    
            }
            if ui.add(egui::Button::image(egui::Image::new(compile_image).fit_to_exact_size(image_size))
                .min_size(image_size)).clicked() {
                println!("compile");    
            }
            if ui.add(egui::Button::image(egui::Image::new(compile_and_run_image).fit_to_exact_size(image_size))
                .min_size(image_size)).clicked() {
                println!("compile and run");    
            }
        });
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
        dbg!(format!("Picked {:?}", self.picked_file));


        //self.file_dialog.update();

        if let Some(path) = self.file_dialog.take_picked() {
            self.picked_file = Some(path.to_path_buf());
        }
    }







}

