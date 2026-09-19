use crate::app::FasdeqApp;
use crate::editor::numbase::{format_value, parse_value, signed_interpretation, NumberBase};
use eframe::egui;

pub fn draw(app: &mut FasdeqApp, ui: &mut egui::Ui) {
    ui.label(egui::RichText::new("Number Base Converter").size(16.0).strong());
    ui.label(egui::RichText::new("Type a value into any field to convert it live across bases.").weak());
    ui.add_space(14.0);

    let mut changed_value: Option<u64> = None;

    egui::Grid::new("numbase_grid")
        .num_columns(2)
        .spacing([16.0, 12.0])
        .show(ui, |ui| {
            for base in [
                NumberBase::Binary,
                NumberBase::Octal,
                NumberBase::Decimal,
                NumberBase::Hexadecimal,
            ] {
                ui.label(egui::RichText::new(base.label()).strong());
                let field = match base {
                    NumberBase::Binary => &mut app.numbase_binary,
                    NumberBase::Octal => &mut app.numbase_octal,
                    NumberBase::Decimal => &mut app.numbase_decimal,
                    NumberBase::Hexadecimal => &mut app.numbase_hex,
                };
                let response = ui.add(
                    egui::TextEdit::singleline(field)
                        .font(egui::TextStyle::Monospace)
                        .desired_width(260.0),
                );
                if response.changed() {
                    if let Some(value) = parse_value(field, base) {
                        changed_value = Some(value);
                    }
                }
                ui.end_row();
            }
        });

    if let Some(value) = changed_value {
        app.set_numbase_value(value);
    }

    ui.add_space(20.0);
    ui.separator();
    ui.add_space(14.0);

    if let Some(value) = parse_value(&app.numbase_decimal, NumberBase::Decimal) {
        draw_details(ui, value);
    }
}

fn draw_details(ui: &mut egui::Ui, value: u64) {
    ui.label(egui::RichText::new("Bit Width Interpretations").strong());
    ui.add_space(8.0);
    egui::Grid::new("numbase_width_grid")
        .num_columns(4)
        .spacing([16.0, 8.0])
        .show(ui, |ui| {
            ui.label(egui::RichText::new("Width").strong());
            ui.label(egui::RichText::new("Hex").strong());
            ui.label(egui::RichText::new("Unsigned").strong());
            ui.label(egui::RichText::new("Signed").strong());
            ui.end_row();

            for width in [1u8, 2, 4, 8] {
                let mask: u64 = if width >= 8 { u64::MAX } else { (1u64 << (width * 8)) - 1 };
                let truncated = value & mask;
                ui.label(format!("{} bits", width * 8));
                ui.label(egui::RichText::new(format!("0x{truncated:x}")).monospace());
                ui.label(egui::RichText::new(format!("{truncated}")).monospace());
                ui.label(egui::RichText::new(format!("{}", signed_interpretation(truncated, width))).monospace());
                ui.end_row();
            }
        });

    ui.add_space(20.0);
    ui.label(egui::RichText::new("Binary Layout (grouped by byte)").strong());
    ui.add_space(8.0);
    let grouped = format_value(value, NumberBase::Binary, true);
    ui.label(egui::RichText::new(grouped).monospace().size(15.0));
}

pub fn draw_compact(ui: &mut egui::Ui, value: u64, byte_width: u8) {
    ui.label(egui::RichText::new("Number Base").strong());
    ui.add_space(4.0);
    egui::Grid::new("compact_numbase_grid")
        .num_columns(2)
        .spacing([10.0, 4.0])
        .show(ui, |ui| {
            ui.label(egui::RichText::new("bin").weak());
            ui.label(egui::RichText::new(format_value(value, NumberBase::Binary, true)).monospace());
            ui.end_row();
            ui.label(egui::RichText::new("oct").weak());
            ui.label(egui::RichText::new(format_value(value, NumberBase::Octal, false)).monospace());
            ui.end_row();
            ui.label(egui::RichText::new("dec").weak());
            ui.label(egui::RichText::new(format_value(value, NumberBase::Decimal, false)).monospace());
            ui.end_row();
            ui.label(egui::RichText::new("hex").weak());
            ui.label(egui::RichText::new(format_value(value, NumberBase::Hexadecimal, false)).monospace());
            ui.end_row();
            ui.label(egui::RichText::new("signed").weak());
            ui.label(egui::RichText::new(format!("{}", signed_interpretation(value, byte_width))).monospace());
            ui.end_row();
        });
}
