use eframe::egui;
use egui::RichText;
use egui_extras::{
    Column, 
    TableBuilder
};
use crate::app::App;


use kf_compiler::parse_program;
use kf_compiler::{
    SymbolType
};

fn _ejecutar(code: &str) -> String {
    match parse_program(code) {
        Ok(program) => format!("{:#?}", program),
        Err(e) => format!("Error: {:#?}", e),
    }
}

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

                ui.allocate_ui_with_layout(
                    egui::Vec2::new(ui.available_width(), 400.0), 
                    egui::Layout::top_down(egui::Align::Min),
                    |ui| {
                        self.show_tokens_table(ui);
                    }
                );
                
                ui.allocate_ui_with_layout(
                    egui::Vec2::new(ui.available_width(), ui.available_height()), 
                    egui::Layout::top_down(egui::Align::Min),
                    |ui| {
                        self.show_symbols_table(ui);
                    }
                );
            });
    }

    pub fn show_tokens_table(&mut self, ui: &mut egui::Ui) {
        ui.allocate_ui_with_layout(
            egui::Vec2::new(ui.available_width(), 18.0), 
            egui::Layout::top_down(egui::Align::Center),
            |ui| {
                ui.painter().rect_filled(
                    ui.available_rect_before_wrap(),
                    egui::CornerRadius::same(0),
                    egui::Color32::WHITE
                );
                self.section_header(ui, "Tokens table");
            }
        );

        // barrita para tokens
        egui::ScrollArea::vertical()
            .id_salt("tokens_scroll")
            .auto_shrink([false, false])
            .show(ui, |ui| {
                TableBuilder::new(ui)
                    .id_salt("tokens_table")
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
                        for token in &self.tokens {
                            body.row(30.0, |mut row| {
                                row.col(|ui| {
                                    ui.label(token.token.name());
                                });
                                row.col(|ui| {
                                    ui.label(token.token.value());
                                });
                                row.col(|ui| {
                                    ui.label(format!("({}, {})", token.position.line, token.position.col));
                                });
                            });
                        }
                    });
            });
    }

    pub fn show_symbols_table(&mut self, ui: &mut egui::Ui) {

        ui.allocate_ui_with_layout(
            egui::Vec2::new(ui.available_width(), 18.0), 
            egui::Layout::top_down(egui::Align::Center),
            |ui| {
                ui.painter().rect_filled(
                    ui.available_rect_before_wrap(),
                    egui::CornerRadius::same(0),
                    egui::Color32::WHITE
                );

                self.section_header(ui, "Tokens table");
            }
        );

        // ScrollArea para la tabla de símbolos
        egui::ScrollArea::vertical()
            .id_salt("symbols_scroll")
            .auto_shrink([false, false])
            .show(ui, |ui| {
                TableBuilder::new(ui)
                    .id_salt("symbols_table")
                    .columns(Column::remainder(), 5)
                    .header(24.0, |mut header| {
                        header.col(|ui| {
                            ui.heading(RichText::new(self.translator.t("Scope")).size(14.0));
                        });
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
                                for symbol in &self.symbols {
                                    body.row(30.0, |mut row| {
                                        row.col(|ui| {
                                            ui.label(&symbol.scope);  
                                        });
                                        row.col(|ui| {
                                            ui.label(&symbol.name);  
                                        });
                                        row.col(|ui| {
                                            ui.label(&symbol.data_type);  
                                        });
                                        row.col(|ui| {
                                            ui.label(symbol.value.as_deref().unwrap_or("-"));  
                                        });
                                        row.col(|ui| {
                                            let varconst = match symbol.symbol_type {
                                                SymbolType::Variable => "Variable",   
                                                SymbolType::Constant => "Constant",
                                                SymbolType::Function => "Function",
                                                SymbolType::Parameter => "Parameter",
                                            };
                                            ui.label(varconst);  
                                        });
                                    });
                                }
                            }
                    );
                });
            //});
    }

    pub fn section_header(&self, ui: &mut egui::Ui, title: &str) {
        let header_color = egui::Color32::from_rgb(200, 200, 250);
        let text_color = egui::Color32::BLACK;

        ui.allocate_ui_with_layout(
            egui::Vec2::new(ui.available_width(), 20.0),
            egui::Layout::top_down(egui::Align::Center),
            |ui| {
                ui.painter().rect_filled(
                    ui.available_rect_before_wrap(),
                    egui::CornerRadius::same(0),
                    header_color,
                );
                ui.label(RichText::new(title).strong().size(14.0).color(text_color));
            },
        );
    }

}
