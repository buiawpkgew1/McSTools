use std::path::Path;
use fastnbt::Value;
use anyhow::Result;
use tauri::{State};
use crate::building_gadges::bg_schematic::BgSchematic;
use crate::create::create_schematic::CreateSchematic;
use crate::data_files::files::FileManager;
use crate::database::db_apis::schematics_api::{new_requirements, new_schematic, find_schematic};
use crate::database::db_control::DatabaseState;
use crate::database::db_data::Schematic;
use crate::litematica::lm_schematic::LmSchematic;
use crate::utils::minecraft_data::je_blocks_data::BlocksData;
use crate::utils::minecraft_data::versions_data::VersionData;
use crate::utils::requirements::{get_requirements, RequirementStr};
use crate::word_edit::we_schematic::WeSchematic;

#[tauri::command]
pub async fn encode_uploaded_schematic(
    db: State<'_, DatabaseState>,
    file_manager: State<'_, FileManager>,
    versions_data: State<'_, VersionData>,
    je_blocks: State<'_, BlocksData>,
    file_name: String,
    data: Vec<u8>,
) -> Result<(), String> {
    async move {
        let path = Path::new(&file_name);
        let (file_ext_str, file_name_str) = {
            let ext = path.extension()
                .and_then(|e| e.to_str())
                .map(|e| e.to_lowercase())
                .unwrap_or_else(|| "unknown".into());

            let name = path.file_name()
                .and_then(|n| n.to_str())
                .map(|n| n.to_lowercase())
                .unwrap_or_else(|| "unnamed".into());

            (ext, name)
        };
        match file_ext_str.as_str() {
            "nbt" => {
                let original_data = data.clone();
                let schematic = CreateSchematic::new_from_bytes(data)?;
                let schematic_data = schematic.get_blocks_pos()?;
                let requirement = get_requirements(&schematic_data.blocks)?;
                let requirements_str = RequirementStr::from_requirements(&requirement, &je_blocks).export_to_string()?;
                let size = schematic.get_size()?;
                let sizes = match size {
                    list => list.iter()
                        .filter_map(|v| match v {
                            Value::Int(n) => Some(*n),
                            _ => None
                        })
                        .collect::<Vec<i32>>(),
                };

                let sizes_str = sizes.iter()
                    .map(|n| n.to_string())
                    .collect::<Vec<_>>()
                    .join(",");

                let mut conn = db.0.get()?;
                let data_version = schematic.get_data_version()?;
                let game_version = versions_data
                    .get_name(data_version)
                    .map(|arc_str| arc_str.to_string())
                    .unwrap_or_else(|| "unknown_version".to_string());
                let schematic = Schematic {
                    id: 0,
                    name: file_name_str,
                    description: "".parse()?,
                    schematic_type: 1,
                    sub_type: -1,
                    is_deleted: false,
                    sizes: sizes_str,
                    user: "your".parse()?,
                    is_upload: false,
                    version: 0,
                    version_list: "0".parse()?,
                    created_at: "".parse()?,
                    updated_at: "".parse()?,
                    game_version,
                };

                let schematic_id = new_schematic(&mut conn, schematic)?;
                new_requirements(&mut conn, schematic_id, requirements_str)?;
                file_manager.save_schematic_data(
                    schematic_id,
                    original_data,
                    0,
                    -1,
                    1,
                    file_ext_str,
                )?
            },
            "json" => {
                let original_data = data.clone();

                let schematic = BgSchematic::new_from_data(data)?;
                let schematic_data = schematic.get_blocks_pos()?;
                let requirement = get_requirements(&schematic_data.blocks)?;
                let requirements_str = RequirementStr::from_requirements(&requirement, &je_blocks).export_to_string()?;

                let sizes = schematic.get_size()?;
                let schematic_type = schematic.get_type()?;
                let mut conn = db.0.get()?;

                let schematic = Schematic {
                    id: 0,
                    name: file_name_str,
                    description: "".parse()?,
                    schematic_type: 4,
                    sub_type: schematic_type,
                    is_deleted: false,
                    sizes: sizes.to_string(),
                    user: "your".parse()?,
                    is_upload: false,
                    version: 0,
                    version_list: "0".parse()?,
                    created_at: "".parse()?,
                    updated_at: "".parse()?,
                    game_version: "".parse()?,
                };

                let schematic_id = new_schematic(&mut conn, schematic)?;
                new_requirements(&mut conn, schematic_id, requirements_str)?;
                file_manager.save_schematic_data(
                    schematic_id,
                    original_data,
                    0,
                    schematic_type,
                    4,
                    file_ext_str,
                )?
            },
            "schem" => {
                let original_data = data.clone();
                let schematic = WeSchematic::new_from_bytes(data)?;
                let schematic_data = schematic.get_blocks_pos()?;
                let requirement = get_requirements(&schematic_data.blocks)?;
                let requirements_str = RequirementStr::from_requirements(&requirement, &je_blocks).export_to_string()?;
                let type_version = schematic.get_type()?;
                let sizes = schematic.get_size(type_version)?;
                let sizes_str = sizes.to_string();
                let mut conn = db.0.get()?;
                let data_version = schematic.get_data_version(type_version)?;
                let game_version = versions_data
                    .get_name(data_version)
                    .map(|arc_str| arc_str.to_string())
                    .unwrap_or_else(|| "unknown_version".to_string());
                let schematic = Schematic {
                    id: 0,
                    name: file_name_str,
                    description: "".parse()?,
                    schematic_type: 3,
                    sub_type: type_version,
                    is_deleted: false,
                    sizes: sizes_str,
                    user: "your".parse()?,
                    is_upload: false,
                    version: 0,
                    version_list: "0".parse()?,
                    created_at: "".parse()?,
                    updated_at: "".parse()?,
                    game_version,
                };
                let schematic_id = new_schematic(&mut conn, schematic)?;
                new_requirements(&mut conn, schematic_id, requirements_str)?;
                file_manager.save_schematic_data(
                    schematic_id,
                    original_data,
                    0,
                    type_version,
                    3,
                    file_ext_str,
                )?
            },
            "litematic" => {
                let original_data = data.clone();
                let schematic = LmSchematic::new_from_bytes(data)?;
                let schematic_data = schematic.get_blocks_pos()?;
                let requirement = get_requirements(&schematic_data.blocks)?;
                let requirements_str = RequirementStr::from_requirements(&requirement, &je_blocks).export_to_string()?;
                let metadata = schematic.read_metadata()?;
                let sizes_pos = metadata.enclosing_size;
                let description = metadata.description;
                let author = metadata.author;
                let mut conn = db.0.get()?;
                let data_version = schematic.get_data_version()?;
                let game_version = versions_data
                    .get_name(data_version)
                    .map(|arc_str| arc_str.to_string())
                    .unwrap_or_else(|| "unknown_version".to_string());
                let name = if metadata.name.trim() == "Unnamed" {
                    file_name_str
                }else {
                    metadata.name
                };
                let schematic = Schematic {
                    id: 0,
                    name,
                    description,
                    schematic_type: 2,
                    sub_type: -1,
                    is_deleted: false,
                    sizes: sizes_pos.to_string(),
                    user: author,
                    is_upload: false,
                    version: 0,
                    version_list: "0".parse()?,
                    created_at: "".parse()?,
                    updated_at: "".parse()?,
                    game_version,
                };
                let schematic_id = new_schematic(&mut conn, schematic)?;
                new_requirements(&mut conn, schematic_id, requirements_str)?;
                file_manager.save_schematic_data(
                    schematic_id,
                    original_data,
                    0,
                    -1,
                    2,
                    file_ext_str,
                )?
            },
            _ => {
                let mut conn = db.0.get()?;
                let original_data = data.clone();
                let schematic = Schematic {
                    id: 0,
                    name: "未解析".parse()?,
                    description: "".parse()?,
                    schematic_type: -1,
                    sub_type: -1,
                    is_deleted: false,
                    sizes: "".to_string(),
                    user: "your".parse()?,
                    is_upload: false,
                    version: 0,
                    version_list: "0".parse()?,
                    created_at: "".parse()?,
                    updated_at: "".parse()?,
                    game_version: "".parse()?,
                };
                let schematic_id = new_schematic(&mut conn, schematic)?;
                new_requirements(&mut conn, schematic_id, "{}".to_string())?;
                file_manager.save_schematic_data(
                    schematic_id,
                    original_data,
                    0,
                    -1,
                    -1,
                    file_ext_str,
                )?
            },
        };
        Ok(())
    }
        .await
        .map_err(|e: anyhow::Error| e.to_string())

}

#[tauri::command]
pub async fn get_schematic_data(
    db: State<'_, DatabaseState>,
    file_manager: State<'_, FileManager>,
    id: i64
) -> Result<String, String> {
    async move {
        let mut conn = db.0.get()?;
        let schematic = find_schematic(&mut conn, id)?;
        let version = schematic.version;
        let sub_version = schematic.sub_type;
        let v_type = schematic.schematic_type;
        let str = file_manager.read_schematic_str(id, version, sub_version, v_type)?;
        Ok(str)
    }
        .await
        .map_err(|e: anyhow::Error| e.to_string())
}
