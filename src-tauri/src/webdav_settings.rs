use crate::database::db_control::DatabaseState;
use anyhow::{Context, Result};
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{params, OptionalExtension};
use tauri::State;

#[tauri::command]
pub fn save_webdav_settings(
    db: State<'_, DatabaseState>,
    url: String,
    username: String,
    password: String,
) -> Result<(), String> {
    let conn = db.0.get().map_err(|e| e.to_string())?;

    // 先删除现有的设置
    conn.execute(
        "DELETE FROM webdav_settings",
        params![],
    )
      .map_err(|e| e.to_string())?;

    // 插入新的设置
    conn.execute(
        "INSERT INTO webdav_settings (url, username, password) VALUES (?1, ?2, ?3)",
        params![url, username, password],
    )
      .map_err(|e| e.to_string())?;

    Ok(())
}