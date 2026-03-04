mod config;
mod scanner;
mod compressor;

use clap::Parser;
use chrono::Local;
use anyhow::Result;

#[derive(Parser)]
#[command(name = "easyzip")]
#[command(about = "High-performance backup tool for frontend projects")]
struct Cli {
    /// Source directory to backup
    source: String,

    /// Output zip file path
    #[arg(short, long)]
    output: Option<String>,

    /// Additional exclude patterns (comma-separated)
    #[arg(long)]
    exclude: Option<String>,

    /// Dry run - preview files without creating archive
    #[arg(long)]
    dry_run: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let mut config = config::Config::load_from_file(&cli.source)
        .unwrap_or_else(|| {
            println!("Auto-detecting project type...");
            config::Config::smart_detect(&cli.source)
        });

    if let Some(exclude) = cli.exclude {
        for pattern in exclude.split(',') {
            config.exclude_patterns.push(pattern.trim().to_string());
        }
    }

    println!("Scanning directory: {}", cli.source);
    let files = scanner::scan_directory(&cli.source, &config)?;
    println!("Found {} files to backup", files.len());

    if cli.dry_run {
        println!("\nDry run - files to be backed up:");
        for file in &files {
            println!("  {}", file);
        }
        return Ok(());
    }

    let output = cli.output.unwrap_or_else(|| {
        format!("backup_{}.zip", Local::now().format("%Y%m%d_%H%M%S"))
    });

    println!("Creating backup: {}", output);
    compressor::compress_files(&files, &output, &cli.source)?;

    println!("\nBackup completed successfully!");
    Ok(())
}
