use anyhow::{Context, Result};
use rusqlite::{params, OptionalExtension};
use tauri::{State};
use crate::datebase::db_control::DatabaseState;
use crate::datebase::db_data::{LogEntry, Schematic};

#[tauri::command]
pub fn add_schematic(
    db: State<'_, DatabaseState>,
    schematic: Schematic
) -> Result<i64, String> {
    let conn = db.0.get().map_err(|e| e.to_string())?;

    conn.execute(
        r#"INSERT INTO schematics (
            name, description, type, bg_type,
            we_type, is_deleted, sizes
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)"#,
        params![
            schematic.name,
            schematic.description,
            schematic.schematic_type,
            schematic.bg_type,
            schematic.we_type,
            schematic.is_deleted,
            schematic.sizes
        ],
    ).map_err(|e| e.to_string())?;
    Ok(conn.last_insert_rowid())
}

#[tauri::command]
pub fn add_logs(
    db: State<'_, DatabaseState>,
    log: LogEntry
) -> Result<i64, String> {
    let conn = db.0.get().map_err(|e| e.to_string())?;

    conn.execute(
        "INSERT INTO app_logs (level, target, message, context)
            VALUES (?1, ?2, ?3, ?4)",
        params![
                log.level,
                log.target,
                log.message,
                log.context
            ],
    ).map_err(|e| e.to_string())?;
    Ok(conn.last_insert_rowid())
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
                bg_type: row.get("bg_type")?,
                we_type: row.get("we_type")?,
                is_deleted: row.get("is_deleted")?,
                sizes: row.get("sizes")?,
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
                    bg_type: row.get("bg_type")?,
                    we_type: row.get("we_type")?,
                    is_deleted: row.get("is_deleted")?,
                    sizes: row.get("sizes")?,
                })
            },
        )?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(schematics)
}

#[tauri::command]
pub fn get_logs(
    db: State<'_, DatabaseState>,
    filter: &str,
    page: i32,
    page_size: i32
) -> Result<Vec<LogEntry>> {
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
        SELECT * FROM app_logs
        WHERE
            (?1 = '' OR
            message LIKE ?1)
        ORDER BY timestamp DESC
        LIMIT ?2 OFFSET ?3
        "#
    )?;

    let logs = stmt
        .query_map(
            rusqlite::params![
                search_pattern,
                page_size,
                offset
            ],
            |row| {
                Ok(LogEntry {
                    level: row.get("level")?,
                    target: row.get("target")?,
                    message: row.get("message")?,
                    context: row.get("context")?,
                })
            },
        )?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(logs)
}