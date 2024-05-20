extern crate git2;
extern crate walkdir;

use std::path::{Path};
use anyhow::Result;
use rayon::prelude::*;
use fhf::{process_chunk, walk_dir};
use clap::Parser;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path of the project root
    #[arg(short, long)]
    path: String,

    /// File extension (currently only "php" is implemented)
    #[arg(short, long)]
    extension: String,

    /// Suffix to name (e.g. company name)
    #[arg(short, long)]
    suffix: String,

    /// Directories to exclude
    #[arg(short, long, value_delimiter(','))]
    ignore: Vec<String>
}

fn main() -> Result<()> {
    let args = Args::parse();

    if args.extension != "php" {
        panic!{"!!! only PHP files are supported as of now !!!"}
    }

    let root_path = Path::new(&args.path);
    let suffix = args.suffix;
    let ignore = args.ignore;
    println!{"{:?}", ignore};

    let paths = walk_dir(root_path, &args.extension, &ignore)?;

    let chunk_size = 250;
    let path_chunks: Vec<_> = paths.chunks(chunk_size).collect();

    path_chunks.par_iter().for_each(|chunk| {
        process_chunk(chunk.to_vec(), &root_path, &suffix).expect("processing chunk failed");
    });

    Ok(())
}
