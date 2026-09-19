use crate::editor::keybindings::KeymapStyle;
use crate::theme::FasdeqTheme;
use crate::toolchain::config::ToolchainConfig;
use crate::toolchain::detect::{detect_tool_at_path, ToolKind, ToolStatus};
use crate::toolchain::distro::{detect_distro, DistroInfo};
use crate::toolchain::installer::{install_tools, InstallEvent};
use std::path::PathBuf;
use std::sync::mpsc::{Receiver, Sender};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WizardStage {
    Welcome,
    Scanning,
    ToolSummary,
    Installing,
    ManualPaths,
    ThemeChoice,
    KeymapChoice,
    Done,
}

pub struct WizardState {
    pub stage: WizardStage,
    pub distro: DistroInfo,
    pub tools: Vec<ToolStatus>,
    pub missing_tools: Vec<ToolKind>,
    pub manual_path_index: usize,
    pub manual_path_input: String,
    pub install_log: Vec<String>,
    pub install_receiver: Option<Receiver<InstallEvent>>,
    pub install_failed: Option<String>,
    pub chosen_theme: FasdeqTheme,
    pub custom_theme_path: Option<PathBuf>,
    pub chosen_keymap: KeymapStyle,
}

impl WizardState {
    pub fn new() -> Self {
        let distro = detect_distro();
        Self {
            stage: WizardStage::Welcome,
            distro,
            tools: Vec::new(),
            missing_tools: Vec::new(),
            manual_path_index: 0,
            manual_path_input: String::new(),
            install_log: Vec::new(),
            install_receiver: None,
            install_failed: None,
            chosen_theme: FasdeqTheme::dark_default(),
            custom_theme_path: None,
            chosen_keymap: KeymapStyle::VsCode,
        }
    }

    pub fn scan(&mut self) {
        self.tools = crate::toolchain::detect::detect_all();
        self.missing_tools = self
            .tools
            .iter()
            .filter(|t| !t.found)
            .map(|t| t.kind.clone())
            .collect();
        self.stage = if self.missing_tools.is_empty() {
            WizardStage::ThemeChoice
        } else {
            WizardStage::ToolSummary
        };
    }

    pub fn start_install(&mut self) {
        let (tx, rx): (Sender<InstallEvent>, Receiver<InstallEvent>) = std::sync::mpsc::channel();
        self.install_receiver = Some(rx);
        self.install_log.clear();
        self.install_failed = None;
        let distro = self.distro.clone();
        let tools = self.missing_tools.clone();
        std::thread::spawn(move || {
            install_tools(&distro, &tools, tx);
        });
        self.stage = WizardStage::Installing;
    }

    pub fn poll_install(&mut self) {
        let mut finished = false;
        let mut failed_msg = None;
        if let Some(receiver) = &self.install_receiver {
            while let Ok(event) = receiver.try_recv() {
                match event {
                    InstallEvent::Started(cmd) => {
                        self.install_log.push(format!("> {cmd}"));
                    }
                    InstallEvent::Line(line) => {
                        self.install_log.push(line);
                    }
                    InstallEvent::Finished(_) => {
                        finished = true;
                    }
                    InstallEvent::Failed(msg) => {
                        failed_msg = Some(msg);
                    }
                }
            }
        }
        if let Some(msg) = failed_msg {
            self.install_log.push(format!("Error: {msg}"));
            self.install_failed = Some(msg);
        }
        if finished {
            self.rescan_after_install();
        }
    }

    fn rescan_after_install(&mut self) {
        self.tools = crate::toolchain::detect::detect_all();
        self.missing_tools = self
            .tools
            .iter()
            .filter(|t| !t.found)
            .map(|t| t.kind.clone())
            .collect();
        if self.missing_tools.is_empty() {
            self.stage = WizardStage::ThemeChoice;
        } else {
            self.stage = WizardStage::ManualPaths;
            self.manual_path_index = 0;
        }
    }

    pub fn switch_to_manual(&mut self) {
        self.stage = WizardStage::ManualPaths;
        self.manual_path_index = 0;
        self.manual_path_input.clear();
    }

    pub fn confirm_manual_path(&mut self, config: &mut ToolchainConfig) {
        if self.manual_path_index >= self.missing_tools.len() {
            return;
        }
        let tool = self.missing_tools[self.manual_path_index].clone();
        let path = PathBuf::from(self.manual_path_input.trim());
        let status = detect_tool_at_path(tool.clone(), path.clone());
        if status.found {
            config.set_path(&tool, path);
        }
        self.manual_path_index += 1;
        self.manual_path_input.clear();
        if self.manual_path_index >= self.missing_tools.len() {
            let _ = config.save();
            self.stage = WizardStage::ThemeChoice;
        }
    }

    pub fn current_manual_tool(&self) -> Option<&ToolKind> {
        self.missing_tools.get(self.manual_path_index)
    }
}
