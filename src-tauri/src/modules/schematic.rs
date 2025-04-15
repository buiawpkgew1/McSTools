use std::collections::HashSet;
use std::path::Path;
use tauri::{command, State};
use crate::create::create_schematic::CreateSchematic;
use crate::data_files::files::FileManager;
use crate::datebase::db_control::DatabaseState;

#[command]
async fn save_uploaded_file(
    db: State<'_, DatabaseState>,
    file_manager: State<'_, FileManager>,
    file_name: String,
    data: Vec<u8>,
    size: usize,
) -> Result<(), String> {
    if data.len() != size {
        return Err("异常的输入文件".into());
    }

    let path = Path::new(&file_name);
    let file_ext = path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_lowercase());

    match file_ext.unwrap().as_str() {
        "nbt" => {
            let schematic = CreateSchematic::new_from_bytes(data)?;
            let size = schematic.get_size()?;
            let user = "user";
            let type_v = 1;


        },
        "json" => {

        },
        "schem" => {

        },
        "litematic" => {

        },
        _ => {}
    }
    Ok(())
}