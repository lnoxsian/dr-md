use std::path::PathBuf;
use std::fs;
use anyhow::Result;

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

    pub fn new_note(&mut self) -> Result<PathBuf> {
        let root = self.root_path.as_ref().ok_or_else(|| anyhow::anyhow!("No active workspace"))?;
        
        let mut count = 0;
        let mut note_path = root.join("Untitled.md");
        while note_path.exists() {
            count += 1;
            note_path = root.join(format!("Untitled {}.md", count));
        }

        fs::write(&note_path, "")?;
        self.active_file = Some(note_path.clone());
        Ok(note_path)
    }

    pub fn read_active(&self) -> Result<String> {
        let path = self.active_file.as_ref().ok_or_else(|| anyhow::anyhow!("No active file"))?;
        let content = fs::read_to_string(path)?;
        Ok(content)
    }

    pub fn save_active(&self, content: &str) -> Result<()> {
        let path = self.active_file.as_ref().ok_or_else(|| anyhow::anyhow!("No active file"))?;
        fs::write(path, content)?;
        Ok(())
    }
}
