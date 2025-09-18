use eframe::egui;
use egui::{text_edit::TextEditOutput};

pub struct CodeEditor {
    id: String,
    fontsize: f32,
    rows: usize,
    desired_width: f32,
}

impl Default for CodeEditor {
    fn default() -> CodeEditor {
        CodeEditor {
            id: String::from("Code Editor"),
            fontsize: 10.0,
            rows: 10,
            desired_width: f32::INFINITY,
        }
    }
}

impl CodeEditor {
    pub fn id_source(self, id_source: impl Into<String>) -> Self {
        CodeEditor {
            id: id_source.into(),
            ..self
        }
    }

    pub fn with_ui_fontsize(self, ui: &mut egui::Ui) -> Self {
        CodeEditor {
            fontsize: egui::TextStyle::Monospace.resolve(ui.style()).size,
            ..self
        }
    }

    pub fn with_rows(self, rows: usize) -> Self {
        CodeEditor { 
            rows,
            ..self
        }
    }

    fn numlines_show(&self, ui: &mut egui::Ui, text: &str) {
        use egui::TextBuffer;

        let total = if text.ends_with('\n') || text.is_empty() {
            text.lines().count() + 1
        } else {
            text.lines().count()
        }
        .max(self.rows) as isize;

        let max_indent = total
            .to_string()
            .len();

        let mut counter = (1..=total)
            .map(|i| {
                let num = i;
                let label = num.to_string();
                format!(
                    "{}{label}",
                    " ".repeat(max_indent.saturating_sub(label.len()))
                )
            })
            .collect::<Vec<String>>()
            .join("\n");

        
        let width = max_indent as f32 * self.fontsize * 0.5;

        let mut layouter = |ui: &egui::Ui, text_buffer: &dyn TextBuffer, _wrap_width: f32| {
            let layout_job = egui::text::LayoutJob::single_section(
                text_buffer.as_str().to_string(),
                egui::TextFormat::simple(
                    egui::FontId::monospace(self.fontsize), 
                    egui::Color32::BLACK),
            );
            ui.fonts(|f| f.layout_job(layout_job))
        };

        ui.add(
            egui::TextEdit::multiline(&mut counter)
            .id_source(format!("{}_numlines", self.id))
            .font(egui::TextStyle::Monospace)
            .interactive(false)
            .frame(false)
            .desired_rows(self.rows)
            .desired_width(width)
            .layouter(&mut layouter)
        );
    }

    pub fn show(&mut self, ui: &mut egui::Ui, text: &mut dyn egui::TextBuffer) -> (TextEditOutput, usize, usize) {
        use egui::TextBuffer;
        
        let mut text_edit_output: Option<TextEditOutput> = None;
        let code_editor = |ui: &mut egui::Ui| {
            
            ui.horizontal_top(|h| {
                self.numlines_show(h, text.as_str());

                egui::ScrollArea::horizontal()
                    .id_salt(format!("{}_inner_scroll", self.id))
                    .show(h, |ui| {
                        let mut layouter = 
                            |ui: &egui::Ui, text_buffer: &dyn TextBuffer, _wrap_width: f32| {
                                let layout_job = egui::text::LayoutJob::single_section(
                                    text_buffer.as_str().to_string(),
                                    egui::TextFormat::simple(
                                        egui::FontId::monospace(self.fontsize), 
                                        egui::Color32::BLACK),
                                );
                                ui.fonts(|f| f.layout_job(layout_job))
                            };
                        let output = egui::TextEdit::multiline(text)
                            .id_source(&self.id)
                            .lock_focus(true)
                            .desired_rows(self.rows)
                            .frame(false)
                            .desired_width(self.desired_width)
                            .layouter(&mut layouter)
                            .show(ui);
                        text_edit_output = Some(output)
                    });
            });

        };

        egui::ScrollArea::vertical()
            .id_salt(format!("{}_outer_scroll", self.id))
            .stick_to_bottom(false)
            .show(ui, code_editor);

        let mut line: usize = 1;
        let mut col: usize = 1;
        if let Some(cursor_range) = text_edit_output.as_ref().unwrap().cursor_range {
            let cursor_pos = cursor_range.primary.index;
            (line, col) = calculate_line_col(text.as_str(), cursor_pos);
        }
    
        (text_edit_output.unwrap(), line, col)
    }
}

fn calculate_line_col(text: &str, cursor_pos: usize) -> (usize, usize) {
    let text_up_to_cursor = &text[..cursor_pos.min(text.len())];
    let line = text_up_to_cursor.matches('\n').count() + 1;
    let col = text_up_to_cursor
        .split('\n')
        .last()
        .map(|last_line| last_line.chars().count() + 1)
        .unwrap_or(1);
    (line, col)
}


