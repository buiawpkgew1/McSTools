use anyhow::{Context, Result};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Deserialize)]
struct RawBlock {
    v: u32,
    n: String,
    ID: String,
    #[serde(default)]
    zj: String,
    #[serde(default)]
    t: String,
    #[serde(default, rename = "oP")]
    o_p: String,
}

#[derive(Debug)]
pub struct SubData {
    pub zh_cn: String,
    pub block_name: String,
    pub block_id: String,
    pub version_map: HashMap<u32, String>,
}

#[derive(Debug)]
pub struct BlocksData {
    pub blocks: Vec<SubData>,
    pub block_to_cn: HashMap<String, String>,
}

impl BlocksData {
    pub fn new() -> Result<BlocksData> {
        let path = "./data/je_blocks.json";
        let str = fs::read_to_string(path)?;
        Ok(Self::parse(str.as_str())?)
    }
    pub fn parse(json: &str) -> Result<Self> {
        let raw_blocks: Vec<RawBlock> = serde_json::from_str(json)?;
        let mut blocks = Vec::new();
        let mut block_to_cn = HashMap::new();
        for raw in raw_blocks {
            let mut id_parts = raw.ID.split(',');

            let num_part = id_parts.next().context("err")?;
            let (_, num_str) = num_part.split_once('.').context("err")?;
            let block_id = num_str.replace('_', "").parse::<String>()?;

            let mut version_map = HashMap::new();
            let mut block_name = String::new();
            for part in id_parts {
                let (ver, name) = part.split_once('.').context("err")?;
                let version = ver.parse::<u32>()?;
                version_map.insert(version, name.to_string());
                block_name = name.to_string();
            }
            let zh_cn = raw.n.clone();
            block_to_cn.insert(block_name.clone(), zh_cn);
            blocks.push(SubData {
                zh_cn: raw.n,
                block_name,
                block_id,
                version_map,
            });
        }

        Ok(BlocksData {
            blocks,
            block_to_cn,
        })
    }

    pub fn get_zh_cn(&self, block_name: &str) -> Option<&str> {
        self.block_to_cn.get(block_name).map(|s| s.as_str())
    }
}
