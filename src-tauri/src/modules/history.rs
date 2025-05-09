use crate::database::db_control::DatabaseState;
use crate::modules::modules_data::history_data::HistoryRecord;
use tauri::State;

#[tauri::command]
pub async fn get_history(
    db: State<'_, DatabaseState>,
    schematic_id: i64,
) -> anyhow::Result<HistoryRecord, String> {
    async move {
        let mut conn = db.0.get()?;
        let tx = conn.transaction()?;

        let old_record =
            crate::database::db_apis::history_api::get_history_record(&tx, schematic_id)
                .unwrap()
                .ok_or(anyhow::anyhow!("err"))?;
        Ok(old_record)
    }
    .await
    .map_err(|e: anyhow::Error| e.to_string())
}
