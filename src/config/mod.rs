use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct AppConfig {
    pub theme: String,
    pub font_size: f32,
    pub line_numbers: bool,
    pub autosave: bool,
    pub tab_width: usize,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            theme: "dark".to_string(),
            font_size: 14.0,
            line_numbers: true,
            autosave: true,
            tab_width: 4,
        }
    }
}

impl AppConfig {
    fn get_config_path() -> Option<std::path::PathBuf> {
        #[cfg(target_os = "windows")]
        {
            directories::ProjectDirs::from("com", "drmd", "dr-md")
                .map(|proj| proj.config_dir().join("config.toml"))
        }
        #[cfg(not(target_os = "windows"))]
        {
            directories::BaseDirs::new().map(|base| {
                base.home_dir().join(".config").join("dr-md").join("config.toml")
            })
        }
    }

    pub fn load() -> Self {
        if let Some(config_path) = Self::get_config_path() {
            if config_path.exists() {
                if let Ok(content) = std::fs::read_to_string(&config_path) {
                    if let Ok(config) = toml::from_str(&content) {
                        return config;
                    }
                }
            } else {
                let default_config = Self::default();
                let _ = default_config.save();
                return default_config;
            }
        }
        Self::default()
    }

    pub fn save(&self) -> Result<(), anyhow::Error> {
        if let Some(config_path) = Self::get_config_path() {
            if let Some(parent) = config_path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            let content = toml::to_string_pretty(self)?;
            std::fs::write(config_path, content)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_path() {
        let path = AppConfig::get_config_path().unwrap();
        assert!(
            path.to_string_lossy().ends_with(".config/dr-md/config.toml")
                || path.to_string_lossy().ends_with(".config\\dr-md\\config.toml")
                || cfg!(target_os = "windows")
        );
    }
}
