use eframe::egui;
use crate::app::App;
use crate::ui_language::UiLanguage;
use crate::translator::Translator;
use crate::app::PathType;

#[derive(PartialEq)]
pub enum ConfigTab {
    Font,
    Compilation,
}

impl ::std::default::Default for ConfigTab {
    fn default() -> Self { 
        ConfigTab::Font
    }
}

impl App {
    pub fn show_config_window(&mut self, ctx: &egui::Context) {
        egui::Window::new(self.translator.t("configuration"))
            .collapsible(false)
            .resizable(true)
            .default_width(700.0)
            .default_height(500.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut self.config_tab, ConfigTab::Font, self.translator.t("font"));
                    ui.selectable_value(&mut self.config_tab, ConfigTab::Compilation, self.translator.t("compilation"));
                });

                ui.separator();

                match self.config_tab {
                    ConfigTab::Font => self.show_font_tab(ui),
                    ConfigTab::Compilation => self.show_compilation_tab(ui),
                }

                ui.add_space(10.0);
                ui.separator();

                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button(self.translator.t("cancel")).clicked() {
                            self.is_config_open = false;
                        }
                        if ui.button(self.translator.t("accept")).clicked() {
                            self.translator = Translator::new(self.config.language);
                            let _ = self.config.save(self.config.clone());
                            self.is_config_open = false;
                        }
                    });
                });
        });
    }

    fn show_font_tab(&mut self, ui: &mut egui::Ui) {
        ui.horizontal_top(|ui| {
            ui.vertical(|ui| {
                ui.label(format!("{}:", self.translator.t("font")));
                ui.add_space(5.0);
                egui::ScrollArea::vertical()
                    .id_salt("font_list")
                    .max_height(180.0)
                    .show(ui, |ui| {
                        ui.set_width(180.0);
                        let fonts = vec![
                            "Arial",
                            "Consola",
                        ];
                        for font in fonts {
                            ui.selectable_value(&mut self.config.font, font.to_string(), font);
                        }
                    });
            });

            ui.add_space(15.0);

            ui.vertical(|ui| {
                ui.label(format!("{}:", self.translator.t("style")));
                ui.add_space(5.0);
                egui::ScrollArea::vertical()
                    .id_salt("style_list")
                    .max_height(70.0)
                    .show(ui, |ui| {
                        ui.set_width(100.0);
                        ui.selectable_value(&mut self.config.style, "Regular".to_string(), "Regular");
                        ui.selectable_value(&mut self.config.style, "Bold".to_string(), "Bold");
                        ui.selectable_value(&mut self.config.style, "Italic".to_string(), "Italic");
                    });

                self.update_font_p(ui.ctx());
                ui.add_space(10.0);


                ui.label(format!("{}:", self.translator.t("example")));
                ui.add_space(5.0);
                egui::Frame::new()
                    .fill(egui::Color32::WHITE)
                    .stroke(egui::Stroke::new(1.0, egui::Color32::GRAY))
                    .inner_margin(8.0)
                    .show(ui, |ui| {
                        ui.set_width(280.0);
                        ui.set_height(90.0);
                        
                        // Texto del ejemplo traducido
                        let mut example_text = format!(
                            "{} {};\n{}\n  {}:{}\n{}\n  {}(\"{}\");\n{}.",
                            self.translator.t("program"),
                            self.translator.t("example1"),
                            self.translator.t("var"),
                            self.translator.t("age"),
                            self.translator.t("byte"),
                            self.translator.t("begin"),
                            self.translator.t("writeln"),
                            self.translator.t("welcome"),
                            self.translator.t("end")
                        );
                        
                        _ = egui::TextEdit::multiline(&mut example_text)
                            .code_editor()
                            .font(egui::FontId::monospace(self.config.size as f32))
                            .show(ui);
                    });
            });

            ui.add_space(15.0);

            ui.vertical(|ui| {
                ui.label(format!("{}:", self.translator.t("size")));
                ui.add_space(5.0);
                egui::ScrollArea::vertical()
                    .id_salt("size_list")
                    .max_height(100.0)
                    .show(ui, |ui| {
                        ui.set_width(50.0);
                        for size in [8, 10, 12, 14, 16, 18, 20, 24] {
                            ui.selectable_value(&mut self.config.size, size, size.to_string());
                        }
                    });

                ui.add_space(15.0);

                ui.label(format!("{}:", self.translator.t("language")));
                ui.add_space(5.0);
                
                let old_language = self.config.language;
                
                for language in UiLanguage::all() {
                    ui.radio_value(&mut self.config.language, language, language.display_name());
                }
                
                if old_language != self.config.language {
                    self.translator = Translator::new(self.config.language);
                }
            });
        });

        ui.add_space(15.0);
        ui.separator();
        ui.add_space(10.0);

        ui.label(format!("{}:", self.translator.t("color")));
        ui.add_space(5.0);
        
        ui.horizontal_top(|ui| {
            ui.vertical(|ui| {
                egui::ScrollArea::vertical()
                    .id_salt("color_categories")
                    .max_height(200.0)
                    .show(ui, |ui| {
                        let categories = vec![
                            (self.translator.t("background"), &mut self.config.background), 
                            (self.translator.t("cursor"), &mut self.config.cursor), 
                            (self.translator.t("selection"), &mut self.config.selection), 
                            (self.translator.t("general"), &mut self.config.general), 
                            (self.translator.t("reserved words"), &mut self.config.keywords), 
                            (self.translator.t("coments"), &mut self.config.comments), 
                            (self.translator.t("identifiers"), &mut self.config.identifiers), 
                            (self.translator.t("numbers"), &mut self.config.numerics), 
                            (self.translator.t("chains"), &mut self.config.strings), 
                            (self.translator.t("punctuation marks"), &mut self.config.punctuation),
                            ("Funciones", &mut self.config.functions),
                            ("Tipos", &mut self.config.types),
                            ("Especiales", &mut self.config.special),
                        ];

                        for (label, color) in categories {
                            ui.horizontal(|ui| {
                                ui.label(label);
                                ui.color_edit_button_srgba(color);
                            });
                        }
                    });
            });
        });
    }
    
    fn show_compilation_tab(&mut self, ui: &mut egui::Ui) {
        ui.add_space(20.0);

        ui.horizontal(|ui| {
            ui.label("Ruta de Trabajo:");
        });
        ui.horizontal(|ui| {
            ui.add(
                egui::TextEdit::singleline(&mut self.config.work_path)
                    .desired_width(500.0)
            );
            if ui.button("Buscar").clicked() {
                self.current_path_type = Some(PathType::WorkPath);
                self.file_dialog.pick_directory();
            }
        });

        ui.add_space(20.0);

        ui.horizontal(|ui| {
            ui.label("Ruta del Ensamblador:");
        });

        ui.horizontal(|ui| {
            ui.add(
                egui::TextEdit::singleline(&mut self.config.assembler_path)
                    .desired_width(500.0)
            );
            if ui.button("Buscar").clicked() {
                self.current_path_type = Some(PathType::AssemblerPath);
                self.file_dialog.pick_directory();
            }
        });

        ui.add_space(20.0);

        ui.horizontal(|ui| {
            ui.label("Ruta Emulador DOS:");
        });
        ui.horizontal(|ui| {
            ui.add(
                egui::TextEdit::singleline(&mut self.config.dos_emulator_path)
                    .desired_width(500.0)
            );
            if ui.button("Buscar").clicked() {
                self.current_path_type = Some(PathType::DosEmulatorPath);
                self.file_dialog.pick_directory();
            }
        });
    }
}
