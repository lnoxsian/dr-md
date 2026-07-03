use super::Editor;
use egui::text::LayoutJob;
use egui::{Color32, FontId, TextFormat};

pub struct EditorRenderer {
    pub content_buffer: String,
    pub last_version: usize,
    pub previous_text: String,
    pub previous_cursor: Option<egui::text::CCursorRange>,
}

impl EditorRenderer {
    pub fn new() -> Self {
        Self {
            content_buffer: String::new(),
            last_version: usize::MAX,
            previous_text: String::new(),
            previous_cursor: None,
        }
    }

    pub fn sync_from_editor(&mut self, editor: &Editor) {
        if editor.version != self.last_version {
            self.content_buffer = editor.buffer.to_string();
            self.last_version = editor.version;
            self.previous_text = self.content_buffer.clone();
            self.previous_cursor = None;
        }
    }

    pub fn sync_to_editor(&mut self, editor: &mut Editor) {
        editor.set_text(&self.content_buffer);
        self.last_version = editor.version;
    }

    fn process_autoclosing(
        &mut self,
        ctx: &egui::Context,
        id: egui::Id,
        mut state: egui::widgets::text_edit::TextEditState,
        ui_changed: bool,
    ) -> bool {
        let mut text_changed = false;

        if let Some(range) = state.cursor.char_range() {
            let current_idx = range.primary.index;
            let secondary_idx = range.secondary.index;

            // Only perform autoclosing/modifications if the text was actually changed by typing/deleting
            if ui_changed {
                // Check if there was a selection in the previous frame
                if let Some(prev_range) = self.previous_cursor {
                    let prev_start = prev_range.primary.index.min(prev_range.secondary.index);
                    let prev_end = prev_range.primary.index.max(prev_range.secondary.index);

                    if prev_start != prev_end {
                        // There was a selection!
                        // Let's see if the user replaced it by typing a single character.
                        // Egui would replace the selection with the character, making the new cursor a single cursor
                        // at start + 1.
                        if current_idx == secondary_idx && current_idx == prev_start + 1 {
                            let new_chars: Vec<char> = self.content_buffer.chars().collect();
                            if prev_start < new_chars.len() {
                                let typed_char = new_chars[prev_start];
                                let selected_text: String = self.previous_text.chars().skip(prev_start).take(prev_end - prev_start).collect();

                                let mut replacement = None;
                                let mut cursor_offset = 0;

                                match typed_char {
                                    '(' => {
                                        replacement = Some(format!("({})", selected_text));
                                        cursor_offset = selected_text.len() + 2;
                                    }
                                    '[' => {
                                        replacement = Some(format!("[{}]()", selected_text));
                                        cursor_offset = selected_text.len() + 3;
                                    }
                                    '{' => {
                                        replacement = Some(format!("{{{}}}", selected_text));
                                        cursor_offset = selected_text.len() + 2;
                                    }
                                    '"' => {
                                        replacement = Some(format!("\"{}\"", selected_text));
                                        cursor_offset = selected_text.len() + 2;
                                    }
                                    '\'' => {
                                        replacement = Some(format!("'{}\'", selected_text));
                                        cursor_offset = selected_text.len() + 2;
                                    }
                                    '`' => {
                                        replacement = Some(format!("`{}`", selected_text));
                                        cursor_offset = selected_text.len() + 2;
                                    }
                                    '*' => {
                                        replacement = Some(format!("*{}*", selected_text));
                                        cursor_offset = selected_text.len() + 2;
                                    }
                                    '_' => {
                                        replacement = Some(format!("_{}_", selected_text));
                                        cursor_offset = selected_text.len() + 2;
                                    }
                                    '~' => {
                                        replacement = Some(format!("~{}~", selected_text));
                                        cursor_offset = selected_text.len() + 2;
                                    }
                                    _ => {}
                                }

                                if let Some(rep) = replacement {
                                    let mut final_text = String::new();
                                    final_text.push_str(&self.content_buffer.chars().take(prev_start).collect::<String>());
                                    final_text.push_str(&rep);
                                    final_text.push_str(&self.content_buffer.chars().skip(prev_start + 1).collect::<String>());

                                    self.content_buffer = final_text;
                                    text_changed = true;

                                    let new_cursor_idx = prev_start + cursor_offset;
                                    let ccursor = egui::text::CCursor::new(new_cursor_idx);
                                    state.cursor.set_char_range(Some(egui::text::CCursorRange::two(ccursor, ccursor)));
                                }
                            }
                        }
                    } else {
                        // No selection in previous frame.
                        let prev_idx = prev_range.primary.index;
                        if current_idx == secondary_idx {
                            if current_idx == prev_idx + 1 {
                                let new_chars: Vec<char> = self.content_buffer.chars().collect();
                                if prev_idx < new_chars.len() {
                                    let typed_char = new_chars[prev_idx];

                                    // 1. STEP OVER CLOSING BRACKETS
                                    let mut stepped_over = false;
                                    if [')', ']', '}', '"', '\'', '`'].contains(&typed_char) {
                                        if current_idx < new_chars.len() && new_chars[current_idx] == typed_char {
                                            let mut final_text = String::new();
                                            final_text.push_str(&self.content_buffer.chars().take(current_idx).collect::<String>());
                                            final_text.push_str(&self.content_buffer.chars().skip(current_idx + 1).collect::<String>());

                                            self.content_buffer = final_text;
                                            text_changed = true;
                                            stepped_over = true;

                                            let ccursor = egui::text::CCursor::new(current_idx);
                                            state.cursor.set_char_range(Some(egui::text::CCursorRange::two(ccursor, ccursor)));
                                        }
                                    }

                                    if !stepped_over {
                                        // 2. AUTOCLOSE OPENING BRACKETS
                                        let mut autoclose_char = None;
                                        match typed_char {
                                            '(' => autoclose_char = Some(')'),
                                            '[' => autoclose_char = Some(']'),
                                            '{' => autoclose_char = Some('}'),
                                            '"' => autoclose_char = Some('"'),
                                            '\'' => autoclose_char = Some('\''),
                                            '`' => {
                                                if prev_idx >= 2
                                                    && new_chars[prev_idx - 1] == '`'
                                                    && new_chars[prev_idx - 2] == '`'
                                                {
                                                    let mut final_text = String::new();
                                                    final_text.push_str(&self.content_buffer.chars().take(current_idx).collect::<String>());
                                                    final_text.push_str("```");
                                                    final_text.push_str(&self.content_buffer.chars().skip(current_idx).collect::<String>());

                                                    self.content_buffer = final_text;
                                                    text_changed = true;

                                                    let ccursor = egui::text::CCursor::new(current_idx);
                                                    state.cursor.set_char_range(Some(egui::text::CCursorRange::two(ccursor, ccursor)));
                                                } else {
                                                    autoclose_char = Some('`');
                                                }
                                            }
                                            '*' => {
                                                if prev_idx >= 1 && new_chars[prev_idx - 1] == '*' {
                                                    let mut final_text = String::new();
                                                    final_text.push_str(&self.content_buffer.chars().take(current_idx).collect::<String>());
                                                    final_text.push_str("**");
                                                    final_text.push_str(&self.content_buffer.chars().skip(current_idx).collect::<String>());

                                                    self.content_buffer = final_text;
                                                    text_changed = true;

                                                    let ccursor = egui::text::CCursor::new(current_idx);
                                                    state.cursor.set_char_range(Some(egui::text::CCursorRange::two(ccursor, ccursor)));
                                                }
                                            }
                                            '_' => {
                                                if prev_idx >= 1 && new_chars[prev_idx - 1] == '_' {
                                                    let mut final_text = String::new();
                                                    final_text.push_str(&self.content_buffer.chars().take(current_idx).collect::<String>());
                                                    final_text.push_str("__");
                                                    final_text.push_str(&self.content_buffer.chars().skip(current_idx).collect::<String>());

                                                    self.content_buffer = final_text;
                                                    text_changed = true;

                                                    let ccursor = egui::text::CCursor::new(current_idx);
                                                    state.cursor.set_char_range(Some(egui::text::CCursorRange::two(ccursor, ccursor)));
                                                }
                                            }
                                            '~' => {
                                                if prev_idx >= 1 && new_chars[prev_idx - 1] == '~' {
                                                    let mut final_text = String::new();
                                                    final_text.push_str(&self.content_buffer.chars().take(current_idx).collect::<String>());
                                                    final_text.push_str("~~");
                                                    final_text.push_str(&self.content_buffer.chars().skip(current_idx).collect::<String>());

                                                    self.content_buffer = final_text;
                                                    text_changed = true;

                                                    let ccursor = egui::text::CCursor::new(current_idx);
                                                    state.cursor.set_char_range(Some(egui::text::CCursorRange::two(ccursor, ccursor)));
                                                }
                                            }
                                            _ => {}
                                        }

                                        if let Some(ac) = autoclose_char {
                                            let mut final_text = String::new();
                                            final_text.push_str(&self.content_buffer.chars().take(current_idx).collect::<String>());
                                            final_text.push_str(&ac.to_string());
                                            final_text.push_str(&self.content_buffer.chars().skip(current_idx).collect::<String>());

                                            self.content_buffer = final_text;
                                            text_changed = true;

                                            let ccursor = egui::text::CCursor::new(current_idx);
                                            state.cursor.set_char_range(Some(egui::text::CCursorRange::two(ccursor, ccursor)));
                                        }
                                    }
                                }
                            } else if current_idx == prev_idx.saturating_sub(1) && prev_idx > 0 {
                                let old_chars: Vec<char> = self.previous_text.chars().collect();
                                let new_chars: Vec<char> = self.content_buffer.chars().collect();
                                if current_idx < old_chars.len() {
                                    let deleted_char = old_chars[current_idx];
                                    let mut matching_close = None;
                                    match deleted_char {
                                        '(' => matching_close = Some(')'),
                                        '[' => matching_close = Some(']'),
                                        '{' => matching_close = Some('}'),
                                        '"' => matching_close = Some('"'),
                                        '\'' => matching_close = Some('\''),
                                        '`' => matching_close = Some('`'),
                                        _ => {}
                                    }
                                    if let Some(close_char) = matching_close {
                                        if current_idx < new_chars.len() && new_chars[current_idx] == close_char {
                                            let mut final_text = String::new();
                                            final_text.push_str(&self.content_buffer.chars().take(current_idx).collect::<String>());
                                            final_text.push_str(&self.content_buffer.chars().skip(current_idx + 1).collect::<String>());

                                            self.content_buffer = final_text;
                                            text_changed = true;

                                            let ccursor = egui::text::CCursor::new(current_idx);
                                            state.cursor.set_char_range(Some(egui::text::CCursorRange::two(ccursor, ccursor)));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            self.previous_cursor = state.cursor.char_range();
        } else {
            self.previous_cursor = None;
        }

        self.previous_text = self.content_buffer.clone();

        if text_changed {
            state.store(ctx, id);
        }

        text_changed
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

                            let (gutter_rect, edit_res, text_changed) = {
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

                                    let text_edit = egui::TextEdit::multiline(&mut self.content_buffer)
                                        .id(egui::Id::new("editor_text_edit"))
                                        .font(FontId::monospace(font_size))
                                        .frame(false)
                                        .layouter(&mut layouter)
                                        .desired_width(f32::INFINITY);
                                    let edit_output = text_edit.show(ui);
                                    let edit_res = edit_output.response;

                                    let text_changed = self.process_autoclosing(ui.ctx(), edit_res.id, edit_output.state, edit_res.changed());

                                    (gutter_rect, edit_res, text_changed)
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

                            (edit_res, text_changed)
                        })
                        .inner
                });

            if output.inner.0.changed() || output.inner.1 {
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

                            let text_edit = egui::TextEdit::multiline(&mut self.content_buffer)
                                .id(egui::Id::new("editor_text_edit"))
                                .font(FontId::monospace(font_size))
                                .frame(false)
                                .layouter(&mut layouter)
                                .desired_width(f32::INFINITY);
                            let edit_output = text_edit.show(ui);
                            let edit_res = edit_output.response;

                            let text_changed = self.process_autoclosing(ui.ctx(), edit_res.id, edit_output.state, edit_res.changed());

                            // Add bottom padding inside scroll viewport
                            ui.add_space(100.0);

                            (edit_res, text_changed)
                        })
                        .inner
                });

            if output.inner.0.changed() || output.inner.1 {
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
