use std::collections::HashMap;
use fastnbt::Value;
use serde::{Deserialize, Serialize};
use crate::utils::block_state_pos_list::{BlockData, BlockId, BlockPos};
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Requirements {
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
    pub size: BlockPos,
    pub state_list: Vec<i32>,
    pub block_state_map: Vec<Value>,
    pub required_items: Vec<Requirements>,
}
