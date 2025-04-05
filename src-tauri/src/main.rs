// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::utils::schematic_data::SchematicError;
use std::time::Instant;
use sysinfo::{System, Pid, Process, ProcessesToUpdate};
use crate::building_gadges::bg_schematic::BgSchematic;
use crate::litematica::lm_schematic::LmSchematic;
use crate::word_edit::var_int_iterator::VarIntIterator;
use crate::word_edit::we_schematic::WeSchematic;

pub mod utils;
pub mod litematica;
pub mod word_edit;
pub mod create;
pub mod building_gadges;

fn main() -> Result<(), SchematicError> {
    //rust_lib::run()
    let mut sys = System::new_all();
    let pid = Pid::from(std::process::id() as usize);

    sys.refresh_processes(ProcessesToUpdate::All, false);
    let start_mem = sys.process(pid)
        .map(|p| p.memory())
        .unwrap_or(0);
    let start_time = Instant::now();

    //let schematic2 = LmSchematic::new("./schematic/36fbf6f4-5f07-4370-b4c5-cefdb12c4b92.litematic")?;
    //let schem2 = schematic2.get_blocks_pos()?;
    //let mut schematic3 = WeSchematic::new("./schematic/3914ec1f-f457-428e-994f-957182d2c8c2.schem")?;
    //let schem3 = schematic3.get_blocks_pos();
    //println!("{:?}", schem3);
    //println!("{:?}", schem2);
    let schematic4 = BgSchematic::new("./schematic/384046fd-ac85-4d97-bfca-0d2d41482cab_type2.json")?;
    let json = schematic4.get_blocks_pos();
    println!("{:?}", json);
    sys.refresh_processes(ProcessesToUpdate::All, false);
    let end_mem = sys.process(pid)
        .map(|p| p.memory())
        .unwrap_or(0);
    let duration = start_time.elapsed();

    println!("执行时间: {:.2} 秒", duration.as_secs_f64());
    println!("内存消耗: {} KB → {} KB (增量: {} KB)",
             start_mem / 1024,
             end_mem / 1024,
             (end_mem - start_mem) / 1024
    );

    Ok(())
}
