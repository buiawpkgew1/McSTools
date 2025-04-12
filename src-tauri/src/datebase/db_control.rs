use anyhow::{Context, Result};
use tauri::{AppHandle, Manager};
use rusqlite::{Connection, params};
use std::path::PathBuf;

fn get_db_path(app: &AppHandle) -> Result<PathBuf> {

    let data_dir = app.path_resolver()
        .app_data_dir()
        .context("无法获取应用数据目录")?;

    if !data_dir.exists() {
        std::fs::create_dir_all(&data_dir)
            .context("创建数据目录失败")?;
    }

    Ok(data_dir.join("mcs_tools.db"))
}

pub fn init_db(app_handle: &AppHandle) -> Result<Connection> {
    let db_path = get_db_path(app_handle)?;
    let conn = Connection::open(&db_path)
        .context("打开数据库连接失败")?;

    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS schematics (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            description TEXT,
            type INTEGER DEFAULT -1,
            bg_type INTEGER DEFAULT -1,
            we_type INTEGER DEFAULT -1,
            is_deleted BLOB DEFAULT FALSE,
            sizes Text,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );

        CREATE INDEX idx_schematic_search
        ON schematics(created_at DESC, name, description);

        CREATE TABLE app_logs (
            id INTEGER PRIMARY KEY,
            timestamp DATETIME DEFAULT CURRENT_TIMESTAMP, --定时情理日志表 config配置具体时间
            level TEXT CHECK(level IN ('TRACE', 'DEBUG', 'INFO', 'WARN', 'ERROR')),
            target TEXT,  -- 日志来源模块
            message TEXT,
            context TEXT
        );

        CREATE INDEX idx_logs_search
        ON app_logs(timestamp DESC, level, target);
        "#
    )?;

    Ok(conn)
}

#[tauri::command]
async fn save_blueprint(
    app_handle: AppHandle,
    name: String,
    data: Vec<u8>
) -> Result<usize, String> {
    let conn = init_db(&app_handle)
        .map_err(|e| e.to_string())?;

    conn.execute(
        "INSERT INTO blueprints (name, data) VALUES (?1, ?2)",
        params![name, data],
    )
        .map_err(|e| e.to_string())
}