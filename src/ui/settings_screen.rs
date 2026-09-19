use crate::app::FasdeqApp;
use crate::editor::keybindings::KeymapStyle;
use crate::theme::FasdeqTheme;
use eframe::egui;

pub fn draw(app: &mut FasdeqApp, ctx: &egui::Context) {
    if !app.show_settings {
        return;
    }

    let mut open = app.show_settings;
    egui::Window::new("Settings")
        .open(&mut open)
        .resizable(true)
        .default_width(420.0)
        .show(ctx, |ui| {
            ui.label(egui::RichText::new("Appearance").strong());
            ui.add_space(6.0);
            let mut changed = false;
            ui.horizontal_wrapped(|ui| {
                let themes = app.available_themes.clone();
                for theme in themes {
                    let selected = app.theme.name == theme.name;
                    if ui.selectable_label(selected, &theme.name).clicked() {
                        app.theme = theme;
                        app.custom_theme_path = None;
                        changed = true;
                    }
                }
            });

            ui.add_space(8.0);
            if ui.button("Load a custom CSS theme").clicked() {
                if let Some(path) = rfd::FileDialog::new().add_filter("theme", &["css", "toml"]).pick_file() {
                    match FasdeqTheme::load_from_file(&path) {
                        Ok(theme) => {
                            app.theme = theme;
                            app.custom_theme_path = Some(path);
                            app.theme_import_error = None;
                            changed = true;
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

            ui.add_space(16.0);
            ui.separator();
            ui.add_space(10.0);

            ui.label(egui::RichText::new("Editor").strong());
            ui.add_space(6.0);
            ui.horizontal(|ui| {
                ui.label("Font size:");
                if ui.add(egui::Slider::new(&mut app.font_size, 10.0..=24.0)).changed() {
                    changed = true;
                }
            });

            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.label("Keybindings:");
                if ui.selectable_label(app.keymap == KeymapStyle::VsCode, "VSCode").clicked() {
                    app.keymap = KeymapStyle::VsCode;
                    changed = true;
                }
                if ui.selectable_label(app.keymap == KeymapStyle::Vim, "Vim").clicked() {
                    app.keymap = KeymapStyle::Vim;
                    changed = true;
                }
            });

            if changed {
                app.save_preferences();
            }

            ui.add_space(16.0);
            ui.separator();
            ui.add_space(10.0);

            ui.label(egui::RichText::new("Toolchain").strong());
            ui.add_space(6.0);
            if ui.button("Re-run first-run setup wizard").clicked() {
                app.toolchain_config.setup_completed = false;
                let _ = app.toolchain_config.save();
                app.screen = crate::app::AppScreen::FirstRunWizard;
                app.wizard = crate::wizard::WizardState::new();
            }
        });

    app.show_settings = open;
}
