use eframe::egui;

pub fn show(ctx: &egui::Context) {

    let mut new_style = (*ctx.style()).clone();
    new_style.visuals.panel_fill = egui::Color32::from_rgb(230, 230, 230);
    ctx.set_style(new_style);

    egui::TopBottomPanel::top("top_menu_bar").show(ctx, |ui| {
        // Menu bar
        egui::MenuBar::new().ui(ui, |ui| {
            show_file_menu(ui);
            show_edit_menu(ui);
            show_compile_menu(ui);
            show_tools_menu(ui);
            show_about_menu(ui);
        });

        // Toolbar with icon buttons
        show_toolbar(ui);
    });
}

fn show_file_menu(ui: &mut egui::Ui) {
    ui.menu_button("File", |ui| {
        if ui.button("New File").clicked() {
            println!("Nuevo");
            ui.close();
        }
        if ui.button("Open").clicked() {
            println!("Abrir");
            ui.close();
        }
        if ui.button("Save").clicked() {
            println!("Guardar");
            ui.close();
        }
        if ui.button("Save all").clicked() {
            println!("Guardar todo");
            ui.close();
        }
        if ui.button("Save as").clicked() {
            println!("Guardar como");
            ui.close();
        }
        if ui.button("Close project").clicked() {
            println!("Cerrar proyecto");
            ui.close();
        }
        if ui.button("Close all").clicked() {
            println!("Cerrar todo");
            ui.close();
        }
        if ui.button("Quit").clicked() {
            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
        }
    });
}

fn show_edit_menu(ui: &mut egui::Ui) {
    ui.menu_button("Edit", |ui| {
        if ui.button("Copy").clicked() {
            println!("Copiar");
            ui.close();
        }
        if ui.button("Cut").clicked() {
            println!("Cortar");
            ui.close();
        }
        if ui.button("Paste").clicked() {
            println!("Pegar");
            ui.close();
        }
    });
}

fn show_compile_menu(ui: &mut egui::Ui) {
    ui.menu_button("Compile", |ui| {
        if ui.button("Compile").clicked() {
            println!("Compilar");
            ui.close();
        }
        if ui.button("Compile and run").clicked() {
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

fn show_toolbar(ui: &mut egui::Ui) {
    let image_size = egui::vec2(24.0, 24.0);
    
    // Load all images
    let new_file_image = egui::include_image!("../../resources/new-file.svg");
    let open_file_image = egui::include_image!("../../resources/open-file.svg");
    let save_all_file_image = egui::include_image!("../../resources/content-save-all.svg");
    let save_as_file_image = egui::include_image!("../../resources/save-as-outline.svg");
    let save_file_image = egui::include_image!("../../resources/save.svg");
    let copy_image = egui::include_image!("../../resources/copy.svg");
    let cut_image = egui::include_image!("../../resources/cut.svg");
    let paste_image = egui::include_image!("../../resources/paste.svg");
    let compile_image = egui::include_image!("../../resources/compliance.svg");
    let compile_and_run_image = egui::include_image!("../../resources/run-all.svg");

    ui.horizontal(|ui| {
        if ui.add(egui::Button::image(egui::Image::new(new_file_image).fit_to_exact_size(image_size))
            .min_size(image_size)).clicked() {
            println!("new file");    
        }
        if ui.add(egui::Button::image(egui::Image::new(open_file_image).fit_to_exact_size(image_size))
            .min_size(image_size)).clicked() {
            println!("open file");    
        }
        if ui.add(egui::Button::image(egui::Image::new(save_file_image).fit_to_exact_size(image_size))
            .min_size(image_size)).clicked() {
            println!("save file");    
        }
        if ui.add(egui::Button::image(egui::Image::new(save_all_file_image).fit_to_exact_size(image_size))
            .min_size(image_size)).clicked() {
            println!("save all file");    
        }
        if ui.add(egui::Button::image(egui::Image::new(save_as_file_image).fit_to_exact_size(image_size))
            .min_size(image_size)).clicked() {
            println!("save as file");    
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
