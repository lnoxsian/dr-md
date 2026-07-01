use egui_commonmark::{CommonMarkCache, CommonMarkViewer};

pub struct MarkdownPreview {
    pub cache: CommonMarkCache,
}

impl MarkdownPreview {
    pub fn new() -> Self {
        Self {
            cache: CommonMarkCache::default(),
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, content: &mut String, font_size: f32) {
        let mut processed = super::parser::preprocess_wiki_links(content);
        let processed_old = processed.clone();
        
        egui::ScrollArea::vertical()
            .id_source("markdown_preview_scroll")
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                let mut style = ui.style().as_ref().clone();
                let body_font = egui::FontId::new(font_size, egui::FontFamily::Proportional);
                let heading_font = egui::FontId::new(font_size * 1.4, egui::FontFamily::Proportional);
                let monospace_font = egui::FontId::new(font_size, egui::FontFamily::Monospace);
                
                style.text_styles.insert(egui::TextStyle::Body, body_font);
                style.text_styles.insert(egui::TextStyle::Heading, heading_font);
                style.text_styles.insert(egui::TextStyle::Monospace, monospace_font);
                
                ui.set_style(style);

                CommonMarkViewer::new("markdown_viewer").show_mut(ui, &mut self.cache, &mut processed);
                
                // Add bottom padding inside scroll viewport
                ui.add_space(100.0);
            });

        if processed != processed_old {
            // Apply checkbox changes back to content line by line
            let old_lines: Vec<&str> = processed_old.lines().collect();
            let new_lines: Vec<&str> = processed.lines().collect();
            let mut orig_lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();

            if old_lines.len() == new_lines.len() && old_lines.len() == orig_lines.len() {
                for i in 0..old_lines.len() {
                    if old_lines[i] != new_lines[i] {
                        let old_trimmed = old_lines[i].trim_start();
                        let new_trimmed = new_lines[i].trim_start();

                        let is_toggle = (old_trimmed.starts_with("- [ ]") && new_trimmed.starts_with("- [x]"))
                            || (old_trimmed.starts_with("- [x]") && new_trimmed.starts_with("- [ ]"))
                            || (old_trimmed.starts_with("* [ ]") && new_trimmed.starts_with("* [x]"))
                            || (old_trimmed.starts_with("* [x]") && new_trimmed.starts_with("* [ ]"));

                        if is_toggle {
                            if let Some(pos) = orig_lines[i].find("- [") {
                                let current_char = orig_lines[i].as_bytes().get(pos + 3).copied().unwrap_or(b' ');
                                let new_char = if current_char == b'x' { ' ' } else { 'x' };
                                let mut orig_chars: Vec<char> = orig_lines[i].chars().collect();
                                let mut char_pos = 0;
                                for (c_idx, _) in orig_lines[i].char_indices() {
                                    if c_idx == pos + 3 {
                                        break;
                                    }
                                    char_pos += 1;
                                }
                                if char_pos < orig_chars.len() {
                                    orig_chars[char_pos] = new_char;
                                    orig_lines[i] = orig_chars.into_iter().collect();
                                }
                            } else if let Some(pos) = orig_lines[i].find("* [") {
                                let current_char = orig_lines[i].as_bytes().get(pos + 3).copied().unwrap_or(b' ');
                                let new_char = if current_char == b'x' { ' ' } else { 'x' };
                                let mut orig_chars: Vec<char> = orig_lines[i].chars().collect();
                                let mut char_pos = 0;
                                for (c_idx, _) in orig_lines[i].char_indices() {
                                    if c_idx == pos + 3 {
                                        break;
                                    }
                                    char_pos += 1;
                                }
                                if char_pos < orig_chars.len() {
                                    orig_chars[char_pos] = new_char;
                                    orig_lines[i] = orig_chars.into_iter().collect();
                                }
                            }
                        }
                    }
                }
                *content = orig_lines.join("\n");
            }
        }
    }
}
