use eframe::egui;

fn main() -> Result<(), eframe::Error> {

    let native_options = eframe::NativeOptions{
        //default_theme: eframe::Theme::Light,
        ..Default::default()
    };

    eframe::run_native(
        "KF IDE", 
        native_options, Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            cc.egui_ctx.set_visuals(egui::Visuals::light());
            Ok(Box::new(KFApp::new(cc)))
        }))
}

#[derive(Default)]
struct KFApp{
    // 
    code: String,
}

impl KFApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}


impl eframe::App for KFApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        egui::TopBottomPanel::top("top_menu_bar").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                
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
                        // Salir
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                
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

                    if ui.button("Quit").clicked() {
                        // Salir
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.menu_button("Compile", |ui| {
                    if ui.button("Compile").clicked() {
                        println!("Compilar");
                        ui.close();
                    }
                    if ui.button("Compile and run").clicked() {
                        println!("Compilar y correr");
                        ui.close();
                    }
                    if ui.button("Quit").clicked() {
                        // Salir
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.menu_button("Tools", |ui| {
                });
                ui.menu_button("About", |ui| {
                });
            });

            let image_size = egui::vec2(24.0, 24.0);
            let new_file_image = egui::include_image!("../resources/new_file.svg");
            let open_file_image = egui::include_image!("../resources/open_file.svg");
            let save_all_file_image = egui::include_image!("../resources/save_all_file.svg");
            let save_as_file_image = egui::include_image!("../resources/save_as_file.svg");
            let save_file_image = egui::include_image!("../resources/save_file.svg");
            let copy_image = egui::include_image!("../resources/copy.svg");
            let cut_image = egui::include_image!("../resources/cut.svg");
            let paste_image = egui::include_image!("../resources/paste.svg");
            let compile_image = egui::include_image!("../resources/compile.svg");
            let compile_and_run_image = egui::include_image!("../resources/compile_and_run.svg");

            ui.horizontal(|ui| {
                if ui.add(egui::Button::image(egui::Image::new(new_file_image).fit_to_exact_size (image_size))
                    .min_size(image_size)).clicked() {
                    println!("new file");    
            }
                if ui.add(egui::Button::image(egui::Image::new(open_file_image).fit_to_exact_size (image_size))
                    .min_size(image_size)).clicked() {
                    println!("open file");    
            }
                if ui.add(egui::Button::image(egui::Image::new(save_file_image).fit_to_exact_size (image_size))
                    .min_size(image_size)).clicked() {
                    println!("save file");    
            }
                if ui.add(egui::Button::image(egui::Image::new(save_all_file_image).fit_to_exact_size (image_size))
                    .min_size(image_size)).clicked() {
                    println!("save all file");    
            }
                if ui.add(egui::Button::image(egui::Image::new(save_as_file_image).fit_to_exact_size (image_size))
                    .min_size(image_size)).clicked() {
                    println!("save as file");    
            }
                if ui.add(egui::Button::image(egui::Image::new(copy_image).fit_to_exact_size (image_size))
                    .min_size(image_size)).clicked() {
                    println!("copy");    
            }
                if ui.add(egui::Button::image(egui::Image::new(cut_image).fit_to_exact_size (image_size))
                    .min_size(image_size)).clicked() {
                    println!("cut");    
            }
                if ui.add(egui::Button::image(egui::Image::new(paste_image).fit_to_exact_size (image_size))
                    .min_size(image_size)).clicked() {
                    println!("paste");    
            }
                if ui.add(egui::Button::image(egui::Image::new(compile_image).fit_to_exact_size (image_size))
                    .min_size(image_size)).clicked() {
                    println!("compile");    
            }
                if ui.add(egui::Button::image(egui::Image::new(compile_and_run_image).fit_to_exact_size (image_size))
                    .min_size(image_size)).clicked() {
                    println!("compile and run");    
            }
                        });

        });


        egui::SidePanel::right("right_panel")
            .resizable(true)
            .default_width(300.0)
            .show(ctx, |ui| {
                //use egui_extras::{Column, TableBuilder};
                //let mut tokens_table = TableBuilder::new(ui);

                ui.vertical_centered(|ui| {
                    ui.label("Hello world");
                });

                egui::TopBottomPanel::bottom("right_bottom_panel")
                    .resizable(true)
                    .default_height(ui.available_height() * 0.5)
                    .min_height(64.0)
                    .show_inside(ui, |ui| {
                        ui.available_size();
                        ui.horizontal_centered(|ui| {
                            ui.label("Hello world");
                        });
                });
            });

        egui::TopBottomPanel::bottom("bottom_panel")
            .resizable(true)
            .default_height(200.0)
            .min_height(64.0)
            .show(ctx, |ui| {
                ui.horizontal_centered(|ui| {
                    ui.label("Hello world");
                });
        });


        // Text editor
        egui::CentralPanel::default().show(ctx, |ui| {

            let response = egui::TextEdit::multiline(&mut self.code)
                .code_editor()
                .min_size(ui.available_size())
                .desired_width(f32::INFINITY)
                .lock_focus(true)
                .show(ui);

            if response.response.changed() {
                
            }


        });
    }
}
