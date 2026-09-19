use crate::app::FasdeqApp;
use crate::editor::diagnostics::{Diagnostic, Severity};
use crate::editor::keybindings::{handle_key, KeymapStyle, VimMode, VimOutcome};
use crate::editor::quickfix::suggest_fixes;
use crate::icons;
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
                format!("{}  {}", crate::icons::DOT, buffer.file_name())
            } else {
                buffer.file_name()
            };
            if ui.selectable_label(is_active, label).clicked() {
                app.active_buffer = Some(index);
            }
            if ui.small_button(icons::X).clicked() {
                close_index = Some(index);
            }
        }
        if let Some(index) = close_index {
            app.close_buffer(index);
        }
    });

    ui.separator();

    ui.horizontal(|ui| {
        if ui.button(format!("{}  Save", icons::FLOPPY_DISK)).clicked() {
            app.save_active_buffer();
        }
        if app.keymap == KeymapStyle::Vim {
            ui.separator();
            ui.label(
                egui::RichText::new(format!("-- {} --", app.vim_state.mode_label()))
                    .monospace()
                    .strong(),
            );
            if app.vim_state.mode == VimMode::Command {
                ui.label(egui::RichText::new(format!(":{}", app.vim_state.command_buffer)).monospace());
            }
            ui.label(egui::RichText::new("(hjkl move, i insert, Esc normal, :w save)").weak().size(11.0));
        }
    });
    ui.add_space(4.0);

    let font_size = app.font_size;
    let theme_name = app.theme.syntax_theme_name.clone();
    let is_vim = app.keymap == KeymapStyle::Vim;

    let Some(index) = app.active_buffer else {
        return;
    };

    let path = app.open_buffers[index].path.clone();
    let diagnostics: Vec<Diagnostic> = app.diagnostics_for(&path).into_iter().cloned().collect();
    let extension = app.open_buffers[index].extension();

    if !diagnostics.is_empty() {
        draw_lightbulb_bar(ui, app, index, &diagnostics);
    }

    let text_edit_id = egui::Id::new(("editor_buffer", index));

    // In Vim normal/visual/command mode, we intercept keys ourselves and must
    // stop egui's TextEdit widget from also consuming them as text input.
    // We do this by draining matched events out of the input queue before
    // the TextEdit widget ever sees them, rather than disabling the widget
    // (which would also kill scrolling, clicking and cursor placement).
    if is_vim && app.vim_state.mode != VimMode::Insert {
        let outcome = ui.ctx().input_mut(|input| {
            let mut outcome = VimOutcome::None;
            let mut remaining = Vec::with_capacity(input.events.len());
            for event in input.events.drain(..) {
                let consumed = match &event {
                    egui::Event::Key { key, pressed: true, modifiers, .. } => {
                        let buffer_text = &mut app.open_buffers[index].content;
                        let result = handle_key(&mut app.vim_state, buffer_text, *key, *modifiers);
                        if result != VimOutcome::None {
                            outcome = result;
                        }
                        app.open_buffers[index].dirty = true;
                        true
                    }
                    egui::Event::Text(t) if app.vim_state.mode == VimMode::Command => {
                        app.vim_state.command_buffer.push_str(t);
                        true
                    }
                    _ => false,
                };
                if !consumed {
                    remaining.push(event);
                }
            }
            input.events = remaining;
            outcome
        });
        if outcome == VimOutcome::Save {
            app.save_active_buffer();
        }
    }

    let highlight_engine = &app.highlight_engine;
    let diagnostics_ref = &diagnostics;

    egui::ScrollArea::vertical().show(ui, |ui| {
        let buffer = &mut app.open_buffers[index];
        let mut layouter = |ui: &egui::Ui, text: &str, wrap_width: f32| {
            let mut job = highlight_engine.highlight(text, &extension, &theme_name, font_size);
            job.wrap.max_width = wrap_width;
            apply_diagnostic_underlines(&mut job, text, diagnostics_ref);
            ui.fonts(|f| f.layout_job(job))
        };

        let mut output = egui::TextEdit::multiline(&mut buffer.content)
            .id(text_edit_id)
            .code_editor()
            .font(egui::FontId::monospace(font_size))
            .desired_width(f32::INFINITY)
            .lock_focus(true)
            .layouter(&mut layouter)
            .show(ui);

        if output.response.changed() {
            buffer.dirty = true;
        }

        // Keep the Vim cursor position in sync with wherever egui's own
        // cursor ends up (mouse clicks, arrow keys inside TextEdit, etc.)
        // so hjkl movement continues from the right place, and push our
        // Vim-owned cursor back into egui's state after hjkl movement so
        // the caret is drawn at the right place.
        if is_vim {
            if output.response.changed() || output.response.clicked() {
                if let Some(cursor_range) = output.state.cursor.char_range() {
                    app.vim_state.cursor = char_to_byte_offset(&buffer.content, cursor_range.primary.index);
                }
            } else {
                let char_index = byte_to_char_offset(&buffer.content, app.vim_state.cursor);
                let ccursor = egui::text::CCursor::new(char_index);
                output.state.cursor.set_char_range(Some(egui::text::CCursorRange::one(ccursor)));
                output.state.store(ui.ctx(), text_edit_id);
            }

            if app.vim_state.mode != VimMode::Insert {
                output.response.request_focus();
            }
        }
    });
}

fn char_to_byte_offset(text: &str, char_index: usize) -> usize {
    text.char_indices()
        .nth(char_index)
        .map(|(b, _)| b)
        .unwrap_or(text.len())
}

fn byte_to_char_offset(text: &str, byte_index: usize) -> usize {
    text[..byte_index.min(text.len())].chars().count()
}

fn apply_diagnostic_underlines(job: &mut egui::text::LayoutJob, text: &str, diagnostics: &[Diagnostic]) {
    if diagnostics.is_empty() {
        return;
    }
    let lines: Vec<&str> = text.split('\n').collect();
    let mut line_offsets = Vec::with_capacity(lines.len());
    let mut offset = 0usize;
    for line in &lines {
        line_offsets.push(offset);
        offset += line.len() + 1;
    }

    for diag in diagnostics {
        if diag.line == 0 || diag.line > lines.len() {
            continue;
        }
        let line_idx = diag.line - 1;
        let start = line_offsets[line_idx];
        let end = start + lines[line_idx].len();
        let color = match diag.severity {
            Severity::Error => egui::Color32::from_rgb(240, 95, 92),
            Severity::Warning => egui::Color32::from_rgb(240, 180, 90),
            Severity::Note => egui::Color32::from_rgb(150, 160, 180),
        };
        for section in &mut job.sections {
            let overlaps = section.byte_range.start < end && section.byte_range.end > start;
            if overlaps {
                section.format.underline = egui::Stroke::new(1.5, color);
            }
        }
    }
}

fn draw_lightbulb_bar(ui: &mut egui::Ui, app: &mut FasdeqApp, buffer_index: usize, diagnostics: &[Diagnostic]) {
    let Some(first) = diagnostics.first() else {
        return;
    };
    let buffer_content = app.open_buffers[buffer_index].content.clone();
    let line_text = buffer_content.lines().nth(first.line.saturating_sub(1)).unwrap_or("").to_string();

    egui::Frame::none()
        .fill(egui::Color32::from_rgb(50, 42, 20))
        .rounding(egui::Rounding::same(6.0))
        .inner_margin(egui::Margin::symmetric(10.0, 6.0))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(icons::WARNING).color(egui::Color32::from_rgb(255, 200, 90)).size(16.0));
                ui.label(
                    egui::RichText::new(format!("Line {}: {}", first.line, first.message))
                        .size(12.0),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.menu_button(format!("{}  Quick Fix", icons::INFO), |ui| {
                        let fixes = suggest_fixes(first, &line_text);
                        for fix in fixes {
                            let enabled = fix.replacement.is_some();
                            if ui.add_enabled(enabled, egui::Button::new(&fix.title)).clicked() {
                                if let Some(replacement) = &fix.replacement {
                                    apply_line_replacement(app, buffer_index, first.line, replacement);
                                }
                                ui.close_menu();
                            }
                            if !enabled {
                                ui.label(egui::RichText::new(&fix.title).weak().size(11.0));
                            }
                        }
                    });
                });
            });
        });
    ui.add_space(6.0);
}

fn apply_line_replacement(app: &mut FasdeqApp, buffer_index: usize, line_number: usize, replacement: &str) {
    let Some(buffer) = app.open_buffers.get_mut(buffer_index) else {
        return;
    };
    let mut lines: Vec<String> = buffer.content.split('\n').map(|s| s.to_string()).collect();
    if line_number == 0 || line_number > lines.len() {
        return;
    }
    if replacement.is_empty() {
        lines.remove(line_number - 1);
    } else {
        lines[line_number - 1] = replacement.to_string();
    }
    buffer.content = lines.join("\n");
    buffer.dirty = true;
}
