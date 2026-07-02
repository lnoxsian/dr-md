use std::path::PathBuf;

pub struct Vault {
    pub root_path: Option<PathBuf>,
    pub active_file: Option<PathBuf>,
}

impl Vault {
    pub fn new(root_path: Option<PathBuf>) -> Self {
        Self {
            root_path,
            active_file: None,
        }
    }

    pub fn set_root(&mut self, path: PathBuf) {
        self.root_path = Some(path);
        self.active_file = None;
    }

}
