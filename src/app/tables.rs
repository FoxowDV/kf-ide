use eframe::egui;
use egui::RichText;
use egui_extras::{
    Column, 
    TableBuilder
};
use crate::app::App;


impl App {
    pub fn show_tables(&mut self, ctx: &egui::Context) {
        egui::SidePanel::right("right_panel") .frame(
                    egui::Frame::default()
                    .outer_margin(0.0)
                    .inner_margin(4.0)
                    .fill(egui::Color32::from_rgb(245, 245, 245))
            )
            .resizable(true)
            .min_width(100.0)
            .default_width(500.0)
            .show(ctx, |ui| {
                ui.visuals_mut().striped = true;
                self.show_tokens_table(ui);
                self.show_symbols_table(ui);
            });
    }

    pub fn show_tokens_table(&mut self, ui: &mut egui::Ui) {


        ui.allocate_ui_with_layout(egui::Vec2::new(ui.available_width(), 18.0), 
            egui::Layout::top_down(egui::Align::Center),
            |ui| {
                ui.painter().rect_filled(
                    ui.available_rect_before_wrap(),
                    egui::CornerRadius::same(0),
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
                    ui.heading(RichText::new(self.translator.t("[Line, Column]")).size(14.0));
                });
            })
        .body(|mut body| {
            body.row(30.0, |mut row| {
                row.col(|ui| {
                    ui.label(self.translator.t("hello"));
                });
                row.col(|ui| {
                    ui.label(self.translator.t("world"));
                });
                row.col(|ui| {
                    ui.label("(aaa,aa)");
                });
            });
        });

    }

    pub fn show_symbols_table(&mut self, ui: &mut egui::Ui) {
        egui::TopBottomPanel::bottom("right_bottom_panel")
            .frame(
                egui::Frame::default()
                    .outer_margin(0.0)
                    .inner_margin(0.0)
            )
            .resizable(true)
            .default_height(ui.available_height() * 0.5)
            .min_height(ui.available_height() * 0.5)
            .show_inside(ui, |ui| {
                ui.available_size();

                ui.allocate_ui_with_layout(egui::Vec2::new(ui.available_width(), 18.0), 
                    egui::Layout::top_down(egui::Align::Center), |ui| {
                        ui.painter().rect_filled(
                            ui.available_rect_before_wrap(),
                            egui::CornerRadius::same(0),
                            egui::Color32::WHITE
                        );
                        ui.label(RichText::new(self.translator.t(" Symbols table")).strong().size(15.0));
                    });
                TableBuilder::new(ui)
                    .id_salt("aa")
                    .columns(Column::remainder(), 4)
                    .header(30.0, |mut header| {
                        header.col(|ui| {
                            ui.heading(RichText::new(self.translator.t("Identifier")).size(14.0));
                        });
                        header.col(|ui| {
                            ui.heading(RichText::new(self.translator.t("Data type")).size(14.0));
                        });
                        header.col(|ui| {
                            ui.heading(RichText::new(self.translator.t("Value")).size(14.0));
                        });
                        header.col(|ui| {
                            ui.heading(RichText::new("VarConst").size(14.0));
                        });
                    })
                    .body(|mut body| {
                        body.row(30.0, |mut row| {
                            row.col(|ui| {
                                ui.label(self.translator.t("hello"));
                            });
                            row.col(|ui| {
                                ui.label(self.translator.t("world"));
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
}
