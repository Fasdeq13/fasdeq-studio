use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExtensionManifest {
    pub id: String,
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub entry: String,
    #[serde(default)]
    pub contributes: Contributes,
    #[serde(default)]
    pub permissions: Permissions,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Contributes {
    #[serde(default)]
    pub languages: Vec<LanguageContribution>,
    #[serde(default)]
    pub themes: Vec<ThemeContribution>,
    #[serde(default)]
    pub commands: Vec<CommandContribution>,
    #[serde(default)]
    pub panels: Vec<PanelContribution>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LanguageContribution {
    pub id: String,
    pub extensions: Vec<String>,
    pub syntax_file: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ThemeContribution {
    pub name: String,
    pub file: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CommandContribution {
    pub id: String,
    pub title: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PanelContribution {
    pub id: String,
    pub title: String,
    pub icon: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Permissions {
    #[serde(default)]
    pub network: bool,
    #[serde(default)]
    pub filesystem: bool,
    #[serde(default)]
    pub process_spawn: bool,
}

impl ExtensionManifest {
    pub fn load(path: &Path) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let manifest: ExtensionManifest = toml::from_str(&content)?;
        Ok(manifest)
    }
}

#[derive(Clone, Debug)]
pub struct InstalledExtension {
    pub manifest: ExtensionManifest,
    pub directory: PathBuf,
    pub enabled: bool,
}

pub fn extensions_directory() -> PathBuf {
    let mut dir = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    dir.push("fasdeq-studio");
    dir.push("extensions");
    std::fs::create_dir_all(&dir).ok();
    dir
}

pub fn discover_extensions() -> Vec<InstalledExtension> {
    let root = extensions_directory();
    let mut result = Vec::new();
    let Ok(entries) = std::fs::read_dir(&root) else {
        return result;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let manifest_path = path.join("extension.toml");
        if !manifest_path.exists() {
            continue;
        }
        if let Ok(manifest) = ExtensionManifest::load(&manifest_path) {
            result.push(InstalledExtension {
                manifest,
                directory: path,
                enabled: true,
            });
        }
    }
    result
}
