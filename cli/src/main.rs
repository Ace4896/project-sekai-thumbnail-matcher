use std::{fs, path::PathBuf};

use anyhow::bail;
use clap::Parser;
use image::io::Reader as ImageReader;

use pjsekai_thumbnail_matcher::hasher::generate_thumbnail_phash;

#[derive(Debug, Parser)]
#[command(name = "Project Sekai Thumbnail Matcher")]
#[command(version = "0.1.0")]
#[command(about = "Generate pHashes for matching Project Sekai card thumbnails")]
struct Args {
    /// The directory containing the thumbnail images
    dir: PathBuf,

    /// The JSON file to put the pHashes into. Defaults to "character_hashes.json"
    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let Args { dir, output } = Args::parse();

    if !dir.exists() {
        bail!("Could not open directory: {}", dir.display());
    }

    let output = output.unwrap_or("character_hashes.json".into());
    println!("Thumbnails Directory: {}", dir.display());
    println!("Output File: {}", output.display());
    println!();

    for entry in fs::read_dir(dir)? {
        let thumbnail_path = entry?.path();
        if !thumbnail_path.is_file() {
            continue;
        }

        if let Ok(img_thumbnail) = ImageReader::open(&thumbnail_path)?.decode() {
            println!("Generating pHash for {}", thumbnail_path.display());
            let phash = generate_thumbnail_phash(&img_thumbnail);
            println!(
                "Generated pHash for {}: {:#0x}",
                thumbnail_path.display(),
                &phash
            );
        } else {
            eprintln!("Unable to load thumbnail {}", thumbnail_path.display());
        }
    }

    Ok(())
}
