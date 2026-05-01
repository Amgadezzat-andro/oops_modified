use crate::app::App;

pub fn show(ctx: &egui::Context, app: &mut App) {
    egui::Window::new("Settings")
        .open(&mut app.settings_open)
        .default_width(340.0)
        .resizable(false)
        .collapsible(false)
        .show(ctx, |ui| {
            ui.heading("Color Theme");
            ui.separator();
            ui.group(|ui| {
                app.theme.ui(ui);
                app.theme.clone().store_in_memory(ui.ctx());
            });

            ui.add_space(10.0);

            ui.heading("Font Size");
            ui.separator();
            ui.horizontal(|ui| {
                ui.add(
                    egui::Slider::new(&mut app.font_size, 10.0..=28.0)
                        .step_by(1.0)
                        .suffix(" px"),
                );
                if ui.button("Reset").clicked() {
                    app.font_size = 14.0;
                }
            });
            // Apply font size as global DPI scale
            let target_ppp = app.font_size / 14.0;
            if (ctx.pixels_per_point() - target_ppp).abs() > 0.01 {
                ctx.set_pixels_per_point(target_ppp);
            }
        });
}
