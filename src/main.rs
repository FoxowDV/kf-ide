mod document;
mod shortcuts;
mod app;
mod syntax;
mod splash;
mod config;
mod ui_language;
mod translator;


use eframe::egui;
use app::App;
use splash::Splash;
use config::Config;

fn main() -> Result<(), eframe::Error> {
    let cfg: Config = match confy::load("kf-ide") {
        Ok(cfg) => cfg,
        Err(_) => { 
            let temp_conf = Config::default();
            let _ = confy::store("kf-ide", &temp_conf);
            temp_conf
        },
    };
    //println!("{:#?}", cfg);

   let options_splash = eframe::NativeOptions {
       viewport: egui::ViewportBuilder::default()
           .with_inner_size([550.0, 400.0])
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
            Ok(Box::new(App::new(cc, cfg)))
        })
    )
}

