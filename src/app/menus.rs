use eframe::egui;
use crate::app::App;

use kf_compiler::parse_program;

impl App {
    pub fn show_menu_bar(&mut self, ui: &mut egui::Ui) {
        egui::MenuBar::new().ui(ui, |ui| {
            self.show_file_menu(ui);
            self.show_edit_menu(ui);
            self.show_compile_menu(ui);
            self.show_tools_menu(ui);
            self.show_about_menu(ui);
        });
    }

    pub fn show_file_menu(&mut self, ui: &mut egui::Ui) {
        ui.menu_button(self.translator.t("file"), |ui| {
            
            if ui.add(egui::Button::new(self.translator.t("new file")).shortcut_text("CTRL+N")).clicked() {
                self.new_file();
                ui.close();
            }
            if ui.add(egui::Button::new(self.translator.t("open")).shortcut_text("CTRL+O")).clicked() {
                self.open_file();
                ui.close();
            }

            ui.separator();

            if ui.add(egui::Button::new(self.translator.t("save")).shortcut_text("CTRL+S")).clicked() {
                self.save_file(self.active_tab);
                ui.close();
            }
            if ui.add(egui::Button::new(self.translator.t("save all")).shortcut_text("CTRL+ALT+S")).clicked() {
                println!("Guardar todo");
                ui.close();
            }
            if ui.add(egui::Button::new(self.translator.t("save as")).shortcut_text("CTRL+SHIFT+S")).clicked() {
                println!("Guardar como");
                ui.close();
            }

            ui.separator();

            if ui.add(egui::Button::new(self.translator.t("close file")).shortcut_text("CTRL+SHIFT+W")).clicked() {
                self.close_file(self.active_tab);
                ui.close();
            }


            if ui.button(self.translator.t("close all")).clicked() {
                println!("Cerrar todo");
                ui.close();
            }

            ui.separator();

            if ui.add(egui::Button::new(self.translator.t("exit")).shortcut_text("ALT+X")).clicked() {
                self.is_closing = true;
                if self.check_for_close(ui.ctx()) {
                    ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                }
            }
        });
    }

    pub fn show_edit_menu(&mut self, ui: &mut egui::Ui) {
        ui.menu_button(self.translator.t("edit"), |ui| {
            if ui.add(egui::Button::new(self.translator.t("copy")).shortcut_text("CTRL+C")).clicked() {
                ui.ctx().copy_text(self.selected_text.clone());
                ui.close();
            }
            if ui.add(egui::Button::new(self.translator.t("cut")).shortcut_text("CTRL+X")).clicked() {
                ui.ctx().copy_text(self.selected_text.clone());

                if let Some(pos) = self.documents[*&self.active_tab].content.find(&self.selected_text) {
                    self.documents[*&self.active_tab].content.replace_range(pos..pos + self.selected_text.len(), "");
                    self.documents[*&self.active_tab].is_modified = true;
                }

                ui.close();
            }
            if ui.add(egui::Button::new(self.translator.t("paste")).shortcut_text("CTRL+V")).clicked() {
                
                ui.ctx().send_viewport_cmd(egui::ViewportCommand::RequestPaste);
                ui.close();
            }
        });
    }

    pub fn show_compile_menu(&mut self, ui: &mut egui::Ui) {
        ui.menu_button(self.translator.t("compile"), |ui| {
            if ui.add(egui::Button::new(self.translator.t("compile")).shortcut_text("CTRL+SHIFT+B")).clicked() {
                println!("Compilar");
                let result = parse_program(self.documents[self.active_tab].content.as_str());
                println!("{:#?}", result);
                ui.close();
            }
            if ui.add(egui::Button::new(self.translator.t("compile and run")).shortcut_text("CTRL+F6")).clicked() {
                println!("Compilar y correr");
                ui.close();
            }
        });
    }

    pub fn show_tools_menu(&mut self, ui: &mut egui::Ui) {
        ui.menu_button(self.translator.t("tools"), |ui| {
            if ui.button(self.translator.t("open config")).clicked() {
                self.is_config_open = true;
                ui.close();
            }
        });
    }

    pub fn show_about_menu(&mut self, ui: &mut egui::Ui) {
        ui.menu_button(self.translator.t("about"), |_ui| {
            // About menu content
        });
    }

    pub fn show_tool_bar(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        let image_size = egui::vec2(24.0, 24.0);
        
        // Load all images
        let new_file_image = egui::include_image!("../../resources/new-file.svg");
        let open_file_image = egui::include_image!("../../resources/open-file.svg");
        let save_all_file_image = egui::include_image!("../../resources/save-all.svg");
        let save_file_image = egui::include_image!("../../resources/save.svg");
        let copy_image = egui::include_image!("../../resources/copy.svg");
        let cut_image = egui::include_image!("../../resources/cut.svg");
        let paste_image = egui::include_image!("../../resources/paste.svg");
        let compile_image = egui::include_image!("../../resources/compliance.svg");
        let compile_and_run_image = egui::include_image!("../../resources/run-all.svg");

        ui.horizontal(|ui| {
            if ui.add(egui::Button::image(egui::Image::new(new_file_image).fit_to_exact_size(image_size))
                .min_size(image_size).frame_when_inactive(false)).clicked() {
                self.new_file();
            }
            if ui.add(egui::Button::image(egui::Image::new(open_file_image).fit_to_exact_size(image_size))
                .min_size(image_size).frame_when_inactive(false)).clicked() {
                self.open_file();
            }
            if ui.add(egui::Button::image(egui::Image::new(save_file_image).fit_to_exact_size(image_size))
                .min_size(image_size).frame_when_inactive(false)).clicked() {
                self.save_file(self.active_tab);
            }
            if ui.add(egui::Button::image(egui::Image::new(save_all_file_image).fit_to_exact_size(image_size))
                .min_size(image_size).frame_when_inactive(false)).clicked() {
                println!("save all file");    
            }
            // CPOY
            if ui.add(egui::Button::image(egui::Image::new(copy_image).fit_to_exact_size(image_size))
                .min_size(image_size).frame_when_inactive(false)).clicked() {
                    ui.ctx().copy_text(self.selected_text.clone());
                    ui.close();
            }
            // CUT
            if ui.add(egui::Button::image(egui::Image::new(cut_image).fit_to_exact_size(image_size))
                .min_size(image_size).frame_when_inactive(false)).clicked() {


                    ui.ctx().copy_text(self.selected_text.clone());

                    if let Some(pos) = self.documents[*&self.active_tab].content.find(&self.selected_text) {
                        self.documents[*&self.active_tab].content.replace_range(pos..pos + self.selected_text.len(), "");
                        self.documents[*&self.active_tab].is_modified = true;
                    }
                    ui.close();

            }
            // APSTE
            if ui.add(egui::Button::image(egui::Image::new(paste_image).fit_to_exact_size(image_size))
                .min_size(image_size).frame_when_inactive(false)).clicked() {
                ui.ctx().send_viewport_cmd(egui::ViewportCommand::RequestPaste);
                ui.close();
            }
            if ui.add(egui::Button::image(egui::Image::new(compile_image).fit_to_exact_size(image_size))
                .min_size(image_size).frame_when_inactive(false)).clicked() {
                println!("compile");    
            }
            if ui.add(egui::Button::image(egui::Image::new(compile_and_run_image).fit_to_exact_size(image_size))
                .min_size(image_size).frame_when_inactive(false)).clicked() {
                println!("compile and run");    
            }
        });
    }
}
