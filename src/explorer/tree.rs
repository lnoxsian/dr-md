use std::fs;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CreatingType {
    File { parent_dir: PathBuf },
    Folder { parent_dir: PathBuf },
    Renaming { path: PathBuf },
}

#[derive(Clone, Debug)]
pub enum ClipboardAction {
    Copy(PathBuf),
    Cut(PathBuf),
}

pub struct FileTree {
    pub creating_type: Option<CreatingType>,
    pub name_buffer: String,
    pub focus_input: bool,
    pub clipboard: Option<ClipboardAction>,
    pub selected_folder: Option<PathBuf>,
    pub dragged_item: Option<PathBuf>,
    pub hovered_folder: Option<PathBuf>,
}

impl FileTree {
    pub fn new() -> Self {
        Self {
            creating_type: None,
            name_buffer: String::new(),
            focus_input: false,
            clipboard: None,
            selected_folder: None,
            dragged_item: None,
            hovered_folder: None,
        }
    }

    pub fn start_creation(&mut self, creation: CreatingType) {
        self.creating_type = Some(creation);
        self.name_buffer.clear();
        self.focus_input = true;
    }

    pub fn start_rename(&mut self, path: PathBuf) {
        self.name_buffer = path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .into_owned();
        self.creating_type = Some(CreatingType::Renaming { path });
        self.focus_input = true;
    }

    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        root: &Path,
        active_file: &mut Option<PathBuf>,
    ) -> Option<PathBuf> {
        self.hovered_folder = None;
        let mut clicked_file = None;
        self.render_dir(ui, root, &mut clicked_file, active_file);

        // Render expanding empty space at the bottom of the tree for root actions
        let remaining_space = ui.available_size();
        if remaining_space.y > 10.0 {
            let (_rect, response) = ui.allocate_at_least(remaining_space, egui::Sense::click());

            if response.hovered() {
                if ui.input(|i| i.pointer.any_released()) {
                    if let Some(src) = self.dragged_item.take() {
                        self.move_to_dir(&src, root, active_file);
                    }
                }
            }

            response.context_menu(|ui| {
                if ui.button("📝 New File in Root").clicked() {
                    self.start_creation(CreatingType::File {
                        parent_dir: root.to_path_buf(),
                    });
                    ui.close_menu();
                }
                if ui.button("📁 New Folder in Root").clicked() {
                    self.start_creation(CreatingType::Folder {
                        parent_dir: root.to_path_buf(),
                    });
                    ui.close_menu();
                }
                ui.separator();
                let can_paste = self.clipboard.is_some();
                if ui
                    .add_enabled(can_paste, egui::Button::new("📋 Paste to Root"))
                    .clicked()
                {
                    self.paste_item(root, active_file);
                    ui.close_menu();
                }
            });
        }

        // Handle drag tooltip
        if let Some(src) = self.dragged_item.clone() {
            if let Some(mouse_pos) = ui.ctx().pointer_latest_pos() {
                egui::Area::new(egui::Id::new("drag_icon"))
                    .order(egui::Order::Tooltip)
                    .fixed_pos(mouse_pos + egui::vec2(10.0, 10.0))
                    .show(ui.ctx(), |ui| {
                        let name = src.file_name().unwrap_or_default().to_string_lossy();
                        ui.label(format!("📄 {}", name));
                    });
            }

            if ui.input(|i| i.pointer.any_released()) {
                self.dragged_item = None;
            }
        }

        clicked_file
    }

    fn render_dir(
        &mut self,
        ui: &mut egui::Ui,
        path: &Path,
        clicked_file: &mut Option<PathBuf>,
        active_file: &mut Option<PathBuf>,
    ) {
        // Render inline input if creating under this directory
        if let Some(creating) = &self.creating_type {
            match creating {
                CreatingType::File { parent_dir } if parent_dir == path => {
                    self.render_creation_input(ui, path, true, active_file);
                }
                CreatingType::Folder { parent_dir } if parent_dir == path => {
                    self.render_creation_input(ui, path, false, active_file);
                }
                _ => {}
            }
        }

        if let Ok(entries) = fs::read_dir(path) {
            let mut entries: Vec<_> = entries.filter_map(|e| e.ok()).collect();
            self.sort_entries(&mut entries);

            for entry in entries {
                let entry_path = entry.path();
                let file_name = entry.file_name().to_string_lossy().into_owned();

                // Skip hidden files/directories
                if file_name.starts_with('.') {
                    continue;
                }

                let is_dir = entry.file_type().map(|t| t.is_dir()).unwrap_or(false);

                // Handle Renaming inline input instead of standard rendering
                let is_renaming = match &self.creating_type {
                    Some(CreatingType::Renaming { path: rename_path }) => {
                        rename_path == &entry_path
                    }
                    _ => false,
                };

                if is_renaming {
                    self.render_creation_input(
                        ui,
                        entry_path.parent().unwrap(),
                        !is_dir,
                        active_file,
                    );
                    continue;
                }

                if is_dir {
                    self.render_folder_node(ui, entry_path, file_name, clicked_file, active_file);
                } else {
                    self.render_file_node(ui, entry_path, file_name, clicked_file, active_file);
                }
            }
        }
    }

    fn sort_entries(&self, entries: &mut [fs::DirEntry]) {
        entries.sort_by(|a, b| {
            let a_is_dir = a.file_type().map(|t| t.is_dir()).unwrap_or(false);
            let b_is_dir = b.file_type().map(|t| t.is_dir()).unwrap_or(false);
            if a_is_dir && !b_is_dir {
                std::cmp::Ordering::Less
            } else if !a_is_dir && b_is_dir {
                std::cmp::Ordering::Greater
            } else {
                a.file_name().cmp(&b.file_name())
            }
        });
    }

    fn render_folder_node(
        &mut self,
        ui: &mut egui::Ui,
        entry_path: PathBuf,
        file_name: String,
        clicked_file: &mut Option<PathBuf>,
        active_file: &mut Option<PathBuf>,
    ) {
        let id = ui.make_persistent_id(&entry_path);

        // Force the collapsing header open if we are creating something inside it!
        if let Some(creating) = &self.creating_type {
            match creating {
                CreatingType::File { parent_dir } | CreatingType::Folder { parent_dir } => {
                    if parent_dir == &entry_path {
                        ui.data_mut(|d| d.insert_temp(id, true));
                    }
                }
                _ => {}
            }
        }

        let is_selected = self.selected_folder.as_ref() == Some(&entry_path);
        let display_name = if is_selected {
            format!("📁 ★ {}", file_name)
        } else {
            format!("📁 {}", file_name)
        };

        let collapsing_response = egui::CollapsingHeader::new(display_name)
            .id_source(&entry_path)
            .show(ui, |ui| {
                self.render_dir(ui, &entry_path, clicked_file, active_file);
            });

        let header_response = ui.interact(
            collapsing_response.header_response.rect,
            collapsing_response.header_response.id,
            egui::Sense::click_and_drag(),
        );

        if header_response.drag_started() {
            self.dragged_item = Some(entry_path.clone());
        }

        // Check if dragged item is dropped on this folder
        let is_hovered = ui.rect_contains_pointer(header_response.rect);
        if is_hovered {
            self.hovered_folder = Some(entry_path.clone());
            if ui.input(|i| i.pointer.any_released()) {
                if let Some(src) = self.dragged_item.take() {
                    if src != entry_path && !entry_path.starts_with(&src) {
                        self.move_to_dir(&src, &entry_path, active_file);
                    }
                }
            }
        }

        if header_response.clicked() {
            self.selected_folder = Some(entry_path.clone());
        }

        collapsing_response.header_response.context_menu(|ui| {
            self.show_folder_context_menu(ui, &entry_path, active_file);
        });
    }

    fn render_file_node(
        &mut self,
        ui: &mut egui::Ui,
        entry_path: PathBuf,
        file_name: String,
        clicked_file: &mut Option<PathBuf>,
        active_file: &mut Option<PathBuf>,
    ) {
        let is_md = entry_path
            .extension()
            .map(|e| e == "md" || e == "markdown")
            .unwrap_or(false);
        if !is_md {
            return;
        }

        let is_active = Some(&entry_path) == active_file.as_ref();
        let label = ui.selectable_label(is_active, file_name);
        let label_response = ui.interact(label.rect, label.id, egui::Sense::click_and_drag());

        if label_response.drag_started() {
            self.dragged_item = Some(entry_path.clone());
        }

        // Check if dragged item is dropped on this file's parent folder
        let is_hovered = ui.rect_contains_pointer(label_response.rect);
        if is_hovered && ui.input(|i| i.pointer.any_released()) {
            if let Some(src) = self.dragged_item.take() {
                if let Some(parent) = entry_path.parent() {
                    if src != parent && !parent.starts_with(&src) {
                        self.move_to_dir(&src, parent, active_file);
                    }
                }
            }
        }

        if label_response.clicked() {
            *clicked_file = Some(entry_path.clone());
        }

        label.context_menu(|ui| {
            self.show_file_context_menu(ui, &entry_path, active_file);
        });
    }

    fn show_folder_context_menu(
        &mut self,
        ui: &mut egui::Ui,
        entry_path: &Path,
        active_file: &mut Option<PathBuf>,
    ) {
        if ui.button("📝 New File").clicked() {
            self.start_creation(CreatingType::File {
                parent_dir: entry_path.to_path_buf(),
            });
            ui.close_menu();
        }
        if ui.button("📁 New Folder").clicked() {
            self.start_creation(CreatingType::Folder {
                parent_dir: entry_path.to_path_buf(),
            });
            ui.close_menu();
        }
        ui.separator();
        if ui.button("✏ Rename").clicked() {
            self.start_rename(entry_path.to_path_buf());
            ui.close_menu();
        }
        if ui.button("✂️ Cut (Move)").clicked() {
            self.clipboard = Some(ClipboardAction::Cut(entry_path.to_path_buf()));
            ui.close_menu();
        }
        if ui.button("📋 Copy").clicked() {
            self.clipboard = Some(ClipboardAction::Copy(entry_path.to_path_buf()));
            ui.close_menu();
        }
        let can_paste = self.clipboard.is_some();
        if ui
            .add_enabled(can_paste, egui::Button::new("📋 Paste"))
            .clicked()
        {
            self.paste_item(entry_path, active_file);
            ui.close_menu();
        }
        ui.separator();
        if ui.button("🗑️ Delete").clicked() {
            self.delete_item(entry_path, active_file);
            ui.close_menu();
        }
    }

    fn show_file_context_menu(
        &mut self,
        ui: &mut egui::Ui,
        entry_path: &Path,
        active_file: &mut Option<PathBuf>,
    ) {
        if ui.button("✏ Rename").clicked() {
            self.start_rename(entry_path.to_path_buf());
            ui.close_menu();
        }
        if ui.button("✂️ Cut (Move)").clicked() {
            self.clipboard = Some(ClipboardAction::Cut(entry_path.to_path_buf()));
            ui.close_menu();
        }
        if ui.button("📋 Copy").clicked() {
            self.clipboard = Some(ClipboardAction::Copy(entry_path.to_path_buf()));
            ui.close_menu();
        }
        ui.separator();
        if ui.button("🗑️ Delete").clicked() {
            self.delete_item(entry_path, active_file);
            ui.close_menu();
        }
    }

    fn render_creation_input(
        &mut self,
        ui: &mut egui::Ui,
        _parent: &Path,
        is_file: bool,
        active_file: &mut Option<PathBuf>,
    ) {
        ui.horizontal(|ui| {
            let icon = if is_file { "📝" } else { "📁" };
            ui.label(icon);

            let text_edit = egui::TextEdit::singleline(&mut self.name_buffer)
                .hint_text(if is_file {
                    "file_name.md"
                } else {
                    "Folder name"
                })
                .desired_width(120.0);

            let response = ui.add(text_edit);

            if self.focus_input {
                response.request_focus();
                self.focus_input = false;
            }

            let enter_pressed = ui.input(|i| i.key_pressed(egui::Key::Enter));
            let esc_pressed = ui.input(|i| i.key_pressed(egui::Key::Escape));

            if esc_pressed {
                self.creating_type = None;
                self.name_buffer.clear();
            } else if enter_pressed {
                let name = self.name_buffer.trim().to_string();
                if !name.is_empty() {
                    match self.creating_type.clone() {
                        Some(CreatingType::Renaming { path: src_path }) => {
                            let mut dest = src_path.parent().unwrap().join(&name);
                            if src_path.is_file() && dest.extension().is_none() {
                                if let Some(ext) = src_path.extension() {
                                    dest.set_extension(ext);
                                } else {
                                    dest.set_extension("md");
                                }
                            }
                            if src_path.exists() && src_path != dest {
                                let unique_dest = self.get_unique_destination(&dest);
                                if let Err(e) = fs::rename(&src_path, &unique_dest) {
                                    tracing::error!("Failed to rename: {:?}", e);
                                } else {
                                    if let Some(active) = active_file {
                                        if *active == src_path {
                                            *active = unique_dest;
                                        } else if active.starts_with(&src_path) {
                                            if let Ok(relative) = active.strip_prefix(&src_path) {
                                                *active = unique_dest.join(relative);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Some(CreatingType::File { parent_dir }) => {
                            let mut dest = parent_dir.join(&name);
                            if dest.extension().is_none() {
                                dest.set_extension("md");
                            }
                            fs::write(&dest, "").ok();
                        }
                        Some(CreatingType::Folder { parent_dir }) => {
                            let dest = parent_dir.join(&name);
                            fs::create_dir_all(&dest).ok();
                        }
                        None => {}
                    }
                }
                self.creating_type = None;
                self.name_buffer.clear();
            } else if response.lost_focus() {
                self.creating_type = None;
                self.name_buffer.clear();
            }
        });
    }

    fn copy_dir_all(&self, src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
        fs::create_dir_all(&dst)?;
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let file_type = entry.file_type()?;
            if file_type.is_dir() {
                self.copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
            } else {
                fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
            }
        }
        Ok(())
    }

    fn get_unique_destination(&self, dest: &Path) -> PathBuf {
        if !dest.exists() {
            return dest.to_path_buf();
        }
        let parent = dest.parent().unwrap();
        let file_stem = dest.file_stem().unwrap().to_string_lossy();
        let extension = dest
            .extension()
            .map(|e| format!(".{}", e.to_string_lossy()))
            .unwrap_or_default();
        let mut counter = 1;
        loop {
            let candidate = parent.join(format!("{}_{}{}", file_stem, counter, extension));
            if !candidate.exists() {
                return candidate;
            }
            counter += 1;
        }
    }

    pub fn move_to_dir(&self, src: &Path, dest_dir: &Path, active_file: &mut Option<PathBuf>) {
        if src.exists() && dest_dir.is_dir() {
            let filename = src.file_name().unwrap();
            let target = dest_dir.join(filename);
            let unique_target = self.get_unique_destination(&target);
            if unique_target.starts_with(src) {
                tracing::error!("Cannot move a directory inside itself");
                return;
            }

            let result = if let Err(_) = fs::rename(src, &unique_target) {
                if src.is_file() {
                    if let Err(e) = fs::copy(src, &unique_target) {
                        Err(e)
                    } else {
                        fs::remove_file(src)
                    }
                } else {
                    if let Err(e) = self.copy_dir_all(src, &unique_target) {
                        Err(e)
                    } else {
                        fs::remove_dir_all(src)
                    }
                }
            } else {
                Ok(())
            };

            if let Err(e) = result {
                tracing::error!("Failed to move item in drag-and-drop: {:?}", e);
            } else {
                if let Some(active) = active_file {
                    if *active == src {
                        *active = unique_target;
                    } else if active.starts_with(src) {
                        if let Ok(relative) = active.strip_prefix(src) {
                            *active = unique_target.join(relative);
                        }
                    }
                }
            }
        }
    }

    pub fn paste_item(&mut self, dest_dir: &Path, active_file: &mut Option<PathBuf>) {
        if let Some(action) = self.clipboard.clone() {
            match action {
                ClipboardAction::Copy(src) => {
                    if src.exists() {
                        let filename = src.file_name().unwrap();
                        let target = dest_dir.join(filename);
                        let unique_target = self.get_unique_destination(&target);
                        if src.is_file() {
                            if let Err(e) = fs::copy(&src, &unique_target) {
                                tracing::error!("Failed to copy file: {:?}", e);
                            }
                        } else {
                            if let Err(e) = self.copy_dir_all(&src, &unique_target) {
                                tracing::error!("Failed to copy directory: {:?}", e);
                            }
                        }
                    }
                }
                ClipboardAction::Cut(src) => {
                    self.move_to_dir(&src, dest_dir, active_file);
                    self.clipboard = None;
                }
            }
        }
    }

    pub fn delete_item(&mut self, path: &Path, active_file: &mut Option<PathBuf>) {
        let result = if path.is_file() {
            fs::remove_file(path)
        } else {
            fs::remove_dir_all(path)
        };

        if let Err(e) = result {
            tracing::error!("Failed to delete item: {:?}", e);
        } else {
            if let Some(active) = active_file {
                if active == path || active.starts_with(path) {
                    *active_file = None;
                }
            }
        }
    }
}
