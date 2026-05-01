use crate::app::App;
use std::fs;
use std::path::PathBuf;

pub fn show(ctx: &egui::Context, app: &mut App) {
    egui::SidePanel::left("explorer")
        .resizable(true)
        .default_width(210.0)
        .width_range(150.0..=400.0)
        .show_animated(ctx, app.left_panel_open, |ui| {
            ui.add_space(4.0);

            // ── Header row ──────────────────────────────────────────────────
            ui.horizontal(|ui| {
                ui.strong("EXPLORER");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui
                        .small_button("⬆")
                        .on_hover_text("Go up one directory")
                        .clicked()
                    {
                        if let Some(dir) = app.current_directory.clone() {
                            if let Some(parent) = dir.parent() {
                                app.current_directory = Some(parent.to_path_buf());
                            }
                        }
                    }
                    if ui
                        .small_button("📂")
                        .on_hover_text("Open folder")
                        .clicked()
                    {
                        if let Some(dir) = rfd::FileDialog::new().pick_folder() {
                            app.current_directory = Some(dir);
                        }
                    }
                });
            });

            // Current directory path
            if let Some(dir) = &app.current_directory {
                ui.label(
                    egui::RichText::new(dir.to_string_lossy().as_ref())
                        .small()
                        .weak()
                        .italics(),
                );
            } else {
                ui.label(egui::RichText::new("No folder open").small().weak().italics());
            }

            ui.separator();

            // ── File tree ───────────────────────────────────────────────────
            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    let dir = match app.current_directory.clone() {
                        Some(d) => d,
                        None => return,
                    };

                    let Ok(entries) = fs::read_dir(&dir) else {
                        ui.label(egui::RichText::new("Cannot read directory").small().weak());
                        return;
                    };

                    let mut entries: Vec<PathBuf> = entries
                        .filter_map(|e| e.ok().map(|e| e.path()))
                        .filter(|p| {
                            // Skip hidden entries
                            p.file_name()
                                .and_then(|n| n.to_str())
                                .map(|n| !n.starts_with('.'))
                                .unwrap_or(false)
                        })
                        .collect();

                    // Directories first, then alphabetical
                    entries.sort_by(|a, b| match (a.is_dir(), b.is_dir()) {
                        (true, false) => std::cmp::Ordering::Less,
                        (false, true) => std::cmp::Ordering::Greater,
                        _ => a.file_name().cmp(&b.file_name()),
                    });

                    for path in entries {
                        let name = match path.file_name().and_then(|n| n.to_str()) {
                            Some(n) => n.to_owned(),
                            None => continue,
                        };

                        let is_dir = path.is_dir();
                        let is_active = app.current_file.as_ref() == Some(&path);

                        let icon = if is_dir {
                            "📁 "
                        } else {
                            icon_for_path(&path)
                        };

                        let display = format!("{}{}", icon, name);

                        let text = if is_active {
                            egui::RichText::new(&display)
                                .strong()
                                .color(egui::Color32::from_rgb(100, 180, 255))
                        } else if is_dir {
                            egui::RichText::new(&display)
                                .color(egui::Color32::from_rgb(200, 200, 200))
                        } else {
                            egui::RichText::new(&display)
                        };

                        let hover = path.to_string_lossy().into_owned();
                        let response = ui
                            .add(egui::Label::new(text).sense(egui::Sense::click()))
                            .on_hover_text(hover);

                        if response.clicked() {
                            if is_dir {
                                app.current_directory = Some(path);
                            } else {
                                crate::file_ops::load_file(app, path);
                            }
                        }
                    }
                });
        });
}

fn icon_for_path(path: &PathBuf) -> &'static str {
    match path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
    {
        "rs" => "🦀 ",
        "toml" | "yaml" | "yml" => "⚙ ",
        "json" => "{ ",
        "md" | "rst" => "📝 ",
        "txt" => "📄 ",
        "sh" | "bash" => "🐚 ",
        "py" => "🐍 ",
        "js" | "mjs" => "🟨 ",
        "ts" => "🔷 ",
        "html" | "htm" => "🌐 ",
        "css" | "scss" => "🎨 ",
        "c" | "h" => "🔵 ",
        "cpp" | "hpp" | "cc" => "🔷 ",
        "go" => "🐹 ",
        "java" => "☕ ",
        "xml" => "📋 ",
        _ => "📄 ",
    }
}
