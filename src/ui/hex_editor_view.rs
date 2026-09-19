use crate::app::FasdeqApp;
use crate::disasm::binfile::{BinaryInfo, SectionCategory};
use crate::disasm::hexview::{classify_byte, ByteKind};
use crate::icons;
use eframe::egui;

const BYTES_PER_ROW: usize = 16;

fn section_category_at(info: &Option<BinaryInfo>, offset: usize) -> Option<SectionCategory> {
    let info = info.as_ref()?;
    info.sections
        .iter()
        .find(|s| {
            let start = s.file_offset as usize;
            let end = start + s.size as usize;
            offset >= start && offset < end
        })
        .map(|s| s.category())
}

fn color_for_byte(category: Option<SectionCategory>, kind: ByteKind, visuals: &egui::Visuals) -> egui::Color32 {
    if let Some(category) = category {
        match category {
            SectionCategory::Code => return egui::Color32::from_rgb(206, 150, 255),
            SectionCategory::ReadOnlyData => return egui::Color32::from_rgb(120, 200, 255),
            SectionCategory::Data => return egui::Color32::from_rgb(140, 220, 150),
            SectionCategory::Uninitialized => return egui::Color32::from_rgb(160, 160, 170),
            SectionCategory::Debug => return egui::Color32::from_rgb(150, 150, 190),
            SectionCategory::Linking => return egui::Color32::from_rgb(240, 190, 120),
            SectionCategory::Other => {}
        }
    }
    match kind {
        ByteKind::Null => egui::Color32::from_rgb(90, 92, 105),
        ByteKind::PrintableAscii => egui::Color32::from_rgb(140, 220, 150),
        ByteKind::Whitespace => egui::Color32::from_rgb(120, 130, 150),
        ByteKind::ControlOrHigh => visuals.text_color(),
    }
}

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
                .add_enabled(has_changes, egui::Button::new(format!("{}  Save In Place", icons::FLOPPY_DISK)))
                .clicked()
            {
                app.save_patched_binary_in_place();
            }
            if ui
                .add_enabled(has_changes, egui::Button::new(format!("{}  Save As Copy", icons::UPLOAD_SIMPLE)))
                .clicked()
            {
                app.save_patched_binary_as_copy();
            }
            if ui
                .add_enabled(has_changes, egui::Button::new(format!("{}  Revert All", icons::ARROW_COUNTER_CLOCKWISE)))
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

    ui.add_space(6.0);
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("Legend:").weak().size(11.0));
        legend_dot(ui, egui::Color32::from_rgb(206, 150, 255), "code");
        legend_dot(ui, egui::Color32::from_rgb(120, 200, 255), "rodata");
        legend_dot(ui, egui::Color32::from_rgb(140, 220, 150), "data");
        legend_dot(ui, egui::Color32::from_rgb(160, 160, 170), "bss");
        legend_dot(ui, egui::Color32::from_rgb(240, 190, 120), "linking");
        legend_dot(ui, egui::Color32::from_rgb(240, 180, 90), "patched");
    });

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

                    ui.add_space(16.0);
                    ui.separator();
                    ui.add_space(8.0);
                    crate::ui::number_base_view::draw_compact(ui, current as u64, 8);
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
                            if ui.small_button(icons::ARROW_COUNTER_CLOCKWISE).clicked() {
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
    let binary_info = app.disasm_panel.binary_info.clone();
    let visuals = ui.visuals().clone();

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
                            let category = section_category_at(&binary_info, offset);
                            let kind = classify_byte(*byte);
                            color_for_byte(category, kind, &visuals)
                        };
                        let text = egui::RichText::new(format!("{byte:02x}")).monospace().color(color);
                        if ui.selectable_label(is_selected, text).clicked() {
                            new_selection = Some(offset);
                        }
                    }
                    ui.add_space(12.0);
                    for (i, byte) in row_bytes.iter().enumerate() {
                        let offset = offset_base + i;
                        let ch = if *byte >= 0x20 && *byte < 0x7f { *byte as char } else { '.' };
                        let color = color_for_byte(
                            section_category_at(&binary_info, offset),
                            classify_byte(*byte),
                            &visuals,
                        );
                        ui.label(egui::RichText::new(ch.to_string()).monospace().color(color));
                    }
                });
            }
        },
    );

    if new_selection != selected_offset {
        app.disasm_panel.selected_byte_offset = new_selection;
        app.disasm_panel.byte_edit_input.clear();
    }
}

fn legend_dot(ui: &mut egui::Ui, color: egui::Color32, label: &str) {
    ui.horizontal(|ui| {
        let (rect, _) = ui.allocate_exact_size(egui::vec2(8.0, 8.0), egui::Sense::hover());
        ui.painter().rect_filled(rect, 2.0, color);
        ui.label(egui::RichText::new(label).size(10.0).weak());
    });
}
