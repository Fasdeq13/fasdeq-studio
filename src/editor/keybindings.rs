use eframe::egui;
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
    pub cursor: usize,
    pub anchor: usize,
}

impl Default for VimState {
    fn default() -> Self {
        Self {
            mode: VimMode::Normal,
            pending: String::new(),
            command_buffer: String::new(),
            cursor: 0,
            anchor: 0,
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VimOutcome {
    None,
    Save,
    SaveAndContinue,
}

pub fn line_start(text: &str, pos: usize) -> usize {
    text[..pos.min(text.len())].rfind('\n').map(|i| i + 1).unwrap_or(0)
}

pub fn line_end(text: &str, pos: usize) -> usize {
    text[pos.min(text.len())..]
        .find('\n')
        .map(|i| pos + i)
        .unwrap_or(text.len())
}

fn move_line(text: &str, pos: usize, delta: i32) -> usize {
    let ls = line_start(text, pos);
    let col = pos - ls;
    let target_line_start = if delta < 0 {
        let mut p = ls;
        for _ in 0..(-delta) {
            if p == 0 {
                break;
            }
            p = line_start(text, p - 1);
        }
        p
    } else {
        let mut p = ls;
        for _ in 0..delta {
            let le = line_end(text, p);
            if le >= text.len() {
                p = le;
                break;
            }
            p = le + 1;
        }
        p
    };
    let target_line_end = line_end(text, target_line_start);
    (target_line_start + col).min(target_line_end)
}

pub fn handle_key(
    state: &mut VimState,
    text: &mut String,
    key: egui::Key,
    modifiers: egui::Modifiers,
) -> VimOutcome {
    match state.mode {
        VimMode::Insert => {
            if key == egui::Key::Escape {
                state.mode = VimMode::Normal;
                if state.cursor > 0 {
                    state.cursor -= 1;
                }
            }
            VimOutcome::None
        }
        VimMode::Normal | VimMode::Visual => handle_normal_key(state, text, key, modifiers),
        VimMode::Command => {
            if key == egui::Key::Enter {
                let cmd = state.command_buffer.clone();
                state.command_buffer.clear();
                state.mode = VimMode::Normal;
                if cmd.trim() == "w" {
                    return VimOutcome::Save;
                } else if cmd.trim() == "wq" || cmd.trim() == "x" {
                    return VimOutcome::Save;
                }
            } else if key == egui::Key::Escape {
                state.command_buffer.clear();
                state.mode = VimMode::Normal;
            }
            VimOutcome::None
        }
    }
}

fn handle_normal_key(
    state: &mut VimState,
    text: &mut String,
    key: egui::Key,
    _modifiers: egui::Modifiers,
) -> VimOutcome {
    state.cursor = state.cursor.min(text.len());
    while state.cursor > 0 && !text.is_char_boundary(state.cursor) {
        state.cursor -= 1;
    }
    match key {
        egui::Key::H | egui::Key::ArrowLeft => {
            if state.cursor > 0 {
                state.cursor -= 1;
            }
        }
        egui::Key::L | egui::Key::ArrowRight => {
            if state.cursor < text.len() {
                state.cursor += 1;
            }
        }
        egui::Key::J | egui::Key::ArrowDown => {
            state.cursor = move_line(text, state.cursor, 1);
        }
        egui::Key::K | egui::Key::ArrowUp => {
            state.cursor = move_line(text, state.cursor, -1);
        }
        egui::Key::Num0 => {
            state.cursor = line_start(text, state.cursor);
        }
        egui::Key::I => {
            state.mode = VimMode::Insert;
        }
        egui::Key::A => {
            if state.cursor < text.len() {
                state.cursor += 1;
            }
            state.mode = VimMode::Insert;
        }
        egui::Key::X => {
            if state.cursor < text.len() {
                let next = text[state.cursor..]
                    .char_indices()
                    .nth(1)
                    .map(|(i, _)| state.cursor + i)
                    .unwrap_or(text.len());
                text.replace_range(state.cursor..next, "");
            }
        }
        egui::Key::D => {
            if state.pending == "d" {
                let ls = line_start(text, state.cursor);
                let le = (line_end(text, state.cursor) + 1).min(text.len());
                text.replace_range(ls..le, "");
                state.cursor = ls;
                state.pending.clear();
            } else {
                state.pending = "d".to_string();
            }
        }
        egui::Key::O => {
            let le = line_end(text, state.cursor);
            text.insert(le, '\n');
            state.cursor = le + 1;
            state.mode = VimMode::Insert;
        }
        egui::Key::V => {
            state.mode = VimMode::Visual;
            state.anchor = state.cursor;
        }
        egui::Key::Colon => {
            state.mode = VimMode::Command;
            state.command_buffer.clear();
        }
        egui::Key::Escape => {
            state.mode = VimMode::Normal;
            state.pending.clear();
        }
        _ => {
            state.pending.clear();
        }
    }
    VimOutcome::None
}
