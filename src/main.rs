extern crate git2;
extern crate walkdir;

use std::path::{Path, PathBuf};
use chrono::Datelike;
use git2::{Repository, Error};
use std::fs;
use std::io::{BufRead, BufReader, BufWriter, Write};
use walkdir::DirEntry;
use rayon::prelude::*;


fn already_has_annotation(path: &Path) -> bool {
    if let Ok(content) = fs::read_to_string(path) {
        let template = "<?php\n/**\n * @author";
        content.starts_with(template)
    } else {
        false
    }
}

fn is_excluded(entry: &DirEntry) -> bool {
    entry.file_name()
        .to_str()
        .map(|s| s.contains(".idea") || s.contains("node_modules") || s.contains("vendor") || s.contains("var/cache"))
        .unwrap_or(false)
}

fn process_chunk(chunk: Vec<PathBuf>) -> Result<(), Error> {
    let repo = Repository::open(".")?;
    for file_path in chunk {
        let file_path= file_path.canonicalize().unwrap();
        let mut revwalk = repo.revwalk()?;
        revwalk.push_head()?;
        if !already_has_annotation(&file_path) {
            // println!("processing {:?}", &file_path);
            revwalk.set_sorting(git2::Sort::TIME | git2::Sort::REVERSE)?;
            for oid in revwalk {
                let commit_id = oid?;
                let commit = repo.find_commit(commit_id)?;

                let tree = commit.tree()?;
                let file_entry = tree.get_path(&file_path.strip_prefix(repo.workdir().unwrap()).unwrap());

                if let Ok(_) = file_entry {
                    let commit_author = commit.author();
                    let commit_time = commit.time();
                    let commit_date = chrono::NaiveDateTime::from_timestamp(commit_time.seconds(), 0);
                    let month = commit_date.month();
                    let year = commit_date.year();
                    let author_line = format!(" * @author {} / IWF Web Solutions", commit_author.name().unwrap());
                    let date_line = format!(" * @since {:02}/{}", month, year);
                    let lines = vec![
                        "/**",
                        &author_line,
                        " *",
                        &date_line,
                        " */",
                        "",
                    ];
                    add_lines_after_position(&file_path, 0, &lines);
                    break;
                }
            }
        }
    }
    Ok(())
}

fn walk_dir(path: &Path) -> Result<Vec<PathBuf>, Error> {
    Ok(
        walkdir::WalkDir::new(path).into_iter()
            .filter_entry(|e| !is_excluded(e))
            .filter_map(|e| e.ok())
            .filter(|entry| entry.path().extension().map_or(false, |ext| ext == "php"))
            .map(|e| e.into_path())
            .collect()
    )
    //
    // {
    // }
    //
    // Ok(())
}

fn main() -> Result<(), Error> {
    let root_path = Path::new(".");

    let paths = walk_dir(root_path)?;

    let chunk_size = 250;
    let path_chunks: Vec<_> = paths.chunks(chunk_size).collect();

    path_chunks.par_iter().for_each(|chunk| {
        process_chunk(chunk.to_vec()).expect("processing chunk failed");
    });

    Ok(())
}

fn add_lines_after_position(file_path: &Path, position: usize, lines: &[&str]) -> std::io::Result<()> {
    let file = fs::File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut content = Vec::new();
    for line in reader.lines() {
        content.push(line?);
    }

    let mut modified_content = Vec::new();
    for (idx, line) in content.iter().enumerate() {
        modified_content.push(line.clone());
        if idx == position {
            for new_line in lines {
                modified_content.push(new_line.to_string());
            }
        }
    }

    let mut file = fs::File::create(file_path)?;
    let mut writer = BufWriter::new(file);
    for line in modified_content {
        writeln!(writer, "{}", line)?;
    }

    Ok(())
}