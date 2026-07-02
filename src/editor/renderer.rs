use super::Editor;
use egui::text::LayoutJob;
use egui::{Color32, FontId, TextFormat};

pub struct EditorRenderer {
    pub content_buffer: String,
    pub last_version: usize,
}

impl EditorRenderer {
    pub fn new() -> Self {
        Self {
            content_buffer: String::new(),
            last_version: usize::MAX,
        }
    }

    pub fn sync_from_editor(&mut self, editor: &Editor) {
        if editor.version != self.last_version {
            self.content_buffer = editor.buffer.to_string();
            self.last_version = editor.version;
        }
    }

    pub fn sync_to_editor(&mut self, editor: &mut Editor) {
        editor.set_text(&self.content_buffer);
        self.last_version = editor.version;
    }

    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        editor: &mut Editor,
        font_size: f32,
        line_numbers: bool,
    ) {
        self.sync_from_editor(editor);

        if line_numbers {
            let mut line_positions = Vec::new();

            let output = egui::ScrollArea::vertical()
                .id_source("editor_scroll")
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    egui::Frame::none()
                        .inner_margin(egui::Margin::symmetric(24.0, 0.0))
                        .show(ui, |ui| {
                            let available_width = ui.available_width();
                            let gutter_width = 30.0;
                            let spacing = ui.spacing().item_spacing.x;
                            let text_wrap_width =
                                (available_width - gutter_width - spacing - 8.0).max(100.0);

                            let (gutter_rect, edit_res) = {
                                let mut layouter = |ui: &egui::Ui, text: &str, wrap_width: f32| {
                                    let default_color = ui.style().visuals.text_color();
                                    let link_color = ui.style().visuals.hyperlink_color;
                                    let mut job = create_layout_job(text, font_size, default_color, link_color);
                                    job.wrap.max_width = wrap_width.min(text_wrap_width);
                                    let galley = ui.fonts(|f| f.layout_job(job));

                                    let mut positions = Vec::new();
                                    let mut line_idx = 1;
                                    let mut next_row_is_new_line = true;
                                    for row in &galley.rows {
                                        if next_row_is_new_line {
                                            positions.push((line_idx, row.rect.min.y));
                                            line_idx += 1;
                                        }
                                        next_row_is_new_line = row.ends_with_newline;
                                    }
                                    line_positions = positions;

                                    galley
                                };

                                ui.horizontal_top(|ui| {
                                    let gutter_width = 30.0;
                                    let (gutter_rect, _) = ui.allocate_at_least(
                                        egui::vec2(gutter_width, ui.available_height()),
                                        egui::Sense::hover(),
                                    );

                                    let edit_res = ui.add(
                                        egui::TextEdit::multiline(&mut self.content_buffer)
                                            .id(egui::Id::new("editor_text_edit"))
                                            .font(FontId::monospace(font_size))
                                            .frame(false)
                                            .layouter(&mut layouter)
                                            .desired_width(f32::INFINITY),
                                    );

                                    (gutter_rect, edit_res)
                                })
                                .inner
                            };

                            // Paint line numbers onto the allocated gutter_rect
                            let painter = ui.painter();
                            let text_color = Color32::from_rgb(100, 100, 100);
                            let font = FontId::monospace(font_size);

                            for (line_num, y_offset) in &line_positions {
                                let text_pos = egui::pos2(
                                    gutter_rect.max.x - 5.0,
                                    edit_res.rect.min.y + y_offset,
                                );
                                painter.text(
                                    text_pos,
                                    egui::Align2::RIGHT_TOP,
                                    line_num.to_string(),
                                    font.clone(),
                                    text_color,
                                );
                            }

                            // Add bottom padding inside scroll viewport
                            ui.add_space(100.0);

                            edit_res
                        })
                        .inner
                });

            if output.inner.changed() {
                self.sync_to_editor(editor);
            }
        } else {
            let output = egui::ScrollArea::vertical()
                .id_source("editor_scroll")
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    egui::Frame::none()
                        .inner_margin(egui::Margin::symmetric(24.0, 0.0))
                        .show(ui, |ui| {
                            let available_width = ui.available_width();
                            let text_wrap_width = (available_width - 8.0).max(100.0);
                            let mut layouter = |ui: &egui::Ui, text: &str, wrap_width: f32| {
                                let default_color = ui.style().visuals.text_color();
                                let link_color = ui.style().visuals.hyperlink_color;
                                let mut job = create_layout_job(text, font_size, default_color, link_color);
                                job.wrap.max_width = wrap_width.min(text_wrap_width);
                                ui.fonts(|f| f.layout_job(job))
                            };

                            let res = ui.add(
                                egui::TextEdit::multiline(&mut self.content_buffer)
                                    .id(egui::Id::new("editor_text_edit"))
                                    .font(FontId::monospace(font_size))
                                    .frame(false)
                                    .layouter(&mut layouter)
                                    .desired_width(f32::INFINITY),
                            );

                            // Add bottom padding inside scroll viewport
                            ui.add_space(100.0);

                            res
                        })
                        .inner
                });
            if output.inner.changed() {
                self.sync_to_editor(editor);
            }
        }
    }
}

fn create_layout_job(text: &str, font_size: f32, text_color: Color32, link_color: Color32) -> LayoutJob {
    let mut job = LayoutJob::default();
    let normal_font = FontId::monospace(font_size);
    let heading_font = FontId::monospace(font_size);

    let default_color = text_color;
    let header_color = link_color;

    // Check if the theme is light by looking at text color lightness
    let is_light_theme = text_color.r() < 128;
    let code_color = if is_light_theme {
        Color32::from_rgb(0, 120, 0)
    } else {
        Color32::from_rgb(140, 220, 140)
    };
    let blockquote_color = if is_light_theme {
        Color32::from_rgb(80, 80, 80)
    } else {
        Color32::from_rgb(140, 140, 140)
    };

    let mut in_code_block = false;
    let mut lines = text.split('\n').peekable();

    while let Some(line) = lines.next() {
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
            if idx + 2 < line_chars.len() && line_chars[idx] == '[' && line_chars[idx + 1] == '[' {
                let mut end_pos = None;
                for j in (idx + 2)..line_chars.len() {
                    if j + 1 < line_chars.len() && line_chars[j] == ']' && line_chars[j + 1] == ']'
                    {
                        end_pos = Some(j);
                        break;
                    }
                }
                if let Some(j) = end_pos {
                    let text_segment: String = line_chars[idx..=j + 1].iter().collect();
                    job.append(
                        &text_segment,
                        0.0,
                        TextFormat {
                            font_id: normal_font.clone(),
                            color: link_color,
                            underline: egui::Stroke::new(1.0, link_color),
                            ..Default::default()
                        },
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
                    if eb + 1 < line_chars.len() && line_chars[eb + 1] == '(' {
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
                                },
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

        if lines.peek().is_some() {
            job.append(
                "\n",
                0.0,
                TextFormat {
                    font_id: normal_font.clone(),
                    color: default_color,
                    ..Default::default()
                },
            );
        }
    }
    job
}
