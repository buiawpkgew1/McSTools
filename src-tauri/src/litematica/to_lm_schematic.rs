use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;
use std::sync::atomic::{AtomicI32, AtomicU64, Ordering};
use fastnbt::{Value};
use fastnbt::Value::Compound;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use crate::utils::block_state_pos_list::{BlockData, BlockId, BlockPos, BlockStatePos};
use crate::utils::schematic_data::SchematicData;
use rayon::iter::IndexedParallelIterator;
#[derive(Debug)]
pub struct ToLmSchematic {
    blocks: Vec<BlockStatePos>,
    pub start_pos: BlockPos,
    end_pos: BlockPos,
    width: i32,
    height: i32,
    length: i32,
    bits: i32,
    pub unique_block_states: Vec<Arc<BlockData>>,
    pub block_state_to_index: HashMap<Arc<BlockData>, usize>,
}

impl ToLmSchematic {
    pub fn new(schematic: &SchematicData) -> Self {
        let mut block_list = schematic.blocks.clone();

        let (min, max) = {
            let elements = &block_list.elements;
            if elements.is_empty() {
                panic!("Block list cannot be empty");
            }

            let init = (
                BlockPos { x: i32::MAX, y: i32::MAX, z: i32::MAX },
                BlockPos { x: i32::MIN, y: i32::MIN, z: i32::MIN },
            );

            let local_pairs: Vec<(BlockPos, BlockPos)> = elements.par_iter()
                .fold(
                    || init,
                    |(mut min, mut max), bp| {
                        min.x = min.x.min(bp.pos.x);
                        min.y = min.y.min(bp.pos.y);
                        min.z = min.z.min(bp.pos.z);
                        max.x = max.x.max(bp.pos.x);
                        max.y = max.y.max(bp.pos.y);
                        max.z = max.z.max(bp.pos.z);
                        (min, max)
                    },
                )
                .collect();

            let (global_min, global_max) = local_pairs.into_iter().fold(
                init,
                |(mut global_min, mut global_max), (local_min, local_max)| {
                    global_min.x = global_min.x.min(local_min.x);
                    global_min.y = global_min.y.min(local_min.y);
                    global_min.z = global_min.z.min(local_min.z);
                    global_max.x = global_max.x.max(local_max.x);
                    global_max.y = global_max.y.max(local_max.y);
                    global_max.z = global_max.z.max(local_max.z);
                    (global_min, global_max)
                },
            );

            (
                BlockPos {
                    x: global_min.x.saturating_sub(1),
                    y: global_min.y,
                    z: global_min.z.saturating_sub(1),
                },
                BlockPos {
                    x: global_max.x.saturating_add(1),
                    y: global_max.y,
                    z: global_max.z.saturating_add(1),
                },
            )
        };

        let air = Arc::new(BlockData {
            id: BlockId { name: Arc::from("minecraft:air") },
            properties: BTreeMap::new(),
        });
        for y in min.y..max.y {
            for z in min.z..max.z {
                block_list.add_to_first(min.x - 1, y, z, &air);
                block_list.add_to_first(max.x + 1, y, z, &air);
            }
        }
        for x in min.x..max.x {
            for y in min.y..max.y {
                block_list.add_to_first(x, y, min.z - 1, &air);
                block_list.add_to_first(x, y, max.z + 1, &air);
            }
        }
        for y in min.y..max.y {
            block_list.add_to_first(min.x - 1, y, min.z - 1, &air);
            block_list.add_to_first(min.x - 1, y, max.z + 1, &air);
            block_list.add_to_first(max.x + 1, y, min.z - 1, &air);
            block_list.add_to_first(max.x + 1, y, max.z + 1, &air);
        }

        let width = (max.x + 1) - (min.x - 1) + 1;
        let height = max.y - min.y + 1;
        let length = (max.z + 1) - (min.z - 1) + 1;

        let (unique_block_states, block_state_to_index) = {
            let mut seen = HashMap::new();
            let mut unique = Vec::new();
            let mut index_map = HashMap::new();

            for block_pos in &block_list.elements {
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
        let palette_size = unique_block_states.len() as i32;
        let adjusted = if palette_size == 0 {
            u32::MAX
        } else {
            palette_size.saturating_sub(1) as u32
        };
        let leading_zeros = adjusted.leading_zeros();
        let bits_unclamped = 32u32.saturating_sub(leading_zeros);
        let bits = (bits_unclamped as f64).max(2.0) as i32;
        let blocks = block_list.elements;
        Self {
            blocks,
            start_pos: min,
            end_pos: max,
            width,
            height,
            length,
            bits,
            unique_block_states,
            block_state_to_index,
        }
    }

    pub fn get_block_id_list(&self) -> Vec<i32> {
        let total_blocks = (self.length * self.width * self.height) as usize;

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
                    .unwrap_or(0);

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

    pub fn encode_block_states(&self) -> Vec<u64> {
        let state_ids = self.get_block_id_list();
        let bits = self.bits as usize;
        let total_bits = state_ids.len() * bits;
        let longs_needed = (total_bits + 63) / 64;

        let long_array: Vec<AtomicU64> = (0..longs_needed)
            .map(|_| AtomicU64::new(0))
            .collect();
        let long_array = Arc::new(long_array);

        state_ids.par_iter().enumerate().for_each(|(index, &state_id)| {
            let state = state_id as u64;
            let start_bit = index * bits;
            let start_long_index = start_bit / 64;
            let start_bit_offset = (start_bit % 64) as u32;
            let end_bit = start_bit + bits - 1;
            let end_long_index = end_bit / 64;

            let mask = (1u64).wrapping_shl(bits as u32).wrapping_sub(1);
            let masked_state = state & mask;

            let long_array = Arc::clone(&long_array);

            if start_long_index == end_long_index {
                let value = masked_state << start_bit_offset;
                long_array[start_long_index].fetch_or(value, Ordering::Relaxed);
            } else {
                let bits_in_first = 64 - start_bit_offset;
                let part1 = masked_state << start_bit_offset;
                let part2 = masked_state >> bits_in_first;

                long_array[start_long_index].fetch_or(part1, Ordering::Relaxed);
                if end_long_index < longs_needed {
                    long_array[end_long_index].fetch_or(part2, Ordering::Relaxed);
                }
            }
        });

        Arc::try_unwrap(long_array)
            .unwrap()
            .into_iter()
            .map(|a| a.into_inner())
            .collect()
    }

    pub fn lm_palette(&self) -> Value {
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

    pub fn lm_metadata(&self) -> Value {
        let mut metadata = HashMap::new();

        let mut enclosing_size = HashMap::new();
        enclosing_size.insert("x".to_string(), Value::Int(self.width));
        enclosing_size.insert("y".to_string(), Value::Int(self.height));
        enclosing_size.insert("z".to_string(), Value::Int(self.length));
        metadata.insert("EnclosingSize".to_string(), Compound(enclosing_size));

        metadata.insert("Description".to_string(), Value::String(
            "来自蓝图站www.mcschematic.top自动转换,不保留实体".to_string()
        ));
        metadata.insert("RegionCount".to_string(), Value::Int(1));
        metadata.insert("Name".to_string(), Value::String("null".to_string()));
        metadata.insert("Author".to_string(), Value::String("www.mcschematic.top".to_string()));

        Compound(metadata)
    }

    pub fn lm_regions(&self) -> Value {
        let mut regions = HashMap::new();
        let mut region:HashMap<String, Value> = HashMap::new();

        let encoded = self.encode_block_states();
        let long_array: Vec<i64> = encoded.iter().map(|&v| v as i64).collect();
        region.insert("BlockStates".to_string(), Value::LongArray(fastnbt::LongArray::new(long_array)));

        let mut position = HashMap::new();
        position.insert("x".to_string(), Value::Int(0));
        position.insert("y".to_string(), Value::Int(0));
        position.insert("z".to_string(), Value::Int(0));
        region.insert("Position".to_string(), Compound(position));

        let mut size = HashMap::new();
        size.insert("x".to_string(), Value::Int(self.width));
        size.insert("y".to_string(), Value::Int(self.height));
        size.insert("z".to_string(), Value::Int(self.length));
        region.insert("Size".to_string(), Compound(size));

        region.insert("BlockStatePalette".to_string(), self.lm_palette());

        regions.insert("null".to_string(), Compound(region));
        Compound(regions)
    }

    pub fn lm_schematic(&self, version:i32) -> Value {
        let mut nbt = HashMap::new();
        nbt.insert("MinecraftDataVersion".to_string(), Value::Int(3465));
        nbt.insert("Version".to_string(), Value::Int(version));
        let metadata = self.lm_metadata();
        nbt.insert("Metadata".to_string(), metadata);
        let regions = self.lm_regions();
        nbt.insert("Regions".to_string(), regions);
        nbt.insert("SubVersion".to_string(),Value::Int(1));
        Compound(nbt)
    }
}