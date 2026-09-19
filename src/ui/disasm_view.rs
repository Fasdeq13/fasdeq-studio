use crate::app::FasdeqApp;
use crate::disasm::engine::Architecture;
use crate::disasm::reference::{self, MnemonicCategory};
use crate::ui::disasm_panel::BinaryViewMode;
use crate::icons;
use eframe::egui;

fn color_for_category(category: MnemonicCategory) -> egui::Color32 {
    match category {
        MnemonicCategory::ControlFlow => egui::Color32::from_rgb(255, 140, 140),
        MnemonicCategory::Stack => egui::Color32::from_rgb(240, 190, 120),
        MnemonicCategory::Arithmetic => egui::Color32::from_rgb(140, 220, 150),
        MnemonicCategory::Logic => egui::Color32::from_rgb(120, 200, 255),
        MnemonicCategory::DataMove => egui::Color32::from_rgb(206, 150, 255),
        MnemonicCategory::System => egui::Color32::from_rgb(255, 120, 200),
        MnemonicCategory::Other => egui::Color32::from_rgb(190, 190, 200),
    }
}

fn highlight_operands(ui: &egui::Ui, operands: &str) -> egui::text::LayoutJob {
    let mut job = egui::text::LayoutJob::default();
    let font_id = egui::FontId::monospace(ui.text_style_height(&egui::TextStyle::Monospace) * 0.72);
    let register_color = egui::Color32::from_rgb(120, 200, 255);
    let number_color = egui::Color32::from_rgb(240, 190, 120);
    let punctuation_color = ui.visuals().weak_text_color();
    let default_color = ui.visuals().text_color();

    let mut current = String::new();
    let mut flush = |job: &mut egui::text::LayoutJob, s: &mut String| {
        if s.is_empty() {
            return;
        }
        let color = if is_register(s) {
            register_color
        } else if is_numberish(s) {
            number_color
        } else {
            default_color
        };
        job.append(
            s,
            0.0,
            egui::TextFormat {
                font_id: font_id.clone(),
                color,
                ..Default::default()
            },
        );
        s.clear();
    };

    for ch in operands.chars() {
        if ch.is_alphanumeric() || ch == '_' {
            current.push(ch);
        } else {
            flush(&mut job, &mut current);
            job.append(
                &ch.to_string(),
                0.0,
                egui::TextFormat {
                    font_id: font_id.clone(),
                    color: punctuation_color,
                    ..Default::default()
                },
            );
        }
    }
    flush(&mut job, &mut current);
    job
}

fn is_register(token: &str) -> bool {
    const REGS: &[&str] = &[
        "rax", "rbx", "rcx", "rdx", "rsi", "rdi", "rbp", "rsp", "rip", "eax", "ebx", "ecx",
        "edx", "esi", "edi", "ebp", "esp", "ax", "bx", "cx", "dx", "si", "di", "bp", "sp",
        "al", "bl", "cl", "dl", "ah", "bh", "ch", "dh", "r8", "r9", "r10", "r11", "r12", "r13",
        "r14", "r15", "r8d", "r9d", "r10d", "r11d", "r12d", "r13d", "r14d", "r15d", "cs", "ds",
        "es", "fs", "gs", "ss", "eflags", "rflags", "xmm0", "xmm1", "xmm2", "xmm3", "xmm4",
        "xmm5", "xmm6", "xmm7",
    ];
    REGS.contains(&token.to_lowercase().as_str())
}

fn is_numberish(token: &str) -> bool {
    !token.is_empty()
        && token
            .chars()
            .all(|c| c.is_ascii_hexdigit() || c == 'x' || c == 'X')
        && token.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false)
}

pub fn draw(app: &mut FasdeqApp, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("Binary Inspector").size(16.0).strong());
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button(format!("{}  Open Binary", icons::FOLDER_NOTCH_OPEN)).clicked() {
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
        if ui.button(format!("{}  Disassemble", icons::PLAY)).clicked() {
            app.run_disassembly();
        }
    });

    if let Some(err) = &app.disasm_panel.error {
        ui.colored_label(egui::Color32::from_rgb(230, 100, 100), err);
    }

    ui.add_space(6.0);
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("Legend:").weak().size(11.0));
        legend_dot(ui, color_for_category(MnemonicCategory::ControlFlow), "control flow");
        legend_dot(ui, color_for_category(MnemonicCategory::Stack), "stack");
        legend_dot(ui, color_for_category(MnemonicCategory::Arithmetic), "arithmetic");
        legend_dot(ui, color_for_category(MnemonicCategory::Logic), "logic");
        legend_dot(ui, color_for_category(MnemonicCategory::DataMove), "data move");
        legend_dot(ui, color_for_category(MnemonicCategory::System), "system");
    });

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
                    let category = reference::categorize(&instr.mnemonic);
                    ui.label(
                        egui::RichText::new(&instr.mnemonic)
                            .monospace()
                            .size(18.0)
                            .color(color_for_category(category)),
                    );
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

                    ui.add_space(16.0);
                    ui.separator();
                    ui.add_space(8.0);
                    crate::ui::number_base_view::draw_compact(ui, instr.address, 8);
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
                    ui.label(egui::RichText::new(bytes_str).monospace().size(11.0).weak());

                    let category = reference::categorize(&instr.mnemonic);
                    ui.label(
                        egui::RichText::new(&instr.mnemonic)
                            .monospace()
                            .color(color_for_category(category)),
                    );

                    let job = highlight_operands(ui, &instr.operands);
                    ui.label(job);
                    ui.end_row();
                }
            });
        },
    );

    app.disasm_panel.selected_instruction = selected;
}

fn legend_dot(ui: &mut egui::Ui, color: egui::Color32, label: &str) {
    ui.horizontal(|ui| {
        let (rect, _) = ui.allocate_exact_size(egui::vec2(8.0, 8.0), egui::Sense::hover());
        ui.painter().rect_filled(rect, 2.0, color);
        ui.label(egui::RichText::new(label).size(10.0).weak());
    });
}
