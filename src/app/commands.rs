use crate::app::state::AppState;
use std::path::PathBuf;

pub fn execute_new_note(state: &mut AppState) {
    if let Ok(path) = state.vault.new_note() {
        let _ = state.editor.load_file(path);
    }
}

pub fn execute_open_file(state: &mut AppState, path: PathBuf) {
    state.vault.active_file = Some(path.clone());
    if let Err(e) = state.editor.load_file(path) {
        tracing::error!("Failed to load file: {:?}", e);
    }
}

pub fn execute_save(state: &mut AppState) {
    if state.editor.active_path.is_some() {
        if let Err(e) = state.editor.save_file() {
            tracing::error!("Failed to save file: {:?}", e);
        }
    }
}

pub fn execute_save_as(state: &mut AppState, path: PathBuf) {
    if let Err(e) = state.editor.save_as(path) {
        tracing::error!("Failed to save file as: {:?}", e);
    }
}

pub fn execute_open_folder(state: &mut AppState, path: PathBuf) {
    state.vault.set_root(path.clone());
    state.config.last_opened_folder = Some(path.to_string_lossy().to_string());
    let _ = state.config.save();
}
