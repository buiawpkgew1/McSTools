use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
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
