use eframe::egui;
use crate::panels::{top_panel, right_panel, bottom_panel};
use crate::document::Document;


#[derive(Default)]
pub struct App {
    pub documents: Vec<Document>,
    pub active_tab: usize,
    pub next_document_id: usize,
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut app = Self::default();
        app.add_document("Program1.kf".to_string());
        app
    }

    pub fn add_document(&mut self, name: String) {
        self.documents.push(Document::new(name));
        self.active_tab = self.documents.len() -1;
    }

    pub fn close_document(&mut self, index: usize) {
        // If not the only document close the tab and change the active tab depending of the current
        // active tab
        if self.documents.len() > 1 && index < self.documents.len() {
            self.documents.remove(index);
            if self.active_tab >= self.documents.len() {
                self.active_tab = self.documents.len() - 1;
            } else if self.active_tab > index {
                self.active_tab -= 1;
            }
        // If the tab is the only one, open a new one always
        } else if index < self.documents.len() {
            self.documents.remove(index);
            self.next_document_id += 1;
            let name = format!("Program{}.kf", self.next_document_id+1);
            self.documents.push(Document::new(name));
            self.active_tab = self.documents.len() - 1;

        }
    }

    pub fn get_active_document_mut(&mut self) -> Option<&mut Document> {
        self.documents.get_mut(self.active_tab)
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        top_panel::show(ctx);

        right_panel::show(ctx);

        bottom_panel::show(ctx);

        self.show_tabs(ctx);
        self.show_central_panel(ctx);
    }
}

impl App {
    fn show_tabs(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("tabs_panel")
            .resizable(false)
            .min_height(32.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    let mut tab_to_close = None;

                    for (i, doc) in self.documents.iter().enumerate() {
                        // The name of each tab
                        let tab_text = if doc.is_modified {
                            format!("*{}", doc.name)
                        } else {
                            doc.name.clone()
                        };
                        

                        // Set the active tab when clicking the tab and show the label
                        let tab_click = ui.selectable_label(self.active_tab == i, &tab_text);
                        if tab_click.clicked() {
                            self.active_tab = i;
                        }
                        
                        // Create a new doc when all of them are closed
                        let close_response = ui.small_button("×");
                        if close_response.clicked() {
                            tab_to_close = Some(i);
                        }
                        
                        ui.separator();
                    }
                    
                    // Handle close tab
                    if let Some(index) = tab_to_close {
                        self.close_document(index);
                    }

                    if ui.button("+").clicked() {
                        self.next_document_id += 1;
                        let name = format!("Program{}.kf", self.next_document_id+1);
                        self.add_document(name);
                    }
                })


            });
    }

    fn show_central_panel(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(doc) = self.get_active_document_mut() {
                let prev_content = doc.content.clone();

                let response = egui::TextEdit::multiline(&mut doc.content)
                    .code_editor()
                    .min_size(ui.available_size())
                    .desired_width(f32::INFINITY)
                    .lock_focus(true)
                    .show(ui);

                if response.response.changed() {
                    if doc.content != prev_content {
                        doc.is_modified = true;
                    }
                }
            }

        });
    }
}
