use serde::{Deserialize, Serialize};
use crate::utils::block_state_pos_list::BlockData;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplacementRule {
    pub schematic_id: i64,
    pub mode: i64,
    pub original_id: String,
    pub replacement_id: String,
    pub original_details: BlockData,
    pub replacement_details: BlockData,
    pub quantity: i64,
    pub global: bool,
}