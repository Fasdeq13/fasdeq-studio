use crate::app::FasdeqApp;
use crate::editor::diagnostics::Severity;
use eframe::egui;

pub fn draw(app: &mut FasdeqApp, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("Build Output").size(16.0).strong());
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button("▶ Build").clicked() {
                app.run_build();
            }
        });
    });

    ui.add_space(8.0);

    if app.diagnostics.is_empty() && app.build_stderr.is_empty() && app.build_stdout.is_empty() {
        ui.label(egui::RichText::new("No build has run yet. Click Build to compile the project.").weak());
        return;
    }

    if !app.diagnostics.is_empty() {
        ui.label(egui::RichText::new("Diagnostics").strong());
        egui::ScrollArea::vertical().max_height(240.0).id_source("diag_scroll").show(ui, |ui| {
            for diag in &app.diagnostics {
                let (icon, color) = match diag.severity {
                    Severity::Error => ("✖", egui::Color32::from_rgb(240, 95, 92)),
                    Severity::Warning => ("⚠", egui::Color32::from_rgb(240, 180, 90)),
                    Severity::Note => ("ℹ", egui::Color32::from_rgb(150, 160, 180)),
                };
                ui.horizontal(|ui| {
                    ui.colored_label(color, icon);
                    ui.label(egui::RichText::new(format!(
                        "{}:{}:{}",
                        diag.file.to_string_lossy(),
                        diag.line,
                        diag.column
                    )).monospace().weak());
                });
                ui.label(&diag.message);
                ui.add_space(4.0);
            }
        });
        ui.add_space(10.0);
        ui.separator();
    }

    ui.add_space(8.0);
    ui.label(egui::RichText::new("Raw Output").strong());
    egui::ScrollArea::vertical().max_height(260.0).id_source("raw_scroll").show(ui, |ui| {
        if !app.build_stdout.is_empty() {
            ui.label(egui::RichText::new(&app.build_stdout).monospace().size(11.0));
        }
        if !app.build_stderr.is_empty() {
            ui.label(egui::RichText::new(&app.build_stderr).monospace().size(11.0).color(egui::Color32::from_rgb(230, 160, 160)));
        }
    });
}
