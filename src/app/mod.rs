pub mod state;
pub mod commands;

use state::{AppState, ViewMode};
use crate::keymap::{handle_key_events, ShortcutAction};
use std::path::PathBuf;

pub struct DoctorMarkdownApp {
    state: AppState,
}

impl DoctorMarkdownApp {
    pub fn new(cc: &eframe::CreationContext<'_>, root_path: Option<PathBuf>) -> Self {
        let mut style = (*cc.egui_ctx.style()).clone();
        style.visuals.dark_mode = true;
        style.visuals.widgets.noninteractive.rounding = egui::Rounding::ZERO;
        style.visuals.widgets.inactive.rounding = egui::Rounding::ZERO;
        style.visuals.widgets.hovered.rounding = egui::Rounding::ZERO;
        style.visuals.widgets.active.rounding = egui::Rounding::ZERO;
        style.visuals.widgets.open.rounding = egui::Rounding::ZERO;
        style.visuals.window_rounding = egui::Rounding::ZERO;
        style.visuals.menu_rounding = egui::Rounding::ZERO;
        cc.egui_ctx.set_style(style);

        Self {
            state: AppState::new(root_path),
        }
    }
}
impl eframe::App for DoctorMarkdownApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Some(open_url) = ctx.output_mut(|o| o.open_url.take()) {
            let url_str = open_url.url.as_str();
            if !url_str.starts_with("http://") && !url_str.starts_with("https://") && !url_str.starts_with("mailto:") {
                let decoded_path = urlencoding::decode(url_str).unwrap_or(std::borrow::Cow::Borrowed(url_str));
                if let Some(ref root) = self.state.vault.root_path {
                    let mut target_path = root.join(decoded_path.as_ref());
                    if !target_path.is_dir() && !target_path.to_string_lossy().ends_with(".md") {
                        target_path = root.join(format!("{}.md", decoded_path));
                    }

                    if target_path.exists() && target_path.is_file() {
                        commands::execute_open_file(&mut self.state, target_path);
                    } else {
                        if let Some(parent) = target_path.parent() {
                            std::fs::create_dir_all(parent).ok();
                        }
                        std::fs::write(&target_path, "").ok();
                        commands::execute_open_file(&mut self.state, target_path);
                    }
                }
            } else {
                ctx.output_mut(|o| o.open_url = Some(open_url));
            }
        }

        if let Some(action) = handle_key_events(ctx) {
            match action {
                ShortcutAction::NewNote => {
                    commands::execute_new_note(&mut self.state);
                }
                ShortcutAction::OpenFolder => {
                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                        commands::execute_open_folder(&mut self.state, path);
                    }
                }
                ShortcutAction::Save => {
                    commands::execute_save(&mut self.state);
                }
                ShortcutAction::SaveAs => {
                    if let Some(path) = rfd::FileDialog::new().save_file() {
                        commands::execute_save_as(&mut self.state, path);
                    }
                }
                ShortcutAction::CloseNote => {
                    self.state.vault.active_file = None;
                    self.state.editor = crate::editor::Editor::new();
                }
                ShortcutAction::Undo => {
                    self.state.editor.undo();
                }
                ShortcutAction::Redo => {
                    self.state.editor.redo();
                }
                ShortcutAction::ViewEditor => {
                    self.state.view_mode = ViewMode::Editor;
                }
                ShortcutAction::ViewPreview => {
                    self.state.view_mode = ViewMode::Preview;
                }
                ShortcutAction::ViewSplit => {
                    self.state.view_mode = ViewMode::Split;
                }
                ShortcutAction::ToggleExplorer => {
                    self.state.explorer_visible = !self.state.explorer_visible;
                }
                ShortcutAction::ToggleFocusMode => {
                    self.state.focus_mode = !self.state.focus_mode;
                    if self.state.focus_mode {
                        self.state.explorer_visible = false;
                    } else {
                        self.state.explorer_visible = true;
                    }
                }
                ShortcutAction::Bold => {
                    self.state.editor.insert_text("****");
                }
                ShortcutAction::Italic => {
                    self.state.editor.insert_text("**");
                }
                ShortcutAction::Link => {
                    self.state.editor.insert_text("[](url)");
                }
                ShortcutAction::CodeBlock => {
                    self.state.editor.insert_text("```\n\n```");
                }
                ShortcutAction::Checkbox => {
                    self.state.editor.insert_text("- [ ] ");
                }
                _ => {}
            }
        }

        if !self.state.focus_mode {
            egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
                egui::menu::bar(ui, |ui| {
                    ui.menu_button("File", |ui| {
                        if ui.button("New Note (Ctrl+N)").clicked() {
                            commands::execute_new_note(&mut self.state);
                            ui.close_menu();
                        }
                        if ui.button("Open Folder (Ctrl+O)").clicked() {
                            if let Some(path) = rfd::FileDialog::new().pick_folder() {
                                commands::execute_open_folder(&mut self.state, path);
                            }
                            ui.close_menu();
                        }
                        ui.separator();
                        if ui.button("Save (Ctrl+S)").clicked() {
                            commands::execute_save(&mut self.state);
                            ui.close_menu();
                        }
                        if ui.button("Save As (Ctrl+Shift+S)").clicked() {
                            if let Some(path) = rfd::FileDialog::new().save_file() {
                                commands::execute_save_as(&mut self.state, path);
                            }
                            ui.close_menu();
                        }
                        ui.separator();
                        if ui.button("Close Note (Ctrl+W)").clicked() {
                            self.state.vault.active_file = None;
                            self.state.editor = crate::editor::Editor::new();
                            ui.close_menu();
                        }
                    });

                    ui.menu_button("Edit", |ui| {
                        if ui.button("Cut (Ctrl+X)").clicked() {
                            if let Some(mut state) = egui::widgets::text_edit::TextEditState::load(ctx, egui::Id::new("editor_text_edit")) {
                                if let Some(range) = state.cursor.char_range() {
                                    let start = range.primary.index.min(range.secondary.index);
                                    let end = range.primary.index.max(range.secondary.index);
                                    let sorted = start..end;
                                    if !sorted.is_empty() {
                                        let text_to_copy = self.state.editor.buffer.rope.slice(sorted.clone()).to_string();
                                        ctx.copy_text(text_to_copy);

                                        self.state.editor.buffer.remove(sorted.start, sorted.end);
                                        self.state.editor.cursor.char_idx = sorted.start;
                                        self.state.editor.selection.clear(sorted.start);
                                        self.state.editor.is_dirty = true;

                                        self.state.editor_renderer.content_buffer = self.state.editor.buffer.to_string();

                                        let cursor = egui::text::CCursor::new(sorted.start);
                                        state.cursor.set_char_range(Some(egui::text::CCursorRange::two(cursor, cursor)));
                                        state.store(ctx, egui::Id::new("editor_text_edit"));
                                    }
                                }
                            }
                            ui.close_menu();
                        }
                        if ui.button("Copy (Ctrl+C)").clicked() {
                            if let Some(state) = egui::widgets::text_edit::TextEditState::load(ctx, egui::Id::new("editor_text_edit")) {
                                if let Some(range) = state.cursor.char_range() {
                                    let start = range.primary.index.min(range.secondary.index);
                                    let end = range.primary.index.max(range.secondary.index);
                                    let sorted = start..end;
                                    if !sorted.is_empty() {
                                        let text_to_copy = self.state.editor.buffer.rope.slice(sorted).to_string();
                                        ctx.copy_text(text_to_copy);
                                    }
                                }
                            }
                            ui.close_menu();
                        }
                        if ui.button("Paste (Ctrl+V)").clicked() {
                            if let Ok(mut clipboard) = arboard::Clipboard::new() {
                                let paste_text = clipboard.get_text().unwrap_or_default();
                                if !paste_text.is_empty() {
                                    if let Some(mut state) = egui::widgets::text_edit::TextEditState::load(ctx, egui::Id::new("editor_text_edit")) {
                                        let range = state.cursor.char_range().unwrap_or_else(|| {
                                            let len = self.state.editor.buffer.len_chars();
                                            let cursor = egui::text::CCursor::new(len);
                                            egui::text::CCursorRange::two(cursor, cursor)
                                        });
                                        let start = range.primary.index.min(range.secondary.index);
                                        let end = range.primary.index.max(range.secondary.index);
                                        let sorted = start..end;

                                        if !sorted.is_empty() {
                                            self.state.editor.buffer.remove(sorted.start, sorted.end);
                                        }

                                        self.state.editor.buffer.insert(sorted.start, &paste_text);
                                        self.state.editor.cursor.char_idx = sorted.start + paste_text.chars().count();
                                        self.state.editor.selection.clear(self.state.editor.cursor.char_idx);
                                        self.state.editor.is_dirty = true;

                                        self.state.editor_renderer.content_buffer = self.state.editor.buffer.to_string();

                                        let cursor = egui::text::CCursor::new(self.state.editor.cursor.char_idx);
                                        state.cursor.set_char_range(Some(egui::text::CCursorRange::two(cursor, cursor)));
                                        state.store(ctx, egui::Id::new("editor_text_edit"));
                                    }
                                }
                            }
                            ui.close_menu();
                        }
                    });

                    ui.menu_button("View", |ui| {
                        if ui.selectable_label(self.state.view_mode == ViewMode::Editor, "Editor Mode (Ctrl+1)").clicked() {
                            self.state.view_mode = ViewMode::Editor;
                            ui.close_menu();
                        }
                        if ui.selectable_label(self.state.view_mode == ViewMode::Preview, "Preview Mode (Ctrl+2)").clicked() {
                            self.state.view_mode = ViewMode::Preview;
                            ui.close_menu();
                        }
                        if ui.selectable_label(self.state.view_mode == ViewMode::Split, "Split Mode (Ctrl+3)").clicked() {
                            self.state.view_mode = ViewMode::Split;
                            ui.close_menu();
                        }
                        ui.separator();
                        if ui.selectable_label(self.state.explorer_visible, "Show File Explorer (Ctrl+E)").clicked() {
                            self.state.explorer_visible = !self.state.explorer_visible;
                            ui.close_menu();
                        }
                        if ui.selectable_label(self.state.focus_mode, "Focus Mode (F11)").clicked() {
                            self.state.focus_mode = !self.state.focus_mode;
                            self.state.explorer_visible = !self.state.focus_mode;
                            ui.close_menu();
                        }
                    });

                    ui.menu_button("Settings", |ui| {
                        ui.label("Font Size:");
                        if ui.add(egui::Slider::new(&mut self.state.config.font_size, 10.0..=30.0)).changed() {
                            let _ = self.state.config.save();
                        }
                        ui.separator();
                        if ui.checkbox(&mut self.state.config.line_numbers, "Show Line Numbers").changed() {
                            let _ = self.state.config.save();
                        }
                        if ui.checkbox(&mut self.state.config.autosave, "Autosave").changed() {
                            let _ = self.state.config.save();
                        }
                    });
                });
            });
        }

        if self.state.explorer_visible && !self.state.focus_mode {
            let screen_width = ctx.screen_rect().width();
            let max_explorer_width = if screen_width < 600.0 {
                (screen_width * 0.35).max(100.0)
            } else {
                300.0
            };
            let default_explorer_width = if screen_width < 600.0 {
                120.0
            } else {
                200.0
            };

            egui::SidePanel::left("file_explorer")
                .resizable(true)
                .default_width(default_explorer_width)
                .max_width(max_explorer_width)
                .show(ctx, |ui| {
                    ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                        ui.add_space(4.0);
                        if let Some(ref path) = self.state.editor.active_path {
                            let name = path.file_name().unwrap_or_default().to_string_lossy();
                            let dirty = if self.state.editor.is_dirty { "*" } else { "" };
                            ui.label(format!("{}{}", name, dirty));
                        } else {
                            ui.label("No file open");
                        }
                        ui.add_space(4.0);
                        ui.separator();

                        ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                            egui::ScrollArea::vertical().show(ui, |ui| {
                                if let Some(ref root) = self.state.vault.root_path {
                                    let mut active_file = self.state.vault.active_file.clone();
                                    if let Some(clicked) = self.state.explorer.show(ui, root, &mut active_file) {
                                        commands::execute_open_file(&mut self.state, clicked);
                                    }
                                } else {
                                    ui.vertical_centered(|ui| {
                                        ui.label("No folder opened");
                                        if ui.button("Open Folder").clicked() {
                                            if let Some(path) = rfd::FileDialog::new().pick_folder() {
                                                commands::execute_open_folder(&mut self.state, path);
                                            }
                                        }
                                    });
                                }
                            });
                        });
                    });
                });
        }

        let panel_frame = egui::Frame::central_panel(&ctx.style())
            .inner_margin(egui::Margin::symmetric(0.0, 8.0));
        egui::CentralPanel::default().frame(panel_frame).show(ctx, |ui| {
            if self.state.vault.active_file.is_some() {
                match self.state.view_mode {
                    ViewMode::Editor => {
                        self.state.editor_renderer.show(
                            ui,
                            &mut self.state.editor,
                            self.state.config.font_size,
                            self.state.config.line_numbers
                        );
                    }
                    ViewMode::Preview => {
                        let mut content = self.state.editor.buffer.to_string();
                        let old_content = content.clone();
                        self.state.preview.show(ui, &mut content, self.state.config.font_size);
                        if content != old_content {
                            self.state.editor.set_text(&content);
                        }
                    }
                    ViewMode::Split => {
                        let mut content = self.state.editor.buffer.to_string();
                        let old_content = content.clone();

                        let min_rect = ui.max_rect();
                        // Allocate the entire space so egui knows we are occupying it
                        ui.allocate_rect(min_rect, egui::Sense::hover());

                        let available_width = min_rect.width();
                        let available_height = min_rect.height();

                        if available_width < 600.0 {
                            // Vertical Split (Top-to-Bottom)
                            let separator_height = 8.0;
                            let total_height = available_height - separator_height;
                            let top_height = (total_height * self.state.split_ratio).max(100.0).min(total_height - 100.0);
                            let bottom_height = total_height - top_height;

                            // Top Rect
                            let top_rect = egui::Rect::from_min_size(
                                min_rect.min,
                                egui::vec2(available_width, top_height)
                            );

                            // Separator Rect
                            let sep_rect = egui::Rect::from_min_size(
                                egui::pos2(min_rect.min.x, min_rect.min.y + top_height),
                                egui::vec2(available_width, separator_height)
                            );

                            // Bottom Rect
                            let bottom_rect = egui::Rect::from_min_size(
                                egui::pos2(min_rect.min.x, min_rect.min.y + top_height + separator_height),
                                egui::vec2(available_width, bottom_height)
                            );

                            // Top Pane: Editor
                            let mut top_ui = ui.child_ui(top_rect, egui::Layout::top_down(egui::Align::Min));
                            top_ui.set_clip_rect(top_rect);
                            self.state.editor_renderer.show(
                                &mut top_ui,
                                &mut self.state.editor,
                                self.state.config.font_size,
                                self.state.config.line_numbers
                            );

                            // Separator / Drag handle
                            let sep_response = ui.allocate_rect(sep_rect, egui::Sense::drag());

                            // Paint separator line
                            let color = if sep_response.dragged() {
                                ui.visuals().widgets.active.bg_fill
                            } else if sep_response.hovered() {
                                ui.visuals().widgets.hovered.bg_fill
                            } else {
                                ui.visuals().widgets.noninteractive.bg_stroke.color
                            };
                            ui.painter().rect_filled(sep_rect, 0.0, color);

                            // Update split_ratio based on drag
                            if sep_response.dragged() {
                                if let Some(mouse_pos) = ui.input(|i| i.pointer.latest_pos()) {
                                    let relative_y = mouse_pos.y - min_rect.top();
                                    self.state.split_ratio = (relative_y / available_height).clamp(0.1, 0.9);
                                }
                            }

                            // Change cursor to vertical resize icon
                            if sep_response.hovered() || sep_response.dragged() {
                                ui.output_mut(|o| o.cursor_icon = egui::CursorIcon::ResizeVertical);
                            }

                            // Bottom Pane: Preview
                            let mut bottom_ui = ui.child_ui(bottom_rect, egui::Layout::top_down(egui::Align::Min));
                            bottom_ui.set_clip_rect(bottom_rect);
                            self.state.preview.show(&mut bottom_ui, &mut content, self.state.config.font_size);
                        } else {
                            // Horizontal Split (Side-by-Side)
                            let separator_width = 8.0;
                            let total_width = available_width - separator_width;
                            let left_width = (total_width * self.state.split_ratio).max(100.0).min(total_width - 100.0);
                            let right_width = total_width - left_width;

                            // Left Rect
                            let left_rect = egui::Rect::from_min_size(
                                min_rect.min,
                                egui::vec2(left_width, available_height)
                            );

                            // Separator Rect
                            let sep_rect = egui::Rect::from_min_size(
                                egui::pos2(min_rect.min.x + left_width, min_rect.min.y),
                                egui::vec2(separator_width, available_height)
                            );

                            // Right Rect
                            let right_rect = egui::Rect::from_min_size(
                                egui::pos2(min_rect.min.x + left_width + separator_width, min_rect.min.y),
                                egui::vec2(right_width, available_height)
                            );

                            // Left Pane: Editor
                            let mut left_ui = ui.child_ui(left_rect, egui::Layout::top_down(egui::Align::Min));
                            left_ui.set_clip_rect(left_rect);
                            self.state.editor_renderer.show(
                                &mut left_ui,
                                &mut self.state.editor,
                                self.state.config.font_size,
                                self.state.config.line_numbers
                            );

                            // Separator / Drag handle
                            let sep_response = ui.allocate_rect(sep_rect, egui::Sense::drag());

                            // Paint separator line
                            let color = if sep_response.dragged() {
                                ui.visuals().widgets.active.bg_fill
                            } else if sep_response.hovered() {
                                ui.visuals().widgets.hovered.bg_fill
                            } else {
                                ui.visuals().widgets.noninteractive.bg_stroke.color
                            };
                            ui.painter().rect_filled(sep_rect, 0.0, color);

                            // Update split_ratio based on drag
                            if sep_response.dragged() {
                                if let Some(mouse_pos) = ui.input(|i| i.pointer.latest_pos()) {
                                    let relative_x = mouse_pos.x - min_rect.left();
                                    self.state.split_ratio = (relative_x / available_width).clamp(0.1, 0.9);
                                }
                            }

                            // Change cursor to horizontal resize icon
                            if sep_response.hovered() || sep_response.dragged() {
                                ui.output_mut(|o| o.cursor_icon = egui::CursorIcon::ResizeHorizontal);
                            }

                            // Right Pane: Preview
                            let mut right_ui = ui.child_ui(right_rect, egui::Layout::top_down(egui::Align::Min));
                            right_ui.set_clip_rect(right_rect);
                            self.state.preview.show(&mut right_ui, &mut content, self.state.config.font_size);
                        }

                        if content != old_content {
                            self.state.editor.set_text(&content);
                        }
                    }
                }
            } else {
                ui.centered_and_justified(|ui| {
                    ui.label("Welcome to dr.md (Doctor Markdown)\n\nPress Ctrl+N to create a new note\nor Ctrl+O to open a workspace folder.");
                });
            }
        });
    }
}
