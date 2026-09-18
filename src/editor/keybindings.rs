use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum KeymapStyle {
    VsCode,
    Vim,
}

impl KeymapStyle {
    pub fn label(&self) -> &'static str {
        match self {
            KeymapStyle::VsCode => "VSCode",
            KeymapStyle::Vim => "Vim",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VimMode {
    Normal,
    Insert,
    Visual,
    Command,
}

#[derive(Clone, Debug)]
pub struct VimState {
    pub mode: VimMode,
    pub pending: String,
    pub command_buffer: String,
}

impl Default for VimState {
    fn default() -> Self {
        Self {
            mode: VimMode::Normal,
            pending: String::new(),
            command_buffer: String::new(),
        }
    }
}

impl VimState {
    pub fn mode_label(&self) -> &'static str {
        match self.mode {
            VimMode::Normal => "NORMAL",
            VimMode::Insert => "INSERT",
            VimMode::Visual => "VISUAL",
            VimMode::Command => "COMMAND",
        }
    }
}
