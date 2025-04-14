#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
pub mod utils;
pub mod litematica;
pub mod create;
mod building_gadges;
mod be_schematic;
mod word_edit;
mod data_files;
mod datebase;

use tauri::Manager;
use data_files::{config, config::get_config, config::update_config};
use crate::datebase::db_control;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let db_state = db_control::init_db(app.handle())?;
            app.manage(db_state);
            config::init_config(app.handle())
                .expect("配置系统初始化失败");
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            get_config,
            update_config
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
