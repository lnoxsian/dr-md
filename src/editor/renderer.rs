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
            let heading_font = FontId::monospace(font_size);
            
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
                    let mut matched = false;

                    // Check for wiki link: [[path]] or [[path|label]]
                    if idx + 2 < line_chars.len() && line_chars[idx] == '[' && line_chars[idx+1] == '[' {
                        let mut end_pos = None;
                        for j in (idx + 2)..line_chars.len() {
                            if j + 1 < line_chars.len() && line_chars[j] == ']' && line_chars[j+1] == ']' {
                                end_pos = Some(j);
                                break;
                            }
                        }
                        if let Some(j) = end_pos {
                            let text_segment: String = line_chars[idx..=j+1].iter().collect();
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
                            idx = j + 2;
                            matched = true;
                        }
                    }

                    // Check for standard markdown link: [label](url)
                    if !matched && line_chars[idx] == '[' {
                        let mut end_bracket = None;
                        for j in (idx + 1)..line_chars.len() {
                            if line_chars[j] == ']' {
                                end_bracket = Some(j);
                                break;
                            }
                        }
                        if let Some(eb) = end_bracket {
                            if eb + 1 < line_chars.len() && line_chars[eb+1] == '(' {
                                let mut end_paren = None;
                                for j in (eb + 2)..line_chars.len() {
                                    if line_chars[j] == ')' {
                                        end_paren = Some(j);
                                        break;
                                    }
                                }
                                if let Some(ep) = end_paren {
                                    let text_segment: String = line_chars[idx..=ep].iter().collect();
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
                                    idx = ep + 1;
                                    matched = true;
                                }
                            }
                        }
                    }

                    if matched {
                        continue;
                    }

                    let mut run = String::new();
                    // Always consume at least one character to ensure progress and prevent infinite loops
                    run.push(line_chars[idx]);
                    idx += 1;

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
            let total_lines = self.content_buffer.lines().count().max(1);
            let mut line_nums_str = String::new();
            for i in 1..=total_lines {
                line_nums_str.push_str(&format!("{}\n", i));
            }

            let output = egui::ScrollArea::vertical()
                .id_source("editor_scroll")
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    let edit_response = ui.horizontal_top(|ui| {
                        ui.add(
                            egui::Label::new(
                                egui::RichText::new(line_nums_str)
                                    .font(FontId::monospace(font_size))
                                    .color(Color32::from_rgb(100, 100, 100))
                            )
                        );
                        
                        ui.add(
                            egui::TextEdit::multiline(&mut self.content_buffer)
                                .font(FontId::monospace(font_size))
                                .frame(false)
                                .layouter(&mut layouter)
                                .desired_width(f32::INFINITY)
                        )
                    });
                    
                    // Add bottom padding inside scroll viewport
                    ui.add_space(100.0);
                    
                    edit_response.inner
                });
            
            if output.inner.changed() {
                self.sync_to_editor(editor);
            }
        } else {
            let output = egui::ScrollArea::vertical()
                .id_source("editor_scroll")
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    let res = ui.add(
                        egui::TextEdit::multiline(&mut self.content_buffer)
                            .font(FontId::monospace(font_size))
                            .frame(false)
                            .layouter(&mut layouter)
                            .desired_width(f32::INFINITY)
                    );
                    
                    // Add bottom padding inside scroll viewport
                    ui.add_space(100.0);
                    
                    res
                });
            if output.inner.changed() {
                self.sync_to_editor(editor);
            }
        }
    }
}
