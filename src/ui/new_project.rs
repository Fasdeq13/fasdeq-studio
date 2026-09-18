use crate::app::{AppScreen, FasdeqApp};
use crate::project::templates::ProjectTemplate;
use crate::project::OpenProject;
use eframe::egui;

pub fn draw(app: &mut FasdeqApp, ctx: &egui::Context) {
    app.apply_theme(ctx);

    egui::TopBottomPanel::top("new_project_top").show(ctx, |ui| {
        ui.horizontal(|ui| {
            if ui.button("← Back").clicked() {
                app.screen = AppScreen::StartMenu;
            }
            ui.heading("Create New Project");
        });
    });

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.add_space(10.0);
        ui.label(egui::RichText::new("Choose a template").size(16.0).strong());
        ui.add_space(10.0);

        egui::ScrollArea::vertical().max_height(360.0).show(ui, |ui| {
            for template in ProjectTemplate::all() {
                let selected = app.selected_template.as_ref() == Some(&template);
                egui::Frame::none()
                    .fill(if selected {
                        ui.visuals().selection.bg_fill
                    } else {
                        ui.visuals().faint_bg_color
                    })
                    .rounding(egui::Rounding::same(8.0))
                    .inner_margin(egui::Margin::same(14.0))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new(template.icon()).size(28.0));
                            ui.vertical(|ui| {
                                ui.label(egui::RichText::new(template.title()).strong());
                                ui.label(egui::RichText::new(template.description()).weak().size(12.0));
                            });
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if ui.button("Select").clicked() {
                                    app.selected_template = Some(template.clone());
                                }
                            });
                        });
                    });
                ui.add_space(8.0);
            }
        });

        ui.add_space(20.0);
        ui.separator();
        ui.add_space(10.0);

        if let Some(template) = app.selected_template.clone() {
            ui.label(egui::RichText::new(format!("Selected: {}", template.title())).strong());
            ui.add_space(8.0);

            ui.horizontal(|ui| {
                ui.label("Project name:");
                ui.text_edit_singleline(&mut app.new_project_name);
            });

            ui.horizontal(|ui| {
                ui.label("Location:");
                ui.label(egui::RichText::new(app.new_project_location.to_string_lossy()).weak());
                if ui.button("Browse...").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                        app.new_project_location = path;
                    }
                }
            });

            ui.add_space(15.0);
            let can_create = !app.new_project_name.trim().is_empty();
            if ui
                .add_enabled(can_create, egui::Button::new(egui::RichText::new("Create Project").size(15.0)))
                .clicked()
            {
                let project_root = app.new_project_location.join(app.new_project_name.trim());
                if template.create_at(&project_root, app.new_project_name.trim()).is_ok() {
                    app.open_project(OpenProject::from_path(project_root));
                }
            }
        } else {
            ui.label(egui::RichText::new("Select a template above to continue.").weak());
        }
    });
}
