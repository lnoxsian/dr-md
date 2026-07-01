use super::Editor;
use egui::text::LayoutJob;
use egui::{Color32, TextFormat, FontId};

pub struct EditorRenderer {
    pub content_buffer: String,
}

impl EditorRenderer {
    pub fn new() -> Self {
        Self {
            content_buffer: String::new(),
        }
    }

    pub fn sync_from_editor(&mut self, editor: &Editor) {
        self.content_buffer = editor.buffer.to_string();
    }

    pub fn sync_to_editor(&self, editor: &mut Editor) {
        if self.content_buffer != editor.buffer.to_string() {
            editor.set_text(&self.content_buffer);
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, editor: &mut Editor, font_size: f32, line_numbers: bool) {
        self.sync_from_editor(editor);

        let mut layouter = |ui: &egui::Ui, text: &str, wrap_width: f32| {
            let mut job = LayoutJob::default();
            let normal_font = FontId::monospace(font_size);
            let heading_font = FontId::monospace(font_size + 3.0);
            
            let default_color = ui.style().visuals.text_color();
            let header_color = Color32::from_rgb(240, 140, 60);
            let link_color = Color32::from_rgb(90, 160, 240);
            let code_color = Color32::from_rgb(140, 220, 140);
            let blockquote_color = Color32::from_rgb(140, 140, 140);

            let mut in_code_block = false;
            let lines: Vec<&str> = text.split('\n').collect();

            for (i, line) in lines.iter().enumerate() {
                let mut line_format = TextFormat {
                    font_id: normal_font.clone(),
                    color: default_color,
                    ..Default::default()
                };

                let trimmed = line.trim();
                if trimmed.starts_with("```") {
                    in_code_block = !in_code_block;
                    line_format.color = code_color;
                } else if in_code_block {
                    line_format.color = code_color;
                } else if trimmed.starts_with('#') {
                    line_format.font_id = heading_font.clone();
                    line_format.color = header_color;
                } else if trimmed.starts_with('>') {
                    line_format.color = blockquote_color;
                }

                let line_chars: Vec<char> = line.chars().collect();
                let mut idx = 0;
                while idx < line_chars.len() {
                    if idx + 1 < line_chars.len() && line_chars[idx] == '[' && line_chars[idx+1] == '[' {
                        if let Some(end_idx) = line[idx..].find("]]") {
                            let text_segment: String = line_chars[idx..idx + end_idx + 2].iter().collect();
                            job.append(
                                &text_segment,
                                0.0,
                                TextFormat {
                                    font_id: normal_font.clone(),
                                    color: link_color,
                                    underline: egui::Stroke::new(1.0, link_color),
                                    ..Default::default()
                                }
                            );
                            idx += end_idx + 2;
                            continue;
                        }
                    }

                    if line_chars[idx] == '[' {
                        if let Some(end_bracket) = line[idx..].find(']') {
                            if idx + end_bracket + 1 < line.len() && line.as_bytes()[idx + end_bracket + 1] == b'(' {
                                if let Some(end_paren) = line[idx + end_bracket + 1..].find(')') {
                                    let total_len = end_bracket + 1 + end_paren + 1;
                                    let text_segment: String = line_chars[idx..idx + total_len].iter().collect();
                                    job.append(
                                        &text_segment,
                                        0.0,
                                        TextFormat {
                                            font_id: normal_font.clone(),
                                            color: link_color,
                                            underline: egui::Stroke::new(1.0, link_color),
                                            ..Default::default()
                                        }
                                    );
                                    idx += total_len;
                                    continue;
                                }
                            }
                        }
                    }

                    let mut run = String::new();
                    while idx < line_chars.len() {
                        let c = line_chars[idx];
                        if c == '[' {
                            break;
                        }
                        run.push(c);
                        idx += 1;
                    }
                    job.append(&run, 0.0, line_format.clone());
                }

                if i < lines.len() - 1 {
                    job.append("\n", 0.0, TextFormat {
                        font_id: normal_font.clone(),
                        color: default_color,
                        ..Default::default()
                    });
                }
            }

            job.wrap.max_width = wrap_width;
            ui.fonts(|f| f.layout_job(job))
        };

        if line_numbers {
            ui.horizontal_top(|ui| {
                let total_lines = self.content_buffer.lines().count().max(1);
                let mut line_nums_str = String::new();
                for i in 1..=total_lines {
                    line_nums_str.push_str(&format!("{}\n", i));
                }
                
                ui.add(
                    egui::Label::new(
                        egui::RichText::new(line_nums_str)
                            .font(FontId::monospace(font_size))
                            .color(Color32::from_rgb(100, 100, 100))
                    )
                );
                
                let output = egui::ScrollArea::vertical()
                    .id_source("editor_scroll")
                    .show(ui, |ui| {
                        ui.add_sized(
                            ui.available_size(),
                            egui::TextEdit::multiline(&mut self.content_buffer)
                                .font(FontId::monospace(font_size))
                                .frame(false)
                                .layouter(&mut layouter)
                        )
                    });
                
                if output.inner.changed() {
                    self.sync_to_editor(editor);
                }
            });
        } else {
            let output = egui::ScrollArea::vertical()
                .id_source("editor_scroll")
                .show(ui, |ui| {
                    ui.add_sized(
                        ui.available_size(),
                        egui::TextEdit::multiline(&mut self.content_buffer)
                            .font(FontId::monospace(font_size))
                            .frame(false)
                            .layouter(&mut layouter)
                    )
                });
            if output.inner.changed() {
                self.sync_to_editor(editor);
            }
        }
    }
}
