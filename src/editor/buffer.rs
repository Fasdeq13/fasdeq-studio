use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct EditorBuffer {
    pub path: PathBuf,
    pub content: String,
    pub dirty: bool,
    pub cursor_line: usize,
    pub cursor_col: usize,
}

impl EditorBuffer {
    pub fn open(path: PathBuf) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(&path).unwrap_or_default();
        Ok(Self {
            path,
            content,
            dirty: false,
            cursor_line: 0,
            cursor_col: 0,
        })
    }

    pub fn extension(&self) -> String {
        self.path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase()
    }

    pub fn file_name(&self) -> String {
        self.path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "untitled".to_string())
    }

    pub fn save(&mut self) -> anyhow::Result<()> {
        std::fs::write(&self.path, &self.content)?;
        self.dirty = false;
        Ok(())
    }

    pub fn line_count(&self) -> usize {
        self.content.lines().count().max(1)
    }
}
