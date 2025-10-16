use eframe::egui::{RichText};
use egui_extras::{TableBuilder, Column};

pub fn show(ctx: &egui::Context) {
    egui::SidePanel::right("right_panel")
        .frame(
                egui::Frame::default()
                .outer_margin(0.0)
                .inner_margin(3.0)
                .fill(egui::Color32::from_rgb(230, 230, 230))
        )
        .resizable(true)
        .min_width(100.0)
        .default_width(500.0)
        .show(ctx, |ui| {
            show_top_panel(ui);
            show_bottom_panel(ui);
        });
}

fn show_top_panel(ui: &mut egui::Ui) {

    ui.visuals_mut().striped = true;

    ui.allocate_ui_with_layout(egui::Vec2::new(ui.available_width(), 18.0), 
        egui::Layout::top_down(egui::Align::Center),
        |ui| {
            ui.painter().rect_filled(
                ui.available_rect_before_wrap(),
                egui::CornerRadius::same(4),
                egui::Color32::WHITE
            );
        ui.label(RichText::new(" Tokens table").strong().size(15.0));
    });

    TableBuilder::new(ui)
        .columns(Column::remainder(), 3)
        .header(30.0, |mut header| {
            header.col(|ui| {
                ui.heading(RichText::new("Token").size(14.0));
            });
            header.col(|ui| {
                ui.heading(RichText::new("Lexema").size(14.0));
            });
            header.col(|ui| {
                ui.heading(RichText::new("[Line, Column]").size(14.0));
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
    //let mut tokens_table =
    ui.visuals_mut().striped = true;
    egui::TopBottomPanel::bottom("right_bottom_panel")
        .frame(
                egui::Frame::default()
                .outer_margin(0.0)
                .inner_margin(0.0)
                .fill(egui::Color32::from_rgb(230, 230, 230))
        )
        .resizable(true)
        .default_height(ui.available_height() * 0.5)
        .min_height(ui.available_height() * 0.5)
        .show_inside(ui, |ui| {
            ui.available_size();

            ui.allocate_ui_with_layout(egui::Vec2::new(ui.available_width(), 18.0), 
                egui::Layout::top_down(egui::Align::Center),
                |ui| {
                    ui.painter().rect_filled(
                        ui.available_rect_before_wrap(),
                        egui::CornerRadius::same(4),
                        egui::Color32::WHITE
                    );
                    ui.label(RichText::new(" Symbols table").strong().size(15.0));
                });
            TableBuilder::new(ui)
                .id_salt("aa")
                .columns(Column::remainder(), 4)
                .header(30.0, |mut header| {
                    header.col(|ui| {
                        ui.heading(RichText::new("Identifier").size(14.0));
                    });
                    header.col(|ui| {
                        ui.heading(RichText::new("Data type").size(14.0));
                    });
                    header.col(|ui| {
                        ui.heading(RichText::new("Value").size(14.0));
                    });
                    header.col(|ui| {
                        ui.heading(RichText::new("VarConst").size(14.0));
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
                        row.col(|ui| {
                            ui.label("(aaa,aa)");
                        });
                    });

            });
        });
}
