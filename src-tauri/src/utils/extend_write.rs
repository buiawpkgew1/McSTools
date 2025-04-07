use std::fs::File;
use std::io::Write;
use flate2::Compression;
use flate2::write::GzEncoder;
use crate::utils::schematic_data::SchematicError;

pub fn to_writer_gzip(data: &impl serde::Serialize, output_path: &str) -> Result<(), SchematicError> {
    let file = File::create(output_path)?;

    let mut encoder = GzEncoder::new(file, Compression::default());
    let bytes = fastnbt::to_bytes(&data)?;
    encoder.write_all(&bytes)?;
    encoder.finish()?;

    Ok(())
}
