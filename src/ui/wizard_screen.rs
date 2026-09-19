use crate::app::{AppScreen, FasdeqApp};
use crate::editor::keybindings::KeymapStyle;
use crate::theme::FasdeqTheme;
use crate::icons;
use crate::wizard::WizardStage;
use eframe::egui;

pub fn draw(app: &mut FasdeqApp, ctx: &egui::Context) {
    if app.wizard.stage == WizardStage::Installing {
        app.wizard.poll_install();
        ctx.request_repaint();
    }

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(40.0);
            ui.heading(egui::RichText::new("Fasdeq Studio").size(36.0).strong());
            ui.label(
                egui::RichText::new("First-run setup wizard")
                    .size(16.0)
                    .weak(),
            );
            ui.add_space(30.0);
        });

        egui::Frame::none()
            .inner_margin(egui::Margin::same(24.0))
            .show(ui, |ui| match app.wizard.stage {
                WizardStage::Welcome => draw_welcome(app, ui),
                WizardStage::Scanning => draw_scanning(app, ui),
                WizardStage::ToolSummary => draw_tool_summary(app, ui),
                WizardStage::Installing => draw_installing(app, ui),
                WizardStage::ManualPaths => draw_manual_paths(app, ui),
                WizardStage::ThemeChoice => draw_theme_choice(app, ui),
                WizardStage::KeymapChoice => draw_keymap_choice(app, ui),
                WizardStage::Done => draw_done(app, ui),
            });
    });
}

fn draw_welcome(app: &mut FasdeqApp, ui: &mut egui::Ui) {
    ui.vertical_centered(|ui| {
        ui.label(format!("Detected distribution: {}", app.wizard.distro.name));
        ui.label(format!(
            "Package manager: {}",
            app.wizard.distro.package_manager.display_name()
        ));
        ui.add_space(20.0);
        ui.label("Fasdeq Studio will now check for the development tools it needs: Rust, GCC, G++, GDB, NASM, binutils, QEMU, CMake and Git.");
        ui.add_space(20.0);
        if ui.button(egui::RichText::new("Start check").size(16.0)).clicked() {
            app.wizard.stage = WizardStage::Scanning;
        }
    });
}

fn draw_scanning(app: &mut FasdeqApp, ui: &mut egui::Ui) {
    app.wizard.scan();
    ui.vertical_centered(|ui| {
        ui.spinner();
        ui.label("Checking installed tools...");
    });
}

fn draw_tool_summary(app: &mut FasdeqApp, ui: &mut egui::Ui) {
    ui.label(egui::RichText::new("Check results").size(18.0).strong());
    ui.add_space(10.0);
    egui::Grid::new("tool_status_grid")
        .num_columns(3)
        .spacing([20.0, 8.0])
        .show(ui, |ui| {
            for tool in &app.wizard.tools {
                let icon = if tool.found { icons::CHECK_CIRCLE } else { icons::X_CIRCLE };
                ui.label(icon);
                ui.label(tool.kind.display_name());
                if let Some(version) = &tool.version {
                    ui.label(egui::RichText::new(version).weak());
                } else {
                    ui.label(egui::RichText::new("not found").color(egui::Color32::from_rgb(230, 100, 100)));
                }
                ui.end_row();
            }
        });

    ui.add_space(20.0);
    if app.wizard.missing_tools.is_empty() {
        ui.label("All required tools are already installed.");
        if ui.button("Continue").clicked() {
            app.wizard.stage = WizardStage::ThemeChoice;
        }
    } else {
        ui.label(format!(
            "{} tool(s) missing. Install them automatically from the official {} repository?",
            app.wizard.missing_tools.len(),
            app.wizard.distro.package_manager.display_name()
        ));
        ui.horizontal(|ui| {
            if ui.button(egui::RichText::new("Install automatically").size(15.0)).clicked() {
                app.wizard.start_install();
            }
            if ui.button("Specify paths manually").clicked() {
                app.wizard.switch_to_manual();
            }
        });
    }
}

fn draw_installing(app: &mut FasdeqApp, ui: &mut egui::Ui) {
    ui.label(egui::RichText::new("Installing tools...").size(18.0).strong());
    ui.add_space(10.0);
    egui::ScrollArea::vertical()
        .max_height(320.0)
        .stick_to_bottom(true)
        .show(ui, |ui| {
            for line in &app.wizard.install_log {
                ui.label(egui::RichText::new(line).monospace().size(12.0));
            }
        });
    ui.add_space(10.0);
    if let Some(err) = app.wizard.install_failed.clone() {
        ui.colored_label(egui::Color32::from_rgb(230, 100, 100), err);
        if ui.button("Specify paths manually").clicked() {
            app.wizard.switch_to_manual();
        }
    } else {
        ui.spinner();
    }
}

fn draw_manual_paths(app: &mut FasdeqApp, ui: &mut egui::Ui) {
    let total = app.wizard.missing_tools.len();
    let current_index = app.wizard.manual_path_index;

    if current_index >= total {
        app.wizard.stage = WizardStage::ThemeChoice;
        return;
    }

    let tool_name = app.wizard.missing_tools[current_index].display_name();
    ui.label(egui::RichText::new(format!(
        "Specify the path to the binary: {tool_name} ({}/{})",
        current_index + 1,
        total
    )).size(17.0).strong());
    ui.add_space(10.0);

    ui.horizontal(|ui| {
        ui.text_edit_singleline(&mut app.wizard.manual_path_input);
        if ui.button("Browse...").clicked() {
            if let Some(path) = rfd::FileDialog::new().pick_file() {
                app.wizard.manual_path_input = path.to_string_lossy().to_string();
            }
        }
    });

    ui.add_space(15.0);
    ui.horizontal(|ui| {
        if ui.button("Confirm").clicked() {
            app.wizard.confirm_manual_path(&mut app.toolchain_config);
        }
        if ui.button("Skip this tool").clicked() {
            app.wizard.manual_path_index += 1;
        }
    });
}

fn draw_theme_choice(app: &mut FasdeqApp, ui: &mut egui::Ui) {
    ui.label(egui::RichText::new("Choose an appearance theme").size(18.0).strong());
    ui.add_space(15.0);

    ui.horizontal_wrapped(|ui| {
        if ui.selectable_label(app.wizard.chosen_theme.is_dark, "🌙 Dark").clicked() {
            app.wizard.chosen_theme = FasdeqTheme::dark_default();
        }
        if ui.selectable_label(!app.wizard.chosen_theme.is_dark, "☀ Light").clicked() {
            app.wizard.chosen_theme = FasdeqTheme::light_default();
        }
    });

    ui.add_space(10.0);
    if ui.button("Load a custom CSS theme").clicked() {
        if let Some(path) = rfd::FileDialog::new().add_filter("theme", &["css", "toml"]).pick_file() {
            match FasdeqTheme::load_from_file(&path) {
                Ok(theme) => {
                    app.wizard.chosen_theme = theme;
                    app.wizard.custom_theme_path = Some(path);
                    app.theme_import_error = None;
                }
                Err(e) => {
                    app.theme_import_error = Some(e.to_string());
                }
            }
        }
    }
    if let Some(err) = &app.theme_import_error {
        ui.colored_label(egui::Color32::from_rgb(230, 100, 100), err);
    }

    app.wizard.chosen_theme.apply(ui.ctx());

    ui.add_space(20.0);
    if ui.button(egui::RichText::new("Continue").size(16.0)).clicked() {
        app.wizard.stage = WizardStage::KeymapChoice;
    }
}

fn draw_keymap_choice(app: &mut FasdeqApp, ui: &mut egui::Ui) {
    ui.label(egui::RichText::new("Choose your editor keybinding style").size(18.0).strong());
    ui.add_space(15.0);

    ui.horizontal(|ui| {
        if ui.selectable_label(app.wizard.chosen_keymap == KeymapStyle::VsCode, "VSCode").clicked() {
            app.wizard.chosen_keymap = KeymapStyle::VsCode;
        }
        if ui.selectable_label(app.wizard.chosen_keymap == KeymapStyle::Vim, "Vim").clicked() {
            app.wizard.chosen_keymap = KeymapStyle::Vim;
        }
    });

    ui.add_space(20.0);
    if ui.button(egui::RichText::new("Finish setup").size(16.0)).clicked() {
        app.theme = app.wizard.chosen_theme.clone();
        app.keymap = app.wizard.chosen_keymap;
        app.custom_theme_path = app.wizard.custom_theme_path.clone();
        app.toolchain_config.setup_completed = true;
        let _ = app.toolchain_config.save();
        app.save_preferences();
        app.wizard.stage = WizardStage::Done;
    }
}

fn draw_done(app: &mut FasdeqApp, ui: &mut egui::Ui) {
    ui.vertical_centered(|ui| {
        ui.label(egui::RichText::new("Setup complete").size(20.0).strong());
        ui.add_space(10.0);
        if ui.button(egui::RichText::new("Open Fasdeq Studio").size(16.0)).clicked() {
            app.screen = AppScreen::StartMenu;
        }
    });
}
