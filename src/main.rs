use eframe::{egui};

fn main() -> Result<(), eframe::Error> {

    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        "KF IDE", 
        native_options, Box::new(|cc| Ok(Box::new(KFApp::new(cc)))))
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
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open??").clicked() {
                        println!("Nuevo");
                        ui.close_menu();
                    }

                    if ui.button("Quit").clicked() {
                        // Salir
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                    }

                });
            });

        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello world");

            let response = egui::TextEdit::multiline(&mut self.code)
                .code_editor()
                .desired_rows(10)
                .lock_focus(true)
                .show(ui);

            if response.response.changed() {
                
            }

        });
    }
}
