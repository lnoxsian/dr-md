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
        }
    }
}
