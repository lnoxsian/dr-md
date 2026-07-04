use crate::app::commands;
use crate::app::state::{AppState, ViewMode};

pub fn render_menu_bar(ctx: &egui::Context, state: &mut AppState) {
    if !state.focus_mode {
        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New Note (Ctrl+N)").clicked() {
                        if let Some(ref root) = state.vault.root_path {
                            state.explorer_visible = true;
                            let target_dir = state
                                .explorer
                                .selected_folder
                                .clone()
                                .unwrap_or_else(|| root.clone());
                            state.explorer.start_creation(
                                crate::explorer::tree::CreatingType::File {
                                    parent_dir: target_dir,
                                },
                            );
                        }
                        ui.close_menu();
                    }
                    if ui.button("New Folder (Ctrl+Shift+N)").clicked() {
                        if let Some(ref root) = state.vault.root_path {
                            state.explorer_visible = true;
                            let target_dir = state
                                .explorer
                                .selected_folder
                                .clone()
                                .unwrap_or_else(|| root.clone());
                            state.explorer.start_creation(
                                crate::explorer::tree::CreatingType::Folder {
                                    parent_dir: target_dir,
                                },
                            );
                        }
                        ui.close_menu();
                    }
                    if ui.button("Open Folder (Ctrl+O)").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_folder() {
                            commands::execute_open_folder(state, path);
                        }
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("Save (Ctrl+S)").clicked() {
                        commands::execute_save(state);
                        ui.close_menu();
                    }
                    if ui.button("Save As (Ctrl+Shift+S)").clicked() {
                        if let Some(path) = rfd::FileDialog::new().save_file() {
                            commands::execute_save_as(state, path);
                        }
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("Close Note (Ctrl+W)").clicked() {
                        state.vault.active_file = None;
                        state.editor = crate::editor::Editor::new();
                        ui.close_menu();
                    }
                });

                ui.menu_button("Edit", |ui| {
                    if ui.button("Cut (Ctrl+X)").clicked() {
                        if let Some(mut text_state) = egui::widgets::text_edit::TextEditState::load(
                            ctx,
                            egui::Id::new("editor_text_edit"),
                        ) {
                            if let Some(range) = text_state.cursor.char_range() {
                                let start = range.primary.index.min(range.secondary.index);
                                let end = range.primary.index.max(range.secondary.index);
                                let sorted = start..end;
                                if !sorted.is_empty() {
                                    let text_to_copy =
                                        state.editor.buffer.rope.slice(sorted.clone()).to_string();
                                    ctx.copy_text(text_to_copy);

                                    state.editor.buffer.remove(sorted.start, sorted.end);
                                    state.editor.cursor.char_idx = sorted.start;
                                    state.editor.selection.clear(sorted.start);
                                    state.editor.is_dirty = true;

                                    state.editor_renderer.content_buffer =
                                        state.editor.buffer.to_string();

                                    let cursor = egui::text::CCursor::new(sorted.start);
                                    text_state.cursor.set_char_range(Some(
                                        egui::text::CCursorRange::two(cursor, cursor),
                                    ));
                                    text_state.store(ctx, egui::Id::new("editor_text_edit"));
                                }
                            }
                        }
                        ui.close_menu();
                    }
                    if ui.button("Copy (Ctrl+C)").clicked() {
                        if let Some(text_state) = egui::widgets::text_edit::TextEditState::load(
                            ctx,
                            egui::Id::new("editor_text_edit"),
                        ) {
                            if let Some(range) = text_state.cursor.char_range() {
                                let start = range.primary.index.min(range.secondary.index);
                                let end = range.primary.index.max(range.secondary.index);
                                let sorted = start..end;
                                if !sorted.is_empty() {
                                    let text_to_copy =
                                        state.editor.buffer.rope.slice(sorted).to_string();
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
                                if let Some(mut text_state) =
                                    egui::widgets::text_edit::TextEditState::load(
                                        ctx,
                                        egui::Id::new("editor_text_edit"),
                                    )
                                {
                                    let range =
                                        text_state.cursor.char_range().unwrap_or_else(|| {
                                            let len = state.editor.buffer.len_chars();
                                            let cursor = egui::text::CCursor::new(len);
                                            egui::text::CCursorRange::two(cursor, cursor)
                                        });
                                    let start = range.primary.index.min(range.secondary.index);
                                    let end = range.primary.index.max(range.secondary.index);
                                    let sorted = start..end;

                                    if !sorted.is_empty() {
                                        state.editor.buffer.remove(sorted.start, sorted.end);
                                    }

                                    state.editor.buffer.insert(sorted.start, &paste_text);
                                    state.editor.cursor.char_idx =
                                        sorted.start + paste_text.chars().count();
                                    state.editor.selection.clear(state.editor.cursor.char_idx);
                                    state.editor.is_dirty = true;

                                    state.editor_renderer.content_buffer =
                                        state.editor.buffer.to_string();

                                    let cursor =
                                        egui::text::CCursor::new(state.editor.cursor.char_idx);
                                    text_state.cursor.set_char_range(Some(
                                        egui::text::CCursorRange::two(cursor, cursor),
                                    ));
                                    text_state.store(ctx, egui::Id::new("editor_text_edit"));
                                }
                            }
                        }
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("Undo (Ctrl+Z)").clicked() {
                        state.sync_cursor_from_egui(ctx);
                        state.editor.undo();
                        ui.close_menu();
                    }
                    if ui.button("Redo (Ctrl+Shift+Z)").clicked() {
                        state.sync_cursor_from_egui(ctx);
                        state.editor.redo();
                        ui.close_menu();
                    }
                    if ui.button("Select All (Ctrl+A)").clicked() {
                        state.sync_cursor_from_egui(ctx);
                        if let Some(mut text_state) = egui::widgets::text_edit::TextEditState::load(
                            ctx,
                            egui::Id::new("editor_text_edit"),
                        ) {
                            let len = state.editor.buffer.len_chars();
                            let anchor = egui::text::CCursor::new(0);
                            let head = egui::text::CCursor::new(len);
                            text_state.cursor.set_char_range(Some(
                                egui::text::CCursorRange::two(anchor, head),
                            ));
                            text_state.store(ctx, egui::Id::new("editor_text_edit"));
                        }
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("Bold (Ctrl+B)").clicked() {
                        state.sync_cursor_from_egui(ctx);
                        state.editor.insert_text("****");
                        ui.close_menu();
                    }
                    if ui.button("Italic (Ctrl+I)").clicked() {
                        state.sync_cursor_from_egui(ctx);
                        state.editor.insert_text("**");
                        ui.close_menu();
                    }
                    if ui.button("Link (Ctrl+K)").clicked() {
                        state.sync_cursor_from_egui(ctx);
                        state.editor.insert_text("[](url)");
                        ui.close_menu();
                    }
                    if ui.button("Code Block (Ctrl+Shift+C)").clicked() {
                        state.sync_cursor_from_egui(ctx);
                        state.editor.insert_text("```\n\n```");
                        ui.close_menu();
                    }
                    if ui.button("Checkbox (Ctrl+L)").clicked() {
                        state.sync_cursor_from_egui(ctx);
                        state.editor.insert_text("- [ ] ");
                        ui.close_menu();
                    }
                });

                ui.menu_button("View", |ui| {
                    if ui
                        .selectable_label(
                            state.view_mode == ViewMode::Editor,
                            "Editor Mode (Ctrl+1)",
                        )
                        .clicked()
                    {
                        state.view_mode = ViewMode::Editor;
                        ui.close_menu();
                    }
                    if ui
                        .selectable_label(
                            state.view_mode == ViewMode::Preview,
                            "Preview Mode (Ctrl+2)",
                        )
                        .clicked()
                    {
                        state.view_mode = ViewMode::Preview;
                        ui.close_menu();
                    }
                    if ui
                        .selectable_label(state.view_mode == ViewMode::Split, "Split Mode (Ctrl+3)")
                        .clicked()
                    {
                        state.view_mode = ViewMode::Split;
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui
                        .selectable_label(state.explorer_visible, "Show File Explorer (Ctrl+E)")
                        .clicked()
                    {
                        state.explorer_visible = !state.explorer_visible;
                        ui.close_menu();
                    }
                    if ui
                        .selectable_label(state.focus_mode, "Focus Mode (F11)")
                        .clicked()
                    {
                        state.focus_mode = !state.focus_mode;
                        state.explorer_visible = !state.focus_mode;
                        ui.close_menu();
                    }
                });

                ui.menu_button("Settings", |ui| {
                    ui.set_min_width(220.0);
                    ui.label("Base Theme:");
                    ui.horizontal(|ui| {
                        let themes = [
                            ("dark", egui::Color32::from_rgb(30, 30, 30), "Dark Theme"),
                            (
                                "light",
                                egui::Color32::from_rgb(220, 220, 220),
                                "Light Theme",
                            ),
                            (
                                "solarized_dark",
                                egui::Color32::from_rgb(0, 43, 54),
                                "Solarized Dark",
                            ),
                            (
                                "solarized_light",
                                egui::Color32::from_rgb(253, 246, 227),
                                "Solarized Light",
                            ),
                            (
                                "gruvbox_dark",
                                egui::Color32::from_rgb(40, 40, 40),
                                "Gruvbox Medium Dark",
                            ),
                            (
                                "gruvbox_light",
                                egui::Color32::from_rgb(251, 241, 199),
                                "Gruvbox Light",
                            ),
                        ];

                        for &(theme_id, color, name) in &themes {
                            let is_selected = state.config.theme == theme_id;

                            let (rect, response) = ui
                                .allocate_exact_size(egui::vec2(16.0, 16.0), egui::Sense::click());

                            let response = response.on_hover_text(name);

                            let painter = ui.painter();
                            let rounding = egui::Rounding::same(8.0); // Circular

                            let fill = if response.hovered() {
                                color.linear_multiply(0.8)
                            } else {
                                color
                            };

                            painter.rect_filled(rect, rounding, fill);

                            if is_selected {
                                painter.rect_stroke(
                                    rect.expand(2.0),
                                    egui::Rounding::same(10.0),
                                    egui::Stroke::new(2.0, ui.visuals().text_color()),
                                );
                            } else if response.hovered() {
                                painter.rect_stroke(
                                    rect.expand(1.0),
                                    egui::Rounding::same(9.0),
                                    egui::Stroke::new(
                                        1.0,
                                        ui.visuals().text_color().linear_multiply(0.5),
                                    ),
                                );
                            }

                            if response.clicked() {
                                state.config.theme = theme_id.to_string();
                                let _ = state.config.save();
                                crate::config::apply_theme(ui.ctx(), &state.config);
                            }
                        }
                    });
                    ui.separator();

                    ui.label("Theme Accent:");
                    ui.horizontal(|ui| {
                        use crate::config::ThemeAccent;
                        for accent in ThemeAccent::all() {
                            let color = accent.color();
                            let is_selected = state.config.theme_accent == *accent;

                            let (rect, response) = ui
                                .allocate_exact_size(egui::vec2(16.0, 16.0), egui::Sense::click());

                            let response = response.on_hover_text(accent.name());

                            let painter = ui.painter();
                            let rounding = egui::Rounding::same(8.0); // Circular

                            let fill = if response.hovered() {
                                color.linear_multiply(0.8)
                            } else {
                                color
                            };

                            painter.rect_filled(rect, rounding, fill);

                            if is_selected {
                                painter.rect_stroke(
                                    rect.expand(2.0),
                                    egui::Rounding::same(10.0),
                                    egui::Stroke::new(2.0, ui.visuals().text_color()),
                                );
                            } else if response.hovered() {
                                painter.rect_stroke(
                                    rect.expand(1.0),
                                    egui::Rounding::same(9.0),
                                    egui::Stroke::new(
                                        1.0,
                                        ui.visuals().text_color().linear_multiply(0.5),
                                    ),
                                );
                            }

                            if response.clicked() {
                                state.config.theme_accent = *accent;
                                let _ = state.config.save();
                                crate::config::apply_theme(ui.ctx(), &state.config);
                            }
                        }
                    });
                    ui.separator();

                    ui.label("Font Size:");
                    if ui
                        .add(egui::Slider::new(&mut state.config.font_size, 10.0..=30.0))
                        .changed()
                    {
                        let _ = state.config.save();
                        crate::config::apply_theme(ui.ctx(), &state.config);
                    }
                    ui.separator();
                    if ui
                        .checkbox(&mut state.config.line_numbers, "Show Line Numbers")
                        .changed()
                    {
                        let _ = state.config.save();
                    }
                    if ui
                        .checkbox(&mut state.config.autosave, "Autosave")
                        .changed()
                    {
                        let _ = state.config.save();
                    }
                });
            });
        });
    }
}
