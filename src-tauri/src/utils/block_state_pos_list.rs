use std::collections::{BTreeMap};
use serde::{Serialize, Deserialize};
use std::sync::Arc;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct BlockPos {
    pub(crate) x: i32,
    pub(crate) y: i32,
    pub(crate) z: i32,
}
#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct BlockId {
    pub(crate) name: Arc<str>,
}
#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct BlockData {
    pub(crate) id: BlockId,
    pub(crate) properties: BTreeMap<Arc<str>, Arc<str>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BlockStatePos {
    pub pos: BlockPos,
    pub block: Arc<BlockData>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct BlockStatePosList {
    pub elements: Vec<BlockStatePos>,
}

impl BlockStatePos {
    pub fn new(pos: BlockPos, block: Arc<BlockData>) -> Self {
        Self { pos, block }
    }
}

impl BlockStatePosList {
    pub fn add(&mut self, pos: BlockPos, block: Arc<BlockData>) {
        self.elements.push(BlockStatePos::new(pos, block));
    }

    pub fn merge(&mut self, other: Self) {
        self.elements.extend(other.elements);
    }

    pub fn add_by_pos(&mut self, x: i32, y:i32, z:i32, block: Arc<BlockData>) {
        self.elements.push(BlockStatePos::new(BlockPos{x, y, z}, block));
    }

    pub fn remove(&mut self, target: &BlockStatePos) -> bool {
        if let Some(index) = self.elements.iter().position(|x| x == target) {
            self.elements.remove(index);
            true
        } else {
            false
        }
    }

    pub fn remove_by_index(&mut self, index: usize) -> Option<BlockStatePos> {
        if index < self.elements.len() {
            Some(self.elements.remove(index))
        } else {
            None
        }
    }

}