use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use zip::write::FileOptions;
use zip::ZipWriter;
use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};

pub fn compress_files(files: &[String], output_path: &str, base_path: &str) -> Result<()> {
    let file = File::create(output_path)?;
    let mut zip = ZipWriter::new(file);
    let options = FileOptions::default().compression_method(zip::CompressionMethod::Deflated);
    let base = Path::new(base_path).canonicalize()?;

    let pb = ProgressBar::new(files.len() as u64);
    pb.set_style(ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg}")?
        .progress_chars("=>-"));

    let mut total_original = 0u64;
    let mut total_compressed = 0u64;

    for file_path in files {
        if let Ok(path) = Path::new(file_path).canonicalize() {
            if let Ok(name) = path.strip_prefix(&base) {
                let name_str = name.to_string_lossy().replace('\\', "/");

                if let Ok(metadata) = std::fs::metadata(&path) {
                    total_original += metadata.len();
                }

                match zip.start_file(name_str.clone(), options) {
                    Ok(_) => {
                        if let Ok(mut f) = File::open(&path) {
                            let mut buffer = [0; 8192];
                            loop {
                                match f.read(&mut buffer) {
                                    Ok(0) => break,
                                    Ok(n) => { zip.write_all(&buffer[..n])?; }
                                    Err(_) => break,
                                }
                            }
                            pb.set_message(name_str);
                            pb.inc(1);
                        }
                    }
                    Err(_) => continue,
                }
            }
        }
    }

    pb.finish_with_message("Done");

    let zip_file = zip.finish()?;
    total_compressed = zip_file.metadata()?.len();

    println!("\nCompression Summary:");
    println!("  Original size: {} bytes", total_original);
    println!("  Compressed size: {} bytes", total_compressed);
    println!("  Compression ratio: {:.1}%",
        (1.0 - total_compressed as f64 / total_original as f64) * 100.0);

    Ok(())
}
