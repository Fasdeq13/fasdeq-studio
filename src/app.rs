use crate::disasm::binfile::BinaryInfo;
use crate::disasm::engine::DisasmInstruction;
use crate::editor::buffer::EditorBuffer;
use crate::editor::diagnostics::Diagnostic;
use crate::editor::highlight::HighlightEngine;
use crate::editor::keybindings::{KeymapStyle, VimState};
use crate::extensions::manager::ExtensionManager;
use crate::project::templates::ProjectTemplate;
use crate::project::tree::FileTree;
use crate::project::{OpenProject, RecentProjects};
use crate::theme::FasdeqTheme;
use crate::toolchain::config::ToolchainConfig;
use crate::ui::disasm_panel::DisasmPanelState;
use crate::wizard::WizardState;
use std::path::PathBuf;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AppScreen {
    FirstRunWizard,
    StartMenu,
    NewProjectMenu,
    CloneProject,
    ExtensionsMenu,
    Workspace,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorkspaceTab {
    Editor,
    Disassembler,
    InstructionReference,
    BuildOutput,
    NumberBaseConverter,
}

pub struct FasdeqApp {
    pub screen: AppScreen,
    pub wizard: WizardState,
    pub toolchain_config: ToolchainConfig,
    pub theme: FasdeqTheme,
    pub available_themes: Vec<FasdeqTheme>,
    pub keymap: KeymapStyle,
    pub vim_state: VimState,

    pub recent_projects: RecentProjects,
    pub current_project: Option<OpenProject>,

    pub new_project_name: String,
    pub new_project_location: PathBuf,
    pub selected_template: Option<ProjectTemplate>,

    pub clone_url: String,
    pub clone_destination: PathBuf,
    pub clone_error: Option<String>,

    pub highlight_engine: HighlightEngine,
    pub open_buffers: Vec<EditorBuffer>,
    pub active_buffer: Option<usize>,
    pub file_tree: Option<FileTree>,

    pub diagnostics: Vec<Diagnostic>,
    pub build_stdout: String,
    pub build_stderr: String,
    pub building: bool,

    pub workspace_tab: WorkspaceTab,
    pub disasm_panel: DisasmPanelState,
    pub extension_manager: ExtensionManager,
    pub show_extensions_panel: bool,
    pub extension_install_url: String,
    pub extension_install_error: Option<String>,

    pub font_size: f32,
    pub show_settings: bool,
    pub theme_import_error: Option<String>,
    pub reference_search: String,
    pub numbase_binary: String,
    pub numbase_octal: String,
    pub numbase_decimal: String,
    pub numbase_hex: String,
    pub custom_theme_path: Option<PathBuf>,
}

impl FasdeqApp {
    pub fn new() -> Self {
        let toolchain_config = ToolchainConfig::load();
        let screen = if toolchain_config.setup_completed {
            AppScreen::StartMenu
        } else {
            AppScreen::FirstRunWizard
        };
        let preferences = crate::settings::UserPreferences::load();
        let theme = if toolchain_config.setup_completed {
            preferences.resolve_theme()
        } else {
            FasdeqTheme::dark_default()
        };
        let keymap = if toolchain_config.setup_completed {
            preferences.keymap
        } else {
            KeymapStyle::VsCode
        };
        let font_size = if toolchain_config.setup_completed {
            preferences.font_size
        } else {
            15.0
        };
        let custom_theme_path = preferences.custom_theme_path.clone();

        Self {
            screen,
            wizard: WizardState::new(),
            toolchain_config,
            theme,
            available_themes: FasdeqTheme::built_in_themes(),
            keymap,
            vim_state: VimState::default(),

            recent_projects: RecentProjects::load(),
            current_project: None,

            new_project_name: String::new(),
            new_project_location: dirs::home_dir().unwrap_or_else(|| PathBuf::from(".")),
            selected_template: None,

            clone_url: String::new(),
            clone_destination: dirs::home_dir().unwrap_or_else(|| PathBuf::from(".")),
            clone_error: None,

            highlight_engine: HighlightEngine::new(),
            open_buffers: Vec::new(),
            active_buffer: None,
            file_tree: None,

            diagnostics: Vec::new(),
            build_stdout: String::new(),
            build_stderr: String::new(),
            building: false,

            workspace_tab: WorkspaceTab::Editor,
            disasm_panel: DisasmPanelState::default_state(),
            extension_manager: ExtensionManager::new().unwrap_or_else(|_| ExtensionManager {
                runtime: crate::extensions::runtime::ExtensionRuntime::new()
                    .expect("failed to initialize the WASM extension runtime"),
                installed: Vec::new(),
                running: Vec::new(),
                registered_commands: Vec::new(),
                registered_panels: Vec::new(),
                log: Vec::new(),
            }),
            show_extensions_panel: false,
            extension_install_url: String::new(),
            extension_install_error: None,

            font_size,
            show_settings: false,
            theme_import_error: None,
            reference_search: String::new(),
            numbase_binary: "0".to_string(),
            numbase_octal: "0".to_string(),
            numbase_decimal: "0".to_string(),
            numbase_hex: "0".to_string(),
            custom_theme_path,
        }
    }

    pub fn save_preferences(&self) {
        crate::settings::UserPreferences::from_current(
            &self.theme,
            self.custom_theme_path.clone(),
            self.keymap,
            self.font_size,
        )
        .save();
    }

    pub fn open_project(&mut self, project: OpenProject) {
        self.file_tree = Some(FileTree::new(project.root.clone()));
        self.recent_projects
            .push(project.name.clone(), project.root.clone());
        self.current_project = Some(project);
        self.open_buffers.clear();
        self.active_buffer = None;
        self.diagnostics.clear();
        self.build_stdout.clear();
        self.build_stderr.clear();
        self.screen = AppScreen::Workspace;
        self.workspace_tab = WorkspaceTab::Editor;
    }

    pub fn open_file_in_editor(&mut self, path: PathBuf) {
        if let Some(idx) = self.open_buffers.iter().position(|b| b.path == path) {
            self.active_buffer = Some(idx);
            return;
        }
        if let Ok(buffer) = EditorBuffer::open(path) {
            self.open_buffers.push(buffer);
            self.active_buffer = Some(self.open_buffers.len() - 1);
        }
    }

    pub fn active_buffer_mut(&mut self) -> Option<&mut EditorBuffer> {
        self.active_buffer.and_then(|i| self.open_buffers.get_mut(i))
    }

    pub fn close_buffer(&mut self, index: usize) {
        if index < self.open_buffers.len() {
            self.open_buffers.remove(index);
            self.active_buffer = if self.open_buffers.is_empty() {
                None
            } else {
                Some(index.min(self.open_buffers.len() - 1))
            };
        }
    }

    pub fn close_buffers_under(&mut self, path: &std::path::Path) {
        self.open_buffers.retain(|b| !b.path.starts_with(path));
        self.active_buffer = if self.open_buffers.is_empty() {
            None
        } else {
            Some(self.active_buffer.unwrap_or(0).min(self.open_buffers.len() - 1))
        };
    }

    pub fn save_active_buffer(&mut self) {
        if let Some(buffer) = self.active_buffer_mut() {
            let _ = buffer.save();
        }
    }

    pub fn run_build(&mut self) {
        let Some(project) = &self.current_project else {
            return;
        };
        let tool = detect_build_tool(&project.root);
        self.building = true;
        match crate::editor::diagnostics::run_build(&project.root, tool) {
            Ok(result) => {
                self.build_stdout = result.stdout;
                self.build_stderr = result.stderr;
                self.diagnostics = result.diagnostics;
            }
            Err(e) => {
                self.build_stderr = format!("Could not start the build: {e}");
            }
        }
        self.building = false;
        self.workspace_tab = WorkspaceTab::BuildOutput;
    }

    pub fn diagnostics_for(&self, path: &std::path::Path) -> Vec<&Diagnostic> {
        self.diagnostics
            .iter()
            .filter(|d| {
                path.ends_with(&d.file) || d.file.ends_with(path.file_name().unwrap_or_default())
            })
            .collect()
    }

    pub fn set_numbase_value(&mut self, value: u64) {
        use crate::editor::numbase::{format_value, NumberBase};
        self.numbase_binary = format_value(value, NumberBase::Binary, false);
        self.numbase_octal = format_value(value, NumberBase::Octal, false);
        self.numbase_decimal = format_value(value, NumberBase::Decimal, false);
        self.numbase_hex = format_value(value, NumberBase::Hexadecimal, false);
    }

    pub fn load_binary_for_disasm(&mut self, path: PathBuf) {
        if let Ok(bytes) = crate::disasm::hexview::read_file_bytes(&path) {
            self.disasm_panel.file_path = Some(path);
            self.disasm_panel.file_bytes = bytes;
            self.disasm_panel.binary_info = crate::disasm::binfile::analyze(&self.disasm_panel.file_bytes).ok();
            self.disasm_panel.reset_for_new_file();
            self.workspace_tab = WorkspaceTab::Disassembler;
        }
    }

    pub fn apply_byte_patch(&mut self, offset: usize, new_value: u8) {
        let data = &mut self.disasm_panel.file_bytes;
        self.disasm_panel.patches.apply_patch(data, offset, new_value);
        self.disasm_panel.binary_info = crate::disasm::binfile::analyze(&self.disasm_panel.file_bytes).ok();
    }

    pub fn revert_byte_patch(&mut self, offset: usize) {
        let data = &mut self.disasm_panel.file_bytes;
        self.disasm_panel.patches.revert_patch(data, offset);
    }

    pub fn revert_all_patches(&mut self) {
        let data = &mut self.disasm_panel.file_bytes;
        self.disasm_panel.patches.revert_all(data);
    }

    pub fn save_patched_binary_in_place(&mut self) {
        let Some(path) = self.disasm_panel.file_path.clone() else {
            return;
        };
        match crate::disasm::hexview::write_patched_file(&path, &self.disasm_panel.file_bytes) {
            Ok(()) => {
                self.disasm_panel.patches = crate::disasm::hexview::PatchSet::new();
                self.disasm_panel.save_message = Some(format!("Saved to {}", path.to_string_lossy()));
            }
            Err(e) => {
                self.disasm_panel.save_message = Some(format!("Failed to save: {e}"));
            }
        }
    }

    pub fn save_patched_binary_as_copy(&mut self) {
        let Some(path) = self.disasm_panel.file_path.clone() else {
            return;
        };
        match crate::disasm::hexview::write_patched_copy(&path, &self.disasm_panel.file_bytes) {
            Ok(new_path) => {
                self.disasm_panel.patches = crate::disasm::hexview::PatchSet::new();
                self.disasm_panel.save_message =
                    Some(format!("Saved patched copy to {}", new_path.to_string_lossy()));
            }
            Err(e) => {
                self.disasm_panel.save_message = Some(format!("Failed to save copy: {e}"));
            }
        }
    }

    pub fn run_disassembly(&mut self) {
        let arch = self.disasm_panel.architecture;
        let base = self
            .disasm_panel
            .binary_info
            .as_ref()
            .map(|b| b.entry_point)
            .unwrap_or(0);
        let bytes = self.disasm_panel.file_bytes.clone();
        match crate::disasm::engine::disassemble(&bytes, arch, base) {
            Ok(instructions) => {
                self.disasm_panel.instructions = instructions;
                self.disasm_panel.error = None;
            }
            Err(e) => {
                self.disasm_panel.error = Some(e.to_string());
            }
        }
    }

    pub fn apply_theme(&self, ctx: &egui::Context) {
        self.theme.apply(ctx);
    }
}

fn detect_build_tool(root: &std::path::Path) -> &'static str {
    if root.join("Cargo.toml").exists() {
        "cargo"
    } else {
        "make"
    }
}

pub struct BinaryContext {
    pub info: Option<BinaryInfo>,
    pub instructions: Vec<DisasmInstruction>,
}
