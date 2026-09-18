use crate::app::FasdeqApp;
use eframe::egui;
use std::path::PathBuf;

pub fn draw(app: &mut FasdeqApp, ui: &mut egui::Ui) {
    ui.label(egui::RichText::new("Explorer").size(14.0).strong());
    ui.add_space(6.0);

    if let Some(project) = &app.current_project {
        ui.label(egui::RichText::new(&project.name).weak().size(12.0));
    }

    ui.add_space(6.0);
    ui.separator();

    let files = app.file_tree_files.clone();
    let root = app.file_tree_root.clone();

    egui::ScrollArea::vertical().show(ui, |ui| {
        for file in &files {
            let display_name = root
                .as_ref()
                .and_then(|r| file.strip_prefix(r).ok())
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|| file.to_string_lossy().to_string());

            let icon = icon_for_file(file);
            if ui
                .selectable_label(false, format!("{icon}  {display_name}"))
                .clicked()
            {
                app.open_file_in_editor(file.clone());
            }
        }
    });
}

fn icon_for_file(path: &PathBuf) -> &'static str {
    match path.extension().and_then(|e| e.to_str()).unwrap_or("") {
        "rs" => "🦀",
        "c" | "h" => "🔧",
        "cpp" | "cc" | "hpp" => "🔩",
        "asm" | "s" => "🧮",
        "toml" => "⚙",
        "md" => "📝",
        "json" => "🗂",
        _ => "📄",
    }
}
