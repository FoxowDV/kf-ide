use eframe::egui;
use std::time::{
    Duration, 
    Instant
};

#[derive(Default)]
pub struct Splash {
    start_time: Option<Instant>,
}

impl Splash {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            start_time: Some(Instant::now()),
        }
    }
}

impl eframe::App for Splash {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Some(start) = self.start_time {
            if start.elapsed() >= Duration::from_secs(3) {
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                return;
            }
        }

        ctx.request_repaint();

        egui::CentralPanel::default().show(ctx, |ui| {
            use egui::{RichText};
            ui.heading(RichText::new("KF IDE").heading().strong().size(25.0));

            ui.label(RichText::new("Desarolladores: ").strong().size(15.0));

            ui.label(RichText::new("Kelpie Athenea Alcalá Padilla").strong().size(15.0));
            ui.label(RichText::new("Daniel Aldahir Amador Ramírez").strong().size(15.0));

            ui.add_space(20.0);

            ui.image(egui::include_image!("../resources/foxoakira.png"));

            
        });

    }
}
