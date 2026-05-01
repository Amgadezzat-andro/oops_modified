use crate::app::App;
use std::fs;
use std::path::PathBuf;

/// Rebuild the directory cache. Called once per navigation, not every frame.
fn rebuild_dir_cache(app: &mut App) {
    app.dir_cache.clear();
    let dir = match &app.current_directory {
        Some(d) => d.clone(),
        None => {
            app.dir_cache_key = None;
            return;
        }
    };

    if let Ok(read) = fs::read_dir(&dir) {
        let mut entries: Vec<(PathBuf, bool)> = read
            .filter_map(|e| {
                let e = e.ok()?;
                let name = e.file_name();
                let name_str = name.to_str()?;
                if name_str.starts_with('.') {
                    return None;
                }
                // file_type() on Linux is free — readdir already returns d_type
                let is_dir = e.file_type().map(|t| t.is_dir()).unwrap_or(false);
                Some((e.path(), is_dir))
            })
            .collect();

        entries.sort_by(|(a, a_dir), (b, b_dir)| match (a_dir, b_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.file_name().cmp(&b.file_name()),
        });

        app.dir_cache = entries;
    }

    app.dir_cache_key = Some(dir);
}

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
                    if ui.small_button("🔄").on_hover_text("Refresh").clicked() {
                        app.invalidate_dir_cache();
                    }
                    if ui
                        .small_button("⬆")
                        .on_hover_text("Go up one directory")
                        .clicked()
                    {
                        if let Some(dir) = app.current_directory.clone() {
                            if let Some(parent) = dir.parent() {
                                app.current_directory = Some(parent.to_path_buf());
                                app.invalidate_dir_cache();
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
                            app.invalidate_dir_cache();
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

            // ── Rebuild cache only when directory changes ────────────────────
            if app.current_directory != app.dir_cache_key {
                rebuild_dir_cache(app);
            }

            // ── File tree ───────────────────────────────────────────────────
            if app.current_directory.is_none() {
                return;
            }

            // Collect into a local snapshot to avoid borrow issues while mutating app
            let entries: Vec<(PathBuf, bool)> = app.dir_cache.clone();

            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    for (path, is_dir) in entries {
                        let name = match path.file_name().and_then(|n| n.to_str()) {
                            Some(n) => n.to_owned(),
                            None => continue,
                        };

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
                                app.invalidate_dir_cache();
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
