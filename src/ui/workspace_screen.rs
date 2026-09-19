use crate::app::{AppScreen, FasdeqApp, WorkspaceTab};
use crate::icons;
use eframe::egui;

pub fn draw(app: &mut FasdeqApp, ctx: &egui::Context) {
    app.apply_theme(ctx);

    egui::TopBottomPanel::top("workspace_top").show(ctx, |ui| {
        ui.horizontal(|ui| {
            if ui.button(format!("{}  Projects", icons::ARROW_LEFT)).clicked() {
                app.screen = AppScreen::StartMenu;
            }
            ui.separator();

            if let Some(project) = &app.current_project {
                ui.label(egui::RichText::new(&project.name).strong());
            }

            ui.separator();

            ui.selectable_value(&mut app.workspace_tab, WorkspaceTab::Editor, format!("{}  Editor", icons::NOTE_PENCIL));
            ui.selectable_value(&mut app.workspace_tab, WorkspaceTab::Disassembler, format!("{}  Binary Inspector", icons::MAGNIFYING_GLASS));
            ui.selectable_value(&mut app.workspace_tab, WorkspaceTab::InstructionReference, format!("{}  Reference", icons::BOOK_OPEN_TEXT));
            ui.selectable_value(&mut app.workspace_tab, WorkspaceTab::NumberBaseConverter, format!("{}  Base Converter", icons::HASH));
            ui.selectable_value(&mut app.workspace_tab, WorkspaceTab::BuildOutput, format!("{}  Build", icons::WRENCH));

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button(format!("{}  Settings", icons::GEAR)).clicked() {
                    app.show_settings = true;
                }
                if ui.button(format!("{}  Build", icons::PLAY)).clicked() {
                    app.run_build();
                }
                if app.keymap == crate::editor::keybindings::KeymapStyle::Vim {
                    ui.separator();
                    ui.label(egui::RichText::new(app.vim_state.mode_label()).monospace().strong());
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
        WorkspaceTab::NumberBaseConverter => crate::ui::number_base_view::draw(app, ui),
    });

    crate::ui::settings_screen::draw(app, ctx);
}
