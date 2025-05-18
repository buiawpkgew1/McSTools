use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockColorData {
    #[serde(rename = "left_top")]
    pub low: Vec<u8>,
    #[serde(rename = "center")]
    pub normal: Vec<u8>,
    #[serde(rename = "right_top")]
    pub high: Vec<u8>,
    #[serde(rename = "left_bottom")]
    pub lowest: Vec<u8>,
    #[serde(rename = "average_rgb")]
    pub average: Vec<u8>,
    #[serde(rename = "average_rgb_hex")]
    pub average_hex: String,
    pub zh_cn: String,
}

pub type CategoryBlocks = HashMap<String, HashMap<String, BlockColorData>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapArtsData {
    #[serde(flatten)]
    pub categories: CategoryBlocks,
}

impl MapArtsData {
    pub fn new() -> Result<Self> {
        let path = "./data/blocks_art.json";
        let str = fs::read_to_string(path)?;
        let raw_block = serde_json::from_str(str.as_str())?;
        Ok(raw_block)
    }
}
