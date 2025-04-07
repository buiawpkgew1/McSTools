use serde::{Deserialize, Serialize};
use thiserror::Error;
use std::io::{Error as IoError};
use crate::utils::block_state_pos_list::{BlockStatePosList};
use crate::utils::tile_entities::TileEntitiesList;
use serde_json::{Error as JsonError};
use regex::Error as RegexError;
use flate2::CompressError;
use flate2::DecompressError;

#[derive(Debug, Error)]
pub enum SchematicError {
    #[error("Root element is not a Compound")]
    RootNotCompound,
    #[error("IO error: {0}")]
    Io(#[from] IoError),
    #[error("JsonError error: {0}")]
    Json(#[from] JsonError),
    #[error("NBT error: {0}")]
    Nbt(#[from] fastnbt::error::Error),
    #[error("SNBT error: {0}")]
    SNbt(#[from] fastsnbt::error::Error),
    #[error("Invalid data format: {0}")]
    InvalidFormat(&'static str),
    #[error("Base64 err: {0}")]
    Base64(#[from] base64::DecodeError),
    #[error("GZIP err: {0}")]
    GzipCompress(#[from] CompressError),
    #[error("GZIP err: {0}")]
    GzipDecompress(#[from] DecompressError),
    #[error("regex err: {0}")]
    Regex(#[from] RegexError),
    #[error("Type")]
    TypeMismatch {
        expected: &'static str,
        actual: String
    },
    #[error("Missing required field: {0}")]
    MissingField(String),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Size {
    pub(crate) width: i32,
    pub(crate) height: i32,
    pub(crate) length: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchematicData {
    pub blocks: BlockStatePosList,
    pub tile_entities_list: TileEntitiesList,
    pub size: Size
}

impl SchematicData {
    pub fn new(blocks: BlockStatePosList, tile_entities_list: TileEntitiesList, size: Size) -> Self {
        Self { blocks, tile_entities_list, size }
    }
}