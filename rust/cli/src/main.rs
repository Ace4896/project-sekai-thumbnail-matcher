use std::{fs, path::PathBuf};

use anyhow::bail;
use clap::Parser;
use image::io::Reader as ImageReader;
use pjsekai_thumbnail_matcher::hasher::generate_thumbnail_phash;
use serde::Serialize;

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

#[derive(Serialize)]
struct ThumbnailHash {
    filename: String,
    phash: String,
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

    let mut hashes = Vec::new();

    for entry in fs::read_dir(dir)? {
        let thumbnail_path = entry?.path();
        if !thumbnail_path.is_file() {
            continue;
        }

        if let Ok(img_thumbnail) = ImageReader::open(&thumbnail_path)?.decode() {
            print!("Generating pHash for {}...", thumbnail_path.display());

            let phash = generate_thumbnail_phash(&img_thumbnail);
            hashes.push(ThumbnailHash {
                filename: thumbnail_path
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .to_string(),

                phash: phash.to_string(),
            });

            println!(" {:#0x}", &phash);
        } else {
            eprintln!("Unable to load thumbnail {}", thumbnail_path.display());
        }
    }

    hashes.sort_unstable_by(|a, b| a.filename.cmp(&b.filename));
    std::fs::write(output, serde_json::to_string(&hashes).unwrap())?;

    Ok(())
}
