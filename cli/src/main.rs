use std::path::PathBuf;

use anyhow::bail;
use clap::Parser;

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

    Ok(())
}
