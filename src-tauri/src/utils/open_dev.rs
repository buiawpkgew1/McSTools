use tauri::Manager;

#[tauri::command]
pub async fn open_dev(
    app: tauri::AppHandle
) -> Result<(), String> {
    #[cfg(all(debug_assertions, target_os = "windows"))]
    {
        let window = app.get_webview_window("main").unwrap();
        window.open_devtools();
    }

    Ok(())
}