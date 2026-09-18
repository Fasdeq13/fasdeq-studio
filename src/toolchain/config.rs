use super::detect::ToolKind;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ToolchainConfig {
    pub paths: HashMap<String, PathBuf>,
    pub setup_completed: bool,
}

impl ToolchainConfig {
    pub fn config_path() -> PathBuf {
        let mut dir = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        dir.push("fasdeq-studio");
        std::fs::create_dir_all(&dir).ok();
        dir.push("toolchain.json");
        dir
    }

    pub fn load() -> Self {
        let path = Self::config_path();
        if let Ok(content) = std::fs::read_to_string(&path) {
            if let Ok(config) = serde_json::from_str(&content) {
                return config;
            }
        }
        Self::default()
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let path = Self::config_path();
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    pub fn set_path(&mut self, tool: &ToolKind, path: PathBuf) {
        self.paths.insert(tool.binary_name().to_string(), path);
    }

    pub fn get_path(&self, tool: &ToolKind) -> Option<PathBuf> {
        self.paths.get(tool.binary_name()).cloned()
    }
}
