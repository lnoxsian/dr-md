use std::path::{Path, PathBuf};
use std::fs;

pub struct FileTree {}

impl FileTree {
    pub fn new() -> Self {
        Self {}
    }

    pub fn show(&mut self, ui: &mut egui::Ui, root: &Path, active_file: &mut Option<PathBuf>) -> Option<PathBuf> {
        let mut clicked_file = None;
        self.render_dir(ui, root, &mut clicked_file, active_file);
        clicked_file
    }

    fn render_dir(
        &mut self,
        ui: &mut egui::Ui,
        path: &Path,
        clicked_file: &mut Option<PathBuf>,
        active_file: &mut Option<PathBuf>,
    ) {
        if let Ok(entries) = fs::read_dir(path) {
            let mut entries: Vec<_> = entries.filter_map(|e| e.ok()).collect();
            // Sort: directories first, then files alphabetically
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

            for entry in entries {
                let entry_path = entry.path();
                let file_name = entry.file_name().to_string_lossy().into_owned();
                
                // Skip hidden files/directories
                if file_name.starts_with('.') {
                    continue;
                }

                let is_dir = entry.file_type().map(|t| t.is_dir()).unwrap_or(false);

                if is_dir {
                    egui::CollapsingHeader::new(file_name)
                        .id_source(&entry_path)
                        .show(ui, |ui| {
                            self.render_dir(ui, &entry_path, clicked_file, active_file);
                        });
                } else {
                    // Check if it is a markdown file
                    let is_md = entry_path.extension().map(|e| e == "md" || e == "markdown").unwrap_or(false);
                    if is_md {
                        let is_active = Some(&entry_path) == active_file.as_ref();
                        let label = ui.selectable_label(is_active, file_name);
                        if label.clicked() {
                            *clicked_file = Some(entry_path);
                        }
                    }
                }
            }
        }
    }
}
