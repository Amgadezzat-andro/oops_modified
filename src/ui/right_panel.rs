use crate::app::App;

pub fn show(ctx: &egui::Context, app: &mut App) {
    egui::SidePanel::right("info_panel")
        .resizable(true)
        .default_width(175.0)
        .width_range(140.0..=300.0)
        .show_animated(ctx, app.right_panel_open, |ui| {
            ui.add_space(4.0);
            ui.strong("FILE INFO");
            ui.separator();

            if let Some(path) = &app.current_file {
                let name = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown");

                ui.label(format!("Name: {}", name));
                ui.label(format!("Lang: {}", app.language.to_uppercase()));

                let saved = if app.current_file_is_saved {
                    egui::RichText::new("Saved")
                        .color(egui::Color32::from_rgb(80, 200, 120))
                } else {
                    egui::RichText::new("Unsaved")
                        .color(egui::Color32::from_rgb(220, 130, 50))
                };
                ui.horizontal(|ui| {
                    ui.label("Status:");
                    ui.label(saved);
                });

                ui.separator();

                let lines = app.buffer.lines().count().max(1);
                let words = app.buffer.split_whitespace().count();
                let chars = app.buffer.chars().count();
                let bytes = app.buffer.len();

                egui::Grid::new("file_stats")
                    .num_columns(2)
                    .spacing([8.0, 2.0])
                    .show(ui, |ui| {
                        ui.label(egui::RichText::new("Lines").weak());
                        ui.label(lines.to_string());
                        ui.end_row();

                        ui.label(egui::RichText::new("Words").weak());
                        ui.label(words.to_string());
                        ui.end_row();

                        ui.label(egui::RichText::new("Chars").weak());
                        ui.label(chars.to_string());
                        ui.end_row();

                        ui.label(egui::RichText::new("Bytes").weak());
                        ui.label(bytes.to_string());
                        ui.end_row();

                        if let Ok(meta) = std::fs::metadata(path) {
                            ui.label(egui::RichText::new("On disk").weak());
                            ui.label(format!("{} B", meta.len()));
                            ui.end_row();
                        }
                    });
            } else {
                ui.label(
                    egui::RichText::new("No file open")
                        .weak()
                        .italics(),
                );
            }
        });
}
