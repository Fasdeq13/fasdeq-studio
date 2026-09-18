use crate::app::FasdeqApp;
use crate::disasm::reference::{self, all_mnemonics};
use eframe::egui;

pub fn draw(app: &mut FasdeqApp, ui: &mut egui::Ui) {
    ui.label(egui::RichText::new("Assembly Instruction Reference").size(16.0).strong());
    ui.add_space(8.0);
    ui.text_edit_singleline(&mut app.reference_search);
    ui.add_space(10.0);

    let query = app.reference_search.trim().to_lowercase();
    let mnemonics = all_mnemonics();

    egui::ScrollArea::vertical().show(ui, |ui| {
        for mnemonic in mnemonics {
            if !query.is_empty() && !mnemonic.contains(query.as_str()) {
                continue;
            }
            if let Some(info) = reference::lookup(mnemonic) {
                egui::CollapsingHeader::new(
                    egui::RichText::new(format!("{}  —  {}", info.mnemonic, info.summary)).monospace(),
                )
                .id_source(mnemonic)
                .show(ui, |ui| {
                    ui.label(info.description);
                    ui.add_space(4.0);
                    ui.label(egui::RichText::new(format!("Flags affected: {}", info.flags_affected)).weak());
                });
            }
        }
    });
}
