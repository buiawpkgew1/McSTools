use crate::utils::block_state_pos_list::BlockId;
use fastnbt::Value;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
struct Requirements {
    pub id: BlockId,
    pub nums: i32,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct JsonData {
    pub name: String,
    pub state_pos_array_list: String,
    pub requirements: Vec<Requirements>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BgSchematicData {
    pub type_version: i32,
    pub data: Value,
}
