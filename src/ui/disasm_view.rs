use crate::app::FasdeqApp;
use crate::disasm::engine::Architecture;
use crate::disasm::reference;
use crate::ui::disasm_panel::BinaryViewMode;
use eframe::egui;

pub fn draw(app: &mut FasdeqApp, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("Binary Inspector").size(16.0).strong());
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button("📂 Open Binary").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    app.load_binary_for_disasm(path);
                }
            }
            ui.selectable_value(&mut app.disasm_panel.view_mode, BinaryViewMode::HexEditor, "Hex Editor");
            ui.selectable_value(&mut app.disasm_panel.view_mode, BinaryViewMode::Disassembly, "Disassembly");
        });
    });

    ui.add_space(8.0);

    let Some(file_path) = app.disasm_panel.file_path.clone() else {
        ui.centered_and_justified(|ui| {
            ui.label(egui::RichText::new("Open a binary file to disassemble or patch it.").weak());
        });
        return;
    };

    if app.disasm_panel.view_mode == BinaryViewMode::HexEditor {
        crate::ui::hex_editor_view::draw(app, ui);
        return;
    }

    ui.label(egui::RichText::new(file_path.to_string_lossy()).weak().size(12.0));

    if let Some(info) = &app.disasm_panel.binary_info {
        ui.add_space(6.0);
        ui.label(format!(
            "Format: {}  |  Architecture: {}  |  Entry point: 0x{:x}  |  {}-bit",
            info.format,
            info.architecture,
            info.entry_point,
            if info.is_64 { 64 } else { 32 }
        ));

        egui::CollapsingHeader::new("Sections").default_open(false).show(ui, |ui| {
            egui::Grid::new("sections_grid").num_columns(5).striped(true).show(ui, |ui| {
                ui.label(egui::RichText::new("Name").strong());
                ui.label(egui::RichText::new("Address").strong());
                ui.label(egui::RichText::new("Size").strong());
                ui.label(egui::RichText::new("File offset").strong());
                ui.label(egui::RichText::new("Purpose").strong());
                ui.end_row();
                for section in &info.sections {
                    ui.label(&section.name);
                    ui.label(format!("0x{:x}", section.address));
                    ui.label(format!("{} bytes", section.size));
                    ui.label(format!("0x{:x}", section.file_offset));
                    ui.label(egui::RichText::new(section.purpose()).weak().size(11.0));
                    ui.end_row();
                }
            });
        });
    }

    ui.add_space(10.0);
    ui.horizontal(|ui| {
        ui.label("Architecture:");
        egui::ComboBox::from_id_source("arch_selector")
            .selected_text(app.disasm_panel.architecture.label())
            .show_ui(ui, |ui| {
                for arch in Architecture::all() {
                    ui.selectable_value(&mut app.disasm_panel.architecture, arch, arch.label());
                }
            });
        if ui.button("Disassemble").clicked() {
            app.run_disassembly();
        }
    });

    if let Some(err) = &app.disasm_panel.error {
        ui.colored_label(egui::Color32::from_rgb(230, 100, 100), err);
    }

    ui.add_space(10.0);
    ui.separator();

    let instructions = app.disasm_panel.instructions.clone();
    let mut selected = app.disasm_panel.selected_instruction;

    egui::SidePanel::right("instruction_detail")
        .resizable(true)
        .default_width(320.0)
        .show_inside(ui, |ui| {
            ui.label(egui::RichText::new("Instruction Reference").strong());
            ui.add_space(6.0);
            if let Some(index) = selected {
                if let Some(instr) = instructions.get(index) {
                    ui.label(egui::RichText::new(&instr.mnemonic).monospace().size(18.0));
                    if let Some(info) = reference::lookup(&instr.mnemonic) {
                        ui.add_space(6.0);
                        ui.label(egui::RichText::new(info.summary).strong());
                        ui.add_space(4.0);
                        ui.label(info.description);
                        ui.add_space(6.0);
                        ui.label(format!("Flags affected: {}", info.flags_affected));
                    } else {
                        ui.label(egui::RichText::new("No reference entry for this mnemonic yet.").weak());
                    }
                }
            } else {
                ui.label(egui::RichText::new("Select an instruction on the left to see details.").weak());
            }
        });

    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("Address").strong());
        ui.add_space(70.0);
        ui.label(egui::RichText::new("Bytes").strong());
        ui.add_space(90.0);
        ui.label(egui::RichText::new("Mnemonic").strong());
        ui.add_space(60.0);
        ui.label(egui::RichText::new("Operands").strong());
    });
    ui.separator();

    egui::ScrollArea::vertical().auto_shrink([false, false]).show_rows(
        ui,
        ui.text_style_height(&egui::TextStyle::Monospace),
        instructions.len(),
        |ui, row_range| {
            egui::Grid::new("disasm_grid").num_columns(4).striped(true).show(ui, |ui| {
                for index in row_range {
                    let instr = &instructions[index];
                    let bytes_str: String = instr.bytes.iter().map(|b| format!("{b:02x} ")).collect();
                    let is_selected = selected == Some(index);
                    if ui.selectable_label(is_selected, format!("0x{:08x}", instr.address)).clicked() {
                        selected = Some(index);
                    }
                    ui.label(egui::RichText::new(bytes_str).monospace().size(11.0));
                    ui.label(egui::RichText::new(&instr.mnemonic).monospace());
                    ui.label(egui::RichText::new(&instr.operands).monospace());
                    ui.end_row();
                }
            });
        },
    );

    app.disasm_panel.selected_instruction = selected;
}
