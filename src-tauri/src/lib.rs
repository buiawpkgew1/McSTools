#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
mod be_schematic;
mod building_gadges;
pub mod create;
mod data_files;
mod database;
pub mod litematica;
mod modules;
pub mod utils;
mod word_edit;
mod map_art;

use crate::database::db_control;
use crate::utils::minecraft_data::je_blocks_data::BlocksData;
use data_files::{config, config::get_config, config::update_config, files::FileManager};
use database::db_apis::logs_api::{add_logs, get_logs};
use database::db_apis::schematics_api::{add_schematic, get_schematic, get_schematics};
use database::db_apis::schematic_data_api::{get_schematic_requirements, get_unique_block};
use database::db_apis::user_api::get_user_data;
use modules::schematic::{encode_uploaded_schematic, get_schematic_data};
use modules::convert::{get_schematic_convert_data, get_je_blocks};
use tauri::Manager;
use utils::minecraft_data::versions_data::VersionData;
use utils::loading::close_splashscreen;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .setup(|app| {
            let db_state = db_control::init_db(app.handle())?;
            app.manage(db_state);
            let config = config::init_config(app.handle()).expect("配置系统初始化失败");
            app.manage(config);
            let file_manager = FileManager::new(app.handle())?;
            app.manage(file_manager);
            let version_data = VersionData::new();
            app.manage(version_data);
            let je_blocks = BlocksData::new()?;
            app.manage(je_blocks);
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            close_splashscreen,
            greet,
            get_config,
            update_config,
            encode_uploaded_schematic,
            get_user_data,
            add_logs,
            get_je_blocks,
            get_logs,
            add_schematic,
            get_schematic,
            get_schematics,
            get_schematic_requirements,
            get_unique_block,
            get_schematic_data,
            get_schematic_convert_data
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
