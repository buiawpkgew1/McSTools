use anyhow::{Context, Result};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

type SqlitePool = Pool<SqliteConnectionManager>;

#[derive(Clone)]
pub struct DatabaseState(pub SqlitePool);

fn get_db_path(app: &AppHandle) -> Result<PathBuf> {
    let data_dir = app
        .path()
        .app_data_dir()
        .context("无法获取应用数据目录")?
        .join("data");

    if !data_dir.exists() {
        std::fs::create_dir_all(&data_dir).context("创建数据目录失败")?;
    }

    Ok(data_dir)
}

pub fn init_db(app_handle: &AppHandle) -> Result<DatabaseState> {
    let db_path = get_db_path(app_handle)?.join("mcs_tools.db");
    let manager = SqliteConnectionManager::file(db_path)
        .with_flags(
            rusqlite::OpenFlags::SQLITE_OPEN_READ_WRITE | rusqlite::OpenFlags::SQLITE_OPEN_CREATE,
        )
        .with_init(|conn| {
            conn.execute_batch(
                "PRAGMA journal_mode = WAL;
                 PRAGMA synchronous = NORMAL;
                 PRAGMA foreign_keys = ON;",
            )
        });

    let pool = Pool::builder()
        .max_size(5)
        .build(manager)
        .context("创建连接池失败")?;

    let conn = pool.get()?;
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS schematics (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            description TEXT DEFAULT '',
            type INTEGER DEFAULT -1, -- TYPE-> nbt 1 litematic 2 schem 3 json 4 mcstruct 5
            sub_type INTEGER DEFAULT -1, -- SUB Schem 0 新 1 旧 json 0 1.20+ 1 1.16+ 2 1.12+
            is_deleted BLOB DEFAULT FALSE,
            sizes TEXT DEFAULT '',
            user TEXT DEFAULT '', -- 简单的记录用户名，个人存储应该不太需要详细记录
            version INTEGER DEFAULT 0,
            game_version TEXT DEFAULT '',
            version_list TEXT DEFAULT '', -- 版本控制器记录版本号id
            is_upload BLOB DEFAULT FALSE,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );

        CREATE INDEX IF NOT EXISTS idx_schematic_search
        ON schematics(created_at DESC, name, description);
        
        CREATE TABLE IF NOT EXISTS requirements (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            schematic_id INTEGER NOT NULL,
            metadata TEXT DEFAULT '{}', -- 元数据（JSON格式存储）
            
            FOREIGN KEY (
                schematic_id
            ) REFERENCES schematics (
                id
            ) ON DELETE CASCADE, 
            
            UNIQUE(schematic_id)
        );
        CREATE INDEX IF NOT EXISTS idx_requirements_schematic 
        ON requirements(schematic_id);
        
        CREATE TABLE IF NOT EXISTS app_logs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
            level TEXT DEFAULT 'INFO' CHECK(level IN ('TRACE', 'DEBUG', 'INFO', 'WARN', 'ERROR')),
            target TEXT DEFAULT '',
            message TEXT DEFAULT '',
            context TEXT DEFAULT ''
        );

        CREATE INDEX IF NOT EXISTS idx_logs_search
        ON app_logs(timestamp DESC, level, target);

        CREATE TABLE IF NOT EXISTS user_data (
            id INTEGER PRIMARY KEY,
            nickname TEXT DEFAULT '',
            avatar TEXT DEFAULT '',
            qq TEXT DEFAULT '',
            accessToken TEXT DEFAULT '', -- qq登录凭证
            openid TEXT DEFAULT '',-- qq登录唯一身份码
            schematics INTEGER DEFAULT 0,
            cloud INTEGER DEFAULT 0
        );

        INSERT INTO user_data (id, nickname, avatar, qq, accessToken, openid, schematics, cloud)
        SELECT 1, '', '', '', '', '', 0, 0
        WHERE NOT EXISTS (SELECT 1 FROM user_data WHERE id = 1);
        "#,
    )?;

    Ok(DatabaseState(pool))
}
