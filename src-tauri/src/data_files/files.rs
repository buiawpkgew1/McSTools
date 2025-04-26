use std::collections::HashMap;
use crate::utils::schematic_data::SchematicError;
use anyhow::Result;
use anyhow::{anyhow, Context};
use fastnbt::Value;
use flate2::read::GzDecoder;
use std::fs;
use std::fs::File;
use std::io;
use std::io::{Cursor, Write};
use std::path::{Path, PathBuf};
use tauri::{AppHandle, Manager};
use crate::modules::convert_data::{ConvertData, SchematicType, Target};

#[derive(Debug)]
pub struct FileData {
    pub version: i32,
    pub v_type: i32,
    pub sub_type: i32,
}

#[derive(Debug)]
pub struct FileManager {
    data_dir: PathBuf,
}

impl FileManager {
    pub fn new(app: &AppHandle) -> Result<FileManager> {
        let data_dir = app
            .path()
            .app_data_dir()
            .context("无法获取应用数据目录")?
            .join("data")
            .join("schematic");
        if !data_dir.exists() {
            fs::create_dir_all(&data_dir).context("创建配置目录失败")?;
        }
        Ok(Self { data_dir })
    }
    pub fn schematic_dir(&self, id: i64) -> Result<PathBuf> {
        let schematic_dir = self.data_dir.join(format!("schematic-{}", id));
        if !schematic_dir.exists() {
            fs::create_dir_all(&schematic_dir).context("创建配置目录失败")?;
        }
        Ok(schematic_dir)
    }
    pub fn save_schematic(
        &self,
        id: i64,
        mut file: File,
        version: i32,
        sub_version: i32,
        v_type: i32,
        file_ext: String,
    ) -> Result<PathBuf> {
        let schematic_dir = self.schematic_dir(id)?;
        let temp_file = schematic_dir.join(format!(
            "temp_{}_{}_{}.{}",
            version, sub_version, v_type, file_ext
        ));

        {
            let mut temp_file = File::create(&temp_file)
                .with_context(|| format!("创建临时文件失败: {}", temp_file.display()))?;

            io::copy(&mut file, &mut temp_file).with_context(|| "文件内容复制失败")?;
        }

        let final_filename = format!(
            "schematic_{}.{}.{}.{}",
            version, sub_version, v_type, file_ext
        );
        let final_path = schematic_dir.join(final_filename);

        fs::rename(&temp_file, &final_path).with_context(|| {
            format!(
                "重命名失败: {} → {}",
                temp_file.display(),
                final_path.display()
            )
        })?;
        Ok(final_path)
    }

    pub fn save_schematic_data(
        &self,
        id: i64,
        data: Vec<u8>,
        version: i32,
        sub_version: i32,
        v_type: i32,
        file_ext: String,
    ) -> Result<PathBuf> {
        let schematic_dir = self.schematic_dir(id)?;
        let temp_file = schematic_dir.join(format!(
            "temp_{}_{}_{}.{}",
            version, sub_version, v_type, file_ext
        ));

        {
            let mut temp_file = File::create(&temp_file)
                .with_context(|| format!("创建临时文件失败: {}", temp_file.display()))?;

            temp_file
                .write_all(&data)
                .with_context(|| "文件写入失败".to_string())?;
        }

        let final_filename = format!(
            "schematic_{}.{}.{}.{}",
            version, sub_version, v_type, file_ext
        );
        let final_path = schematic_dir.join(final_filename);

        fs::rename(&temp_file, &final_path).with_context(|| {
            format!(
                "重命名失败: {} → {}",
                temp_file.display(),
                final_path.display()
            )
        })?;
        Ok(final_path)
    }

    pub fn read_schematic_str(
        &self,
        id: i64,
        version: i32,
        sub_version: i32,
        v_type: i32,
    ) -> Result<String> {
        let schematic_dir = self.schematic_dir(id)?;
        let file_ext = match v_type {
            1 => "nbt",
            2 => "litematic",
            3 => "schem",
            4 => "json",
            5 => "mcstruct",
            _ => "unknown",
        };
        let filename = format!(
            "schematic_{}.{}.{}.{}",
            version, sub_version, v_type, file_ext
        );

        let file_path = schematic_dir.join(filename);

        let data = fs::read(&file_path)
            .with_context(|| format!("无法读取蓝图文件: {}", file_path.display()))?;
        match v_type {
            1 => {
                if data.len() > 8 * 1024 * 1024 {
                    return Ok(String::new());
                }
                let cursor = Cursor::new(data);
                let mut decoder = GzDecoder::new(cursor);

                let nbt: Value = fastnbt::from_reader(&mut decoder)?;
                let ser = fastsnbt::to_string(&nbt)?;
                Ok(ser)
            }
            2 => {
                if data.len() > 1 * 512 * 1024 {
                    return Ok(String::new());
                }
                let cursor = Cursor::new(data);
                let mut decoder = GzDecoder::new(cursor);

                let nbt: Value = fastnbt::from_reader(&mut decoder)?;
                let ser = fastsnbt::to_string(&nbt)?;
                Ok(ser)
            }
            3 => {
                if data.len() > 8 * 1024 * 1024 {
                    return Ok(String::new());
                }
                let cursor = Cursor::new(data);
                let mut decoder = GzDecoder::new(cursor);

                let nbt: Value = fastnbt::from_reader(&mut decoder)?;
                let ser = fastsnbt::to_string(&nbt)?;
                Ok(ser)
            }
            4 => {
                if data.len() > 8 * 1024 * 1024 {
                    return Ok(String::new());
                }
                let json_str = String::from_utf8(data)?;
                Ok(json_str)
            }
            5 => {
                if data.len() > 8 * 1024 * 1024 {
                    return Ok(String::new());
                }
                let nbt: Value = fastnbt::from_bytes(&data)?;
                let ser = fastsnbt::to_string(&nbt)?;
                Ok(ser)
            }
            _ => Err(anyhow!("UNK: {}", v_type)),
        }
    }

    pub fn get_convert_data(
        &self,
        id: i64,
        version: i32,
        main_sub_version: i32,
        v_type: i32,
    ) -> Result<ConvertData> {
        let schematic_dir = self.schematic_dir(id)?;
        let mut convert_data = ConvertData {
            schematic_type: SchematicType::from_code(v_type).unwrap_or(SchematicType::Create),
            schematic_type_id: v_type,
            sub_type: main_sub_version,
            version,
            size: String::new(),
            schematics: HashMap::new(),
        };

        for schematic_type in [
            SchematicType::Create,
            SchematicType::Litematic,
            SchematicType::Bg,
            SchematicType::We,
            SchematicType::Be,
        ] {
            let mut version_map = HashMap::new();

            for sub_v in schematic_type.get_sub_versions() {
                let filename = format!(
                    "schematic_{}.{}.{}.{}",
                    version,
                    sub_v,
                    schematic_type.type_id(),
                    schematic_type.file_extension()
                );
                let path = schematic_dir.join(&filename);

                if path.exists() {
                    let metadata = fs::metadata(&path)?;
                    version_map.insert(
                        sub_v,
                        Target {
                            has: true,
                            size: metadata.len().to_string(),
                            version,
                        },
                    );
                }
            }

            if !version_map.is_empty() {
                convert_data.schematics.insert(schematic_type, version_map);
            }
        }

        if let Some(versions) = convert_data.schematics.get(&convert_data.schematic_type) {
            convert_data.size = versions
                .get(&main_sub_version)
                .map(|t| t.size.clone())
                .unwrap_or_default();
        }

        Ok(convert_data)
    }
}
