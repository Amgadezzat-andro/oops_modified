mod editor;
mod left_panel;
mod menu_bar;
mod right_panel;
mod settings;
mod status_bar;

use crate::app::App;

pub fn render(ctx: &egui::Context, _frame: &mut eframe::Frame, app: &mut App) {
    menu_bar::show(ctx, app);
    status_bar::show(ctx, app);
    left_panel::show(ctx, app);
    right_panel::show(ctx, app);
    editor::show(ctx, app);
    settings::show(ctx, app);
}
