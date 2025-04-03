use serde::{Deserialize, Serialize};
use crate::utils::block_state_pos_list::{BlockPos, BlockStatePos};
use fastnbt::{Value};

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