use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use zip::write::FileOptions;
use zip::ZipWriter;
use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

pub fn compress_files(files: &[String], output_path: &str, base_path: &str) -> Result<()> {
    let file = File::create(output_path)?;
    let zip = Arc::new(Mutex::new(ZipWriter::new(file)));
    let options = FileOptions::default().compression_method(zip::CompressionMethod::Deflated);
    let base = Path::new(base_path).canonicalize()?;

    let pb = ProgressBar::new(files.len() as u64);
    pb.set_style(ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg}")?
        .progress_chars("=>-"));

    let total_original = Arc::new(Mutex::new(0u64));

    files.par_iter().for_each(|file_path| {
        if let Ok(path) = Path::new(file_path).canonicalize() {
            if let Ok(name) = path.strip_prefix(&base) {
                let name_str = name.to_string_lossy().replace('\\', "/");

                if let Ok(metadata) = std::fs::metadata(&path) {
                    *total_original.lock().unwrap() += metadata.len();
                }

                if let Ok(mut f) = File::open(&path) {
                    let mut buffer = Vec::new();
                    if f.read_to_end(&mut buffer).is_ok() {
                        let mut zip_lock = zip.lock().unwrap();
                        if zip_lock.start_file(name_str.clone(), options).is_ok() {
                            let _ = zip_lock.write_all(&buffer);
                            pb.set_message(name_str);
                            pb.inc(1);
                        }
                    }
                }
            }
        }
    });

    pb.finish_with_message("Done");

    let mut zip_lock = zip.lock().unwrap();
    let zip_file = zip_lock.finish()?;
    let total_compressed = zip_file.metadata()?.len();
    let total_orig = *total_original.lock().unwrap();

    println!("\nCompression Summary:");
    println!("  Original size: {} bytes", total_orig);
    println!("  Compressed size: {} bytes", total_compressed);
    if total_orig > 0 {
        println!("  Compression ratio: {:.1}%",
            (1.0 - total_compressed as f64 / total_orig as f64) * 100.0);
    }

    Ok(())
}
