use std::path::PathBuf;
use tauri::{path, AppHandle, Manager};
use std::fs;
use serde_json::{json, Value};
use anyhow::{Result, Context};

pub fn get_config_dir(app: &AppHandle) -> Result<PathBuf> {
    let path_resolver = app.path();

    let config_dir = path_resolver.app_config_dir()
        .context("无法获取配置目录")?
        .join("mc-blueprint-tool");

    if !config_dir.exists() {
        fs::create_dir_all(&config_dir)
            .context("创建配置目录失败")?;
    }
    Ok(config_dir)
}

pub fn init_config(app: &AppHandle) -> Result<()> {
    let config_file = get_config_dir(app)?.join("config.json");

    if !config_file.exists() {
        let default_config = json!({
            "theme": "dark",
            "auto_update": true,
            "max_history": 10,
            "workspace": {
                "last_project": "",
                "recent_files": []
            }
        });

        fs::write(
            &config_file,
            serde_json::to_string_pretty(&default_config)?
        ).context("写入默认配置失败")?;
    }

    Ok(())
}

pub fn read_config(app: &AppHandle) -> Result<Value> {
    let config_file = get_config_dir(app)?.join("config.json");
    let content = fs::read_to_string(config_file)
        .context("读取配置文件失败")?;

    serde_json::from_str(&content)
        .context("解析配置文件失败")
}


pub fn save_config(app: &AppHandle, config: Value) -> Result<()> {
    let config_file = get_config_dir(app)?.join("config.json");
    fs::write(
        &config_file,
        serde_json::to_string_pretty(&config)?
    ).context("保存配置失败")
}

#[tauri::command]
pub async fn get_config(app: AppHandle) -> Result<Value, String> {
    read_config(&app)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_config(
    app: AppHandle,
    new_config: Value
) -> Result<(), String> {
    save_config(&app, new_config)
        .map_err(|e| e.to_string())
}