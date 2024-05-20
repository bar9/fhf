use std::fs;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use chrono::{Datelike, DateTime};
use git2::{Error, Repository};
use walkdir::DirEntry;
use anyhow::Result;
use regex::Regex;

pub fn already_has_annotation(path: &Path) -> bool {
    if let Ok(content) = fs::read_to_string(path) {
        let re = Regex::new(r"<\?php\s*\n\s*/\*\*").unwrap();
        re.is_match(&content)
    } else {
        false
    }
}

pub fn is_excluded(entry: &DirEntry, ignore: &Vec<String>) -> bool {
    entry.file_name()
        .to_str()
        .map(|s| ignore.iter().any(|i| s.contains(i)))
        .unwrap_or(false)
}

pub fn process_chunk(chunk: Vec<PathBuf>, root_path: &Path, suffix: &String) -> Result<()> {
    let repo = Repository::open(root_path)?;
    for file_path in chunk {
        let file_path= file_path.canonicalize().unwrap();
        println!("{:?}", file_path);
        let mut revwalk = repo.revwalk()?;
        revwalk.push_head()?;
        if !already_has_annotation(&file_path) {
            revwalk.set_sorting(git2::Sort::TIME | git2::Sort::REVERSE)?;
            for oid in revwalk {
                let commit_id = oid?;
                let commit = repo.find_commit(commit_id)?;

                let tree = commit.tree()?;
                let file_entry = tree.get_path(&file_path.strip_prefix(repo.workdir().unwrap()).unwrap());

                if let Ok(_) = file_entry {
                    let commit_author = commit.author();
                    let commit_time = commit.time();
                    if let Some(commit_date) = DateTime::from_timestamp(commit_time.seconds(), 0) {
                        let month = commit_date.month();
                        let year = commit_date.year();
                        let author_line = format!(" * @author {} / {}", commit_author.name().unwrap(), suffix);
                        let date_line = format!(" * @since {:02}/{}", month, year);
                        let lines = vec![
                            "/**",
                            &author_line,
                            " *",
                            &date_line,
                            " */",
                            "",
                        ];
                        let _ = add_lines_after_position(&file_path, 0, &lines)?;
                    }
                    break;
                }
            }
        }
    }
    Ok(())
}

pub fn walk_dir(path: &Path, extension: &str, ignore: &Vec<String>) -> Result<Vec<PathBuf>, Error> {
    Ok(
        walkdir::WalkDir::new(path).into_iter()
            .filter_entry(|e| !is_excluded(e, ignore))
            .filter_map(|e| e.ok())
            .filter(|entry| entry.path().extension().map_or(false, |ext| ext == extension))
            .map(|e| e.into_path())
            .collect()
    )
}


pub fn add_lines_after_position(file_path: &Path, position: usize, lines: &[&str]) -> std::io::Result<()> {
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

    let file = fs::File::create(file_path)?;
    let mut writer = BufWriter::new(file);
    for line in modified_content {
        writeln!(writer, "{}", line)?;
    }

    Ok(())
}