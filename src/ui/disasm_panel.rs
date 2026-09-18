use crate::disasm::binfile::BinaryInfo;
use crate::disasm::engine::{Architecture, DisasmInstruction};
use crate::disasm::hexview::PatchSet;
use std::path::PathBuf;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BinaryViewMode {
    Disassembly,
    HexEditor,
}

pub struct DisasmPanelState {
    pub file_path: Option<PathBuf>,
    pub file_bytes: Vec<u8>,
    pub binary_info: Option<BinaryInfo>,
    pub architecture: Architecture,
    pub instructions: Vec<DisasmInstruction>,
    pub error: Option<String>,
    pub selected_instruction: Option<usize>,
    pub hex_scroll_offset: usize,
    pub view_mode: BinaryViewMode,
    pub patches: PatchSet,
    pub selected_byte_offset: Option<usize>,
    pub byte_edit_input: String,
    pub save_message: Option<String>,
}

impl DisasmPanelState {
    pub fn default_state() -> Self {
        Self {
            file_path: None,
            file_bytes: Vec::new(),
            binary_info: None,
            architecture: Architecture::X86_64,
            instructions: Vec::new(),
            error: None,
            selected_instruction: None,
            hex_scroll_offset: 0,
            view_mode: BinaryViewMode::Disassembly,
            patches: PatchSet::new(),
            selected_byte_offset: None,
            byte_edit_input: String::new(),
            save_message: None,
        }
    }

    pub fn reset_for_new_file(&mut self) {
        self.instructions.clear();
        self.patches = PatchSet::new();
        self.selected_byte_offset = None;
        self.byte_edit_input.clear();
        self.save_message = None;
        self.error = None;
    }
}
