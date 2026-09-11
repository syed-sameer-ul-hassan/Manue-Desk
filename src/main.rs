use std::env;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::rc::Rc;
use slint::VecModel;

slint::include_modules!();

fn get_applications_directory() -> Result<PathBuf, String> {
    let mut app_dir = PathBuf::from(env::var("HOME").map_err(|e| format!("Failed to get HOME directory: {}", e))?);
    app_dir.push(".local/share/applications");
    Ok(app_dir)
}

fn sanitize_filename(name: &str) -> String {
    name.to_lowercase()
        .chars()
        .map(|c| if c.is_whitespace() { '-' } else { c })
        .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_')
        .collect()
}

fn create_desktop_file(
    app_name: &str,
    exec_path: &str,
    icon_path: &str,
    run_in_terminal: bool,
) -> Result<String, String> {
    let app_dir = get_applications_directory()?;

    let sanitized_name = sanitize_filename(app_name);
    let filename = format!("{}.desktop", sanitized_name);

    fs::create_dir_all(&app_dir).map_err(|e| format!("Failed to create applications directory: {}", e))?;

    let terminal_flag = if run_in_terminal { "true" } else { "false" };
    let desktop_content = format!(
        "[Desktop Entry]\n\
         Version=1.0\n\
         Type=Application\n\
         Name={}\n\
         Exec={}\n\
         Icon={}\n\
         Terminal={}\n\
         Categories=Application;\n",
        app_name, exec_path, icon_path, terminal_flag
    );

    let file_path = app_dir.join(&filename);
    let mut file = fs::File::create(&file_path)
        .map_err(|e| format!("Failed to create desktop file: {}", e))?;

    file.write_all(desktop_content.as_bytes())
        .map_err(|e| format!("Failed to write desktop file: {}", e))?;

    // Make the file executable on Unix
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&file_path)
            .map_err(|e| format!("Failed to get file metadata: {}", e))?
            .permissions();
        perms.set_mode(0o755);
        let _ = fs::set_permissions(&file_path, perms);
    }

    Ok(file_path.to_string_lossy().to_string())
}

#[derive(Clone, Debug)]
struct DesktopEntryItem {
    name: String,
    exec: String,
    icon: String,
    terminal: bool,
    file_path: String,
    filename: String,
}

fn scan_desktop_files() -> Vec<DesktopEntryItem> {
    let mut entries = Vec::new();
    let Ok(app_dir) = get_applications_directory() else { return entries; };
    let Ok(read_dir) = fs::read_dir(&app_dir) else { return entries; };

    for entry in read_dir.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("desktop") {
            let filename = path.file_name().unwrap_or_default().to_string_lossy().to_string();
            if let Ok(content) = fs::read_to_string(&path) {
                let mut name = String::new();
                let mut exec = String::new();
                let mut icon = String::new();
                let mut terminal = false;

                for line in content.lines() {
                    let line = line.trim();
                    if line.starts_with("Name=") && name.is_empty() {
                        name = line.strip_prefix("Name=").unwrap_or("").to_string();
                    } else if line.starts_with("Exec=") && exec.is_empty() {
                        exec = line.strip_prefix("Exec=").unwrap_or("").to_string();
                    } else if line.starts_with("Icon=") && icon.is_empty() {
                        icon = line.strip_prefix("Icon=").unwrap_or("").to_string();
                    } else if line.starts_with("Terminal=") {
                        let term_val = line.strip_prefix("Terminal=").unwrap_or("").to_lowercase();
                        terminal = term_val == "true" || term_val == "1";
                    }
                }

                if name.is_empty() {
                    name = filename.trim_end_matches(".desktop").to_string();
                }

                entries.push(DesktopEntryItem {
                    name,
                    exec,
                    icon,
                    terminal,
                    file_path: path.to_string_lossy().to_string(),
                    filename,
                });
            }
        }
    }

    entries.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    entries
}

fn load_and_filter_shortcuts(ui: &MainWindow, filter: &str) {
    let all_items = scan_desktop_files();
    let filter_lower = filter.trim().to_lowercase();

    let filtered: Vec<DesktopEntryData> = all_items
        .into_iter()
        .filter(|item| {
            if filter_lower.is_empty() {
                true
            } else {
                item.name.to_lowercase().contains(&filter_lower)
                    || item.exec.to_lowercase().contains(&filter_lower)
                    || item.filename.to_lowercase().contains(&filter_lower)
            }
        })
        .map(|item| DesktopEntryData {
            name: slint::SharedString::from(&item.name),
            exec: slint::SharedString::from(&item.exec),
            icon: slint::SharedString::from(&item.icon),
            terminal: item.terminal,
            file_path: slint::SharedString::from(&item.file_path),
            filename: slint::SharedString::from(&item.filename),
        })
        .collect();

    ui.set_shortcuts_count(filtered.len() as i32);
    let model = Rc::new(VecModel::from(filtered));
    ui.set_shortcuts_list(model.into());
}

fn update_desktop_file(
    file_path: &str,
    name: &str,
    exec: &str,
    icon: &str,
    terminal: bool,
) -> Result<(), String> {
    let path = std::path::Path::new(file_path);
    let terminal_str = if terminal { "true" } else { "false" };

    let content = format!(
        "[Desktop Entry]\n\
         Version=1.0\n\
         Type=Application\n\
         Name={}\n\
         Exec={}\n\
         Icon={}\n\
         Terminal={}\n\
         Categories=Application;\n",
        name, exec, icon, terminal_str
    );

    let mut file = fs::File::create(path).map_err(|e| format!("Failed to update file: {}", e))?;
    file.write_all(content.as_bytes()).map_err(|e| format!("Failed to write file: {}", e))?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Ok(metadata) = fs::metadata(path) {
            let mut perms = metadata.permissions();
            perms.set_mode(0o755);
            let _ = fs::set_permissions(path, perms);
        }
    }

    Ok(())
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = MainWindow::new()?;

    // Initial load of shortcuts
    load_and_filter_shortcuts(&ui, "");

    // ─── Browse Executable (Create Tab) ────────────────────────────────────
    let ui_weak = ui.as_weak();
    ui.on_browse_exec(move || {
        let ui = ui_weak.upgrade().unwrap();
        let result = rfd::FileDialog::new()
            .set_title("Select Executable")
            .pick_file();
        if let Some(path) = result {
            ui.set_exec_path(slint::SharedString::from(path.to_string_lossy().as_ref()));
        }
    });

    // ─── Browse Icon (Create Tab) ──────────────────────────────────────────
    let ui_weak = ui.as_weak();
    ui.on_browse_icon(move || {
        let ui = ui_weak.upgrade().unwrap();
        let result = rfd::FileDialog::new()
            .set_title("Select Icon")
            .add_filter("Images", &["png", "jpg", "jpeg", "svg", "webp", "xpm", "ico"])
            .add_filter("All Files", &["*"])
            .pick_file();
        if let Some(path) = result {
            ui.set_icon_path(slint::SharedString::from(path.to_string_lossy().as_ref()));
        }
    });

    // ─── Browse Output Directory (Settings Tab) ────────────────────────────
    let ui_weak = ui.as_weak();
    ui.on_browse_output_dir(move || {
        let ui = ui_weak.upgrade().unwrap();
        let result = rfd::FileDialog::new()
            .set_title("Select Output Directory")
            .pick_folder();
        if let Some(path) = result {
            ui.set_default_output_dir(slint::SharedString::from(path.to_string_lossy().as_ref()));
        }
    });

    // ─── Browse Edit Executable (Manage Tab) ────────────────────────────────
    let ui_weak = ui.as_weak();
    ui.on_browse_edit_exec(move || {
        let ui = ui_weak.upgrade().unwrap();
        let result = rfd::FileDialog::new()
            .set_title("Select Executable")
            .pick_file();
        if let Some(path) = result {
            ui.set_edit_exec_path(slint::SharedString::from(path.to_string_lossy().as_ref()));
        }
    });

    // ─── Browse Edit Icon (Manage Tab) ──────────────────────────────────────
    let ui_weak = ui.as_weak();
    ui.on_browse_edit_icon(move || {
        let ui = ui_weak.upgrade().unwrap();
        let result = rfd::FileDialog::new()
            .set_title("Select Icon")
            .add_filter("Images", &["png", "jpg", "jpeg", "svg", "webp", "xpm", "ico"])
            .add_filter("All Files", &["*"])
            .pick_file();
        if let Some(path) = result {
            ui.set_edit_icon_path(slint::SharedString::from(path.to_string_lossy().as_ref()));
        }
    });

    // ─── Filter Shortcuts ──────────────────────────────────────────────────
    let ui_weak = ui.as_weak();
    ui.on_filter_shortcuts(move |query| {
        let ui = ui_weak.upgrade().unwrap();
        load_and_filter_shortcuts(&ui, query.as_str());
    });

    // ─── Refresh Shortcuts ─────────────────────────────────────────────────
    let ui_weak = ui.as_weak();
    ui.on_refresh_shortcuts(move || {
        let ui = ui_weak.upgrade().unwrap();
        let query = ui.get_shortcuts_search_query().to_string();
        load_and_filter_shortcuts(&ui, &query);
    });

    // ─── Delete Shortcut ───────────────────────────────────────────────────
    let ui_weak = ui.as_weak();
    ui.on_delete_shortcut(move |file_path| {
        let ui = ui_weak.upgrade().unwrap();
        let path_str = file_path.to_string();
        let path = std::path::Path::new(&path_str);
        if path.exists() {
            if let Err(e) = fs::remove_file(path) {
                let msg = format!("✗  Failed to delete: {}", e);
                ui.set_status_message(slint::SharedString::from(msg.as_str()));
                ui.set_success(false);
                return;
            }
        }
        let msg = "✓  Shortcut deleted successfully".to_string();
        ui.set_status_message(slint::SharedString::from(msg.as_str()));
        ui.set_success(true);

        let query = ui.get_shortcuts_search_query().to_string();
        load_and_filter_shortcuts(&ui, &query);
    });

    // ─── Save / Update Shortcut ────────────────────────────────────────────
    let ui_weak = ui.as_weak();
    ui.on_save_shortcut(move |file_path, name, exec, icon, terminal| {
        let ui = ui_weak.upgrade().unwrap();
        let file_path_str = file_path.to_string();
        let name_str = name.to_string();
        let exec_str = exec.to_string();
        let icon_str = icon.to_string();

        if name_str.trim().is_empty() {
            ui.set_status_message(slint::SharedString::from("⚠  Application name cannot be empty."));
            ui.set_success(false);
            return;
        }

        match update_desktop_file(&file_path_str, &name_str, &exec_str, &icon_str, terminal) {
            Ok(()) => {
                let msg = format!("✓  Saved changes for {}", name_str);
                ui.set_status_message(slint::SharedString::from(msg.as_str()));
                ui.set_success(true);
                ui.set_is_editing_shortcut(false);

                let query = ui.get_shortcuts_search_query().to_string();
                load_and_filter_shortcuts(&ui, &query);
            }
            Err(e) => {
                let msg = format!("✗  Failed to update shortcut: {}", e);
                ui.set_status_message(slint::SharedString::from(msg.as_str()));
                ui.set_success(false);
            }
        }
    });

    // ─── Create Launcher ───────────────────────────────────────────────────
    let ui_weak = ui.as_weak();
    ui.on_create_launcher(move || {
        let ui = ui_weak.upgrade().unwrap();

        let app_name_str = ui.get_app_name().to_string();
        let exec_path_str = ui.get_exec_path().to_string();
        let icon_path_str = ui.get_icon_path().to_string();
        let run_in_terminal = ui.get_run_in_terminal();

        if app_name_str.trim().is_empty() {
            ui.set_status_message(slint::SharedString::from("⚠  Application name cannot be empty."));
            ui.set_success(false);
            return;
        }

        if exec_path_str.trim().is_empty() {
            ui.set_status_message(slint::SharedString::from("⚠  Executable path cannot be empty. Use Browse or type it in."));
            ui.set_success(false);
            return;
        }

        match create_desktop_file(
            &app_name_str,
            &exec_path_str,
            &icon_path_str,
            run_in_terminal,
        ) {
            Ok(path) => {
                let msg = format!("✓  Launcher created: {}", path);
                ui.set_status_message(slint::SharedString::from(msg.as_str()));
                ui.set_success(true);

                if ui.get_auto_clear_after_create() {
                    ui.set_app_name(slint::SharedString::new());
                    ui.set_exec_path(slint::SharedString::new());
                    ui.set_icon_path(slint::SharedString::new());
                    ui.set_run_in_terminal(false);
                }

                // Refresh shortcuts list so newly created appears immediately
                let query = ui.get_shortcuts_search_query().to_string();
                load_and_filter_shortcuts(&ui, &query);
            }
            Err(e) => {
                let msg = format!("✗  Error: {}", e);
                ui.set_status_message(slint::SharedString::from(msg.as_str()));
                ui.set_success(false);
            }
        }
    });

    // ─── Window Controls ───────────────────────────────────────────────────
    let ui_weak = ui.as_weak();
    ui.on_close_window(move || {
        if let Some(ui) = ui_weak.upgrade() {
            let _ = ui.window().hide();
        }
    });

    let ui_weak = ui.as_weak();
    ui.on_minimize_window(move || {
        if let Some(ui) = ui_weak.upgrade() {
            ui.window().set_minimized(true);
        }
    });

    ui.on_open_url(move |url| {
        let _ = std::process::Command::new("xdg-open").arg(url.as_str()).spawn();
    });

    ui.run()
}
