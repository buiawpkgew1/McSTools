use rusqlite::params;
use tauri::State;
use crate::datebase::db_control::DatabaseState;
use crate::datebase::db_data::LogEntry;

#[tauri::command]
pub fn get_logs(
    db: State<'_, DatabaseState>,
    filter: &str,
    page: i32,
    page_size: i32
) -> anyhow::Result<Vec<LogEntry>> {
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
        .collect::<anyhow::Result<Vec<_>, _>>()?;

    Ok(logs)
}

#[tauri::command]
pub fn add_logs(
    db: State<'_, DatabaseState>,
    log: LogEntry
) -> anyhow::Result<i64, String> {
    let conn = db.0.get()?;

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