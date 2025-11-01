use eframe::egui;
use crate::app::App;
use crate::code_editor::CodeEditor;
use chrono::Local;

use crate::shortcuts::{NEW_FILE};

impl App {

    pub fn show_top_panel(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top_menu_bar")
            .frame(
                egui::Frame::default()
                    .outer_margin(0.0)
                    .inner_margin(1.0)
                    .fill(egui::Color32::from_rgb(245, 245, 245))
                    .stroke(egui::Stroke::NONE)
            )
            .show(ctx, |ui| {
                self.show_menu_bar(ui);
                self.show_tool_bar(ctx, ui);
                self.shortcuts(ctx, ui);
        });
    }


    pub fn show_tabs(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("tabs_panel")
            .resizable(false)
            .min_height(32.0)
            .frame(
                egui::Frame::default()
                .outer_margin(0.0)
                .inner_margin(egui::Margin {
                    left: 8,
                    right: 8,
                    top: 2,
                    bottom: 0,
                })
                .fill(egui::Color32::from_rgb(245, 245, 245))
                .stroke(egui::Stroke::NONE)
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
                                egui::Color32::from_rgb(200, 200, 250)
                            } else {
                                egui::Color32::from_rgb(235, 235, 235)
                            };
                        
                            ui.painter().rect_filled(
                                tab_rect,
                                egui::CornerRadius::same(0),
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

    pub fn show_central_panel(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().
            frame(egui::Frame::default()
                .outer_margin(0.0)
                .inner_margin(0.0)
                .fill(egui::Color32::WHITE)
                .stroke(egui::Stroke::NONE)
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

    pub fn show_bottom(&mut self, ctx: &egui::Context) {
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


    pub fn shortcuts(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        if ui.input_mut(|i| {i.consume_shortcut(&NEW_FILE)}) {
            self.new_file();
        }
    }

}
