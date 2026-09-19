pub mod templates;
pub mod tree;

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RecentProject {
    pub name: String,
    pub path: PathBuf,
    pub last_opened: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct RecentProjects {
    pub entries: Vec<RecentProject>,
}

impl RecentProjects {
    pub fn config_path() -> PathBuf {
        let mut dir = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        dir.push("fasdeq-studio");
        std::fs::create_dir_all(&dir).ok();
        dir.push("recent_projects.json");
        dir
    }

    pub fn load() -> Self {
        let path = Self::config_path();
        if let Ok(content) = std::fs::read_to_string(&path) {
            if let Ok(data) = serde_json::from_str(&content) {
                return data;
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

    pub fn push(&mut self, name: String, path: PathBuf) {
        self.entries.retain(|e| e.path != path);
        self.entries.insert(
            0,
            RecentProject {
                name,
                path,
                last_opened: chrono::Local::now().format("%Y-%m-%d %H:%M").to_string(),
            },
        );
        self.entries.truncate(20);
        self.save();
    }

    pub fn remove(&mut self, path: &Path) {
        self.entries.retain(|e| e.path != path);
        self.save();
    }
}

#[derive(Clone, Debug)]
pub struct OpenProject {
    pub name: String,
    pub root: PathBuf,
}

impl OpenProject {
    pub fn from_path(root: PathBuf) -> Self {
        let name = root
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "Project".to_string());
        Self { name, root }
    }
}

pub fn clone_repository(url: &str, destination: &Path) -> anyhow::Result<()> {
    let status = Command::new("git")
        .arg("clone")
        .arg(url)
        .arg(destination)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?;
    if !status.success() {
        anyhow::bail!("git clone exited with an error");
    }
    Ok(())
}

#[allow(dead_code)]
pub fn list_files_recursive(root: &Path) -> Vec<PathBuf> {
    walkdir::WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| {
            !e.path()
                .components()
                .any(|c| c.as_os_str() == ".git" || c.as_os_str() == "target" || c.as_os_str() == "build")
        })
        .map(|e| e.path().to_path_buf())
        .collect()
}
