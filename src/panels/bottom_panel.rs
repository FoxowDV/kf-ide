use eframe::egui;

pub fn show(ctx: &egui::Context) {
    egui::TopBottomPanel::bottom("bottom_panel")
        .resizable(true)
        .default_height(200.0)
        .min_height(64.0)
        .show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                ui.label("Hello ");
            });
        });
}
