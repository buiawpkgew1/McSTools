use anyhow::{Context, Result};
use tauri::State;
use crate::database::db_control::DatabaseState;
use crate::database::db_data::{UserData};
use rusqlite::OptionalExtension;
#[tauri::command]
pub fn get_user_data(
    db: State<'_, DatabaseState>,
) -> Result<UserData, String> {
    let conn = db.0.get().map_err(|e| e.to_string())?;
    Ok(conn.query_row(
        "SELECT * FROM user_data WHERE id = 1",
        [],
        |row| {
            Ok(UserData {
                id: row.get("id")?,
                nickname: row.get("nickname")?,
                avatar: row.get("avatar")?,
                qq: row.get("qq")?,
                access_token: row.get("accessToken")?,
                openid: row.get("openid")?,
                schematics: row.get("schematics")?,
                cloud: row.get("cloud")?,
            })
        }
    ).map_err(|e| e.to_string())?)
}