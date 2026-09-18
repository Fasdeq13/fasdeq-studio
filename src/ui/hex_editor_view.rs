use crate::app::FasdeqApp;
use eframe::egui;

const BYTES_PER_ROW: usize = 16;

pub fn draw(app: &mut FasdeqApp, ui: &mut egui::Ui) {
    let Some(file_path) = app.disasm_panel.file_path.clone() else {
        ui.centered_and_justified(|ui| {
            ui.label(egui::RichText::new("Open a binary file to inspect and patch its bytes.").weak());
        });
        return;
    };

    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("Hex Editor").size(16.0).strong());
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let has_changes = app.disasm_panel.patches.has_changes();
            if ui
                .add_enabled(has_changes, egui::Button::new("💾 Save In Place"))
                .clicked()
            {
                app.save_patched_binary_in_place();
            }
            if ui
                .add_enabled(has_changes, egui::Button::new("📄 Save As Patched Copy"))
                .clicked()
            {
                app.save_patched_binary_as_copy();
            }
            if ui
                .add_enabled(has_changes, egui::Button::new("↩ Revert All"))
                .clicked()
            {
                app.revert_all_patches();
            }
        });
    });

    ui.label(egui::RichText::new(file_path.to_string_lossy()).weak().size(12.0));

    if app.disasm_panel.patches.has_changes() {
        ui.colored_label(
            egui::Color32::from_rgb(240, 180, 90),
            format!("{} unsaved byte patch(es)", app.disasm_panel.patches.patches.len()),
        );
    }

    if let Some(msg) = &app.disasm_panel.save_message {
        ui.label(egui::RichText::new(msg).weak());
    }

    ui.add_space(8.0);
    ui.separator();

    egui::SidePanel::right("byte_patch_panel")
        .resizable(true)
        .default_width(300.0)
        .show_inside(ui, |ui| {
            ui.label(egui::RichText::new("Byte Inspector").strong());
            ui.add_space(8.0);

            if let Some(offset) = app.disasm_panel.selected_byte_offset {
                if offset < app.disasm_panel.file_bytes.len() {
                    let current = app.disasm_panel.file_bytes[offset];
                    ui.label(format!("Offset: 0x{offset:08x} ({offset})"));
                    ui.label(format!("Current value: 0x{current:02x} ({current})"));
                    if let Some(info) = &app.disasm_panel.binary_info {
                        if let Some(section) = info
                            .sections
                            .iter()
                            .find(|s| {
                                let start = s.file_offset as usize;
                                let end = start + s.size as usize;
                                offset >= start && offset < end
                            })
                        {
                            ui.add_space(6.0);
                            ui.label(egui::RichText::new(format!("Section: {}", section.name)).strong());
                            ui.label(egui::RichText::new(section.purpose()).weak());
                        } else {
                            ui.add_space(6.0);
                            ui.label(egui::RichText::new("Not inside any known section (likely header or padding).").weak());
                        }
                    }

                    ui.add_space(12.0);
                    ui.label("New value (hex, e.g. 90):");
                    ui.text_edit_singleline(&mut app.disasm_panel.byte_edit_input);
                    ui.horizontal(|ui| {
                        if ui.button("Apply Patch").clicked() {
                            let cleaned = app
                                .disasm_panel
                                .byte_edit_input
                                .trim()
                                .trim_start_matches("0x")
                                .trim_start_matches("0X");
                            if let Ok(value) = u8::from_str_radix(cleaned, 16) {
                                app.apply_byte_patch(offset, value);
                            }
                        }
                        if app.disasm_panel.patches.is_patched(offset) && ui.button("Revert").clicked() {
                            app.revert_byte_patch(offset);
                        }
                    });

                    ui.add_space(6.0);
                    ui.label(egui::RichText::new("Quick patch: NOP (0x90)").weak());
                    if ui.button("Patch to NOP").clicked() {
                        app.apply_byte_patch(offset, 0x90);
                    }
                } else {
                    ui.label(egui::RichText::new("Select a byte to inspect it.").weak());
                }
            } else {
                ui.label(egui::RichText::new("Click any byte in the hex grid to inspect and patch it.").weak());
            }

            if app.disasm_panel.patches.has_changes() {
                ui.add_space(16.0);
                ui.separator();
                ui.label(egui::RichText::new("Pending Patches").strong());
                let patches = app.disasm_panel.patches.patches.clone();
                egui::ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
                    for patch in &patches {
                        ui.horizontal(|ui| {
                            ui.label(format!(
                                "0x{:08x}: {:02x} → {:02x}",
                                patch.offset, patch.original, patch.new_value
                            ));
                            if ui.small_button("↩").clicked() {
                                app.revert_byte_patch(patch.offset);
                            }
                        });
                    }
                });
            }
        });

    let total_bytes = app.disasm_panel.file_bytes.len();
    let total_rows = total_bytes.div_ceil(BYTES_PER_ROW).max(1);
    let selected_offset = app.disasm_panel.selected_byte_offset;
    let mut new_selection = selected_offset;

    let row_height = ui.text_style_height(&egui::TextStyle::Monospace);

    egui::ScrollArea::vertical().auto_shrink([false, false]).show_rows(
        ui,
        row_height,
        total_rows,
        |ui, row_range| {
            for row_index in row_range {
                let offset_base = row_index * BYTES_PER_ROW;
                let end = (offset_base + BYTES_PER_ROW).min(total_bytes);
                let row_bytes = &app.disasm_panel.file_bytes[offset_base..end];

                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new(format!("{offset_base:08x}"))
                            .monospace()
                            .weak(),
                    );
                    ui.add_space(8.0);
                    for (i, byte) in row_bytes.iter().enumerate() {
                        let offset = offset_base + i;
                        let is_patched = app.disasm_panel.patches.is_patched(offset);
                        let is_selected = selected_offset == Some(offset);
                        let color = if is_patched {
                            egui::Color32::from_rgb(240, 180, 90)
                        } else {
                            ui.visuals().text_color()
                        };
                        let text = egui::RichText::new(format!("{byte:02x}")).monospace().color(color);
                        if ui.selectable_label(is_selected, text).clicked() {
                            new_selection = Some(offset);
                        }
                    }
                    ui.add_space(12.0);
                    let ascii: String = row_bytes
                        .iter()
                        .map(|b| if *b >= 0x20 && *b < 0x7f { *b as char } else { '.' })
                        .collect();
                    ui.label(egui::RichText::new(ascii).monospace().weak());
                });
            }
        },
    );

    if new_selection != selected_offset {
        app.disasm_panel.selected_byte_offset = new_selection;
        app.disasm_panel.byte_edit_input.clear();
    }
}
