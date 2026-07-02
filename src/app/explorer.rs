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
                ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                    ui.add_space(4.0);
                    if let Some(ref path) = state.editor.active_path {
                        let name = path.file_name().unwrap_or_default().to_string_lossy();
                        let dirty = if state.editor.is_dirty { "*" } else { "" };
                        ui.label(format!("{}{}", name, dirty));
                    } else {
                        ui.label("No file open");
                    }
                    ui.add_space(4.0);
                    ui.separator();

                    ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            if let Some(ref root) = state.vault.root_path {
                                let mut active_file = state.vault.active_file.clone();
                                let clicked = state.explorer.show(ui, root, &mut active_file);

                                if active_file != state.vault.active_file {
                                    state.vault.active_file = active_file.clone();
                                    if active_file.is_none() {
                                        state.editor = crate::editor::Editor::new();
                                    } else if let Some(ref path) = active_file {
                                        state.editor.active_path = Some(path.clone());
                                    }
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
                });
            });
    }
}
