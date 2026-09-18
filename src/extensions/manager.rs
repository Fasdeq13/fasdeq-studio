use super::manifest::{discover_extensions, extensions_directory, InstalledExtension};
use super::runtime::{drain_pending_calls, ExtensionRuntime, ExtensionState, HostCall};
use std::path::PathBuf;
use wasmtime::{Instance, Module, Store};

pub struct RunningExtension {
    pub id: String,
    pub store: Store<ExtensionState>,
    pub instance: Instance,
}

pub struct ExtensionManager {
    pub runtime: ExtensionRuntime,
    pub installed: Vec<InstalledExtension>,
    pub running: Vec<RunningExtension>,
    pub registered_commands: Vec<(String, String, String)>,
    pub registered_panels: Vec<(String, String, String)>,
    pub log: Vec<String>,
}

impl ExtensionManager {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            runtime: ExtensionRuntime::new()?,
            installed: discover_extensions(),
            running: Vec::new(),
            registered_commands: Vec::new(),
            registered_panels: Vec::new(),
            log: Vec::new(),
        })
    }

    pub fn refresh_installed(&mut self) {
        self.installed = discover_extensions();
    }

    pub fn load_all_enabled(&mut self) {
        let extensions = self.installed.clone();
        for ext in extensions.iter().filter(|e| e.enabled) {
            if let Err(e) = self.load_extension(ext) {
                self.log
                    .push(format!("Failed to load extension {}: {e}", ext.manifest.id));
            }
        }
    }

    fn load_extension(&mut self, ext: &InstalledExtension) -> anyhow::Result<()> {
        let entry_path = ext.directory.join(&ext.manifest.entry);
        let bytes = std::fs::read(&entry_path)?;
        let module = self.runtime.load_module(&bytes)?;
        let (mut store, instance) = self.runtime.instantiate(
            &module,
            ext.manifest.id.clone(),
            ext.manifest.permissions.network,
            ext.manifest.permissions.filesystem,
        )?;
        self.runtime.call_activate(&mut store, &instance)?;
        self.process_calls(&ext.manifest.id, &mut store);

        self.running.push(RunningExtension {
            id: ext.manifest.id.clone(),
            store,
            instance,
        });

        for lang in &ext.manifest.contributes.languages {
            self.log.push(format!(
                "Extension {} contributed language: {}",
                ext.manifest.id, lang.id
            ));
        }
        for theme in &ext.manifest.contributes.themes {
            self.log.push(format!(
                "Extension {} contributed theme: {}",
                ext.manifest.id, theme.name
            ));
        }
        for command in &ext.manifest.contributes.commands {
            self.registered_commands.push((
                ext.manifest.id.clone(),
                command.id.clone(),
                command.title.clone(),
            ));
        }
        for panel in &ext.manifest.contributes.panels {
            self.registered_panels.push((
                ext.manifest.id.clone(),
                panel.id.clone(),
                panel.title.clone(),
            ));
        }

        Ok(())
    }

    fn process_calls(&mut self, extension_id: &str, store: &mut Store<ExtensionState>) {
        let calls = drain_pending_calls(store.data());
        for call in calls {
            match call {
                HostCall::ShowMessage(text) => {
                    self.log.push(format!("[{extension_id}] {text}"));
                }
                HostCall::RegisterCommand(id, title) => {
                    self.registered_commands
                        .push((extension_id.to_string(), id, title));
                }
                HostCall::RegisterPanel(id, title) => {
                    self.registered_panels
                        .push((extension_id.to_string(), id, title));
                }
                HostCall::NetworkRequest { url, method, .. } => {
                    self.log
                        .push(format!("[{extension_id}] network request {method} {url}"));
                }
                HostCall::ReadActiveBuffer => {}
                HostCall::WriteActiveBuffer(_) => {}
            }
        }
    }

    pub fn run_command(&mut self, command_id: &str) {
        for running in &mut self.running {
            if let Err(e) = self
                .runtime
                .call_command(&mut running.store, &running.instance, command_id)
            {
                self.log
                    .push(format!("Command execution failed in {}: {e}", running.id));
            }
        }
    }

    pub fn install_from_git(&mut self, url: &str) -> anyhow::Result<()> {
        let target_dir = extensions_directory().join(sanitize_repo_name(url));
        if target_dir.exists() {
            anyhow::bail!("Extension directory already exists");
        }
        git2::Repository::clone(url, &target_dir)?;
        self.refresh_installed();
        Ok(())
    }

    pub fn install_from_local(&mut self, source_dir: &PathBuf) -> anyhow::Result<()> {
        let manifest_path = source_dir.join("extension.toml");
        if !manifest_path.exists() {
            anyhow::bail!("extension.toml not found in the selected directory");
        }
        let manifest = super::manifest::ExtensionManifest::load(&manifest_path)?;
        let target_dir = extensions_directory().join(&manifest.id);
        copy_dir_recursive(source_dir, &target_dir)?;
        self.refresh_installed();
        Ok(())
    }

    pub fn uninstall(&mut self, extension_id: &str) -> anyhow::Result<()> {
        let dir = extensions_directory().join(extension_id);
        if dir.exists() {
            std::fs::remove_dir_all(dir)?;
        }
        self.running.retain(|r| r.id != extension_id);
        self.refresh_installed();
        Ok(())
    }
}

fn sanitize_repo_name(url: &str) -> String {
    url.rsplit('/')
        .next()
        .unwrap_or("extension")
        .trim_end_matches(".git")
        .to_string()
}

fn copy_dir_recursive(source: &PathBuf, target: &PathBuf) -> anyhow::Result<()> {
    std::fs::create_dir_all(target)?;
    for entry in std::fs::read_dir(source)? {
        let entry = entry?;
        let dest_path = target.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_dir_recursive(&entry.path(), &dest_path)?;
        } else {
            std::fs::copy(entry.path(), dest_path)?;
        }
    }
    Ok(())
}
