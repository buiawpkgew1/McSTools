use std::collections::HashMap;
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::utils::block_state_pos_list::{BlockId, BlockStatePosList};
use crate::utils::schematic_data::SchematicError;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Requirements {
    requirements: HashMap<BlockId, i32>,
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

pub fn get_requirements(blocks: &BlockStatePosList) -> Result<Requirements, SchematicError> {
    let requirements_map = blocks.elements
        .par_iter()
        .fold(
            || HashMap::new(),
            |mut acc, block| {
                let data = Arc::as_ref(&block.block);
                let block_id = data.id.clone();
                *acc.entry(block_id).or_insert(0) += 1;
                acc
            }
        )
        .reduce(
            || HashMap::new(),
            |mut a, b| {
                for (k, v) in b {
                    *a.entry(k).or_insert(0) += v;
                }
                a
            }
        );

    Ok(Requirements {
        requirements: requirements_map
    })
}