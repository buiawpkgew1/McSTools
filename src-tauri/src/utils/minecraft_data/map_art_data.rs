use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockColorData {
    #[serde(rename = "low_rgb")]
    pub low: Vec<u8>,
    #[serde(rename = "normal_rgb")]
    pub normal: Vec<u8>,
    #[serde(rename = "high_rgb")]
    pub high: Vec<u8>,
    #[serde(rename = "lowest_rgb")]
    pub lowest: Vec<u8>,
    #[serde(rename = "average_rgb")]
    pub average: Vec<u8>,
}

pub type CategoryBlocks = HashMap<String, HashMap<String, BlockColorData>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockDatabase {
    #[serde(flatten)]
    pub categories: CategoryBlocks,
}

impl BlockDatabase {

    pub fn new() -> Result<Self> {
        let path = "./data/blocks_art.json";
        let str = fs::read_to_string(path)?;
        let raw_block = serde_json::from_str(str.as_str())?;
        Ok(raw_block)
    }
}
