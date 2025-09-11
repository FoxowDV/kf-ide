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
                    if ui.button("Open??").clicked() {
                        println!("Nuevo");
                        ui.close();
                    }

                    if ui.button("Quit").clicked() {
                        // Salir
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
            });


            let new_file_image = egui::include_image!("../resources/new_file.svg");
            if ui.add(egui::Button::image(egui::Image::new(new_file_image).fit_to_exact_size(egui::Vec2{x : 32.0, y : 32.0}))
                .min_size(egui::vec2(32.0, 32.0))).clicked() {
                println!("new file");
            }

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
