use anyhow::{Result};
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{params};
use tauri::{State};
use crate::building_gadges::bg_schematic_data::JsonData;
use crate::database::db_control::DatabaseState;
use crate::database::db_data::{PaginatedResponse, Schematic};
use crate::utils::schematic_data::SchematicError;

pub fn new_schematic(
    mut conn: &mut PooledConnection<SqliteConnectionManager>,
    schematic: Schematic,
) -> Result<i64> {
    let tx = conn.transaction()?;
    tx.execute(
        r#"INSERT INTO schematics (
            name, description, type, sub_type,
            sizes, user, version_list, game_version
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)"#,
        params![
            schematic.name,
            schematic.description,
            schematic.schematic_type,
            schematic.sub_type,
            schematic.sizes,
            schematic.user,
            schematic.version_list,
            schematic.game_version
        ],
    )?;
    let rowid = tx.last_insert_rowid();
    tx.commit()?;

    Ok(rowid)
}

pub fn new_requirements(
    conn: &mut PooledConnection<SqliteConnectionManager>,
    schematic_id: i64,
    metadata: String,
) -> Result<i64> {
    let tx = conn.transaction()?;
    tx.execute(
        r#"INSERT INTO requirements (
            schematic_id, metadata
        ) VALUES (?1, ?2)"#,
        params![
            schematic_id,
            metadata,
        ],
    )?;
    let rowid = tx.last_insert_rowid();
    tx.commit()?;

    Ok(rowid)
}
#[tauri::command]
pub fn add_schematic(
    db: State<'_, DatabaseState>,
    schematic: Schematic
) -> Result<i64, String> {
    let mut conn = db.0.get().map_err(|e| e.to_string())?;

    let new = new_schematic(&mut conn, schematic).map_err(|e| e.to_string())?;
    Ok(new)
}
#[tauri::command]
pub fn get_schematic(
    db: State<'_, DatabaseState>,
    id: i64
) -> Result<Schematic, String> {
    let conn = db.0.get().map_err(|e| e.to_string())?;
    Ok(conn.query_row(
        "SELECT * FROM schematics WHERE id = ? AND is_deleted = FALSE",
        [id],
        |row| {
            Ok(Schematic {
                id: row.get("id")?,
                name: row.get("name")?,
                description: row.get("description")?,
                schematic_type: row.get("type")?,
                sub_type: row.get("sub_type")?,
                is_deleted: row.get("is_deleted")?,
                sizes: row.get("sizes")?,
                user: row.get("user")?,
                is_upload: row.get("is_upload")?,
                version: row.get("version")?,
                version_list: row.get("version_list")?,
                created_at: row.get("created_at")?,
                updated_at: row.get("updated_at")?,
                game_version: row.get("game_version")?,
            })
        }
    ).map_err(|e| e.to_string())?)
}

#[tauri::command]
pub fn get_requirements(
    db: State<'_, DatabaseState>,
    id: i64
) -> Result<String, String> {
    let conn = db.0.get().map_err(|e| e.to_string())?;

    conn.query_row(
        "SELECT metadata FROM requirements WHERE schematic_id = ?1", 
        [id],
        |row| {
            let metadata_str: String = row.get("metadata")?;
            Ok(metadata_str)
        }
    )
        .map_err(|e| e.to_string()) 
}

#[tauri::command]
pub fn get_schematics(
    db: State<'_, DatabaseState>,
    filter: &str,
    page: i32,
    page_size: i32
) -> Result<PaginatedResponse<Schematic>, String> {
    let conn = db.0.get().map_err(|e| e.to_string())?;
    let page = page.max(1);
    let page_size = page_size.clamp(1, 100);

    let offset = (page - 1) * page_size;
    let search_pattern = if filter.is_empty() {
        "".to_string()
    } else {
        format!("%{}%", filter)
    };
    let mut stmt = conn.prepare(
        r#"
        SELECT * FROM schematics
        WHERE
            (?1 = '' OR
            (name LIKE ?1 OR description LIKE ?1))
            AND is_deleted = FALSE
        ORDER BY created_at DESC
        LIMIT ?2 OFFSET ?3
        "#
    ).map_err(|e| e.to_string())?;

    let schematics = stmt
        .query_map(
            rusqlite::params![
                search_pattern,
                page_size,
                offset
            ],
            |row| {
                Ok(Schematic {
                    id: row.get("id")?,
                    name: row.get("name")?,
                    description: row.get("description")?,
                    schematic_type: row.get("type")?,
                    sub_type: row.get("sub_type")?,
                    is_deleted: row.get("is_deleted")?,
                    sizes: row.get("sizes")?,
                    user: row.get("user")?,
                    is_upload: row.get("is_upload")?,
                    version: row.get("version")?,
                    version_list: row.get("version_list")?,
                    created_at: row.get("created_at")?,
                    updated_at: row.get("updated_at")?,
                    game_version: row.get("game_version")?,
                })
            },
        ).map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(PaginatedResponse {
        data: schematics,
        page,
        page_size,
    })
}

