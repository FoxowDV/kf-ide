use eframe::egui;

pub fn show(ctx: &egui::Context) {
    egui::TopBottomPanel::bottom("bottom_panel")
        .resizable(true)
        .default_height(200.0)
        .min_height(64.0)
        .show(ctx, |ui| {

            //ui.add(
            let mut content = String::from("** Generación correcta del analizador léxico **\nArchivo guardado en /home/wallace/Documents/");
                _ = egui::TextEdit::multiline(&mut content)
                .code_editor()
                .min_size(ui.available_size())
                .desired_width(f32::INFINITY)
                .show(ui)
             //   )
        });
}
