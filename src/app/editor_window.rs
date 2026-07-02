use crate::app::state::{AppState, ViewMode};

pub fn render_editor_window(ctx: &egui::Context, state: &mut AppState) {
    let panel_frame =
        egui::Frame::central_panel(&ctx.style()).inner_margin(egui::Margin::symmetric(0.0, 8.0));
    egui::CentralPanel::default().frame(panel_frame).show(ctx, |ui| {
        if state.vault.active_file.is_some() {
            match state.view_mode {
                ViewMode::Editor => {
                    state.editor_renderer.show(
                        ui,
                        &mut state.editor,
                        state.config.font_size,
                        state.config.line_numbers
                    );
                }
                ViewMode::Preview => {
                    state.preview.show(ui, &mut state.editor, state.config.font_size);
                }
                ViewMode::Split => {
                    let min_rect = ui.max_rect();
                    // Allocate the entire space so egui knows we are occupying it
                    ui.allocate_rect(min_rect, egui::Sense::hover());

                    let available_width = min_rect.width();
                    let available_height = min_rect.height();

                    if available_width < 600.0 {
                        // Vertical Split (Top-to-Bottom)
                        let separator_height = 2.0;
                        let drag_handle_height = 8.0;
                        let total_height = available_height - separator_height;
                        let top_height = (total_height * state.split_ratio).max(100.0).min(total_height - 100.0);
                        let bottom_height = total_height - top_height;

                        // Top Rect
                        let top_rect = egui::Rect::from_min_size(
                            min_rect.min,
                            egui::vec2(available_width, top_height)
                        );

                        // Separator Rect (visible thin line)
                        let sep_rect = egui::Rect::from_min_size(
                            egui::pos2(min_rect.min.x, min_rect.min.y + top_height),
                            egui::vec2(available_width, separator_height)
                        );

                        // Drag Handle Rect (wider click target)
                        let drag_rect = egui::Rect::from_min_size(
                            egui::pos2(min_rect.min.x, min_rect.min.y + top_height - (drag_handle_height - separator_height) / 2.0),
                            egui::vec2(available_width, drag_handle_height)
                        );

                        // Bottom Rect
                        let bottom_rect = egui::Rect::from_min_size(
                            egui::pos2(min_rect.min.x, min_rect.min.y + top_height + separator_height),
                            egui::vec2(available_width, bottom_height)
                        );

                        // Top Pane: Editor
                        let mut top_ui = ui.child_ui(top_rect, egui::Layout::top_down(egui::Align::Min));
                        top_ui.set_clip_rect(top_rect);
                        state.editor_renderer.show(
                            &mut top_ui,
                            &mut state.editor,
                            state.config.font_size,
                            state.config.line_numbers
                        );

                        // Separator / Drag handle
                        let sep_response = ui.allocate_rect(drag_rect, egui::Sense::drag());

                        // Paint separator line
                        let color = if sep_response.dragged() {
                            ui.visuals().selection.bg_fill
                        } else if sep_response.hovered() {
                            ui.visuals().selection.bg_fill.linear_multiply(0.8)
                        } else {
                            ui.visuals().widgets.noninteractive.bg_stroke.color
                        };
                        ui.painter().rect_filled(sep_rect, 0.0, color);

                        // Update split_ratio based on drag
                        if sep_response.dragged() {
                            if let Some(mouse_pos) = ui.input(|i| i.pointer.latest_pos()) {
                                let relative_y = mouse_pos.y - min_rect.top();
                                state.split_ratio = (relative_y / available_height).clamp(0.1, 0.9);
                            }
                        }

                        // Change cursor to vertical resize icon
                        if sep_response.hovered() || sep_response.dragged() {
                            ui.output_mut(|o| o.cursor_icon = egui::CursorIcon::ResizeVertical);
                        }

                        // Bottom Pane: Preview
                        let mut bottom_ui = ui.child_ui(bottom_rect, egui::Layout::top_down(egui::Align::Min));
                        bottom_ui.set_clip_rect(bottom_rect);
                        state.preview.show(&mut bottom_ui, &mut state.editor, state.config.font_size);
                    } else {
                        // Horizontal Split (Side-by-Side)
                        let separator_width = 2.0;
                        let drag_handle_width = 8.0;
                        let total_width = available_width - separator_width;
                        let left_width = (total_width * state.split_ratio).max(100.0).min(total_width - 100.0);
                        let right_width = total_width - left_width;

                        // Left Rect
                        let left_rect = egui::Rect::from_min_size(
                            min_rect.min,
                            egui::vec2(left_width, available_height)
                        );

                        // Separator Rect (visible thin line)
                        let sep_rect = egui::Rect::from_min_size(
                            egui::pos2(min_rect.min.x + left_width, min_rect.min.y),
                            egui::vec2(separator_width, available_height)
                        );

                        // Drag Handle Rect (wider click target)
                        let drag_rect = egui::Rect::from_min_size(
                            egui::pos2(min_rect.min.x + left_width - (drag_handle_width - separator_width) / 2.0, min_rect.min.y),
                            egui::vec2(drag_handle_width, available_height)
                        );

                        // Right Rect
                        let right_rect = egui::Rect::from_min_size(
                            egui::pos2(min_rect.min.x + left_width + separator_width, min_rect.min.y),
                            egui::vec2(right_width, available_height)
                        );

                        // Left Pane: Editor
                        let mut left_ui = ui.child_ui(left_rect, egui::Layout::top_down(egui::Align::Min));
                        left_ui.set_clip_rect(left_rect);
                        state.editor_renderer.show(
                            &mut left_ui,
                            &mut state.editor,
                            state.config.font_size,
                            state.config.line_numbers
                        );

                        // Separator / Drag handle
                        let sep_response = ui.allocate_rect(drag_rect, egui::Sense::drag());

                        // Paint separator line
                        let color = if sep_response.dragged() {
                            ui.visuals().selection.bg_fill
                        } else if sep_response.hovered() {
                            ui.visuals().selection.bg_fill.linear_multiply(0.8)
                        } else {
                            ui.visuals().widgets.noninteractive.bg_stroke.color
                        };
                        ui.painter().rect_filled(sep_rect, 0.0, color);

                        // Update split_ratio based on drag
                        if sep_response.dragged() {
                            if let Some(mouse_pos) = ui.input(|i| i.pointer.latest_pos()) {
                                let relative_x = mouse_pos.x - min_rect.left();
                                state.split_ratio = (relative_x / available_width).clamp(0.1, 0.9);
                            }
                        }

                        // Change cursor to horizontal resize icon
                        if sep_response.hovered() || sep_response.dragged() {
                            ui.output_mut(|o| o.cursor_icon = egui::CursorIcon::ResizeHorizontal);
                        }

                        // Right Pane: Preview
                        let mut right_ui = ui.child_ui(right_rect, egui::Layout::top_down(egui::Align::Min));
                        right_ui.set_clip_rect(right_rect);
                        state.preview.show(&mut right_ui, &mut state.editor, state.config.font_size);
                    }
                }
            }
        } else {
            ui.centered_and_justified(|ui| {
                ui.label("Welcome to Doctor. Markdown\n\nPress Ctrl+N to create a new note\nPress Ctrl+Shift+N to create a new folder\nor Ctrl+O to open a workspace folder.");
            });
        }
    });
}
