use crate::app::{AppScreen, FasdeqApp};
use crate::project::{clone_repository, OpenProject};
use eframe::egui;

pub fn draw(app: &mut FasdeqApp, ctx: &egui::Context) {
    app.apply_theme(ctx);

    egui::TopBottomPanel::top("clone_top").show(ctx, |ui| {
        ui.horizontal(|ui| {
            if ui.button("← Back").clicked() {
                app.screen = AppScreen::StartMenu;
            }
            ui.heading("Clone Repository");
        });
    });

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.add_space(20.0);
        ui.label("Repository URL:");
        ui.text_edit_singleline(&mut app.clone_url);

        ui.add_space(12.0);
        ui.horizontal(|ui| {
            ui.label("Destination:");
            ui.label(egui::RichText::new(app.clone_destination.to_string_lossy()).weak());
            if ui.button("Browse...").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    app.clone_destination = path;
                }
            }
        });

        ui.add_space(20.0);
        let can_clone = !app.clone_url.trim().is_empty();
        if ui
            .add_enabled(can_clone, egui::Button::new(egui::RichText::new("Clone").size(15.0)))
            .clicked()
        {
            let repo_name = app
                .clone_url
                .trim()
                .rsplit('/')
                .next()
                .unwrap_or("repository")
                .trim_end_matches(".git")
                .to_string();
            let destination = app.clone_destination.join(&repo_name);
            match clone_repository(app.clone_url.trim(), &destination) {
                Ok(()) => {
                    app.open_project(OpenProject::from_path(destination));
                }
                Err(e) => {
                    app.clone_error = Some(e.to_string());
                }
            }
        }

        if let Some(err) = &app.clone_error {
            ui.add_space(10.0);
            ui.colored_label(egui::Color32::from_rgb(230, 100, 100), err);
        }
    });
}
