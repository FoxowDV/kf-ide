fn show_file_menu(ui: &mut egui::Ui) {
    ui.menu_button("File", |ui| {
        if ui.add(egui::Button::new("New File").shortcut_text("CTRL+N")).clicked() {
            println!("Nuevo");
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
            println!("Cerrar proyecto");
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

fn show_edit_menu(ui: &mut egui::Ui) {
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

fn show_compile_menu(ui: &mut egui::Ui) {
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

fn show_tools_menu(ui: &mut egui::Ui) {
    ui.menu_button("Tools", |_ui| {
        // Tools menu content
    });
}

fn show_about_menu(ui: &mut egui::Ui) {
    ui.menu_button("About", |_ui| {
        // About menu content
    });
}

