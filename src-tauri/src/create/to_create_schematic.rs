use std::collections::HashMap;
use std::sync::Arc;
use fastnbt::Value;
use fastnbt::Value::Compound;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use crate::utils::block_state_pos_list::{BlockData, BlockPos, BlockStatePos};
use crate::utils::schematic_data::SchematicData;

#[derive(Debug)]
pub struct ToCreateSchematic {
    blocks: Vec<BlockStatePos>,
    start_pos: BlockPos,
    end_pos: BlockPos,
    width: i32,
    height: i32,
    length: i32,
    pub unique_block_states: Vec<Arc<BlockData>>,
    pub block_state_to_index: HashMap<Arc<BlockData>, usize>,
}

impl ToCreateSchematic {
    pub fn new(schematic: &SchematicData) -> Self {
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

        let (unique_block_states, block_state_to_index) = {
            let mut seen = HashMap::new();
            let mut unique = Vec::new();
            let mut index_map = HashMap::new();

            for block_pos in &blocks {
                let block_data = block_pos.block.clone();

                if !seen.contains_key(&block_data) {
                    let index = unique.len();
                    seen.insert(block_data.clone(), index);
                    unique.push(block_data.clone());
                    index_map.insert(block_data, index);
                }
            }

            (unique, index_map)
        };

        Self {
            blocks,
            start_pos: min,
            end_pos: max,
            width,
            height,
            length,
            unique_block_states,
            block_state_to_index,
        }
    }

    pub fn create_palette(&self) -> Value {
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

    pub fn create_blocks(&self) -> Value {
        let block_list: Vec<Value> = self.blocks
            .par_iter()
            .filter_map(|block_pos| {
                let data = (*block_pos.block).clone();
                if data.id.name.to_string() == "minecraft:air" {
                    return None;
                }

                let state_id = self.block_state_to_index.get(&data)
                    .expect("Block state not found in palette");

                let pos = Value::List(vec![
                    Value::Int(block_pos.pos.x - self.start_pos.x),
                    Value::Int(block_pos.pos.y - self.start_pos.y),
                    Value::Int(block_pos.pos.z - self.start_pos.z),
                ]);

                let mut block_tag = HashMap::new();
                block_tag.insert("state".to_string(), Value::Int(*state_id as i32));
                block_tag.insert("pos".to_string(), pos);

                Some(Compound(block_tag))
            })
            .collect();

        Value::List(block_list)
    }

    pub fn create_schematic(&self) -> Value {
        let mut tag = HashMap::new();

        let size = Value::List(vec![
            Value::Int(self.end_pos.x - self.start_pos.x + 1),
            Value::Int(self.end_pos.y - self.start_pos.y + 1),
            Value::Int(self.end_pos.z - self.start_pos.z + 1),
        ]);
        tag.insert("size".to_string(), size);
        tag.insert("blocks".to_string(), self.create_blocks());
        tag.insert("palette".to_string(), self.create_palette());
        tag.insert("DataVersion".to_string(), Value::Int(3465));

        Compound(tag)
    }
}