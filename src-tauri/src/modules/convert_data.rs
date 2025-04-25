use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Target {
    pub has: bool,
    pub size: String,
    pub version: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvertData {
    pub schematic_type: SchematicType,
    pub schematic_type_id: i32,
    pub sub_type: i32,
    pub version: i32,
    pub size: String,
    pub schematics: HashMap<SchematicType, HashMap<i32, Target>>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum SchematicType {
    Create,
    Litematic,
    Bg,
    We,
    Be,
}

impl SchematicType {
    pub fn from_code(code: i32) -> Option<Self> {
        match code {
            1 => Some(Self::Create),
            2 => Some(Self::Litematic),
            3 => Some(Self::We),
            4 => Some(Self::Bg),
            5 => Some(Self::Be),
            _ => None,
        }
    }

    pub fn get_sub_versions(&self) -> Vec<i32> {
        match self {
            SchematicType::Bg => vec![-1, 0, 1, 2],
            SchematicType::We => vec![-1, 0, 1],
            _ => vec![-1],
        }
    }

    pub fn file_extension(&self) -> &'static str {
        match self {
            SchematicType::Create => "nbt",
            SchematicType::Litematic => "litematic",
            SchematicType::We => "schem",
            SchematicType::Bg => "json",
            SchematicType::Be => "mcstruct",
        }
    }
    pub fn type_id(&self) -> &'static i32 {
        match self {
            SchematicType::Create => &1,
            SchematicType::Litematic => &2,
            SchematicType::We => &3,
            SchematicType::Bg => &4,
            SchematicType::Be => &5,
        }
    }
}