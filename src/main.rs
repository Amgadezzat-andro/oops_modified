mod app;
mod file_ops;
mod ui;

fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1100.0, 700.0])
            .with_min_inner_size([600.0, 400.0]),
        ..Default::default()
    };

    let _ = eframe::run_native(
        "OOPS Editor",
        options,
        Box::new(|cc| Ok(Box::new(app::App::new(cc)))),
    );
}
