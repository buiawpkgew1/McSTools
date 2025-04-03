use std::fs;
use serde_json::Value as JsonValue;
use crate::utils::schematic_data::SchematicError;

#[derive(Debug)]
pub struct BgSchematic {
    json: String,
}

impl BgSchematic {
    pub fn read_json_file(path: &str) -> Result<String, std::io::Error> {
        fs::read_to_string(path)
    }

    pub fn parse_json(json_str: &str) -> Result<JsonValue, serde_json::Error> {
        serde_json::from_str(json_str)
    }

    pub fn new(file_path: &str) -> Result<Self, SchematicError> {
        let json_str = fs::read_to_string(file_path)
            .map_err(|e| SchematicError::Io(e))?;
        Ok(Self { json: json_str })
    }

    pub fn get_type(&self) -> Result<i32, SchematicError> {
        let json_data: JsonValue = BgSchematic::parse_json(&self.json)
            .map_err(|e| SchematicError::InvalidFormat("parse err"))?;
        println!("{:?}", json_data);
        if let Some(list) = json_data.get("statePosArrayList") {
            return Ok(0);
        }
        if json_data.get("body").is_some() {
            return Ok(1);
        }

        if let Some(map) = json_data.get("mapIntState") {
            return Ok(2);
        }

        Err(SchematicError::MissingField("notfound".into()))
    }
}