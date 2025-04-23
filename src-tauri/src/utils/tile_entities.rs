use crate::utils::block_state_pos_list::BlockPos;
use fastnbt::Value;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TileEntities {
    pos: BlockPos,
    nbt: Value,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TileEntitiesList {
    elements: Vec<TileEntities>,
}

impl TileEntitiesList {
    pub fn new() -> Self {
        Self::default()
    }
}
