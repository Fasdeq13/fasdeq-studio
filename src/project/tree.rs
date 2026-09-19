use std::collections::HashSet;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug)]
pub struct TreeEntry {
    pub path: PathBuf,
    pub name: String,
    pub is_dir: bool,
}

pub struct FileTree {
    pub root: PathBuf,
    pub expanded: HashSet<PathBuf>,
    pub children_cache: std::collections::HashMap<PathBuf, Vec<TreeEntry>>,
    pub selected: Option<PathBuf>,
    pub renaming: Option<PathBuf>,
    pub rename_input: String,
    pub rename_input_focused: bool,
    pub creating_in: Option<(PathBuf, bool)>,
    pub create_input: String,
    pub create_input_focused: bool,
    pub pending_delete: Option<PathBuf>,
}

impl FileTree {
    pub fn new(root: PathBuf) -> Self {
        let mut tree = Self {
            root: root.clone(),
            expanded: HashSet::new(),
            children_cache: std::collections::HashMap::new(),
            selected: None,
            renaming: None,
            rename_input: String::new(),
            rename_input_focused: false,
            creating_in: None,
            create_input: String::new(),
            create_input_focused: false,
            pending_delete: None,
        };
        tree.expanded.insert(root.clone());
        tree.load_children(&root);
        tree
    }

    pub fn load_children(&mut self, dir: &Path) {
        let mut entries: Vec<TreeEntry> = std::fs::read_dir(dir)
            .into_iter()
            .flatten()
            .filter_map(|e| e.ok())
            .filter(|e| {
                let name = e.file_name().to_string_lossy().to_string();
                !matches!(name.as_str(), ".git" | "target" | "build" | "node_modules")
            })
            .map(|e| {
                let path = e.path();
                let is_dir = path.is_dir();
                let name = e.file_name().to_string_lossy().to_string();
                TreeEntry { path, name, is_dir }
            })
            .collect();

        entries.sort_by(|a, b| match (a.is_dir, b.is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
        });

        self.children_cache.insert(dir.to_path_buf(), entries);
    }

    pub fn toggle_expanded(&mut self, dir: &Path) {
        if self.expanded.contains(dir) {
            self.expanded.remove(dir);
        } else {
            self.expanded.insert(dir.to_path_buf());
            if !self.children_cache.contains_key(dir) {
                self.load_children(dir);
            }
        }
    }

    pub fn refresh(&mut self, dir: &Path) {
        self.load_children(dir);
    }

    pub fn refresh_root(&mut self) {
        let expanded: Vec<PathBuf> = self.expanded.iter().cloned().collect();
        for dir in expanded {
            if dir.is_dir() {
                self.load_children(&dir);
            }
        }
    }

    pub fn begin_rename(&mut self, path: PathBuf) {
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();
        self.rename_input = name;
        self.renaming = Some(path);
        self.rename_input_focused = false;
    }

    pub fn confirm_rename(&mut self) -> anyhow::Result<()> {
        let Some(path) = self.renaming.take() else {
            return Ok(());
        };
        self.rename_input_focused = false;
        let new_name = self.rename_input.trim();
        if new_name.is_empty() {
            return Ok(());
        }
        let new_path = path
            .parent()
            .map(|p| p.join(new_name))
            .unwrap_or_else(|| PathBuf::from(new_name));
        if new_path != path {
            std::fs::rename(&path, &new_path)?;
        }
        if let Some(parent) = path.parent() {
            self.refresh(parent);
        }
        self.rename_input.clear();
        Ok(())
    }

    pub fn cancel_rename(&mut self) {
        self.renaming = None;
        self.rename_input.clear();
        self.rename_input_focused = false;
    }

    pub fn begin_create(&mut self, dir: PathBuf, is_dir: bool) {
        self.expanded.insert(dir.clone());
        self.creating_in = Some((dir, is_dir));
        self.create_input.clear();
        self.create_input_focused = false;
    }

    pub fn confirm_create(&mut self) -> anyhow::Result<Option<PathBuf>> {
        let Some((dir, is_dir)) = self.creating_in.take() else {
            return Ok(None);
        };
        self.create_input_focused = false;
        let name = self.create_input.trim();
        if name.is_empty() {
            return Ok(None);
        }
        let new_path = dir.join(name);
        if is_dir {
            std::fs::create_dir_all(&new_path)?;
        } else {
            if let Some(parent) = new_path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::write(&new_path, "")?;
        }
        self.refresh(&dir);
        self.create_input.clear();
        Ok(Some(new_path))
    }

    pub fn cancel_create(&mut self) {
        self.creating_in = None;
        self.create_input.clear();
        self.create_input_focused = false;
    }

    pub fn delete(&mut self, path: &Path) -> anyhow::Result<()> {
        if path.is_dir() {
            std::fs::remove_dir_all(path)?;
        } else {
            std::fs::remove_file(path)?;
        }
        if let Some(parent) = path.parent() {
            self.refresh(parent);
        }
        self.expanded.remove(path);
        self.children_cache.remove(path);
        Ok(())
    }
}
