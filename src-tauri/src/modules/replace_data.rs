use serde::{Deserialize, Serialize};
use crate::utils::block_state_pos_list::BlockData;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplacementRule {
    pub schematic_id: i64,
    pub mode: i64,
    pub original_id: Option<String>,
    pub replacement_id: Option<String>,
    pub original_details: Option<BlockData>,
    pub replacement_details: Option<BlockData>,
    pub quantity: i64,
    pub global: bool,
}
#[derive(Debug, Clone)]
pub enum RuleMatcher {
    IdMatch {
        original: String,
        replacement: String,
    },
    FullMatch {
        original: BlockData,
        replacement: BlockData,
    },
}