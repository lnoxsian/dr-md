use std::path::PathBuf;
use crate::config::AppConfig;
use crate::workspace::Vault;
use crate::explorer::FileTree;
use crate::editor::{Editor, renderer::EditorRenderer};
use crate::markdown::MarkdownPreview;

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
}

impl AppState {
    pub fn new(root_path: Option<PathBuf>) -> Self {
        let config = AppConfig::load();
        Self {
            config,
            vault: Vault::new(root_path),
            explorer: FileTree::new(),
            editor: Editor::new(),
            editor_renderer: EditorRenderer::new(),
            preview: MarkdownPreview::new(),
            view_mode: ViewMode::Split,
            explorer_visible: true,
            focus_mode: false,
        }
    }
}
