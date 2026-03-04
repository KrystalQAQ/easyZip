use walkdir::WalkDir;
use anyhow::Result;
use crate::config::Config;

pub fn scan_directory(path: &str, config: &Config) -> Result<Vec<String>> {
    let mut files = Vec::new();

    for entry in WalkDir::new(path).into_iter().filter_entry(|e| {
        !config.should_exclude(e.path().to_str().unwrap_or(""))
    }) {
        let entry = entry?;
        if entry.file_type().is_file() {
            files.push(entry.path().to_string_lossy().to_string());
        }
    }

    Ok(files)
}
