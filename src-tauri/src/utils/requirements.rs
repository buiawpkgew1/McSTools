use crate::utils::block_state_pos_list::{BlockId, BlockStatePosList};
use crate::utils::minecraft_data::je_blocks_data::BlocksData;
use crate::utils::schematic_data::SchematicError;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Requirements {
    requirements: HashMap<BlockId, i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct BlockData {
    id: String,
    zh_cn: String,
    num: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RequirementStr {
    requirements: HashMap<BlockId, BlockData>,
}
impl Requirements {
    pub fn new() -> Self {
        Self {
            requirements: HashMap::new(),
        }
    }

    pub fn set_requirement(&mut self, key: BlockId, value: i32) {
        self.requirements.insert(key, value);
    }

    pub fn modify_requirement(&mut self, key: &BlockId, new_value: i32) {
        if let Some(v) = self.requirements.get_mut(key) {
            *v = new_value;
        } else {
            eprintln!("Requirement not found: {:?}", key);
        }
    }

    pub fn add_requirement(&mut self, key: BlockId, value: i32) {
        self.requirements
            .entry(key)
            .and_modify(|v| *v += value)
            .or_insert(value);
    }

    pub fn get_requirement(&self, key: &BlockId) -> Option<&i32> {
        self.requirements.get(key)
    }

    pub fn get_requirements(&self) -> &HashMap<BlockId, i32> {
        &self.requirements
    }

    pub fn export_to_string(&self) -> String {
        serde_json::to_string(&self.requirements).unwrap_or_default()
    }
}

impl RequirementStr {
    pub fn new() -> Self {
        Self {
            requirements: HashMap::new(),
        }
    }

    pub fn upsert(&mut self, block_id: BlockId, data: BlockData) {
        self.requirements.insert(block_id, data);
    }

    pub fn get(&self, block_id: &BlockId) -> Option<&BlockData> {
        self.requirements.get(block_id)
    }

    pub fn get_all(&self) -> &HashMap<BlockId, BlockData> {
        &self.requirements
    }

    pub fn export_to_string(&self) -> Result<String, SchematicError> {
        let converted: HashMap<String, &BlockData> = self
            .requirements
            .iter()
            .map(|(k, v)| (k.name.to_string(), v))
            .collect();

        serde_json::to_string(&converted).map_err(SchematicError::Json)
    }

    pub fn from_requirements(req: &Requirements, data: &BlocksData) -> Self {
        let mut map = HashMap::new();

        for (block_id, &count) in req.get_requirements() {
            let zh_cn = data
                .get_zh_cn(&block_id.name.replace("minecraft:", ""))
                .map(|s| s.to_owned())
                .unwrap_or_else(|| block_id.name.to_string());

            map.insert(
                block_id.clone(),
                BlockData {
                    id: block_id.name.to_string(),
                    zh_cn,
                    num: count as i64,
                },
            );
        }

        Self { requirements: map }
    }

    pub fn par_iter(&self) -> rayon::collections::hash_map::Iter<'_, BlockId, BlockData> {
        self.requirements.par_iter()
    }
}

pub fn get_requirements(blocks: &BlockStatePosList) -> Result<Requirements, SchematicError> {
    let lava = Arc::from("minecraft:lava");
    let water = Arc::from("minecraft:water");
    let excluded_blocks: HashSet<Arc<str>> = [
        Arc::from("minecraft:air"),
        Arc::from("minecraft:cave_air"),
        Arc::from("minecraft:void_air"),
        Arc::from("minecraft:piston_head"),
    ].into_iter().collect();
    let requirements_map = blocks
        .elements
        .par_iter()
        .fold(
            || HashMap::new(),
            |mut acc, block| {
                let data = Arc::as_ref(&block.block);
                if data.id.name == lava || data.id.name == water {
                    let palette = data.properties.get("level");
                    if let Some(palette) = palette {
                        if palette.to_string()  != "0" { return acc };
                    }
                }
                if excluded_blocks.contains(&data.id.name) {
                    return acc;
                }
                let mut block_id = data.id.clone();
                if data.id.name.to_string().contains("wall_") {
                    let renamed_name = data.id.name.replace("wall_", "");

                    block_id = BlockId {
                        name: Arc::from(renamed_name),
                    };
                }
                if data.id.name.to_string() == "minecraft:redstone_wire" {
                    block_id = BlockId {
                        name: Arc::from("minecraft:redstone"),
                    };
                }
                if data.id.name.to_string() == "minecraft:big_dripleaf_stem" {
                    block_id = BlockId {
                        name: Arc::from("minecraft:big_dripleaf"),
                    };
                }

                *acc.entry(block_id).or_insert(0) += 1;
                acc
            },
        )
        .reduce(
            || HashMap::new(),
            |mut a, b| {
                for (k, v) in b {
                    *a.entry(k).or_insert(0) += v;
                }
                a
            },
        );

    Ok(Requirements {
        requirements: requirements_map,
    })
}
