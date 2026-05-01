use crate::app::App;
use std::time::Duration;

pub fn show(ctx: &egui::Context, app: &mut App) {
    egui::TopBottomPanel::bottom("status_bar")
        .min_height(22.0)
        .show_animated(ctx, app.bottom_panel_open, |ui| {
            ui.horizontal(|ui| {
                ui.add_space(4.0);

                // Save/unsaved indicator
                let (dot_color, save_label) = if app.current_file_is_saved {
                    (egui::Color32::from_rgb(80, 200, 120), "Saved")
                } else {
                    (egui::Color32::from_rgb(220, 130, 50), "Unsaved")
                };
                ui.label(egui::RichText::new("●").color(dot_color).small());
                ui.label(egui::RichText::new(save_label).small());

                ui.separator();

                // Language
                ui.label(
                    egui::RichText::new(app.language.to_uppercase())
                        .small()
                        .monospace(),
                );

                ui.separator();

                // Use pre-computed stats — no per-frame scan of the buffer
                ui.label(
                    egui::RichText::new(format!("Ln {}  Ch {}", app.stat_lines, app.stat_chars))
                        .small(),
                );

                ui.separator();

                // Current file path (weak, small)
                if let Some(path) = &app.current_file {
                    ui.label(
                        egui::RichText::new(path.to_string_lossy().as_ref())
                            .small()
                            .weak(),
                    );
                }

                // Transient status message — right-aligned, auto-dismiss after 3 s.
                // Schedule a repaint only for the moment it should disappear so we
                // don't spin at 60 fps just to check elapsed time.
                let should_clear = if let Some((msg, ts)) = &app.status_message {
                    let elapsed = ts.elapsed();
                    if elapsed < Duration::from_secs(3) {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.add_space(4.0);
                            ui.label(
                                egui::RichText::new(msg.as_str())
                                    .small()
                                    .color(egui::Color32::from_rgb(150, 210, 255)),
                            );
                        });
                        // Wake up exactly when the message should vanish
                        ctx.request_repaint_after(Duration::from_secs(3) - elapsed);
                        false
                    } else {
                        true
                    }
                } else {
                    false
                };

                if should_clear {
                    app.status_message = None;
                }
            });
        });
}

