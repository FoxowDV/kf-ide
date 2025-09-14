use eframe::egui;

mod app;
mod panels;

use app::App;

fn main() -> Result<(), eframe::Error> {
   let native_options = eframe::NativeOptions {
        ..Default::default()
    };

    eframe::run_native(
        "KF IDE", 
        native_options, 
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            cc.egui_ctx.set_visuals(egui::Visuals::light());
            Ok(Box::new(App::new(cc)))
        })
    )
}

