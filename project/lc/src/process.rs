use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::{Path, PathBuf},
};

use walkdir::{DirEntry, WalkDir};

use crate::{util::is_hidden_type_file, CodeLines};

// scan dir files
pub fn scan_dir_files(path: &str, excluded: Vec<PathBuf>, max_depth: usize) -> Vec<DirEntry> {
    let mut files: Vec<DirEntry> = Vec::new();

    let mut default_exclude_list: Vec<&str> = vec![
        "Cargo.lock",
        "target",
        "debug",
        "build",
        "deps",
        "node_modules",
        "bin",
    ];

    if !excluded.is_empty() {
        for path in excluded.iter() {
            if let Some(name) = path.file_name() {
                default_exclude_list.push(name.to_str().unwrap());
            }
        }
    }

    let filter_handler = |entry: &DirEntry| {
        if is_hidden_type_file(entry) {
            return false;
        }
        if entry.file_name().to_string_lossy().starts_with('.') {
            return false;
        }
        if entry.file_type().is_dir() {}

        if let Some(file_name) = entry.file_name().to_str() {
            if default_exclude_list.contains(&file_name) {
                return false;
            }
        }

        true
    };

    for entry in WalkDir::new(path)
        .min_depth(1)
        .max_depth(max_depth)
        .into_iter()
        .filter_entry(filter_handler)
    {
        let entry = match entry {
            Ok(entry) => entry,
            Err(err) => {
                eprintln!("Scanning directory error: {:?}", err);
                continue;
            }
        };
        // in exclude directories, skip current directory

        if entry.file_type().is_file() {
            files.push(entry);
        }
    }

    files
}

// count code lines
pub fn count_code_lines(path: &Path, language_type: &str) -> io::Result<CodeLines> {
    let file = match File::open(path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Opening file error: {:?}", err);
            return Err(err);
        }
    };
    let reader = BufReader::new(file);

    let mut code_lines = CodeLines::new(language_type.to_string());

    for line in reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(err) => {
                eprintln!(
                    "Reading line error in {:?}: {:?}",
                    path.to_string_lossy(),
                    err
                );
                return Err(err);
            }
        };

        // split command line and code linesargs.limit_deargs.limit_depth);
        code_lines.total_lines += 1;
        match line.trim() {
            l if l.is_empty() => code_lines.empty_lines += 1,
            l if l.starts_with("//") => code_lines.comment_lines += 1,
            _ => code_lines.code_lines += 1,
        }
    }

    Ok(code_lines)
}
