pub mod commands;
pub mod editor_window;
pub mod explorer;
pub mod menu_bar;
pub mod state;

use crate::keymap::{ShortcutAction, handle_key_events};
use state::{AppState, ViewMode};
use std::path::PathBuf;

pub struct DoctorMarkdownApp {
    state: AppState,
}

impl DoctorMarkdownApp {
    pub fn new(cc: &eframe::CreationContext<'_>, root_path: Option<PathBuf>) -> Self {
        let state = AppState::new(root_path);
        crate::config::apply_theme(&cc.egui_ctx, &state.config);

        Self { state }
    }
}
impl eframe::App for DoctorMarkdownApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Some(open_url) = ctx.output_mut(|o| o.open_url.take()) {
            let url_str = open_url.url.as_str();
            if !url_str.starts_with("http://")
                && !url_str.starts_with("https://")
                && !url_str.starts_with("mailto:")
            {
                let decoded_path =
                    urlencoding::decode(url_str).unwrap_or(std::borrow::Cow::Borrowed(url_str));
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
            self.state.sync_cursor_from_egui(ctx);
            match action {
                ShortcutAction::NewNote => {
                    if let Some(ref root) = self.state.vault.root_path {
                        self.state.explorer_visible = true;
                        let target_dir = self
                            .state
                            .explorer
                            .selected_folder
                            .clone()
                            .unwrap_or_else(|| root.clone());
                        self.state.explorer.start_creation(
                            crate::explorer::tree::CreatingType::File {
                                parent_dir: target_dir,
                            },
                        );
                    }
                }
                ShortcutAction::NewFolder => {
                    if let Some(ref root) = self.state.vault.root_path {
                        self.state.explorer_visible = true;
                        let target_dir = self
                            .state
                            .explorer
                            .selected_folder
                            .clone()
                            .unwrap_or_else(|| root.clone());
                        self.state.explorer.start_creation(
                            crate::explorer::tree::CreatingType::Folder {
                                parent_dir: target_dir,
                            },
                        );
                    }
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
                ShortcutAction::SelectAll => {
                    if let Some(mut text_state) = egui::widgets::text_edit::TextEditState::load(
                        ctx,
                        egui::Id::new("editor_text_edit"),
                    ) {
                        let len = self.state.editor.buffer.len_chars();
                        let anchor = egui::text::CCursor::new(0);
                        let head = egui::text::CCursor::new(len);
                        text_state.cursor.set_char_range(Some(
                            egui::text::CCursorRange::two(anchor, head),
                        ));
                        text_state.store(ctx, egui::Id::new("editor_text_edit"));
                    }
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
                    self.state.editor.format_selection("bold");
                }
                ShortcutAction::Italic => {
                    self.state.editor.format_selection("italic");
                }
                ShortcutAction::Link => {
                    self.state.editor.format_selection("link");
                }
                ShortcutAction::CodeBlock => {
                    self.state.editor.format_selection("code");
                }
                ShortcutAction::Checkbox => {
                    self.state.editor.format_selection("checkbox");
                }
                ShortcutAction::Comment => {
                    self.state.editor.format_selection("comment");
                }
                _ => {}
            }
        }

        menu_bar::render_menu_bar(ctx, &mut self.state);
        explorer::render_explorer(ctx, &mut self.state);
        editor_window::render_editor_window(ctx, &mut self.state);

        self.state.check_autosave(ctx);
    }
}
