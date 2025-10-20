use eframe::egui;
use crate::app::App;


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

    pub fn show_edit_menu(&mut self, ui: &mut egui::Ui) {
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

    pub fn show_compile_menu(&mut self, ui: &mut egui::Ui) {
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

    pub fn show_tools_menu(&mut self, ui: &mut egui::Ui) {
        ui.menu_button("Tools", |_ui| {
            // Tools menu content
        });
    }

    pub fn show_about_menu(&mut self, ui: &mut egui::Ui) {
        ui.menu_button("About", |_ui| {
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
                .min_size(image_size)).clicked() {
                self.new_file();
            }
            if ui.add(egui::Button::image(egui::Image::new(open_file_image).fit_to_exact_size(image_size))
                .min_size(image_size)).clicked() {
                self.open_file();
            }
            if ui.add(egui::Button::image(egui::Image::new(save_file_image).fit_to_exact_size(image_size))
                .min_size(image_size)).clicked() {
                self.save_file();
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
