use rfd::FileDialog;
use std::fs::{self, File};
use std::io::{self, Read};
use std::path::PathBuf;

use crate::app::App;

/// Read any file as UTF-8, falling back gracefully on encoding errors.
pub fn read_file(path: &std::path::Path) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;
    let (result, _, _) = encoding_rs::UTF_8.decode(&bytes);
    Ok(result.into_owned())
}

/// Detect language from file extension.
fn lang_from_path(path: &std::path::Path) -> String {
    path.extension()
        .and_then(|e| e.to_str())
        .unwrap_or("txt")
        .to_owned()
}

/// Load a file into the editor buffer and update related state.
pub fn load_file(app: &mut App, path: PathBuf) {
    match read_file(&path) {
        Ok(contents) => {
            app.buffer = contents;
            app.language = lang_from_path(&path);
            if let Some(parent) = path.parent() {
                app.current_directory = Some(parent.to_path_buf());
            }
            app.current_file = Some(path);
            app.current_file_is_saved = true;
        }
        Err(e) => {
            app.set_status(format!("Error opening file: {}", e));
        }
    }
}

/// Open a native file-picker dialog and load the chosen file.
pub fn open_file_dialog(app: &mut App) {
    let start = app
        .current_directory
        .clone()
        .unwrap_or_else(|| PathBuf::from("/"));

    if let Some(path) = FileDialog::new().set_directory(&start).pick_file() {
        load_file(app, path);
    }
}

/// Save the current buffer to `current_file`, or prompt if none.
pub fn save_current(app: &mut App) {
    if let Some(path) = app.current_file.clone() {
        write_buffer(app, path);
    } else {
        save_as_dialog(app);
    }
}

/// Prompt for a save location and write the buffer there.
pub fn save_as_dialog(app: &mut App) {
    let start = app
        .current_directory
        .clone()
        .unwrap_or_else(|| PathBuf::from("/"));

    if let Some(path) = FileDialog::new().set_directory(&start).save_file() {
        app.language = lang_from_path(&path);
        if let Some(parent) = path.parent() {
            app.current_directory = Some(parent.to_path_buf());
        }
        write_buffer(app, path);
    }
}

fn write_buffer(app: &mut App, path: PathBuf) {
    match fs::write(&path, &app.buffer) {
        Ok(_) => {
            app.current_file = Some(path);
            app.current_file_is_saved = true;
            app.set_status("File saved.");
        }
        Err(e) => {
            app.set_status(format!("Save error: {}", e));
        }
    }
}
