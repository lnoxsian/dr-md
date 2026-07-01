use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct AppConfig {
    pub theme: String,
    pub font_size: f32,
    pub preview_font_size: f32,
    pub line_numbers: bool,
    pub autosave: bool,
    pub tab_width: usize,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            theme: "dark".to_string(),
            font_size: 14.0,
            preview_font_size: 14.0,
            line_numbers: true,
            autosave: true,
            tab_width: 4,
        }
    }
}

impl AppConfig {
    pub fn load() -> Self {
        if let Some(proj_dirs) = directories::ProjectDirs::from("com", "drmd", "dr-md") {
            let config_dir = proj_dirs.config_dir();
            let config_path = config_dir.join("config.toml");
            if config_path.exists() {
                if let Ok(content) = std::fs::read_to_string(&config_path) {
                    if let Ok(config) = toml::from_str(&content) {
                        return config;
                    }
                }
            }
        }
        Self::default()
    }

    pub fn save(&self) -> Result<(), anyhow::Error> {
        if let Some(proj_dirs) = directories::ProjectDirs::from("com", "drmd", "dr-md") {
            let config_dir = proj_dirs.config_dir();
            std::fs::create_dir_all(config_dir)?;
            let config_path = config_dir.join("config.toml");
            let content = toml::to_string_pretty(self)?;
            std::fs::write(config_path, content)?;
        }
        Ok(())
    }
}
