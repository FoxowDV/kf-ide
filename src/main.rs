use eframe::egui;

mod document;

mod app;
mod panels;
mod code_editor;


use app::App;

mod splash;
use splash::Splash;

fn main() -> Result<(), eframe::Error> {
   let options_splash = eframe::NativeOptions {
       viewport: egui::ViewportBuilder::default()
           .with_inner_size([800.0, 400.0])
           .with_resizable(false),
        ..Default::default()
    };

    _ = eframe::run_native(
        "KF IDE", 
        options_splash, 
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            cc.egui_ctx.set_visuals(egui::Visuals::light());
            Ok(Box::new(Splash::new(cc)))
        })
    );


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

