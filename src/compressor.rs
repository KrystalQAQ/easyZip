use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use zip::write::FileOptions;
use zip::ZipWriter;
use anyhow::Result;

pub fn compress_files(files: &[String], output_path: &str, base_path: &str) -> Result<()> {
    let file = File::create(output_path)?;
    let mut zip = ZipWriter::new(file);
    let options = FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    for file_path in files {
        let path = Path::new(file_path);
        let name = path.strip_prefix(base_path).unwrap_or(path);

        zip.start_file(name.to_string_lossy().to_string(), options)?;

        let mut f = File::open(file_path)?;
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer)?;
        zip.write_all(&buffer)?;

        println!("Added: {}", name.display());
    }

    zip.finish()?;
    Ok(())
}
