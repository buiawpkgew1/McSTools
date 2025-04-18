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
mod database;
mod modules;
use utils::minecraft_data::versions_data::VersionData;
use tauri::Manager;
use data_files::{config, config::get_config, config::update_config, files::FileManager};
use crate::database::db_control;
use modules::schematic::encode_uploaded_schematic;
use database::db_apis::schematics_api::{add_schematic, get_schematic, get_schematics};
use database::db_apis::logs_api::{add_logs, get_logs};
use database::db_apis::user_api::{get_user_data};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let db_state = db_control::init_db(app.handle())?;
            app.manage(db_state);
            let config = config::init_config(app.handle())
                .expect("配置系统初始化失败");
            app.manage(config);
            let file_manager = FileManager::new(app.handle())?;
            app.manage(file_manager);
            let version_data = VersionData::new();
            app.manage(version_data);
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            get_config,
            update_config,
            encode_uploaded_schematic,
            get_user_data,
            add_logs,
            get_logs,
            add_schematic,
            get_schematic,
            get_schematics
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
