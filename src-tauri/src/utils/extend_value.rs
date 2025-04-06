use std::collections::HashMap;
use fastnbt::{ByteArray, IntArray, LongArray, Value};
use crate::utils::block_state_pos_list::BlockPos;
use crate::utils::schematic_data::SchematicError;

pub trait NbtExt {
    fn get_compound(&self, key: &str) -> Result<&HashMap<String, Value>, SchematicError>;
    fn get_str(&self, key: &str) -> Result<&String, SchematicError>;
    fn get_list(&self, key: &str) -> Result<&Vec<Value>, SchematicError>;
    fn get_i16(&self, key: &str) -> Result<i16, SchematicError>;
    fn get_i32(&self, key: &str) -> Result<i32, SchematicError>;
    fn get_i64(&self, key: &str) -> Result<i64, SchematicError>;
    fn get_long_array(&self, key: &str) -> Result<&LongArray, SchematicError>;
    fn get_i32_array(&self, key: &str) -> Result<&IntArray, SchematicError>;
    fn get_i8_array(&self, key: &str) -> Result<&ByteArray, SchematicError>;
    fn get_pos(&self, key: &str) -> Result<BlockPos, SchematicError>;
    fn get_pos_t2(&self, key: &str) -> Result<BlockPos, SchematicError>;
}

impl NbtExt for HashMap<String, Value> {
    fn get_compound(&self, key: &str) -> Result<&HashMap<String, Value>, SchematicError> {
        let key_owned = key.to_string();
        self.get(key)
            .and_then(|v| match v {
                Value::Compound(c) => Some(c),
                _ => None,
            })
            .ok_or_else(move || SchematicError::MissingField(key_owned))
    }

    fn get_str(&self, key: &str) -> Result<&String, SchematicError> {
        self.get(key)
            .and_then(|v| match v {
                Value::String(n) => Some(n),
                _ => None
            })
            .ok_or_else(|| SchematicError::MissingField(key.into()))
    }

    fn get_list(&self, key: &str) -> Result<&Vec<Value>, SchematicError> {
        self.get(key)
            .and_then(|v| match v {
                Value::List(n) => Some(n),
                _ => None
            })
            .ok_or_else(|| SchematicError::MissingField(key.into()))
    }

    fn get_i16(&self, key: &str) -> Result<i16, SchematicError> {
        self.get(key)
            .and_then(|v| match v {
                Value::Short(n) => Some(*n),
                _ => None
            })
            .ok_or_else(|| SchematicError::MissingField(key.into()))
    }

    fn get_i32(&self, key: &str) -> Result<i32, SchematicError> {
        self.get(key)
            .and_then(|v| match v {
                Value::Int(n) => Some(*n),
                _ => None
            })
            .ok_or_else(|| SchematicError::MissingField(key.into()))
    }

    fn get_i64(&self, key: &str) -> Result<i64, SchematicError> {
        self.get(key)
            .and_then(Value::as_i64)
            .ok_or_else(|| SchematicError::MissingField(key.into()))
    }

    fn get_long_array(&self, key: &str) -> Result<&LongArray, SchematicError> {
        self.get(key)
            .and_then(|v| match v {
                Value::LongArray(n) => Some(n),
                _ => None
            })
            .ok_or_else(|| SchematicError::MissingField(key.into()))
    }

    fn get_i32_array(&self, key: &str) -> Result<&IntArray, SchematicError> {
        self.get(key)
            .and_then(|v| match v {
                Value::IntArray(n) => Some(n),
                _ => None
            })
            .ok_or_else(|| SchematicError::MissingField(key.into()))
    }

    fn get_i8_array(&self, key: &str) -> Result<&ByteArray, SchematicError> {
        self.get(key)
            .and_then(|v| match v {
                Value::ByteArray(n) => Some(n),
                _ => None
            })
            .ok_or_else(|| SchematicError::MissingField(key.into()))
    }

    fn get_pos(&self, key: &str) -> Result<BlockPos, SchematicError> {
        let compound = self.get_compound(key)?;
        Ok(BlockPos {
            x: compound.get_i32("x")?,
            y: compound.get_i32("y")?,
            z: compound.get_i32("z")?,
        })
    }

    fn get_pos_t2(&self, key: &str) -> Result<BlockPos, SchematicError> {
        let compound = self.get_compound(key)?;
        Ok(BlockPos {
            x: compound.get_i32("X")?,
            y: compound.get_i32("Y")?,
            z: compound.get_i32("Z")?,
        })
    }
}