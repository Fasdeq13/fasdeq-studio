use crate::app::FasdeqApp;
use crate::project::tree::TreeEntry;
use crate::icons;
use eframe::egui;
use std::path::{Path, PathBuf};

pub fn draw(app: &mut FasdeqApp, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("EXPLORER").size(11.0).strong().weak());
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if let Some(tree) = &mut app.file_tree {
                let root = tree.root.clone();
                if ui.small_button(icons::FOLDER_PLUS).on_hover_text("New Folder").clicked() {
                    tree.begin_create(root.clone(), true);
                }
                if ui.small_button(icons::NOTE_PENCIL).on_hover_text("New File").clicked() {
                    tree.begin_create(root, false);
                }
                if ui.small_button(icons::ARROW_CLOCKWISE).on_hover_text("Refresh").clicked() {
                    tree.refresh_root();
                }
            }
        });
    });

    if let Some(project) = &app.current_project {
        ui.label(egui::RichText::new(&project.name).weak().size(12.0));
    }

    ui.add_space(4.0);
    ui.separator();

    let Some(tree) = &mut app.file_tree else {
        return;
    };

    let root = tree.root.clone();
    let mut action: Option<TreeAction> = None;

    egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
        draw_node(ui, tree, &root, 0, &mut action);
    });

    apply_action(app, action);
}

enum TreeAction {
    Open(PathBuf),
    Toggle(PathBuf),
    BeginRename(PathBuf),
    ConfirmRename,
    CancelRename,
    BeginCreateFile(PathBuf),
    BeginCreateFolder(PathBuf),
    ConfirmCreate,
    CancelCreate,
    Delete(PathBuf),
    Refresh(PathBuf),
}

fn draw_node(
    ui: &mut egui::Ui,
    tree: &mut crate::project::tree::FileTree,
    dir: &Path,
    depth: usize,
    action: &mut Option<TreeAction>,
) {
    let is_expanded = tree.expanded.contains(dir);
    let entries: Vec<TreeEntry> = tree.children_cache.get(dir).cloned().unwrap_or_default();

    if let Some((create_dir, is_dir)) = &tree.creating_in {
        if create_dir == dir {
            ui.horizontal(|ui| {
                ui.add_space(depth as f32 * 16.0 + 18.0);
                ui.label(if *is_dir { icons::FOLDER } else { icons::FILE });
                let edit_id = egui::Id::new("tree_create_input");
                let response = ui.add(egui::TextEdit::singleline(&mut tree.create_input).id(edit_id));
                if !tree.create_input_focused {
                    response.request_focus();
                    tree.create_input_focused = true;
                }
                if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    *action = Some(TreeAction::ConfirmCreate);
                } else if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                    *action = Some(TreeAction::CancelCreate);
                }
            });
        }
    }

    for entry in &entries {
        if tree.renaming.as_deref() == Some(entry.path.as_path()) {
            ui.horizontal(|ui| {
                ui.add_space(depth as f32 * 16.0 + 18.0);
                ui.label(if entry.is_dir { icons::FOLDER } else { icons::extension_icon(ext_of(&entry.path)) });
                let edit_id = egui::Id::new("tree_rename_input");
                let response = ui.add(egui::TextEdit::singleline(&mut tree.rename_input).id(edit_id));
                if !tree.rename_input_focused {
                    response.request_focus();
                    tree.rename_input_focused = true;
                }
                if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    *action = Some(TreeAction::ConfirmRename);
                } else if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                    *action = Some(TreeAction::CancelRename);
                }
            });
            continue;
        }

        if entry.is_dir {
            let entry_expanded = tree.expanded.contains(&entry.path);
            let row = ui.horizontal(|ui| {
                ui.add_space(depth as f32 * 16.0);
                let caret = if entry_expanded { icons::CARET_DOWN } else { icons::CARET_RIGHT };
                if ui.small_button(caret).clicked() {
                    *action = Some(TreeAction::Toggle(entry.path.clone()));
                }
                let icon = if entry_expanded { icons::FOLDER_OPEN } else { icons::FOLDER };
                let selected = tree.selected.as_deref() == Some(entry.path.as_path());
                let label = ui.selectable_label(selected, format!("{icon}  {}", entry.name));
                if label.clicked() {
                    *action = Some(TreeAction::Toggle(entry.path.clone()));
                }
                label.context_menu(|ui| {
                    if ui.button("New File").clicked() {
                        *action = Some(TreeAction::BeginCreateFile(entry.path.clone()));
                        ui.close_menu();
                    }
                    if ui.button("New Folder").clicked() {
                        *action = Some(TreeAction::BeginCreateFolder(entry.path.clone()));
                        ui.close_menu();
                    }
                    if ui.button("Rename").clicked() {
                        *action = Some(TreeAction::BeginRename(entry.path.clone()));
                        ui.close_menu();
                    }
                    if ui.button("Refresh").clicked() {
                        *action = Some(TreeAction::Refresh(entry.path.clone()));
                        ui.close_menu();
                    }
                    if ui.button("Delete").clicked() {
                        *action = Some(TreeAction::Delete(entry.path.clone()));
                        ui.close_menu();
                    }
                });
            });
            let _ = row;

            if entry_expanded {
                if !tree.children_cache.contains_key(&entry.path) {
                    tree.load_children(&entry.path);
                }
                draw_node(ui, tree, &entry.path, depth + 1, action);
            }
        } else {
            ui.horizontal(|ui| {
                ui.add_space(depth as f32 * 16.0 + 18.0);
                let icon = icons::extension_icon(ext_of(&entry.path));
                let selected = tree.selected.as_deref() == Some(entry.path.as_path());
                let label = ui.selectable_label(selected, format!("{icon}  {}", entry.name));
                if label.clicked() {
                    *action = Some(TreeAction::Open(entry.path.clone()));
                }
                label.context_menu(|ui| {
                    if ui.button("Rename").clicked() {
                        *action = Some(TreeAction::BeginRename(entry.path.clone()));
                        ui.close_menu();
                    }
                    if ui.button("Delete").clicked() {
                        *action = Some(TreeAction::Delete(entry.path.clone()));
                        ui.close_menu();
                    }
                });
            });
        }
    }
}

fn ext_of(path: &Path) -> &str {
    path.extension().and_then(|e| e.to_str()).unwrap_or("")
}

fn apply_action(app: &mut FasdeqApp, action: Option<TreeAction>) {
    let Some(action) = action else {
        return;
    };
    match action {
        TreeAction::Open(path) => {
            if let Some(tree) = &mut app.file_tree {
                tree.selected = Some(path.clone());
            }
            app.open_file_in_editor(path);
        }
        TreeAction::Toggle(path) => {
            if let Some(tree) = &mut app.file_tree {
                tree.selected = Some(path.clone());
                tree.toggle_expanded(&path);
            }
        }
        TreeAction::BeginRename(path) => {
            if let Some(tree) = &mut app.file_tree {
                tree.begin_rename(path);
            }
        }
        TreeAction::ConfirmRename => {
            if let Some(tree) = &mut app.file_tree {
                let _ = tree.confirm_rename();
            }
        }
        TreeAction::CancelRename => {
            if let Some(tree) = &mut app.file_tree {
                tree.cancel_rename();
            }
        }
        TreeAction::BeginCreateFile(dir) => {
            if let Some(tree) = &mut app.file_tree {
                tree.begin_create(dir, false);
            }
        }
        TreeAction::BeginCreateFolder(dir) => {
            if let Some(tree) = &mut app.file_tree {
                tree.begin_create(dir, true);
            }
        }
        TreeAction::ConfirmCreate => {
            if let Some(tree) = &mut app.file_tree {
                if let Ok(Some(new_path)) = tree.confirm_create() {
                    if new_path.is_file() {
                        app.open_file_in_editor(new_path);
                    }
                }
            }
        }
        TreeAction::CancelCreate => {
            if let Some(tree) = &mut app.file_tree {
                tree.cancel_create();
            }
        }
        TreeAction::Delete(path) => {
            if let Some(tree) = &mut app.file_tree {
                let _ = tree.delete(&path);
            }
            app.close_buffers_under(&path);
        }
        TreeAction::Refresh(dir) => {
            if let Some(tree) = &mut app.file_tree {
                tree.refresh(&dir);
            }
        }
    }
}
