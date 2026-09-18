use crate::app::FasdeqApp;
use eframe::egui;

pub fn draw(app: &mut FasdeqApp, ui: &mut egui::Ui) {
    ui.label(egui::RichText::new("Extensions").size(16.0).strong());
    ui.label(egui::RichText::new("Extend Fasdeq Studio with new languages, themes, commands, panels, and AI assistant integrations such as Claude.").weak());
    ui.add_space(10.0);

    ui.horizontal(|ui| {
        if ui.button("📁 Install from Folder").clicked() {
            if let Some(path) = rfd::FileDialog::new().pick_folder() {
                if let Err(e) = app.extension_manager.install_from_local(&path) {
                    app.extension_install_error = Some(e.to_string());
                } else {
                    app.extension_install_error = None;
                }
            }
        }
        if ui.button("🔄 Refresh").clicked() {
            app.extension_manager.refresh_installed();
        }
    });

    ui.add_space(8.0);
    ui.horizontal(|ui| {
        ui.label("Git URL:");
        ui.text_edit_singleline(&mut app.extension_install_url);
        if ui.button("Clone & Install").clicked() {
            let url = app.extension_install_url.trim().to_string();
            if !url.is_empty() {
                if let Err(e) = app.extension_manager.install_from_git(&url) {
                    app.extension_install_error = Some(e.to_string());
                } else {
                    app.extension_install_error = None;
                    app.extension_install_url.clear();
                }
            }
        }
    });

    if let Some(err) = &app.extension_install_error {
        ui.colored_label(egui::Color32::from_rgb(230, 100, 100), err);
    }

    ui.add_space(16.0);
    ui.separator();
    ui.add_space(10.0);

    ui.label(egui::RichText::new("Installed Extensions").strong());
    ui.add_space(6.0);

    let installed = app.extension_manager.installed.clone();
    if installed.is_empty() {
        ui.label(egui::RichText::new("No extensions installed yet. Try installing the bundled examples in examples/extensions/.").weak());
    }

    let mut to_uninstall: Option<String> = None;
    let mut to_load: Option<String> = None;

    for ext in &installed {
        egui::Frame::none()
            .fill(ui.visuals().faint_bg_color)
            .rounding(egui::Rounding::same(8.0))
            .inner_margin(egui::Margin::same(12.0))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label(egui::RichText::new(&ext.manifest.name).strong());
                        ui.label(egui::RichText::new(format!("v{}  by {}", ext.manifest.version, ext.manifest.author)).weak().size(11.0));
                        ui.label(egui::RichText::new(&ext.manifest.description).size(12.0));

                        let mut perms = Vec::new();
                        if ext.manifest.permissions.network {
                            perms.push("network");
                        }
                        if ext.manifest.permissions.filesystem {
                            perms.push("filesystem");
                        }
                        if ext.manifest.permissions.process_spawn {
                            perms.push("process spawn");
                        }
                        if !perms.is_empty() {
                            ui.label(egui::RichText::new(format!("Permissions: {}", perms.join(", "))).weak().size(10.0));
                        }
                    });
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("Uninstall").clicked() {
                            to_uninstall = Some(ext.manifest.id.clone());
                        }
                        if ui.button("Load").clicked() {
                            to_load = Some(ext.manifest.id.clone());
                        }
                    });
                });
            });
        ui.add_space(6.0);
    }

    if let Some(id) = to_uninstall {
        let _ = app.extension_manager.uninstall(&id);
    }
    if to_load.is_some() {
        app.extension_manager.load_all_enabled();
    }

    if !app.extension_manager.registered_commands.is_empty() {
        ui.add_space(16.0);
        ui.separator();
        ui.add_space(10.0);
        ui.label(egui::RichText::new("Registered Commands").strong());
        let commands = app.extension_manager.registered_commands.clone();
        for (ext_id, command_id, title) in &commands {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(title).monospace());
                ui.label(egui::RichText::new(format!("({ext_id})")).weak().size(10.0));
                if ui.small_button("Run").clicked() {
                    app.extension_manager.run_command(command_id);
                }
            });
        }
    }

    if !app.extension_manager.log.is_empty() {
        ui.add_space(16.0);
        ui.separator();
        ui.label(egui::RichText::new("Extension Log").strong());
        egui::ScrollArea::vertical().max_height(160.0).show(ui, |ui| {
            for line in &app.extension_manager.log {
                ui.label(egui::RichText::new(line).monospace().size(11.0).weak());
            }
        });
    }
}
