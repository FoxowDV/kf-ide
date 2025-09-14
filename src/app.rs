use eframe::egui;
use crate::panels::{top_panel, right_panel, bottom_panel, central_panel};

#[derive(Default)]
pub struct App {
    pub code: String,
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        top_panel::show(ctx);

        right_panel::show(ctx);

        bottom_panel::show(ctx);

        central_panel::show(ctx, &mut self.code);
    }
}

