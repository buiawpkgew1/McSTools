use std::fs;
use serde::{Deserialize, Serialize};
use crate::utils::minecraft_data::je_blocks_data::BlocksData;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RawBlock {
    bid: u32,
    bname: String,
    bname_eng: String,
    bclass: Vec<RawBlocksClass>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RawBlocksClass {
    name: String,
    low: String,
    normal: String,
    high: String,
    lowest: String,
    x: u32,
    y: u32,
    offset: String,
    name_eng: String,
    low_rgb: Vec<u32>,
    high_rgb: Vec<u32>,
    lowest_rgb: Vec<u32>,
}

impl RawBlock {
    pub fn new() -> anyhow::Result<Vec<RawBlock>> {
        let path = "./data/blocks_art.json";
        let str = fs::read_to_string(path)?;
        let raw_block: Vec<RawBlock> = serde_json::from_str(str.as_str())?;
        Ok(raw_block)
    }
}