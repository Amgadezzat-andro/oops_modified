use crate::app::App;

pub fn show(ctx: &egui::Context, app: &mut App) {
    egui::CentralPanel::default().show(ctx, |ui| {
        // ── Tab bar ─────────────────────────────────────────────────────────
        if let Some(path) = &app.current_file.clone() {
            let name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown");

            egui::Frame::none()
                .fill(ui.visuals().extreme_bg_color)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.add_space(2.0);
                        let tab_text = if app.current_file_is_saved {
                            egui::RichText::new(format!("  {}  ", name)).small()
                        } else {
                            egui::RichText::new(format!("  ● {}  ", name))
                                .small()
                                .color(egui::Color32::from_rgb(220, 130, 50))
                        };
                        // Highlighted tab (always "active" since we show one file)
                        let tab_btn = egui::Button::new(tab_text)
                            .fill(ui.visuals().widgets.active.bg_fill)
                            .rounding(egui::Rounding {
                                nw: 4.0,
                                ne: 4.0,
                                sw: 0.0,
                                se: 0.0,
                            });
                        ui.add(tab_btn).on_hover_text(path.to_string_lossy().as_ref());
                    });
                });

            ui.separator();
        } else {
            // ── Welcome screen ───────────────────────────────────────────────
            let available = ui.available_size();
            ui.add_space(available.y * 0.25);
            ui.vertical_centered(|ui| {
                ui.heading("Welcome to OOPS Editor");
                ui.add_space(16.0);
                ui.label("Open a file to start editing.");
                ui.add_space(8.0);
                ui.label(
                    egui::RichText::new(
                        "Ctrl+O  Open file     Ctrl+S  Save     Ctrl+Shift+S  Save As",
                    )
                    .weak()
                    .small()
                    .monospace(),
                );
                ui.add_space(20.0);
                if ui.button("  Open file…  ").clicked() {
                    crate::file_ops::open_file_dialog(app);
                }
            });
            return;
        }

        // ── Code editor ─────────────────────────────────────────────────────
        // Clone what the layouter needs to avoid borrow conflicts with app.buffer
        let theme = app.theme.clone();
        let language = app.language.clone();

        egui::ScrollArea::both()
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                let mut layouter = |ui: &egui::Ui, string: &str, wrap_width: f32| {
                    let mut layout_job = egui_extras::syntax_highlighting::highlight(
                        ui.ctx(),
                        &theme,
                        string,
                        &language,
                    );
                    layout_job.wrap.max_width = wrap_width;
                    ui.fonts(|f| f.layout_job(layout_job))
                };

                let te = egui::TextEdit::multiline(&mut app.buffer)
                    .code_editor()
                    .frame(false)
                    .desired_width(f32::INFINITY)
                    .layouter(&mut layouter);

                let response = ui.add_sized(ui.available_size(), te);

                if response.changed() {
                    app.current_file_is_saved = false;
                }
            });
    });
}
