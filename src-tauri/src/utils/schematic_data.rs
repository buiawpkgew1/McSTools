use serde::{Deserialize, Serialize};
use thiserror::Error;
use std::io::{Error as IoError};
use crate::utils::block_state_pos_list::{BlockId, BlockStatePosList};
use crate::utils::tile_entities::TileEntitiesList;

#[derive(Debug, Error)]
pub enum SchematicError {
    #[error("Root element is not a Compound")]
    RootNotCompound,
    #[error("IO error: {0}")]
    Io(#[from] IoError),
    #[error("NBT error: {0}")]
    Nbt(#[from] fastnbt::error::Error),
    #[error("Invalid data format: {0}")]
    InvalidFormat(&'static str),
    #[error("Type")]
    TypeMismatch {
        expected: &'static str,
        actual: String
    },
    #[error("Missing required field: {0}")]
    MissingField(String),
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchematicData {
    blocks: BlockStatePosList,
    tile_entities_list: TileEntitiesList,
}

impl SchematicData {
    pub fn new(blocks: BlockStatePosList, tile_entities_list: TileEntitiesList) -> Self {
        Self { blocks, tile_entities_list }
    }
}