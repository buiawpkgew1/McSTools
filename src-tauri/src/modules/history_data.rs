use std::collections::HashMap;
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::database::db_data::Schematic;
use crate::utils::block_state_pos_list::BlockData;
use crate::utils::requirements::Requirements;

#[derive(Debug)]
pub struct HistoryRecord {
    pub schematic: String,
    pub requirements: String,
    pub unique_blocks: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct History {
    pub schematic: HashMap<i64, Schematic>,
    pub requirements: HashMap<i64, Requirements>,
    pub unique_blocks: HashMap<i64, Vec<Arc<BlockData>>>,
}

impl History {
    pub fn to_json_str(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}