use std::collections::{BTreeMap, HashMap, VecDeque};
use std::sync::Arc;
use std::sync::atomic::{AtomicI32, Ordering};
use fastnbt::Value;
use fastsnbt::to_string;
use fastnbt::Value::Compound;
use rayon::iter::IntoParallelRefIterator;
use crate::utils::block_state_pos_list::{BlockData, BlockId, BlockPos, BlockStatePos};
use crate::utils::schematic_data::{SchematicData, SchematicError};
use rayon::iter::ParallelIterator;
use serde_json::{json, Value as JsonValue};
#[derive(Debug)]
pub struct ToBgSchematic {
    blocks: VecDeque<BlockStatePos>,
    start_pos: BlockPos,
    end_pos: BlockPos,
    width: i32,
    height: i32,
    length: i32,
    air_index: usize,
    pub unique_block_states: Vec<Arc<BlockData>>,
    pub block_state_to_index: HashMap<Arc<BlockData>, usize>,
}

impl ToBgSchematic {
    pub fn new(schematic: &SchematicData) -> Self {
        let block_list = schematic.blocks.clone();
        let blocks = schematic.blocks.clone().elements;
        if blocks.is_empty() {
            panic!("Block list cannot be empty");
        }
        let (min, max) = blocks.iter().fold(
            (
                BlockPos { x: i32::MAX, y: i32::MAX, z: i32::MAX },
                BlockPos { x: i32::MIN, y: i32::MIN, z: i32::MIN },
            ),
            |(mut min, mut max), bp| {
                min.x = min.x.min(bp.pos.x);
                min.y = min.y.min(bp.pos.y);
                min.z = min.z.min(bp.pos.z);
                max.x = max.x.max(bp.pos.x);
                max.y = max.y.max(bp.pos.y);
                max.z = max.z.max(bp.pos.z);
                (min, max)
            },
        );

        let width = max.x - min.x + 1;
        let height = max.y - min.y + 1;
        let length = max.z - min.z + 1;

        let (unique_block_states, block_state_to_index, air_index) = {
            let mut seen = HashMap::new();
            let mut unique = Vec::new();
            let mut index_map = HashMap::new();
            let air = Arc::new(BlockData {
                id: BlockId { name: Arc::from("minecraft:air") },
                properties: BTreeMap::new(),
            });

            for block_pos in &block_list.elements {
                let block_data = block_pos.block.clone();

                if !seen.contains_key(&block_data) {
                    let index = unique.len();
                    seen.insert(block_data.clone(), index);
                    unique.push(block_data.clone());
                    index_map.insert(block_data, index);
                }
            }

            if !seen.contains_key(&air) {
                let index = unique.len();
                seen.insert(air.clone(), index);
                unique.push(air.clone());
                index_map.insert(air.clone(), index);
            }

            let air_index = *seen.get(&air).unwrap();


            (unique, index_map, air_index)
        };

        Self {
            blocks,
            start_pos: min,
            end_pos: max,
            width,
            height,
            length,
            air_index,
            unique_block_states,
            block_state_to_index,
        }
    }

    pub fn bg_palette(&self) -> Value {
        let mut palette = Vec::new();

        for block in &self.unique_block_states {
            let mut compound = HashMap::new();
            compound.insert("Name".to_string(), Value::String(block.id.name.to_string()));

            if !block.properties.is_empty() {
                let mut props = HashMap::new();
                for (k, v) in &block.properties {
                    props.insert(k.to_string(), Value::String(v.to_string()));
                }
                compound.insert("Properties".to_string(), Compound(props));
            }

            palette.push(Compound(compound));
        }

        Value::List(palette)
    }

    pub fn get_block_id_list(&self) -> Vec<i32> {
        let total_blocks = (self.length * self.width * self.height) as usize;
        let air_index = self.air_index as i32;
        let atomic_block_list: Vec<AtomicI32> = (0..total_blocks)
            .map(|_| AtomicI32::new(0))
            .collect();
        let atomic_block_list = Arc::new(atomic_block_list);

        self.blocks.par_iter().for_each(|block| {
            let dx = block.pos.x - self.start_pos.x;
            let dy = block.pos.y - self.start_pos.y;
            let dz = block.pos.z - self.start_pos.z;

            let id = (dy * self.width * self.length)
                + (dz * self.width)
                + dx;

            if id >= 0 && (id as usize) < atomic_block_list.len() {
                let state_id = self.block_state_to_index
                    .get(&block.block)
                    .map(|v| *v as i32)
                    .unwrap_or(air_index);

                atomic_block_list[id as usize]
                    .store(state_id, Ordering::Relaxed);
            }
        });

        Arc::try_unwrap(atomic_block_list)
            .unwrap()
            .into_iter()
            .map(|atomic| atomic.into_inner())
            .collect()
    }

    pub fn state_pos_array_list(&self) -> Result<String, SchematicError>  {
        let mut region = HashMap::new();
        let int_array: Vec<i32> = self.get_block_id_list();
        region.insert("blockstatemap".to_string(), self.bg_palette());
        let mut end_pos = HashMap::new();
        end_pos.insert("X".to_string(), Value::Int(self.end_pos.x));
        end_pos.insert("Y".to_string(), Value::Int(self.end_pos.y));
        end_pos.insert("Z".to_string(), Value::Int(self.end_pos.z));
        region.insert("endpos".to_string(), Compound(end_pos));
        let mut start_pos = HashMap::new();
        start_pos.insert("X".to_string(), Value::Int(self.start_pos.x));
        start_pos.insert("Y".to_string(), Value::Int(self.start_pos.y));
        start_pos.insert("Z".to_string(), Value::Int(self.start_pos.z));
        region.insert("startpos".to_string(), Compound(start_pos));
        region.insert("statelist".to_string(), Value::IntArray(fastnbt::IntArray::new(int_array)));
        Ok(to_string(&region)?)
    }

    pub fn bg_schematic(&self) -> Result<JsonValue, SchematicError>  {
        let state_pos_array_list = self.state_pos_array_list()?;
        let dynamic_json = json!({
            "name": "null",
            "statePosArrayList": state_pos_array_list,

        });
        Ok(dynamic_json)
    }
}