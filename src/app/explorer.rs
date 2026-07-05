use crate::app::commands;
use crate::app::state::AppState;

pub fn render_explorer(ctx: &egui::Context, state: &mut AppState) {
    if state.explorer_visible && !state.focus_mode {
        let screen_width = ctx.screen_rect().width();
        let max_explorer_width = if screen_width < 600.0 {
            (screen_width * 0.35).max(100.0)
        } else {
            300.0
        };
        let default_explorer_width = if screen_width < 600.0 { 120.0 } else { 200.0 };

        egui::SidePanel::left("file_explorer")
            .resizable(true)
            .default_width(default_explorer_width)
            .max_width(max_explorer_width)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    if let Some(ref root) = state.vault.root_path {
                        let mut active_file = state.vault.active_file.clone();
                        let clicked = state.explorer.show(ui, root, &mut active_file);

                        if active_file != state.vault.active_file {
                            if active_file.is_none() {
                                if let Some(ref prev_path) = state.vault.active_file {
                                    if let Some(idx) =
                                        state.tabs.iter().position(|t| t.path == *prev_path)
                                    {
                                        state.close_tab(idx);
                                    }
                                }
                            } else if let (Some(prev_path), Some(new_path)) =
                                (state.vault.active_file.as_ref(), active_file.as_ref())
                            {
                                if let Some(idx) =
                                    state.tabs.iter().position(|t| t.path == *prev_path)
                                {
                                    state.tabs[idx].path = new_path.clone();
                                    state.tabs[idx].editor.active_path = Some(new_path.clone());
                                }
                            }
                            state.vault.active_file = active_file.clone();
                        }

                        if let Some(clicked) = clicked {
                            commands::execute_open_file(state, clicked);
                        }
                    } else {
                        ui.vertical_centered(|ui| {
                            ui.label("No folder opened");
                            if ui.button("Open Folder").clicked() {
                                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                                    commands::execute_open_folder(state, path);
                                }
                            }
                        });
                    }
                });
            });
    }
}
