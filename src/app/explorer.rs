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
                        if let Some(ref root) = state.vault.root_path {
                            ui.horizontal(|ui| {
                                ui.style_mut().spacing.item_spacing.x = 4.0;
                                let target_dir = state
                                    .explorer
                                    .selected_folder
                                    .clone()
                                    .unwrap_or_else(|| root.clone());

                                if ui.button("📝 New File").clicked() {
                                    state.explorer.start_creation(
                                        crate::explorer::tree::CreatingType::File {
                                            parent_dir: target_dir.clone(),
                                        },
                                    );
                                }
                                if ui.button("📁 New Folder").clicked() {
                                    state.explorer.start_creation(
                                        crate::explorer::tree::CreatingType::Folder {
                                            parent_dir: target_dir.clone(),
                                        },
                                    );
                                }
                                let can_paste = state.explorer.clipboard.is_some();
                                if ui
                                    .add_enabled(can_paste, egui::Button::new("📋 Paste"))
                                    .clicked()
                                {
                                    let mut active_file = state.vault.active_file.clone();
                                    state.explorer.paste_item(&target_dir, &mut active_file);
                                    if active_file != state.vault.active_file {
                                        state.vault.active_file = active_file.clone();
                                        if active_file.is_none() {
                                            state.editor = crate::editor::Editor::new();
                                        } else if let Some(ref path) = active_file {
                                            state.editor.active_path = Some(path.clone());
                                        }
                                    }
                                }
                            });
                            ui.horizontal(|ui| {
                                if let Some(ref selected) = state.explorer.selected_folder {
                                    let name =
                                        selected.file_name().unwrap_or_default().to_string_lossy();
                                    ui.label(format!("Target: 📁 {}", name));
                                    if ui
                                        .button("❌")
                                        .on_hover_text("Deselect folder (target root)")
                                        .clicked()
                                    {
                                        state.explorer.selected_folder = None;
                                    }
                                } else {
                                    ui.label("Target: 📁 Root");
                                }
                            });
                            ui.separator();
                        }

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
