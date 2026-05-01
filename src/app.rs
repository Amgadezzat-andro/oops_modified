use egui_extras::syntax_highlighting::CodeTheme;
use std::path::PathBuf;
use std::time::Instant;

pub struct App {
    // Editor state
    pub buffer: String,
    pub language: String,
    pub current_file: Option<PathBuf>,
    pub current_file_is_saved: bool,
    pub current_directory: Option<PathBuf>,

    // Panel visibility
    pub left_panel_open: bool,
    pub right_panel_open: bool,
    pub bottom_panel_open: bool,

    // Windows
    pub settings_open: bool,

    // Appearance
    pub theme: CodeTheme,
    pub font_size: f32,

    // Transient status message with auto-dismiss
    pub status_message: Option<(String, Instant)>,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Use dark visuals for a code editor feel
        cc.egui_ctx.set_visuals(egui::Visuals::dark());

        Self {
            buffer: String::new(),
            language: "rs".to_owned(),
            current_file: None,
            current_file_is_saved: true,
            current_directory: None,

            left_panel_open: true,
            right_panel_open: false,
            bottom_panel_open: true,

            settings_open: false,

            theme: CodeTheme::default(),
            font_size: 14.0,

            status_message: None,
        }
    }

    /// Window title: filename + unsaved dot indicator
    pub fn window_title(&self) -> String {
        match &self.current_file {
            Some(p) => {
                let name = p
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown");
                let marker = if self.current_file_is_saved { "" } else { " ●" };
                format!("OOPS — {}{}", name, marker)
            }
            None => "OOPS Editor".to_owned(),
        }
    }

    pub fn set_status(&mut self, msg: impl Into<String>) {
        self.status_message = Some((msg.into(), Instant::now()));
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Keyboard shortcuts — capture flags first, then act
        let (do_save, do_open, do_save_as) = ctx.input(|i| {
            (
                i.key_pressed(egui::Key::S) && i.modifiers.ctrl && !i.modifiers.shift,
                i.key_pressed(egui::Key::O) && i.modifiers.ctrl,
                i.key_pressed(egui::Key::S) && i.modifiers.ctrl && i.modifiers.shift,
            )
        });

        if do_save {
            crate::file_ops::save_current(self);
        }
        if do_open {
            crate::file_ops::open_file_dialog(self);
        }
        if do_save_as {
            crate::file_ops::save_as_dialog(self);
        }

        // Dynamic window title
        ctx.send_viewport_cmd(egui::ViewportCommand::Title(self.window_title()));

        crate::ui::render(ctx, frame, self);
    }
}
