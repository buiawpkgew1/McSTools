use std::path::Path;
use fastnbt::Value;
use anyhow::Result;
use rusqlite::fallible_iterator::FallibleIterator;
use tauri::{State};
use crate::building_gadges::bg_schematic::BgSchematic;
use crate::create::create_schematic::CreateSchematic;
use crate::data_files::files::FileManager;
use crate::database::db_apis::schematics_api::new_schematic;
use crate::database::db_control::DatabaseState;
use crate::database::db_data::Schematic;
use crate::litematica::lm_schematic::LmSchematic;
use crate::word_edit::we_schematic::WeSchematic;

#[tauri::command]
pub async fn encode_uploaded_schematic(
    db: State<'_, DatabaseState>,
    file_manager: State<'_, FileManager>,
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
                let size = schematic.get_size()?;
                let sizes = match size {
                    list => list.iter()
                        .filter_map(|v| match v {
                            Value::Int(n) => Some(*n as i32),
                            _ => None
                        })
                        .collect::<Vec<i32>>(),
                };

                let sizes_str = sizes.iter()
                    .map(|n| n.to_string())
                    .collect::<Vec<_>>()
                    .join(",");

                let conn = db.0.get()?;

                let schematic = Schematic {
                    id: None,
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
                };

                let schematic_id = new_schematic(conn, schematic)?;
                println!("{}", schematic_id);
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
                let sizes = schematic.get_size()?;
                let schematic_type = schematic.get_type()?;
                let conn = db.0.get()?;

                let schematic = Schematic {
                    id: None,
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
                };

                let schematic_id = new_schematic(conn, schematic)?;

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
                let type_version = schematic.get_type()?;
                let sizes = schematic.get_size(type_version)?;
                let sizes_str = sizes.to_string();
                let conn = db.0.get()?;
                let schematic = Schematic {
                    id: None,
                    name: file_name_str,
                    description: "".parse()?,
                    schematic_type: 2,
                    sub_type: type_version,
                    is_deleted: false,
                    sizes: sizes_str,
                    user: "your".parse()?,
                    is_upload: false,
                    version: 0,
                    version_list: "0".parse()?,
                };
                let schematic_id = new_schematic(conn, schematic)?;
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
                let metadata = schematic.read_metadata()?;
                let sizes_pos = metadata.enclosing_size;
                let description = metadata.description;
                let author = metadata.author;
                let conn = db.0.get()?;
                let name = if metadata.name.trim() == "Unnamed" {
                    file_name_str
                }else {
                    metadata.name
                };
                let schematic = Schematic {
                    id: None,
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
                };
                let schematic_id = new_schematic(conn, schematic)?;
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
                let conn = db.0.get()?;
                let original_data = data.clone();
                let schematic = Schematic {
                    id: None,
                    name: "未解析".parse()?,
                    description: "".parse()?,
                    schematic_type: 2,
                    sub_type: -1,
                    is_deleted: false,
                    sizes: "".to_string(),
                    user: "your".parse()?,
                    is_upload: false,
                    version: 0,
                    version_list: "0".parse()?,
                };
                let schematic_id = new_schematic(conn, schematic)?;
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