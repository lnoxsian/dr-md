use crate::app::state::{AppState, ViewMode};

pub fn render_editor_window(ctx: &egui::Context, state: &mut AppState) {
    let panel_frame =
        egui::Frame::central_panel(&ctx.style()).inner_margin(egui::Margin::symmetric(0.0, 0.0));
    egui::CentralPanel::default()
        .frame(panel_frame)
        .show(ctx, |ui| {
            if !state.tabs.is_empty() {
                // 1. Render Tab Bar
                render_tab_bar(ui, state);

                // 2. Render separator line
                let sep_rect = egui::Rect::from_min_size(
                    egui::pos2(ui.max_rect().min.x, ui.cursor().top()),
                    egui::vec2(ui.available_width(), 1.0),
                );
                ui.painter().rect_filled(
                    sep_rect,
                    0.0,
                    ui.visuals().widgets.noninteractive.bg_stroke.color,
                );
                ui.add_space(1.0);

                // 3. Render Editor / Preview Content inside a child UI filling the rest of the window
                let content_rect = ui.available_rect_before_wrap();
                let mut content_ui =
                    ui.child_ui(content_rect, egui::Layout::top_down(egui::Align::Min));
                content_ui.set_clip_rect(content_rect);

                if let Some(active_idx) = state.active_tab_index {
                    let font_size = state.config.font_size;
                    let line_numbers = state.config.line_numbers;
                    let cursor_style = state.config.cursor_style;
                    let theme = state.config.theme.clone();

                    let mut new_split_ratio = None;

                    if let Some(tab) = state.tabs.get_mut(active_idx) {
                        match tab.view_mode {
                            ViewMode::Editor => {
                                tab.editor_renderer.show(
                                    &mut content_ui,
                                    &mut tab.editor,
                                    font_size,
                                    line_numbers,
                                    cursor_style,
                                );
                            }
                            ViewMode::Preview => {
                                state.preview.show(
                                    &mut content_ui,
                                    &mut tab.editor,
                                    &tab.path,
                                    font_size,
                                    &theme,
                                );
                            }
                            ViewMode::Split => {
                                let min_rect = content_ui.max_rect();
                                // Allocate the entire space so egui knows we are occupying it
                                content_ui.allocate_rect(min_rect, egui::Sense::hover());

                                let available_width = min_rect.width();
                                let available_height = min_rect.height();

                                if available_width < 600.0 {
                                    // Vertical Split (Top-to-Bottom)
                                    let separator_height = 2.0;
                                    let drag_handle_height = 8.0;
                                    let total_height = available_height - separator_height;
                                    let top_height = (total_height * state.split_ratio)
                                        .max(100.0)
                                        .min(total_height - 100.0);
                                    let bottom_height = total_height - top_height;

                                    // Top Rect
                                    let top_rect = egui::Rect::from_min_size(
                                        min_rect.min,
                                        egui::vec2(available_width, top_height),
                                    );

                                    // Separator Rect (visible thin line)
                                    let sep_rect = egui::Rect::from_min_size(
                                        egui::pos2(min_rect.min.x, min_rect.min.y + top_height),
                                        egui::vec2(available_width, separator_height),
                                    );

                                    // Drag Handle Rect (wider click target)
                                    let drag_rect = egui::Rect::from_min_size(
                                        egui::pos2(
                                            min_rect.min.x,
                                            min_rect.min.y + top_height
                                                - (drag_handle_height - separator_height) / 2.0,
                                        ),
                                        egui::vec2(available_width, drag_handle_height),
                                    );

                                    // Bottom Rect
                                    let bottom_rect = egui::Rect::from_min_size(
                                        egui::pos2(
                                            min_rect.min.x,
                                            min_rect.min.y + top_height + separator_height,
                                        ),
                                        egui::vec2(available_width, bottom_height),
                                    );

                                    // Top Pane: Editor
                                    let mut top_ui = content_ui.child_ui(
                                        top_rect,
                                        egui::Layout::top_down(egui::Align::Min),
                                    );
                                    top_ui.set_clip_rect(top_rect);
                                    tab.editor_renderer.show(
                                        &mut top_ui,
                                        &mut tab.editor,
                                        font_size,
                                        line_numbers,
                                        cursor_style,
                                    );

                                    // Separator / Drag handle
                                    let sep_response =
                                        content_ui.allocate_rect(drag_rect, egui::Sense::drag());

                                    // Paint separator line
                                    let color = if sep_response.dragged() {
                                        content_ui.visuals().selection.bg_fill
                                    } else if sep_response.hovered() {
                                        content_ui.visuals().selection.bg_fill.linear_multiply(0.8)
                                    } else {
                                        content_ui.visuals().widgets.noninteractive.bg_stroke.color
                                    };
                                    content_ui.painter().rect_filled(sep_rect, 0.0, color);

                                    // Update split_ratio based on drag
                                    if sep_response.dragged() {
                                        if let Some(mouse_pos) =
                                            content_ui.input(|i| i.pointer.latest_pos())
                                        {
                                            let relative_y = mouse_pos.y - min_rect.top();
                                            new_split_ratio = Some(
                                                (relative_y / available_height).clamp(0.1, 0.9),
                                            );
                                        }
                                    }

                                    // Change cursor to vertical resize icon
                                    if sep_response.hovered() || sep_response.dragged() {
                                        content_ui.output_mut(|o| {
                                            o.cursor_icon = egui::CursorIcon::ResizeVertical
                                        });
                                    }

                                    // Bottom Pane: Preview
                                    let mut bottom_ui = content_ui.child_ui(
                                        bottom_rect,
                                        egui::Layout::top_down(egui::Align::Min),
                                    );
                                    bottom_ui.set_clip_rect(bottom_rect);
                                    state.preview.show(
                                        &mut bottom_ui,
                                        &mut tab.editor,
                                        &tab.path,
                                        font_size,
                                        &theme,
                                    );
                                } else {
                                    // Horizontal Split (Side-by-Side)
                                    let separator_width = 2.0;
                                    let drag_handle_width = 8.0;
                                    let total_width = available_width - separator_width;
                                    let left_width = (total_width * state.split_ratio)
                                        .max(100.0)
                                        .min(total_width - 100.0);
                                    let right_width = total_width - left_width;

                                    // Left Rect
                                    let left_rect = egui::Rect::from_min_size(
                                        min_rect.min,
                                        egui::vec2(left_width, available_height),
                                    );

                                    // Separator Rect (visible thin line)
                                    let sep_rect = egui::Rect::from_min_size(
                                        egui::pos2(min_rect.min.x + left_width, min_rect.min.y),
                                        egui::vec2(separator_width, available_height),
                                    );

                                    // Drag Handle Rect (wider click target)
                                    let drag_rect = egui::Rect::from_min_size(
                                        egui::pos2(
                                            min_rect.min.x + left_width
                                                - (drag_handle_width - separator_width) / 2.0,
                                            min_rect.min.y,
                                        ),
                                        egui::vec2(drag_handle_width, available_height),
                                    );

                                    // Right Rect
                                    let right_rect = egui::Rect::from_min_size(
                                        egui::pos2(
                                            min_rect.min.x + left_width + separator_width,
                                            min_rect.min.y,
                                        ),
                                        egui::vec2(right_width, available_height),
                                    );

                                    // Left Pane: Editor
                                    let mut left_ui = content_ui.child_ui(
                                        left_rect,
                                        egui::Layout::top_down(egui::Align::Min),
                                    );
                                    left_ui.set_clip_rect(left_rect);
                                    tab.editor_renderer.show(
                                        &mut left_ui,
                                        &mut tab.editor,
                                        font_size,
                                        line_numbers,
                                        cursor_style,
                                    );

                                    // Separator / Drag handle
                                    let sep_response =
                                        content_ui.allocate_rect(drag_rect, egui::Sense::drag());

                                    // Paint separator line
                                    let color = if sep_response.dragged() {
                                        content_ui.visuals().selection.bg_fill
                                    } else if sep_response.hovered() {
                                        content_ui.visuals().selection.bg_fill.linear_multiply(0.8)
                                    } else {
                                        content_ui.visuals().widgets.noninteractive.bg_stroke.color
                                    };
                                    content_ui.painter().rect_filled(sep_rect, 0.0, color);

                                    // Update split_ratio based on drag
                                    if sep_response.dragged() {
                                        if let Some(mouse_pos) =
                                            content_ui.input(|i| i.pointer.latest_pos())
                                        {
                                            let relative_x = mouse_pos.x - min_rect.left();
                                            new_split_ratio = Some(
                                                (relative_x / available_width).clamp(0.1, 0.9),
                                            );
                                        }
                                    }

                                    // Change cursor to horizontal resize icon
                                    if sep_response.hovered() || sep_response.dragged() {
                                        content_ui.output_mut(|o| {
                                            o.cursor_icon = egui::CursorIcon::ResizeHorizontal
                                        });
                                    }

                                    // Right Pane: Preview
                                    let mut right_ui = content_ui.child_ui(
                                        right_rect,
                                        egui::Layout::top_down(egui::Align::Min),
                                    );
                                    right_ui.set_clip_rect(right_rect);
                                    state.preview.show(
                                        &mut right_ui,
                                        &mut tab.editor,
                                        &tab.path,
                                        font_size,
                                        &theme,
                                    );
                                }
                            }
                        }
                    }
                    if let Some(ratio) = new_split_ratio {
                        state.split_ratio = ratio;
                    }
                }
            } else {
                let is_dark = ui.visuals().dark_mode;
                let logo_texture = if is_dark {
                    state.logo_dark_mode.get_or_insert_with(|| {
                        let image_bytes = include_bytes!("../../assets/logo/dr-md-inapp-dark.png");
                        let image = image::load_from_memory_with_format(
                            image_bytes,
                            image::ImageFormat::Png,
                        )
                        .expect("Failed to load dark-mode logo image");
                        let size = [image.width() as _, image.height() as _];
                        let image_buffer = image.to_rgba8();
                        let pixels = image_buffer.as_flat_samples();
                        let color_image =
                            egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());
                        ctx.load_texture("logo_dark", color_image, Default::default())
                    })
                } else {
                    state.logo_light_mode.get_or_insert_with(|| {
                        let image_bytes = include_bytes!("../../assets/logo/dr-md-inapp-light.png");
                        let image = image::load_from_memory_with_format(
                            image_bytes,
                            image::ImageFormat::Png,
                        )
                        .expect("Failed to load light-mode logo image");
                        let size = [image.width() as _, image.height() as _];
                        let image_buffer = image.to_rgba8();
                        let pixels = image_buffer.as_flat_samples();
                        let color_image =
                            egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());
                        ctx.load_texture("logo_light", color_image, Default::default())
                    })
                };

                ui.vertical_centered(|ui| {
                    ui.add_space(ui.available_height() / 4.0);
                    ui.add(egui::Image::new(&*logo_texture).max_width(128.0));
                    ui.add_space(16.0);
                    ui.label(
                        egui::RichText::new("Welcome to Doctor. Markdown")
                            .heading()
                            .strong(),
                    );
                    ui.add_space(12.0);
                    ui.add(egui::Label::new(
                        "Press Ctrl+N to create a new note\n\
                         Press Ctrl+Shift+N to create a new folder\n\
                         or Ctrl+O to open a workspace folder.",
                    ));
                });
            }
        });
}

fn render_tab_bar(ui: &mut egui::Ui, state: &mut AppState) {
    let tab_bar_frame = egui::Frame::none()
        .fill(ui.visuals().panel_fill)
        .inner_margin(egui::Margin::symmetric(8.0, 0.0));

    tab_bar_frame.show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 4.0;

            // Render Tabs scroll area
            egui::ScrollArea::horizontal()
                .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        let mut tab_to_close = None;
                        let mut tab_to_switch = None;
                        let mut tab_to_swap = None;

                        for idx in 0..state.tabs.len() {
                            let is_active = state.active_tab_index == Some(idx);
                            let tab_path = state.tabs[idx].path.clone();
                            let filename = tab_path
                                .file_name()
                                .unwrap_or_default()
                                .to_string_lossy()
                                .to_string();
                            let is_dirty = state.tabs[idx].editor.is_dirty;

                            let tab_id = ui.id().with(&tab_path);
                            let tab_rect = ui
                                .memory(|mem| mem.data.get_temp::<egui::Rect>(tab_id))
                                .unwrap_or(egui::Rect::NOTHING);

                            let is_tab_hovered = ui.rect_contains_pointer(tab_rect);
                            let bg = if is_active {
                                ui.visuals().panel_fill
                            } else if is_tab_hovered {
                                ui.visuals().widgets.hovered.weak_bg_fill
                            } else {
                                egui::Color32::TRANSPARENT
                            };

                            let mut close_rect = egui::Rect::NOTHING;

                            let tab_inner_resp = ui.vertical(|ui| {
                                let tab_frame = egui::Frame::none()
                                    .fill(bg)
                                    .inner_margin(egui::Margin::symmetric(12.0, 6.0));

                                tab_frame.show(ui, |ui| {
                                    ui.horizontal(|ui| {
                                        ui.spacing_mut().item_spacing.x = 8.0;

                                        let text_color = if is_active {
                                            ui.visuals().strong_text_color()
                                        } else {
                                            ui.visuals().widgets.noninteractive.fg_stroke.color
                                        };

                                        let mut rich_text =
                                            egui::RichText::new(&filename).color(text_color);
                                        if is_active {
                                            rich_text = rich_text.strong();
                                        }

                                        ui.add(egui::Label::new(rich_text));

                                        // Close / Dirty dot spacer
                                        let (crect, _) = ui.allocate_exact_size(
                                            egui::vec2(12.0, 12.0),
                                            egui::Sense::click(),
                                        );
                                        close_rect = crect;

                                        let painter = ui.painter();
                                        let is_close_hovered = ui.rect_contains_pointer(close_rect);
                                        if is_close_hovered {
                                            painter.text(
                                                close_rect.center(),
                                                egui::Align2::CENTER_CENTER,
                                                "×",
                                                egui::FontId::monospace(14.0),
                                                egui::Color32::from_rgb(220, 50, 50),
                                            );
                                        } else if is_tab_hovered {
                                            painter.text(
                                                close_rect.center(),
                                                egui::Align2::CENTER_CENTER,
                                                "×",
                                                egui::FontId::monospace(14.0),
                                                text_color.linear_multiply(0.4),
                                            );
                                        } else if is_dirty {
                                            let accent = state.config.theme_accent.color();
                                            painter.circle_filled(close_rect.center(), 4.0, accent);
                                        }
                                    });
                                });
                            });

                            let new_tab_rect = tab_inner_resp.response.rect;
                            ui.memory_mut(|mem| {
                                mem.data.insert_temp::<egui::Rect>(tab_id, new_tab_rect)
                            });

                            if is_active {
                                // Draw accent line at the bottom of the tab to connect it nicely
                                let accent = state.config.theme_accent.color();
                                let bottom_line = egui::Rect::from_min_max(
                                    egui::pos2(new_tab_rect.min.x, new_tab_rect.max.y - 2.0),
                                    egui::pos2(new_tab_rect.max.x, new_tab_rect.max.y),
                                );
                                ui.painter().rect_filled(bottom_line, 0.0, accent);
                            }

                            let response = ui.interact(new_tab_rect, tab_id, egui::Sense::drag());

                            if response.dragged() {
                                ui.output_mut(|o| o.cursor_icon = egui::CursorIcon::Grabbing);

                                let press_origin = ui.input(|i| i.pointer.press_origin());
                                let is_drag_on_close =
                                    press_origin.map_or(false, |pos| close_rect.contains(pos));

                                if !is_drag_on_close {
                                    if let Some(mouse_pos) = ui.input(|i| i.pointer.hover_pos()) {
                                        let mut target_idx = None;

                                        // Check right neighbor (dragging right)
                                        if idx + 1 < state.tabs.len() {
                                            let other_idx = idx + 1;
                                            let other_path = &state.tabs[other_idx].path;
                                            let other_id = ui.id().with(other_path);
                                            let other_rect = ui
                                                .memory(|mem| {
                                                    mem.data.get_temp::<egui::Rect>(other_id)
                                                })
                                                .unwrap_or(egui::Rect::NOTHING);
                                            if other_rect != egui::Rect::NOTHING
                                                && mouse_pos.x > other_rect.center().x
                                            {
                                                target_idx = Some(other_idx);
                                            }
                                        }

                                        // Check left neighbor (dragging left)
                                        if target_idx.is_none() && idx > 0 {
                                            let other_idx = idx - 1;
                                            let other_path = &state.tabs[other_idx].path;
                                            let other_id = ui.id().with(other_path);
                                            let other_rect = ui
                                                .memory(|mem| {
                                                    mem.data.get_temp::<egui::Rect>(other_id)
                                                })
                                                .unwrap_or(egui::Rect::NOTHING);
                                            if other_rect != egui::Rect::NOTHING
                                                && mouse_pos.x < other_rect.center().x
                                            {
                                                target_idx = Some(other_idx);
                                            }
                                        }

                                        if let Some(other_idx) = target_idx {
                                            tab_to_swap = Some((idx, other_idx));
                                        }
                                    }
                                }
                            } else if response.hovered() {
                                ui.output_mut(|o| o.cursor_icon = egui::CursorIcon::PointingHand);
                            }

                            // Switch/close tab on click/press (excluding close button area)
                            if ui.input(|i| i.pointer.button_pressed(egui::PointerButton::Primary))
                            {
                                if let Some(press_pos) = ui.input(|i| i.pointer.press_origin()) {
                                    if new_tab_rect.contains(press_pos)
                                        && !close_rect.contains(press_pos)
                                    {
                                        tab_to_switch = Some(idx);
                                    }
                                }
                            }
                            // Close button click check
                            if ui.input(|i| i.pointer.button_clicked(egui::PointerButton::Primary))
                            {
                                if let Some(click_pos) = ui.input(|i| i.pointer.interact_pos()) {
                                    if close_rect.contains(click_pos) {
                                        tab_to_close = Some(idx);
                                    }
                                }
                            }
                            if ui.input(|i| i.pointer.button_pressed(egui::PointerButton::Middle)) {
                                if let Some(press_pos) = ui.input(|i| i.pointer.press_origin()) {
                                    if new_tab_rect.contains(press_pos) {
                                        tab_to_close = Some(idx);
                                    }
                                }
                            }
                        }

                        if let Some(idx) = tab_to_close {
                            state.close_tab(idx);
                        } else if let Some(idx) = tab_to_switch {
                            state.switch_tab(idx);
                        } else if let Some((idx, other_idx)) = tab_to_swap {
                            state.tabs.swap(idx, other_idx);
                            if state.active_tab_index == Some(idx) {
                                state.active_tab_index = Some(other_idx);
                            } else if state.active_tab_index == Some(other_idx) {
                                state.active_tab_index = Some(idx);
                            }
                            state.sync_session_state();
                        }
                    });
                });
        });
    });
}
