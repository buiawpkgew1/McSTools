use tauri::State;
use crate::data_files::files::FileManager;
use crate::database::db_apis::schematics_api::find_schematic;
use crate::database::db_control::DatabaseState;
use crate::modules::convert_data::ConvertData;

#[tauri::command]
pub async fn get_schematic_convert_data(
    db: State<'_, DatabaseState>,
    file_manager: State<'_, FileManager>,
    id: i64,
) -> anyhow::Result<ConvertData, String> {
    async move {
        let mut conn = db.0.get()?;
        let schematic = find_schematic(&mut conn, id)?;
        let version = schematic.version;
        let sub_version = schematic.sub_type;
        let v_type = schematic.schematic_type;
        let data = file_manager.get_convert_data(id, version, sub_version, v_type)?;
        Ok(data)
    }
        .await
        .map_err(|e: anyhow::Error| e.to_string())
}