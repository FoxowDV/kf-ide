use eframe::egui;
use egui_extras::{TableBuilder, Column};

pub fn show(ctx: &egui::Context) {
    egui::SidePanel::right("right_panel")
        .resizable(true)
        .min_width(100.0)
        .default_width(300.0)
        .show(ctx, |ui| {
            show_top_panel(ui);
            show_bottom_panel(ui);
        });
}

fn show_top_panel(ui: &mut egui::Ui) {
    use egui_extras::{Column, TableBuilder};
    //let mut tokens_table = 
    ui.visuals_mut().striped = true;
    TableBuilder::new(ui)
        .columns(Column::remainder(), 3)
        .header(30.0, |mut header| {
            header.col(|ui| {
                ui.heading("Token");
            });
            header.col(|ui| {
                ui.heading("Lexema");
            });
            header.col(|ui| {
                ui.heading("[Región, Column]");
            });
        })
    .body(|mut body| {
        body.row(30.0, |mut row| {
            row.col(|ui| {
                ui.label("hello");
            });
            row.col(|ui| {
                ui.label("world");
            });
            row.col(|ui| {
                ui.label("(aaa,aa)");
            });
        });
    });




}

fn show_bottom_panel(ui: &mut egui::Ui) {
    egui::TopBottomPanel::bottom("right_bottom_panel")
    //let mut tokens_table =
        .resizable(true)
        .min_height(64.0)
        .show_inside(ui, |ui| {
            ui.visuals_mut().striped = true;
            TableBuilder::new(ui)
                .columns(Column::remainder(), 4)
                .header(200.0, |mut header| {
                    use egui::{RichText};
                    header.col(|ui| {
                        ui.heading(RichText::new("Identificador").size(11.0));
                    });
                    header.col(|ui| {
                        ui.heading(RichText::new("Tipo de dato").size(11.0));
                    });
                    header.col(|ui| {
                        ui.heading(RichText::new("Valor").size(11.0));
                    });
                    header.col(|ui| {
                        ui.heading(RichText::new("VarConst").size(11.0));
                    });
                });
            //ui.available_size();
            //ui.horizontal_centered(|ui| {
                //ui.label("Hello world");
            });
}
