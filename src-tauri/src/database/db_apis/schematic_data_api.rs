use crate::database::db_control::DatabaseState;
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;
use tauri::State;

pub fn new_schematic_data(
    conn: &mut PooledConnection<SqliteConnectionManager>,
    schematic_id: i64,
    metadata: String,
    unique_blocks: String,
) -> anyhow::Result<i64> {
    let tx = conn.transaction()?;
    tx.execute(
        r#"INSERT INTO schematic_data (
            schematic_id, requirements, unique_blocks
        ) VALUES (?1, ?2, ?3)"#,
        params![schematic_id, metadata, unique_blocks],
    )?;
    let rowid = tx.last_insert_rowid();
    tx.commit()?;

    Ok(rowid)
}

pub fn update_schematic_data(
    conn: &mut PooledConnection<SqliteConnectionManager>,
    schematic_id: i64,
    metadata: String,
    unique_blocks: String,
) -> anyhow::Result<i64> {
    let tx = conn.transaction()?;
    tx.execute(
        r#"UPDATE schematic_data
        SET
            requirements = ?1,
            unique_blocks = ?2
        WHERE schematic_id = ?3"#,
        params![metadata, unique_blocks, schematic_id],
    )?;
    let rowid = tx.last_insert_rowid();
    tx.commit()?;

    Ok(rowid)
}
#[tauri::command]
pub fn get_schematic_requirements(
    db: State<'_, DatabaseState>,
    id: i64,
) -> anyhow::Result<String, String> {
    let conn = db.0.get().map_err(|e| e.to_string())?;

    conn.query_row(
        "SELECT requirements FROM schematic_data WHERE schematic_id = ?1",
        [id],
        |row| {
            let metadata_str: String = row.get("requirements")?;
            Ok(metadata_str)
        },
    )
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_unique_block(db: State<'_, DatabaseState>, id: i64) -> anyhow::Result<String, String> {
    let conn = db.0.get().map_err(|e| e.to_string())?;

    conn.query_row(
        "SELECT unique_blocks FROM schematic_data WHERE schematic_id = ?1",
        [id],
        |row| {
            let unique_block_str: String = row.get("unique_blocks")?;
            Ok(unique_block_str)
        },
    )
    .map_err(|e| e.to_string())
}
