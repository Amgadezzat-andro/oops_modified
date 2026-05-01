use crate::app::App;

pub fn show(ctx: &egui::Context, app: &mut App) {
    egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui
                    .add(egui::Button::new("Open…").shortcut_text("Ctrl+O"))
                    .clicked()
                {
                    crate::file_ops::open_file_dialog(app);
                    ui.close_menu();
                }
                ui.separator();
                if ui
                    .add(egui::Button::new("Save").shortcut_text("Ctrl+S"))
                    .clicked()
                {
                    crate::file_ops::save_current(app);
                    ui.close_menu();
                }
                if ui
                    .add(egui::Button::new("Save As…").shortcut_text("Ctrl+Shift+S"))
                    .clicked()
                {
                    crate::file_ops::save_as_dialog(app);
                    ui.close_menu();
                }
            });

            ui.menu_button("Edit", |ui| {
                if ui.button("Settings…").clicked() {
                    app.settings_open = !app.settings_open;
                    ui.close_menu();
                }
            });

            ui.menu_button("View", |ui| {
                if ui.button("Zen Mode").on_hover_text("Hide all panels").clicked() {
                    let any_open = app.left_panel_open
                        || app.right_panel_open
                        || app.bottom_panel_open;
                    app.left_panel_open = !any_open;
                    app.right_panel_open = !any_open;
                    app.bottom_panel_open = !any_open;
                    ui.close_menu();
                }
                ui.separator();
                ui.checkbox(&mut app.left_panel_open, "Explorer");
                ui.checkbox(&mut app.right_panel_open, "Info Panel");
                ui.checkbox(&mut app.bottom_panel_open, "Status Bar");
            });

            ui.menu_button("Help", |ui| {
                ui.label(egui::RichText::new("OOPS Editor").strong());
                ui.label("A lightweight code editor built with egui.");
                ui.separator();
                ui.label("Ctrl+O   Open file");
                ui.label("Ctrl+S   Save");
                ui.label("Ctrl+Shift+S   Save As");
            });
        });
    });
}
