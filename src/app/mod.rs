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
                        self.state.vault.set_root(path);
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
                                self.state.vault.set_root(path);
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
                        ui.add(egui::Slider::new(&mut self.state.config.font_size, 10.0..=30.0));
                        ui.separator();
                        ui.checkbox(&mut self.state.config.line_numbers, "Show Line Numbers");
                        ui.checkbox(&mut self.state.config.autosave, "Autosave");
                    });
                });
            });
        }

        if self.state.explorer_visible && !self.state.focus_mode {
            egui::SidePanel::left("file_explorer")
                .resizable(true)
                .default_width(200.0)
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
                                                self.state.vault.set_root(path);
                                            }
                                        }
                                    });
                                }
                            });
                        });
                    });
                });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
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
                        ui.columns(2, |columns| {
                            self.state.editor_renderer.show(
                                &mut columns[0],
                                &mut self.state.editor,
                                self.state.config.font_size,
                                self.state.config.line_numbers
                            );
                            self.state.preview.show(&mut columns[1], &mut content, self.state.config.font_size);
                        });
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
