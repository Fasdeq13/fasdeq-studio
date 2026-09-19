use crate::editor::keybindings::KeymapStyle;
use crate::theme::FasdeqTheme;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserPreferences {
    pub theme_name: String,
    pub custom_theme_path: Option<PathBuf>,
    pub keymap: KeymapStyle,
    pub font_size: f32,
}

impl Default for UserPreferences {
    fn default() -> Self {
        Self {
            theme_name: "Fasdeq Dark".to_string(),
            custom_theme_path: None,
            keymap: KeymapStyle::VsCode,
            font_size: 15.0,
        }
    }
}

impl UserPreferences {
    pub fn config_path() -> PathBuf {
        let mut dir = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        dir.push("fasdeq-studio");
        std::fs::create_dir_all(&dir).ok();
        dir.push("preferences.json");
        dir
    }

    pub fn load() -> Self {
        let path = Self::config_path();
        if let Ok(content) = std::fs::read_to_string(&path) {
            if let Ok(prefs) = serde_json::from_str(&content) {
                return prefs;
            }
        }
        Self::default()
    }

    pub fn save(&self) {
        let path = Self::config_path();
        if let Ok(content) = serde_json::to_string_pretty(self) {
            let _ = std::fs::write(path, content);
        }
    }

    pub fn resolve_theme(&self) -> FasdeqTheme {
        if let Some(custom_path) = &self.custom_theme_path {
            if let Ok(theme) = FasdeqTheme::load_from_file(custom_path) {
                return theme;
            }
        }
        FasdeqTheme::built_in_themes()
            .into_iter()
            .find(|t| t.name == self.theme_name)
            .unwrap_or_else(FasdeqTheme::dark_default)
    }

    pub fn from_current(theme: &FasdeqTheme, custom_theme_path: Option<PathBuf>, keymap: KeymapStyle, font_size: f32) -> Self {
        Self {
            theme_name: theme.name.clone(),
            custom_theme_path,
            keymap,
            font_size,
        }
    }
}
