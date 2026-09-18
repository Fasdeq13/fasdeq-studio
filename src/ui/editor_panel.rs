use crate::app::FasdeqApp;
use eframe::egui;

pub fn draw(app: &mut FasdeqApp, ui: &mut egui::Ui) {
    if app.open_buffers.is_empty() {
        ui.centered_and_justified(|ui| {
            ui.label(egui::RichText::new("Open a file from the Explorer to start editing.").weak());
        });
        return;
    }

    ui.horizontal(|ui| {
        let mut close_index = None;
        for (index, buffer) in app.open_buffers.iter().enumerate() {
            let is_active = app.active_buffer == Some(index);
            let label = if buffer.dirty {
                format!("● {}", buffer.file_name())
            } else {
                buffer.file_name()
            };
            if ui.selectable_label(is_active, label).clicked() {
                app.active_buffer = Some(index);
            }
            if ui.small_button("✕").clicked() {
                close_index = Some(index);
            }
        }
        if let Some(index) = close_index {
            app.close_buffer(index);
        }
    });

    ui.separator();

    if ui.button("💾 Save").clicked() {
        app.save_active_buffer();
    }
    ui.add_space(4.0);

    let font_size = app.font_size;
    let theme_name = app.theme.syntax_theme_name.clone();

    if let Some(index) = app.active_buffer {
        let extension = app.open_buffers[index].extension();
        let highlight_engine = &app.highlight_engine;

        egui::ScrollArea::vertical().show(ui, |ui| {
            let buffer = &mut app.open_buffers[index];
            let mut layouter = |ui: &egui::Ui, text: &str, wrap_width: f32| {
                let mut job = highlight_engine.highlight(text, &extension, &theme_name, font_size);
                job.wrap.max_width = wrap_width;
                ui.fonts(|f| f.layout_job(job))
            };

            let response = ui.add(
                egui::TextEdit::multiline(&mut buffer.content)
                    .code_editor()
                    .font(egui::FontId::monospace(font_size))
                    .desired_width(f32::INFINITY)
                    .layouter(&mut layouter),
            );

            if response.changed() {
                buffer.dirty = true;
            }
        });
    }
}
