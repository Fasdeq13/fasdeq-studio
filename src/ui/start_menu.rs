use crate::app::{AppScreen, FasdeqApp};
use crate::project::OpenProject;
use crate::icons;
use eframe::egui;
use std::path::PathBuf;

pub fn draw(app: &mut FasdeqApp, ctx: &egui::Context) {
    app.apply_theme(ctx);

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.add_space(60.0);
        ui.vertical_centered(|ui| {
            ui.heading(egui::RichText::new("Fasdeq Studio").size(44.0).strong());
            ui.label(
                egui::RichText::new("Low-level development environment for C, C++, Assembler and Rust")
                    .size(15.0)
                    .weak(),
            );
        });

        ui.add_space(50.0);

        ui.columns(2, |columns| {
            columns[0].vertical(|ui| {
                ui.add_space(10.0);
                ui.set_max_width(320.0);
                if big_button(ui, &format!("{}  New Project", icons::PLUS_CIRCLE)) {
                    app.screen = AppScreen::NewProjectMenu;
                    app.new_project_name.clear();
                    app.selected_template = None;
                }
                ui.add_space(12.0);
                if big_button(ui, &format!("{}  Open Project", icons::FOLDER_NOTCH_OPEN)) {
                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                        app.open_project(OpenProject::from_path(path));
                    }
                }
                ui.add_space(12.0);
                if big_button(ui, &format!("{}  Clone Repository", icons::GIT_BRANCH)) {
                    app.screen = AppScreen::CloneProject;
                    app.clone_url.clear();
                    app.clone_error = None;
                }
                ui.add_space(12.0);
                if big_button(ui, &format!("{}  Extensions", icons::PUZZLE_PIECE)) {
                    app.screen = AppScreen::ExtensionsMenu;
                }
                ui.add_space(12.0);
                if big_button(ui, &format!("{}  Exit", icons::SIGN_OUT)) {
                    std::process::exit(0);
                }

                let dropped: Vec<PathBuf> = ui.ctx().input(|i| {
                    i.raw.dropped_files
                        .iter()
                        .filter_map(|f| f.path.clone())
                        .collect()
                });
                if let Some(path) = dropped.into_iter().next() {
                    if path.is_dir() {
                        app.open_project(OpenProject::from_path(path));
                    }
                }

                ui.add_space(20.0);
                ui.label(egui::RichText::new("Tip: drag and drop a project folder anywhere on this window to open it.").weak().size(12.0));
            });

            columns[1].vertical(|ui| {
                ui.label(egui::RichText::new("Recent Projects").size(18.0).strong());
                ui.add_space(10.0);
                egui::ScrollArea::vertical().max_height(420.0).show(ui, |ui| {
                    let entries = app.recent_projects.entries.clone();
                    if entries.is_empty() {
                        ui.label(egui::RichText::new("No recent projects yet.").weak());
                    }
                    for entry in entries {
                        egui::Frame::none()
                            .fill(ui.visuals().faint_bg_color)
                            .rounding(egui::Rounding::same(8.0))
                            .inner_margin(egui::Margin::same(12.0))
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    ui.vertical(|ui| {
                                        ui.label(egui::RichText::new(&entry.name).strong());
                                        ui.label(egui::RichText::new(entry.path.to_string_lossy()).weak().size(11.0));
                                        ui.label(egui::RichText::new(format!("Last opened: {}", entry.last_opened)).weak().size(10.0));
                                    });
                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                        if ui.small_button(icons::X).clicked() {
                                            app.recent_projects.remove(&entry.path);
                                        }
                                        if ui.button("Open").clicked() {
                                            app.open_project(OpenProject::from_path(entry.path.clone()));
                                        }
                                    });
                                });
                            });
                        ui.add_space(6.0);
                    }
                });
            });
        });
    });
}

fn big_button(ui: &mut egui::Ui, label: &str) -> bool {
    ui.add_sized(
        [ui.available_width(), 46.0],
        egui::Button::new(egui::RichText::new(label).size(16.0)),
    )
    .clicked()
}
