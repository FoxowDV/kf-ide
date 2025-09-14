use eframe::egui;

pub fn show(ctx: &egui::Context, code: &mut String) {
    egui::CentralPanel::default().show(ctx, |ui| {
        let response = egui::TextEdit::multiline(code)
            .code_editor()
            .min_size(ui.available_size())
            .desired_width(f32::INFINITY)
            .lock_focus(true)
            .show(ui);

        if response.response.changed() {
        }
    });
}
