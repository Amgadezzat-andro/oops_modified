src/
  main.rs          ← entry point only (10 lines)
  app.rs           ← App struct + eframe::App impl
  file_ops.rs      ← all file I/O (read, load, save, save-as)
  ui/
    mod.rs         ← orchestrates panel rendering
    menu_bar.rs    ← top menu bar
    left_panel.rs  ← file explorer
    right_panel.rs ← file info panel
    status_bar.rs  ← bottom status bar
    editor.rs      ← code editor + tab bar
    settings.rs    ← settings window