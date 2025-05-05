use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{params, Transaction};
use serde_json::{json, Value};
use anyhow::{Context, Result};
use crate::modules::history_data::HistoryRecord;

pub fn new_history(
    conn: &mut PooledConnection<SqliteConnectionManager>,
    schematic_id: i64,
    schematic: String,
    requirements: String,
    unique_blocks: String
) -> Result<i64> {
    let tx = conn.transaction()?;
    let processed_schematic = convert_to_array(schematic).context("schematic json error")?;
    let processed_requirements = convert_to_array(requirements).context("schematic json error")?;
    let processed_unique_blocks = convert_to_array(unique_blocks).context("schematic json error")?;
    tx.execute(
        r#"INSERT INTO schematics_history (
            schematic_id, schematic, requirements, unique_blocks
        ) VALUES (?1, ?2, ?3, ?4)"#,
        params![schematic_id, processed_schematic, processed_requirements, processed_unique_blocks],
    )?;
    let rowid = tx.last_insert_rowid();
    tx.commit()?;

    Ok(rowid)
}

pub fn update_history(
    conn: &mut PooledConnection<SqliteConnectionManager>,
    schematic_id: i64,
    new_schematic: String,
    requirements: String,
    unique_blocks: String
) -> Result<i64> {
    let tx = conn.transaction()?;

    let old_record = get_history_record(&tx, schematic_id)?;
    let (old_schematic, old_requirements, old_blocks) = match old_record {
        Some(ref record) => (
            Some(&record.schematic),
            Some(&record.requirements),
            Some(&record.unique_blocks)
        ),
        None => (None, None, None)
    };
    let schematic_json = build_updated_schematic(old_schematic, new_schematic)?;
    let requirements_json = build_updated_schematic(old_requirements, requirements)?;
    let unique_blocks_json = build_updated_schematic(old_blocks, unique_blocks)?;

    tx.execute(
        r#"UPDATE schematics_history
        SET
            schematic = ?1,
            requirements = ?2,
            unique_blocks = ?3
        WHERE schematic_id = ?4"#,
        params![
            schematic_json.to_string(),
            requirements_json.to_string(),
            unique_blocks_json.to_string(),
            schematic_id
        ],
    )?;

    let changes = tx.changes();
    if changes == 0 {
        anyhow::bail!("未找到schematic_id: {}", schematic_id);
    }

    tx.commit()?;
    Ok(schematic_id)
}

fn build_updated_schematic(
    old_data: Option<&String>,
    new_entry: String
) -> Result<Value> {
    let new_value: Value = serde_json::from_str(&new_entry)
        .map_err(|e| anyhow::anyhow!("ERR JSON: {}", e))?;

    let mut array = match old_data {
        Some(s) => {
            let parsed: Value = serde_json::from_str(&s)
                .map_err(|e| anyhow::anyhow!("解析旧数据失败: {}", e))?;

            if parsed.is_array() {
                parsed.as_array().unwrap().clone()
            } else {
                anyhow::bail!("现有数据不是JSON数组")
            }
        }
        None => Vec::new(),
    };

    array.push(new_value);

    Ok(json!(array))
}

fn convert_to_array(input: String) -> Result<String> {
    let value: Value = serde_json::from_str(&input)?;

    match value {
        Value::Object(_) => Ok(json!([value]).to_string()),
        Value::Array(_) => Ok(json!([value]).to_string()),
        _ => anyhow::bail!("EER JSON<UNK>"),
    }
}

pub(crate) fn get_history_record(
    tx: &Transaction,
    schematic_id: i64
) -> Result<Option<HistoryRecord>> {
    let mut stmt = tx.prepare(
        r#"SELECT
            schematic,
            requirements,
            unique_blocks
        FROM schematics_history
        WHERE schematic_id = ?"#
    )?;

    let mut rows = stmt.query([schematic_id])?;

    if let Some(row) = rows.next()? {
        let record = HistoryRecord {
            schematic: row.get(0)?,
            requirements: row.get(1)?,
            unique_blocks: row.get(2)?,
        };
        Ok(Some(record))
    } else {
        Ok(None)
    }
}