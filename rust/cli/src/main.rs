use std::{ffi::OsString, fs, path::PathBuf};

use anyhow::bail;
use clap::{Parser, Subcommand};
use image::io::Reader as ImageReader;
use pjsekai_thumbnail_matcher::{
    extractor::extract_thumbnail_images, hasher::generate_thumbnail_phash,
};
use serde::Serialize;

#[derive(Parser)]
#[command(name = "Project Sekai Thumbnail Matcher")]
#[command(version = "0.1.0")]
#[command(about = "Tool for matching Project Sekai card thumbnails")]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generates pHashes for a folder of thumbnail images
    Hash {
        /// The folder containing the thumbnail images
        folder: PathBuf,

        /// The JSON file to put the pHashes into. Defaults to "$(pwd)/character_hashes.json"
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Extracts thumbnail images from a character list screenshot
    Extract {
        /// The character list screenshot
        screenshot: PathBuf,

        /// The folder to put the pHashes into. Defaults to "$(pwd)/output"
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
}

#[derive(Serialize)]
struct ThumbnailHash {
    filename: String,
    phash: String,
}

fn main() -> anyhow::Result<()> {
    match Args::parse().command {
        Commands::Hash { folder, output } => {
            hash(folder, output.unwrap_or("./character_hashes.json".into()))
        }
        Commands::Extract { screenshot, output } => {
            extract(screenshot, output.unwrap_or("./output".into()))
        }
    }
}

fn hash(folder: PathBuf, output: PathBuf) -> anyhow::Result<()> {
    if !folder.is_dir() {
        bail!("Could not open input folder {}", folder.display());
    }

    println!("Thumbnails Folder: {}", folder.display());
    println!("Output File: {}", output.display());
    println!();

    let mut hashes = Vec::new();

    for file_path in fs::read_dir(folder)? {
        let thumbnail_path = file_path?.path();
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
    std::fs::write(&output, serde_json::to_string(&hashes).unwrap())?;

    println!(
        "Successfully wrote {} pHashes to {}",
        hashes.len(),
        output.display()
    );

    Ok(())
}

fn extract(screenshot: PathBuf, output: PathBuf) -> anyhow::Result<()> {
    if !screenshot.is_file() {
        bail!(
            "Could not open character list screenshot {}",
            screenshot.display()
        );
    }

    if output.is_file() {
        bail!("Output must point to a (new) directory");
    }

    println!("Screenshot: {}", screenshot.display());
    println!("Output Directory: {}", output.display());
    println!();

    if !output.exists() {
        print!("Creating output directory {}...", output.display());
        std::fs::create_dir_all(&output)?;
        println!(" done");
    }

    let img_character_list = ImageReader::open(&screenshot)?.decode()?;
    let img_thumbnails = extract_thumbnail_images(&img_character_list);

    if img_thumbnails.is_empty() {
        println!("No card thumbnails found");
    } else {
        println!("Found {} card thumbnails", img_thumbnails.len());

        let filename_base = screenshot.file_stem().unwrap();

        for (i, img_thumbnail) in img_thumbnails.iter().enumerate() {
            let mut filename = OsString::from(filename_base);
            filename.push("-card-");
            filename.push(i.to_string());
            filename.push(".png");

            let filepath = output.join(filename);
            print!("Saving thumbnail {}...", filepath.display());
            img_thumbnail.save(&filepath)?;
            println!(" done");
        }
    }

    Ok(())
}
