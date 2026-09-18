use crate::app::{AppScreen, FasdeqApp, WorkspaceTab};
use eframe::egui;

pub fn draw(app: &mut FasdeqApp, ctx: &egui::Context) {
    app.apply_theme(ctx);

    egui::TopBottomPanel::top("workspace_top").show(ctx, |ui| {
        ui.horizontal(|ui| {
            if ui.button("← Projects").clicked() {
                app.screen = AppScreen::StartMenu;
            }
            ui.separator();

            if let Some(project) = &app.current_project {
                ui.label(egui::RichText::new(&project.name).strong());
            }

            ui.separator();

            ui.selectable_value(&mut app.workspace_tab, WorkspaceTab::Editor, "📝 Editor");
            ui.selectable_value(&mut app.workspace_tab, WorkspaceTab::Disassembler, "🔍 Binary Inspector");
            ui.selectable_value(&mut app.workspace_tab, WorkspaceTab::InstructionReference, "📖 Reference");
            ui.selectable_value(&mut app.workspace_tab, WorkspaceTab::BuildOutput, "🛠 Build");
            ui.selectable_value(&mut app.workspace_tab, WorkspaceTab::Extensions, "🧩 Extensions");

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("⚙ Settings").clicked() {
                    app.show_settings = true;
                }
                if ui.button("▶ Build").clicked() {
                    app.run_build();
                }
            });
        });
    });

    if app.workspace_tab == WorkspaceTab::Editor {
        egui::SidePanel::left("file_tree_panel")
            .resizable(true)
            .default_width(240.0)
            .show(ctx, |ui| {
                crate::ui::file_tree::draw(app, ui);
            });
    }

    egui::CentralPanel::default().show(ctx, |ui| match app.workspace_tab {
        WorkspaceTab::Editor => crate::ui::editor_panel::draw(app, ui),
        WorkspaceTab::Disassembler => crate::ui::disasm_view::draw(app, ui),
        WorkspaceTab::InstructionReference => crate::ui::reference_view::draw(app, ui),
        WorkspaceTab::BuildOutput => crate::ui::build_output_view::draw(app, ui),
        WorkspaceTab::Extensions => crate::ui::extensions_panel::draw(app, ui),
    });

    crate::ui::settings_screen::draw(app, ctx);
}
