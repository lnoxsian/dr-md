use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ThemeAccent {
    Blue,
    Purple,
    Orange,
    Red,
    Green,
    Pink,
    Teal,
    Yellow,
}

impl Default for ThemeAccent {
    fn default() -> Self {
        Self::Blue
    }
}

impl ThemeAccent {
    pub fn color(&self) -> egui::Color32 {
        match self {
            Self::Blue => egui::Color32::from_rgb(59, 130, 246),
            Self::Purple => egui::Color32::from_rgb(168, 85, 247),
            Self::Orange => egui::Color32::from_rgb(249, 115, 22),
            Self::Red => egui::Color32::from_rgb(239, 68, 68),
            Self::Green => egui::Color32::from_rgb(16, 185, 129),
            Self::Pink => egui::Color32::from_rgb(236, 72, 153),
            Self::Teal => egui::Color32::from_rgb(20, 184, 166),
            Self::Yellow => egui::Color32::from_rgb(234, 179, 8),
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Blue => "Blue",
            Self::Purple => "Purple",
            Self::Orange => "Orange",
            Self::Red => "Red",
            Self::Green => "Green",
            Self::Pink => "Pink",
            Self::Teal => "Teal",
            Self::Yellow => "Yellow",
        }
    }

    pub fn all() -> &'static [Self] {
        &[
            Self::Blue,
            Self::Purple,
            Self::Orange,
            Self::Red,
            Self::Green,
            Self::Pink,
            Self::Teal,
            Self::Yellow,
        ]
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct AppConfig {
    pub theme: String,
    pub theme_accent: ThemeAccent,
    pub font_size: f32,
    pub line_numbers: bool,
    pub autosave: bool,
    pub tab_width: usize,
    pub last_opened_folder: Option<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            theme: "dark".to_string(),
            theme_accent: ThemeAccent::default(),
            font_size: 14.0,
            line_numbers: true,
            autosave: true,
            tab_width: 4,
            last_opened_folder: None,
        }
    }
}

pub fn apply_theme(ctx: &egui::Context, config: &AppConfig) {
    let mut style = (*ctx.style()).clone();
    
    // Set theme base mode (dark/light)
    style.visuals = if config.theme == "dark" {
        egui::Visuals::dark()
    } else {
        egui::Visuals::light()
    };
    
    // Apply styling options
    style.visuals.widgets.noninteractive.rounding = egui::Rounding::ZERO;
    style.visuals.widgets.inactive.rounding = egui::Rounding::ZERO;
    style.visuals.widgets.hovered.rounding = egui::Rounding::ZERO;
    style.visuals.widgets.active.rounding = egui::Rounding::ZERO;
    style.visuals.widgets.open.rounding = egui::Rounding::ZERO;
    style.visuals.window_rounding = egui::Rounding::ZERO;
    style.visuals.menu_rounding = egui::Rounding::ZERO;
    
    let accent_color = config.theme_accent.color();
    
    // Apply theme accent color to selection
    style.visuals.selection.bg_fill = accent_color;
    
    // Apply to hyperlinks
    style.visuals.hyperlink_color = accent_color;
    
    // Apply to cursor stroke
    style.visuals.text_cursor.color = accent_color;
    
    // Set active/hovered widget border accents to make the UI pop
    style.visuals.widgets.hovered.bg_stroke = egui::Stroke::new(1.0, accent_color);
    style.visuals.widgets.active.bg_stroke = egui::Stroke::new(1.0, accent_color);

    // Apply uniform font sizes to all text styles (body, button, heading, monospace, small)
    let font_size = config.font_size;
    let body_font = egui::FontId::new(font_size, egui::FontFamily::Proportional);
    let button_font = egui::FontId::new(font_size, egui::FontFamily::Proportional);
    let heading_font = egui::FontId::new(font_size * 1.3, egui::FontFamily::Proportional);
    let monospace_font = egui::FontId::new(font_size, egui::FontFamily::Monospace);
    let small_font = egui::FontId::new(font_size * 0.85, egui::FontFamily::Proportional);

    style.text_styles.insert(egui::TextStyle::Body, body_font);
    style.text_styles.insert(egui::TextStyle::Button, button_font);
    style.text_styles.insert(egui::TextStyle::Heading, heading_font);
    style.text_styles.insert(egui::TextStyle::Monospace, monospace_font);
    style.text_styles.insert(egui::TextStyle::Small, small_font);
    
    ctx.set_style(style);
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
                base.home_dir()
                    .join(".config")
                    .join("dr-md")
                    .join("config.toml")
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
            path.to_string_lossy()
                .ends_with(".config/dr-md/config.toml")
                || path
                    .to_string_lossy()
                    .ends_with(".config\\dr-md\\config.toml")
                || cfg!(target_os = "windows")
        );
    }
}
