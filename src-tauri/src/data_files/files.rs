use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::{Path, PathBuf};
use anyhow::Context;
use anyhow::Result;
use tauri::{AppHandle, Manager};

#[derive(Debug)]
pub struct FileData {
    pub version: i32,
    pub v_type: i32,
    pub sub_type: i32,
}

#[derive(Debug)]
pub struct FileManager{
    data_dir: PathBuf,
}

impl FileManager {
    pub fn new(app: &AppHandle) -> Result<FileManager> {
        let data_dir = app.path()
            .app_data_dir()
            .context("无法获取应用数据目录")?
            .join("data")
            .join("schematic");
        if !data_dir.exists() {
            fs::create_dir_all(&data_dir)
                .context("创建配置目录失败")?;
        }
        Ok(Self { data_dir })
    }
    pub fn schematic_dir(&self, id: i64) -> Result<PathBuf> {
        let schematic_dir = self.data_dir.join(format!("schematic-{}", id));
        if !schematic_dir.exists() {
            fs::create_dir_all(&schematic_dir)
                .context("创建配置目录失败")?;
        }
        Ok(schematic_dir)
    }
    pub fn save_schematic(&self, id: i64, mut file: File, version: i32, sub_version: i32, v_type: i32, file_ext: String) -> Result<PathBuf> {
        let schematic_dir = self.schematic_dir(id)?;
        let temp_file = schematic_dir.join(format!(
            "temp_{}_{}_{}.{}",
            version,
            sub_version,
            v_type,
            file_ext
        ));

        {
            let mut temp_file = File::create(&temp_file)
                .with_context(|| format!("创建临时文件失败: {}", temp_file.display()))?;

            io::copy(&mut file, &mut temp_file)
                .with_context(|| "文件内容复制失败")?;
        }

        let final_filename = format!(
            "schematic_{}.{}.{}.{}",
            version,
            sub_version,
            v_type,
            file_ext
        );
        let final_path = schematic_dir.join(final_filename);

        fs::rename(&temp_file, &final_path)
            .with_context(|| format!("重命名失败: {} → {}", temp_file.display(), final_path.display()
            ))?;
        Ok(final_path)
    }

    pub fn save_schematic_data(&self, id: i64, mut data: Vec<u8>, version: i32, sub_version: i32, v_type: i32, file_ext: String) -> Result<PathBuf> {
        let schematic_dir = self.schematic_dir(id)?;
        let temp_file = schematic_dir.join(format!(
            "temp_{}_{}_{}.{}",
            version,
            sub_version,
            v_type,
            file_ext
        ));

        {
            let mut temp_file = File::create(&temp_file)
                .with_context(|| format!("创建临时文件失败: {}", temp_file.display()))?;

            temp_file.write_all(&data)
                .with_context(|| "文件写入失败".to_string())?;
        }

        let final_filename = format!(
            "schematic_{}.{}.{}.{}",
            version,
            sub_version,
            v_type,
            file_ext
        );
        let final_path = schematic_dir.join(final_filename);

        fs::rename(&temp_file, &final_path)
            .with_context(|| format!("重命名失败: {} → {}", temp_file.display(), final_path.display()
            ))?;
        Ok(final_path)
    }
}