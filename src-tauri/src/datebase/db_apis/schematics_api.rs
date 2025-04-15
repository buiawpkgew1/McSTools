use anyhow::{Context, Result};
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{params, OptionalExtension};
use tauri::{State};
use crate::datebase::db_control::DatabaseState;
use crate::datebase::db_data::{LogEntry, Schematic};


pub fn new_schematic(conn: PooledConnection<SqliteConnectionManager>, schematic: Schematic) -> Result<i64, String> {
    conn.execute(
        r#"INSERT INTO schematics (
            name, description, type, sub_type,
            sizes, user
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6)"#,
        params![
            schematic.name,
            schematic.description,
            schematic.schematic_type,
            schematic.sub_type,
            schematic.sizes,
            schematic.user
        ],
    ).map_err(|e| e.to_string())?;
    Ok(conn.last_insert_rowid())
}
#[tauri::command]
pub fn add_schematic(
    db: State<'_, DatabaseState>,
    schematic: Schematic
) -> Result<i64, String> {
    let conn = db.0.get()?;

    let new = new_schematic(conn, schematic)?;
    Ok(new)
}
#[tauri::command]
pub fn get_schematic(
    db: State<'_, DatabaseState>,
    id: i64
) -> Result<Option<Schematic>> {
    let conn = db.0.get()?;
    conn.query_row(
        "SELECT * FROM schematics WHERE id = ? AND is_deleted = FALSE",
        [id],
        |row| {
            Ok(Schematic {
                id: Some(row.get("id")?),
                name: row.get("name")?,
                description: row.get("description")?,
                schematic_type: row.get("type")?,
                sub_type: row.get("sub_type")?,
                is_deleted: row.get("is_deleted")?,
                sizes: row.get("sizes")?,
                user: row.get("user")?,
                is_upload: row.get("is_upload")?,
                version: row.get("version")?,
            })
        }
    ).optional().context("查询失败")
}

#[tauri::command]
pub fn get_schematics(
    db: State<'_, DatabaseState>,
    filter: &str,
    page: i32,
    page_size: i32
) -> Result<Vec<Schematic>> {
    let conn = db.0.get()?;
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
    )?;

    let schematics = stmt
        .query_map(
            rusqlite::params![
                search_pattern,
                page_size,
                offset
            ],
            |row| {
                Ok(Schematic {
                    id: Some(row.get("id")?),
                    name: row.get("name")?,
                    description: row.get("description")?,
                    schematic_type: row.get("type")?,
                    sub_type: row.get("sub_type")?,
                    is_deleted: row.get("is_deleted")?,
                    sizes: row.get("sizes")?,
                    user: row.get("user")?,
                    is_upload: row.get("is_upload")?,
                    version: row.get("version")?,
                })
            },
        )?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(schematics)
}

