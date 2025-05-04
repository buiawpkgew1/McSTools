use tauri::State;
use crate::building_gadges::to_bg_schematic::ToBgSchematic;
use crate::create::to_create_schematic::ToCreateSchematic;
use crate::data_files::files::FileManager;
use crate::database::db_apis::schematics_api::{find_schematic, new_schematic};
use crate::database::db_control::DatabaseState;
use crate::database::db_data::Schematic;
use crate::litematica::to_lm_schematic::ToLmSchematic;
use crate::modules::replace_data::ReplacementRule;
use crate::word_edit::to_we_schematic::ToWeSchematic;

#[tauri::command]
async fn schematic_replacement(
    rules: Vec<ReplacementRule>,
    db: State<'_, DatabaseState>,
    file_manager: State<'_, FileManager>,
) -> Result<bool, String> {
    async move {
        let schematic_id = rules[0].schematic_id.clone();
        let mut conn = db.0.get()?;
        let mut schematic = find_schematic(&mut conn, schematic_id)?;
        let version = schematic.version;
        let sub_version = schematic.sub_type;
        let v_type = schematic.schematic_type;
        let data = file_manager.get_schematic_data(schematic_id, version, sub_version, v_type)?;
        match v_type {
            1 => {
                let data = ToCreateSchematic::new(&data).create_schematic(true);
                schematic.name = format!("replace_{}", schematic_id);
                let new_id = new_schematic(&mut conn, schematic)?;
                let path = file_manager.save_nbt_value(new_id, data, version, sub_version, v_type, true)?;
                Ok(path)
            }
            2 => {
                let data = ToLmSchematic::new(&data).lm_schematic(sub_version);
                schematic.name = format!("replace_{}", schematic_id);
                let new_id = new_schematic(&mut conn, schematic)?;
                let path = file_manager.save_nbt_value(new_id, data, version, sub_version, v_type, true)?;
                Ok(path)
            }
            3 => {
                let data = ToWeSchematic::new(&data).we_schematic(sub_version)?;
                schematic.name = format!("replace_{}", schematic_id);
                let new_id = new_schematic(&mut conn, schematic)?;
                let path = file_manager.save_nbt_value(new_id, data, version, sub_version, v_type, true)?;
                Ok(path)
            }
            4 => {
                let data = ToBgSchematic::new(&data).bg_schematic()?;
                schematic.name = format!("replace_{}", schematic_id);
                let new_id = new_schematic(&mut conn, schematic)?;
                let path = file_manager.save_json_value(new_id, data, version, sub_version, v_type)?;
                Ok(path)
            }
            //5 => {}
            _ => {anyhow::bail!("unknown schematic type: {}", v_type);}
        }
        Ok(true)
    }
        .await
        .map_err(|e: anyhow::Error| e.to_string())

}