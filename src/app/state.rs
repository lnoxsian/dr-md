use crate::config::AppConfig;
use crate::editor::{Editor, renderer::EditorRenderer};
use crate::explorer::FileTree;
use crate::markdown::MarkdownPreview;
use crate::workspace::Vault;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewMode {
    Editor,
    Preview,
    Split,
}

pub struct AppState {
    pub config: AppConfig,
    pub vault: Vault,
    pub explorer: FileTree,
    pub editor: Editor,
    pub editor_renderer: EditorRenderer,
    pub preview: MarkdownPreview,
    pub view_mode: ViewMode,
    pub explorer_visible: bool,
    pub focus_mode: bool,
    pub split_ratio: f32,
    pub logo_light_mode: Option<egui::TextureHandle>,
    pub logo_dark_mode: Option<egui::TextureHandle>,
    pub last_edit_time: Option<std::time::Instant>,
    pub last_editor_version: usize,
}

impl AppState {
    pub fn new(root_path: Option<PathBuf>) -> Self {
        let config = AppConfig::load();
        let resolved_path =
            root_path.or_else(|| config.last_opened_folder.as_ref().map(PathBuf::from));
        Self {
            config,
            vault: Vault::new(resolved_path),
            explorer: FileTree::new(),
            editor: Editor::new(),
            editor_renderer: EditorRenderer::new(),
            preview: MarkdownPreview::new(),
            view_mode: ViewMode::Split,
            explorer_visible: true,
            focus_mode: false,
            split_ratio: 0.5,
            logo_light_mode: None,
            logo_dark_mode: None,
            last_edit_time: None,
            last_editor_version: 0,
        }
    }

    pub fn sync_cursor_from_egui(&mut self, ctx: &egui::Context) {
        if let Some(text_state) =
            egui::widgets::text_edit::TextEditState::load(ctx, egui::Id::new("editor_text_edit"))
        {
            if let Some(range) = text_state.cursor.char_range() {
                self.editor.cursor.char_idx = range.primary.index;
                self.editor.selection.anchor = range.secondary.index;
                self.editor.selection.head = range.primary.index;
            }
        }
    }

    pub fn check_autosave(&mut self, ctx: &egui::Context) {
        if self.config.autosave && self.editor.is_dirty && self.editor.active_path.is_some() {
            let current_version = self.editor.version;
            if self.last_editor_version != current_version {
                self.last_editor_version = current_version;
                self.last_edit_time = Some(std::time::Instant::now());
            }

            let last_edit = self
                .last_edit_time
                .get_or_insert_with(std::time::Instant::now);
            if last_edit.elapsed() >= std::time::Duration::from_secs(1) {
                if let Err(e) = self.editor.save_file() {
                    tracing::error!("Failed to save file: {:?}", e);
                }
                self.last_edit_time = None;
            } else {
                ctx.request_repaint_after(std::time::Duration::from_millis(500));
            }
        } else {
            self.last_edit_time = None;
            self.last_editor_version = self.editor.version;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_state_autosave() {
        let unique_id = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let file_path = std::env::temp_dir().join(format!("dr_md_test_note_{}.md", unique_id));

        let mut state = AppState::new(None);
        state.config.autosave = true;

        // Setup editor with an active path
        state.editor.active_path = Some(file_path.clone());
        state.editor.set_text("Initial text");

        // Verify set_text makes it dirty and updates version
        assert!(state.editor.is_dirty);
        let version1 = state.editor.version;
        state.last_editor_version = version1;

        // Mock egui context
        let ctx = egui::Context::default();

        // 1. If edit is recent (less than 1s ago), check_autosave should request repaint and NOT save
        state.last_edit_time = Some(std::time::Instant::now());
        state.check_autosave(&ctx);
        assert!(state.editor.is_dirty); // Still dirty
        assert!(!file_path.exists()); // File not written yet

        // 2. If last edit time was more than 1s ago, check_autosave should save the file and clear is_dirty
        state.last_edit_time = Some(std::time::Instant::now() - std::time::Duration::from_secs(2));
        state.check_autosave(&ctx);

        assert!(!state.editor.is_dirty); // No longer dirty
        assert!(file_path.exists()); // File written!
        let content = std::fs::read_to_string(&file_path).unwrap();
        assert_eq!(content, "Initial text");

        // Clean up
        let _ = std::fs::remove_file(&file_path);
    }
}
